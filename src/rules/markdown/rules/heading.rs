mod spacing;
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
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD022");
        spacing::check_headings(ctx, config, &meta)
    }
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
