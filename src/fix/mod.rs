use crate::types::{FixResult, LintResult};

use edits::{Edit, EditSelection};

mod edits;
mod engine;
mod plan;
mod policy;
mod position;
pub(crate) use engine::{FixEngine, DEFAULT_MAX_FIX_PASSES};
pub(crate) use position::LineOffsetIndex;

pub(crate) struct FixApplicator;

impl FixApplicator {
    pub(crate) fn has_effective(
        results: &[LintResult],
        content: &str,
        include_unsafe: bool,
    ) -> bool {
        Self::apply(results, content, include_unsafe).applied_fixes > 0
    }

    pub(crate) fn apply(results: &[LintResult], content: &str, include_unsafe: bool) -> FixResult {
        let line_index = LineOffsetIndex::new(content);

        let edits = results
            .iter()
            .filter_map(|result| Edit::from_diagnostic(result, &line_index, include_unsafe))
            .collect();
        let mut selection = EditSelection::non_overlapping(edits, content);
        plan::retain_semantics(&mut selection.accepted, &mut selection.skipped, content);

        FixResult {
            content: plan::render(content, selection.accepted.iter()),
            applied_fixes: selection.accepted.len(),
            details: selection.details(),
        }
    }
}

#[cfg(test)]
fn offset_for_position(content: &str, line: usize, column: usize) -> Option<usize> {
    LineOffsetIndex::new(content).offset_for_position(line, column)
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

    fn overlapping_heading_fixes() -> FixResult {
        FixApplicator::apply(
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
            "#Title\n",
            false,
        )
    }

    #[test]
    fn skips_overlapping_fix_ranges() {
        let fixed = overlapping_heading_fixes();
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
        let fixed = overlapping_heading_fixes();
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
    fn no_op_fix_is_not_counted_as_applied() {
        let content = "# Title\n";
        let fixed = FixApplicator::apply(
            &[result(
                "MD018",
                Range {
                    start_line: 1,
                    start_column: 1,
                    end_line: 1,
                    end_column: 2,
                },
                "#",
            )],
            content,
            false,
        );

        assert_eq!(fixed.content, content);
        assert_eq!(fixed.applied_fixes, 0);
        assert_eq!(fixed.details.len(), 1);
        assert!(!fixed.details[0].applied);
    }

    #[test]
    fn applies_identical_insertions_at_the_same_offset_once() {
        let content = "abc";
        let insertion = Range {
            start_line: 1,
            start_column: 2,
            end_line: 1,
            end_column: 2,
        };
        let fixed = FixApplicator::apply(
            &[
                result("MD018", insertion.clone(), "X"),
                result("MD019", insertion, "X"),
            ],
            content,
            false,
        );

        assert_eq!(fixed.content, "aXbc");
        assert_eq!(fixed.applied_fixes, 1);
        assert_eq!(
            fixed.details.iter().filter(|detail| detail.applied).count(),
            1
        );
    }

    #[test]
    fn conflicting_same_offset_insertions_choose_deterministically() {
        let content = "abc";
        let insertion = Range {
            start_line: 1,
            start_column: 2,
            end_line: 1,
            end_column: 2,
        };
        let first = result("RULE_Z", insertion.clone(), "z");
        let second = result("RULE_A", insertion, "a");
        let forward = FixApplicator::apply(&[first.clone(), second.clone()], content, false);
        let reverse = FixApplicator::apply(&[second, first], content, false);

        assert_eq!(forward.content, "aabc");
        assert_eq!(reverse.content, forward.content);
        assert_eq!(forward.applied_fixes, 1);
        assert_eq!(reverse.applied_fixes, 1);
    }

    #[test]
    fn adjacent_non_overlapping_edits_are_both_applied() {
        let edits = [(1, "RULE_A", "A"), (2, "RULE_B", "B")].map(|(column, rule, replacement)| {
            result(
                rule,
                Range {
                    start_line: 1,
                    start_column: column,
                    end_line: 1,
                    end_column: column + 1,
                },
                replacement,
            )
        });
        let fixed = FixApplicator::apply(&edits, "ab", false);
        assert_eq!(fixed.content, "AB");
        assert_eq!(fixed.applied_fixes, 2);
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
