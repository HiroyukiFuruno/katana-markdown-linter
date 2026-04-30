use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleParityStatus,
};
use crate::types::RuleConfig;
use std::path::Path;

/* WHY: Section: Heading-related markdownlint rule implementations
======================================================= */

/// MD022 / blanks-around-headings — Headings should be surrounded by blank lines.
pub struct BlanksAroundHeadingsRule;

impl MarkdownRule for BlanksAroundHeadingsRule {
    fn id(&self) -> &'static str {
        "MD022"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD022")
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
        let meta = self.official_meta().expect("always Some for MD022");
        let mut diagnostics = Vec::new();
        for heading in ctx.headings() {
            let i = heading.line;
            let line = &ctx.lines()[i];
            let needs_blank_before =
                i > 0 && !is_blank_for_heading_spacing(ctx.lines()[i - 1].text);
            let needs_blank_after =
                i + 1 < ctx.lines().len() && !is_blank_for_heading_spacing(ctx.lines()[i + 1].text);
            if needs_blank_before || needs_blank_after {
                let mut replacement = String::new();
                if needs_blank_before {
                    replacement.push('\n');
                }
                replacement.push_str(line.text);
                if needs_blank_after {
                    replacement.push('\n');
                }
                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: 1,
                    end_line: i + 1,
                    end_column: line.text.len().max(1) + if line.text.is_empty() { 0 } else { 1 },
                    replacement,
                };
                RuleHelpers::push_diag_with_fix(
                    &mut diagnostics,
                    ctx.file_path(),
                    i,
                    line.text,
                    &meta,
                    DiagnosticSeverity::Warning,
                    Some(fix),
                );
            }
        }
        diagnostics
    }
}

fn is_blank_for_heading_spacing(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || (trimmed.starts_with("<!--") && trimmed.ends_with("-->"))
}

/// MD023 / heading-start-left — Headings must start at the beginning of the line.
pub struct HeadingStartLeftRule;

impl MarkdownRule for HeadingStartLeftRule {
    fn id(&self) -> &'static str {
        "MD023"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD023",
            title: "heading-start-left",
            description: "Headings must start at the beginning of the line.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md023.md",
            aliases: &["heading-start-left"],
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[],
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
        let meta = self.official_meta().expect("always Some for MD023");
        let mut diagnostics = Vec::new();
        for heading in ctx.headings() {
            let line = &ctx.lines()[heading.line];
            let indent = heading.marker_range.start - line.content_range.start;
            if indent == 0 {
                continue;
            }
            let fix = crate::rules::markdown::types::DiagnosticFix {
                start_line: heading.line + 1,
                start_column: 1,
                end_line: heading.line + 1,
                end_column: indent + 1,
                replacement: String::new(),
            };
            RuleHelpers::push_diag_with_fix(
                &mut diagnostics,
                ctx.file_path(),
                heading.line,
                line.text,
                &meta,
                DiagnosticSeverity::Warning,
                Some(fix),
            );
        }
        diagnostics
    }
}
