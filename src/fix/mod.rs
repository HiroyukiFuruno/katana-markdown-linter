use crate::types::{FixDetail, FixResult, LintResult, Range};

pub(crate) struct FixApplicator;

impl FixApplicator {
    pub(crate) fn apply(results: &[LintResult], content: &str, include_unsafe: bool) -> FixResult {
        let mut applied_fixes = 0;
        let line_index = LineOffsetIndex::new(content);

        struct Edit<'a> {
            byte_start: usize,
            byte_end: usize,
            replacement: &'a str,
            rule_id: &'a str,
            range: Range,
        }

        let mut edits = results
            .iter()
            .filter_map(|result| {
                let fix = result.fix.as_ref()?;
                if fix.safety == crate::FixSafety::Unsafe && !include_unsafe {
                    return None;
                }
                let byte_start =
                    line_index.offset_for_position(fix.range.start_line, fix.range.start_column)?;
                let byte_end =
                    line_index.offset_for_position(fix.range.end_line, fix.range.end_column)?;
                if byte_start > byte_end {
                    return None;
                }
                Some(Edit {
                    byte_start,
                    byte_end,
                    replacement: fix.replacement.as_str(),
                    rule_id: result.rule_id.as_str(),
                    range: fix.range.clone(),
                })
            })
            .collect::<Vec<_>>();

        edits.sort_by(|left, right| {
            right
                .byte_start
                .cmp(&left.byte_start)
                .then_with(|| right.byte_end.cmp(&left.byte_end))
        });

        let mut accepted: Vec<Edit<'_>> = Vec::new();
        let mut skipped: Vec<Edit<'_>> = Vec::new();
        let mut previous_start = content.len();
        for edit in edits {
            if edit.byte_end > previous_start {
                skipped.push(edit);
                continue;
            }
            previous_start = edit.byte_start;
            applied_fixes += 1;
            accepted.push(edit);
        }
        accepted.reverse();

        let mut details: Vec<FixDetail> = accepted
            .iter()
            .map(|edit| FixDetail {
                rule_id: edit.rule_id.to_string(),
                range: edit.range.clone(),
                applied: true,
            })
            .collect();
        for edit in &skipped {
            details.push(FixDetail {
                rule_id: edit.rule_id.to_string(),
                range: edit.range.clone(),
                applied: false,
            });
        }

        let mut fixed = String::with_capacity(content.len());
        let mut cursor = 0;
        for edit in &accepted {
            fixed.push_str(&content[cursor..edit.byte_start]);
            fixed.push_str(edit.replacement);
            cursor = edit.byte_end;
        }
        fixed.push_str(&content[cursor..]);

        FixResult {
            content: fixed,
            applied_fixes,
            details,
        }
    }
}

struct LineOffsetIndex<'a> {
    content: &'a str,
    line_starts: Vec<usize>,
}

impl<'a> LineOffsetIndex<'a> {
    fn new(content: &'a str) -> Self {
        let mut line_starts = Vec::with_capacity(
            content
                .as_bytes()
                .iter()
                .filter(|byte| **byte == b'\n')
                .count()
                + 1,
        );
        line_starts.push(0);
        for (index, byte) in content.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(index + 1);
            }
        }
        Self {
            content,
            line_starts,
        }
    }

    fn offset_for_position(&self, line: usize, column: usize) -> Option<usize> {
        if line == 0 || column == 0 {
            return None;
        }

        let line_start = if let Some(line_start) = self.line_starts.get(line - 1) {
            *line_start
        } else if line == self.line_starts.len() + 1 && column == 1 {
            return Some(self.content.len());
        } else {
            return None;
        };

        let raw = line_start.saturating_add(column.saturating_sub(1));
        Some(previous_char_boundary(
            self.content,
            raw.min(self.content.len()),
        ))
    }
}

#[cfg(test)]
fn offset_for_position(content: &str, line: usize, column: usize) -> Option<usize> {
    LineOffsetIndex::new(content).offset_for_position(line, column)
}

