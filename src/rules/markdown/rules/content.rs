use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleParityStatus,
};
use crate::types::RuleConfig;
use std::path::Path;

/* WHY: Section: Content-level markdownlint rule implementations
======================================================= */

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
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        _config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD040");
        let mut diagnostics = Vec::new();
        for block in ctx.code_blocks() {
            let line = &ctx.lines()[block.start_line];
            let trimmed = line.text.trim_start();
            let after_fence = trimmed.trim_start_matches('`').trim_start_matches('~');
            if !after_fence.trim().is_empty() {
                continue;
            }
            let indent = line.text.len() - trimmed.len();
            let fence_len = trimmed
                .chars()
                .take_while(|ch| *ch == '`' || *ch == '~')
                .count();
            let column = indent + fence_len + 1;
            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: block.start_line + 1,
                    start_column: column,
                    end_line: block.start_line + 1,
                    end_column: column,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                    start_line: block.start_line + 1,
                    start_column: column,
                    end_line: block.start_line + 1,
                    end_column: column,
                    replacement: "text".to_string(),
                }),
            });
        }
        diagnostics
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
