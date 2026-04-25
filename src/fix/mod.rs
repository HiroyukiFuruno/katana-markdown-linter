use crate::types::{FixResult, LintResult};

pub fn apply(results: &[LintResult], content: &str) -> FixResult {
    let mut applied_fixes = 0;
    let mut edits = results
        .iter()
        .filter_map(|result| {
            let fix = result.fix.as_ref()?;
            let start = offset_for_position(content, fix.range.start_line, fix.range.start_column)?;
            let end = offset_for_position(content, fix.range.end_line, fix.range.end_column)?;
            if start > end {
                return None;
            }
            Some((start, end, fix.replacement.as_str()))
        })
        .collect::<Vec<_>>();

    edits.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| right.1.cmp(&left.1)));

    let mut accepted = Vec::new();
    let mut previous_start = content.len();
    for (start, end, replacement) in edits {
        if end > previous_start {
            continue;
        }
        previous_start = start;
        accepted.push((start, end, replacement));
        applied_fixes += 1;
    }
    accepted.reverse();

    let mut fixed = String::with_capacity(content.len());
    let mut cursor = 0;
    for (start, end, replacement) in accepted {
        fixed.push_str(&content[cursor..start]);
        fixed.push_str(replacement);
        cursor = end;
    }
    fixed.push_str(&content[cursor..]);

    FixResult {
        content: fixed,
        applied_fixes,
    }
}

fn offset_for_position(content: &str, line: usize, column: usize) -> Option<usize> {
    if line == 0 || column == 0 {
        return None;
    }

    let mut current_line = 1;
    let mut line_start = 0;
    for (index, byte) in content.bytes().enumerate() {
        if current_line == line {
            break;
        }
        if byte == b'\n' {
            current_line += 1;
            line_start = index + 1;
        }
    }

    if current_line != line {
        return if current_line + 1 == line && column == 1 {
            Some(content.len())
        } else {
            None
        };
    }

    let raw = line_start.saturating_add(column.saturating_sub(1));
    Some(previous_char_boundary(content, raw.min(content.len())))
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
    use crate::types::{Fix, Range, Severity};

    fn result(rule_id: &str, range: Range, replacement: &str) -> LintResult {
        LintResult {
            rule_id: rule_id.to_string(),
            rule_name: String::new(),
            message: String::new(),
            message_id: "rule.generic".to_string(),
            message_params: crate::i18n::diagnostic_message_params(rule_id, "", ""),
            severity: Severity::Warning,
            line: range.start_line,
            column: range.start_column,
            end_line: range.end_line,
            end_column: range.end_column,
            fix: Some(Fix {
                range,
                replacement: replacement.to_string(),
            }),
        }
    }

    #[test]
    fn applies_multi_line_fix_ranges() {
        let content = "# Title\n\n\nParagraph\n";
        let fixed = apply(
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
        );

        assert_eq!(fixed.content, "# Title\n\nParagraph\n");
        assert_eq!(fixed.applied_fixes, 1);
    }

    #[test]
    fn skips_overlapping_fix_ranges() {
        let content = "#Title\n";
        let fixed = apply(
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
        );

        assert_eq!(fixed.content, "# Title\n");
        assert_eq!(fixed.applied_fixes, 1);
    }

    #[test]
    fn offset_supports_virtual_eof_and_unicode_boundaries() {
        assert_eq!(offset_for_position("a", 2, 1), Some(1));
        assert_eq!(offset_for_position("é", 1, 2), Some(0));
        assert_eq!(offset_for_position("a", 3, 1), None);
    }
}
