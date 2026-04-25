use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleParityStatus,
};
use std::collections::BTreeMap;
use std::path::Path;

/* WHY: Section: List-related markdownlint rule implementations
======================================================= */

/// MD004 / ul-style — Unordered list style. Enforces consistent bullet character.
pub struct UlStyleRule;

impl MarkdownRule for UlStyleRule {
    fn id(&self) -> &'static str {
        "MD004"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD004")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD004");
        let mut diagnostics = Vec::new();
        let mut first_bullet: Option<char> = None;
        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let line = line.text;
            let trimmed = line.trim_start();
            if let Some(bullet) = RuleHelpers::get_bullet_char(trimmed) {
                match first_bullet {
                    None => first_bullet = Some(bullet),
                    Some(expected) if bullet != expected => {
                        let bullet_pos = line.find(bullet).unwrap();
                        let fix = crate::rules::markdown::types::DiagnosticFix {
                            start_line: i + 1,
                            start_column: bullet_pos + 1,
                            end_line: i + 1,
                            end_column: bullet_pos + 2,
                            replacement: expected.to_string(),
                        };
                        RuleHelpers::push_diag_with_fix(
                            &mut diagnostics,
                            file_path,
                            i,
                            line,
                            &meta,
                            DiagnosticSeverity::Warning,
                            Some(fix),
                        );
                    }
                    _ => {}
                }
            }
        }
        diagnostics
    }
}

/// MD029 / ol-prefix — Ordered list item prefix.
pub struct OlPrefixRule;

impl MarkdownRule for OlPrefixRule {
    fn id(&self) -> &'static str {
        "MD029"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD029",
            title: "ol-prefix",
            description: "Ordered list item prefix.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md029.md",
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[crate::rule_prop_enum!(
                "style",
                "List style",
                "one_or_ordered",
                &["one", "ordered", "one_or_ordered", "zero"]
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD029");
        let mut diagnostics = Vec::new();
        let mut expected_numbers = BTreeMap::<usize, u32>::new();
        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                expected_numbers.clear();
                continue;
            }
            let line = line.text;
            let trimmed = line.trim_start();
            if let Some(num) = RuleHelpers::get_ordered_number(trimmed) {
                let indent = line.len() - trimmed.len();
                expected_numbers.retain(|level, _| *level <= indent);
                let expected_number = expected_numbers.entry(indent).or_insert(1);
                if num != *expected_number {
                    let dot_pos = line.find(". ").unwrap();
                    let start_col = line.find(|c: char| c.is_ascii_digit()).unwrap();
                    let fix = crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: start_col + 1,
                        end_line: i + 1,
                        end_column: dot_pos + 1,
                        replacement: expected_number.to_string(),
                    };
                    RuleHelpers::push_diag_with_fix(
                        &mut diagnostics,
                        file_path,
                        i,
                        line,
                        &meta,
                        DiagnosticSeverity::Warning,
                        Some(fix),
                    );
                }
                *expected_number += 1;
            } else if !trimmed.is_empty() {
                expected_numbers.clear();
            }
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::OlPrefixRule;
    use crate::rules::markdown::MarkdownRule;
    use std::path::Path;

    #[test]
    fn md029_accepts_nested_ordered_lists_with_independent_numbering() {
        let content = "\
1. First item
2. Second item
   1. Nested 2-1
   2. Nested 2-2
3. Third item
";

        let diagnostics = OlPrefixRule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn md029_rejects_broken_nested_numbering_at_the_same_level() {
        let content = "\
1. First item
2. Second item
   1. Nested 2-1
   3. Nested 2-2
3. Third item
";

        let diagnostics = OlPrefixRule.evaluate(Path::new("doc.md"), content);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].range.start_line, 4);
    }
}
