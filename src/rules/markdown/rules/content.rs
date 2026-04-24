use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleParityStatus,
};
use std::path::Path;

/* WHY: Section: Content-level markdownlint rule implementations
======================================================= */

/// MD033 / no-inline-html — Inline HTML.
pub struct NoInlineHtmlRule;

impl MarkdownRule for NoInlineHtmlRule {
    fn id(&self) -> &'static str {
        "MD033"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD033")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD033");
        let mut diagnostics = Vec::new();
        let mut in_code_block = false;
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }
            if RuleHelpers::contains_html_tag(line) {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    file_path,
                    i,
                    line,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}

/// MD040 / fenced-code-language — Fenced code blocks should have a language specified.
pub struct FencedCodeLanguageRule;

impl MarkdownRule for FencedCodeLanguageRule {
    fn id(&self) -> &'static str {
        "MD040"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD040",
            title: "fenced-code-language",
            description: "Fenced code blocks should have a language specified.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md040.md",
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[
                crate::rule_prop!(StringArray, "allowed_languages", "List of languages", "[]"),
                crate::rule_prop!(Boolean, "language_only", "Require language only", "false"),
            ],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD040");
        let mut diagnostics = Vec::new();
        let mut in_code_block = false;
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if !RuleHelpers::is_fence(trimmed) {
                continue;
            }
            /* WHY: Opening fence only — check if language is specified */
            if !in_code_block {
                let after_fence = trimmed.trim_start_matches('`').trim_start_matches('~');
                if after_fence.trim().is_empty() {
                    let indent = line.len() - trimmed.len();
                    let fence_len = trimmed
                        .chars()
                        .take_while(|ch| *ch == '`' || *ch == '~')
                        .count();
                    let column = indent + fence_len + 1;
                    diagnostics.push(MarkdownDiagnostic {
                        file: file_path.to_path_buf(),
                        severity: DiagnosticSeverity::Warning,
                        range: DiagnosticRange {
                            start_line: i + 1,
                            start_column: column,
                            end_line: i + 1,
                            end_column: column,
                        },
                        message: meta.description.to_string(),
                        rule_id: meta.code.to_string(),
                        official_meta: Some(meta.clone()),
                        fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                            start_line: i + 1,
                            start_column: column,
                            end_line: i + 1,
                            end_column: column,
                            replacement: "text".to_string(),
                        }),
                    });
                }
            }
            in_code_block = !in_code_block;
        }
        diagnostics
    }
}

#[cfg(test)]
mod content_tests {
    use super::*;

    #[test]
    fn fixes_missing_fence_language_with_text() {
        let rule = FencedCodeLanguageRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```\ncode\n```\n");
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("missing language should be fixable");
        assert_eq!(fix.start_column, 4);
        assert_eq!(fix.replacement, "text");
    }
}

/// MD041 / first-line-heading — First line should be a top-level heading.
pub struct FirstLineHeadingRule;

impl MarkdownRule for FirstLineHeadingRule {
    fn id(&self) -> &'static str {
        "MD041"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD041",
            title: "first-line-heading",
            description: "First line in a file should be a top-level heading.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md041.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[
                crate::rule_prop!(
                    Boolean,
                    "allow_preamble",
                    "Allow content before first heading",
                    "false"
                ),
                crate::rule_prop!(
                    String,
                    "front_matter_title",
                    "RegExp for matching title in front matter",
                    "^\\s*title\\s*[:=]"
                ),
                crate::rule_prop!(Number, "level", "Heading level", "1"),
            ],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD041");
        let first_line = match content.lines().next() {
            Some(line) => line,
            None => return Vec::new(),
        };
        let trimmed = first_line.trim();
        if trimmed.starts_with("# ") && !trimmed.starts_with("## ") {
            return Vec::new();
        }
        vec![MarkdownDiagnostic {
            file: file_path.to_path_buf(),
            severity: DiagnosticSeverity::Warning,
            range: DiagnosticRange {
                start_line: 1,
                start_column: 1,
                end_line: 1,
                end_column: first_line.len().max(1),
            },
            message: meta.description.to_string(),
            rule_id: meta.code.to_string(),
            official_meta: Some(meta),
            fix_info: None,
        }]
    }
}