fn previous_char_boundary(content: &str, mut offset: usize) -> usize {
    while offset > 0 && !content.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Fix, FixSafety, Range, Severity};

    fn result(rule_id: &str, range: Range, replacement: &str) -> LintResult {
        LintResult {
            rule_id: rule_id.to_string(),
            rule_name: String::new(),
            message: String::new(),
            message_id: "rule.generic".to_string(),
            message_params: crate::i18n::MessageCatalog::diagnostic_message_params(rule_id, "", ""),
            severity: Severity::Warning,
            line: range.start_line,
            column: range.start_column,
            end_line: range.end_line,
            end_column: range.end_column,
            fix: Some(Fix {
                range,
                replacement: replacement.to_string(),
                safety: FixSafety::Safe,
            }),
        }
    }

    #[test]
    fn applies_multi_line_fix_ranges() {
        let content = "# Title\n\n\nParagraph\n";
        let fixed = FixApplicator::apply(
            &[result(
                "MD012",
                Range {
                    start_line: 3,
                    start_column: 1,
                    end_line: 4,
                    end_column: 1,
                },
                "",
            )],
            content,
            false,
        );

        assert_eq!(fixed.content, "# Title\n\nParagraph\n");
        assert_eq!(fixed.applied_fixes, 1);
    }

    #[test]
    fn skips_overlapping_fix_ranges() {
        let content = "#Title\n";
        let fixed = FixApplicator::apply(
            &[
                result(
                    "A",
                    Range {
                        start_line: 1,
                        start_column: 2,
                        end_line: 1,
                        end_column: 2,
                    },
                    " ",
                ),
                result(
                    "B",
                    Range {
                        start_line: 1,
                        start_column: 1,
                        end_line: 1,
                        end_column: 3,
                    },
                    "## ",
                ),
            ],
            content,
            false,
        );

        assert_eq!(fixed.content, "# Title\n");
        assert_eq!(fixed.applied_fixes, 1);
    }

    #[test]
    fn fix_detail_applied_true_for_accepted_edit() {
        let content = "# Title\n\n\nParagraph\n";
        let range = Range {
            start_line: 3,
            start_column: 1,
            end_line: 4,
            end_column: 1,
        };
        let fixed = FixApplicator::apply(&[result("MD012", range.clone(), "")], content, false);

        assert_eq!(fixed.applied_fixes, 1);
        assert_eq!(fixed.details.len(), 1);
        assert_eq!(fixed.details[0].rule_id, "MD012");
        assert_eq!(fixed.details[0].range, range);
        assert!(fixed.details[0].applied);
    }

    #[test]
    fn fix_detail_applied_false_for_skipped_edit() {
        let content = "#Title\n";
        let fixed = FixApplicator::apply(
            &[
                result(
                    "A",
                    Range {
                        start_line: 1,
                        start_column: 2,
                        end_line: 1,
                        end_column: 2,
                    },
                    " ",
                ),
                result(
                    "B",
                    Range {
                        start_line: 1,
                        start_column: 1,
                        end_line: 1,
                        end_column: 3,
                    },
                    "## ",
                ),
            ],
            content,
            false,
        );

        assert_eq!(fixed.applied_fixes, 1);
        assert_eq!(fixed.details.len(), 2);
        let applied: Vec<_> = fixed.details.iter().filter(|d| d.applied).collect();
        let skipped: Vec<_> = fixed.details.iter().filter(|d| !d.applied).collect();
        assert_eq!(applied.len(), 1);
        assert_eq!(skipped.len(), 1);
        assert_eq!(
            fixed.applied_fixes,
            fixed.details.iter().filter(|d| d.applied).count()
        );
    }

    #[test]
    fn offset_supports_virtual_eof_and_unicode_boundaries() {
        assert_eq!(offset_for_position("a", 2, 1), Some(1));
        assert_eq!(offset_for_position("a\n", 2, 1), Some(2));
        assert_eq!(offset_for_position("a\n", 3, 1), Some(2));
        assert_eq!(offset_for_position("é", 1, 2), Some(0));
        assert_eq!(offset_for_position("a", 3, 1), None);
    }

    #[test]
    fn offset_index_handles_multi_line_ranges_without_rescanning() {
        let content = "alpha\nbeta\ngamma\n";
        assert_eq!(offset_for_position(content, 1, 1), Some(0));
        assert_eq!(offset_for_position(content, 2, 1), Some(6));
        assert_eq!(offset_for_position(content, 3, 3), Some(13));
        assert_eq!(offset_for_position(content, 4, 1), Some(content.len()));
        assert_eq!(offset_for_position(content, 5, 1), Some(content.len()));
        assert_eq!(offset_for_position(content, 6, 1), None);
    }
}
