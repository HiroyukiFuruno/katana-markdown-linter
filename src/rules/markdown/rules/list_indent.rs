mod state;

use crate::rules::markdown::{DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta};
use state::ListParents;
use std::path::Path;

/// MD007 / ul-indent — Unordered list indentation.
pub struct UnorderedListIndentRule;

impl MarkdownRule for UnorderedListIndentRule {
    fn id(&self) -> &'static str {
        "MD007"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD007")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&crate::types::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, config)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&crate::types::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD007");
        let mut diagnostics = Vec::new();
        let indent = config
            .and_then(|c| c.properties.get("indent"))
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(2);
        let mut parents = ListParents::default();
        for (i, line) in ctx.lines().iter().enumerate() {
            parents.check_line(ctx, i, line.text, indent, &meta, &mut diagnostics);
        }

        diagnostics
    }
}
