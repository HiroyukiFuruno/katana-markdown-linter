use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/* WHY: Section: Blockquote-related markdownlint rule implementations
=======================================================
  Split from whitespace.rs to stay within 200-line limit. */

/// MD028 / no-blanks-blockquote — Blank line inside blockquote.
pub struct NoBlanksBlockquoteRule;

impl MarkdownRule for NoBlanksBlockquoteRule {
    fn id(&self) -> &'static str {
        "MD028"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD028")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD028");
        let mut diagnostics = Vec::new();
        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            let trimmed = line.text.trim();
            if ctx.is_code_line(i) || !trimmed.is_empty() {
                continue;
            }
            /* WHY: A blank between two blockquote lines creates separated blockquotes */
            let has_bq_before = i > 0 && ctx.lines()[i - 1].text.trim_start().starts_with('>');
            let has_bq_after =
                i + 1 < ctx.lines().len() && ctx.lines()[i + 1].text.trim_start().starts_with('>');
            if has_bq_before && has_bq_after {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    ctx.file_path(),
                    i,
                    line.text,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}
