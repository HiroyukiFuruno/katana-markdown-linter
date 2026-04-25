use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

/* WHY: Section: Content-level markdownlint rule implementations (Extended)
======================================================= */

/// MD042 / no-empty-links — No empty links.
pub struct NoEmptyLinksRule;

impl MarkdownRule for NoEmptyLinksRule {
    fn id(&self) -> &'static str {
        "MD042"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD042")
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
        let meta = self.official_meta().expect("always Some for MD042");
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            if line.text.contains("]()") || line.text.contains("](#)") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_empty_links_inside_fenced_code_blocks() {
        let rule = NoEmptyLinksRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```md\n[example]()\n```\n");

        assert!(diagnostics.is_empty());
    }
}
