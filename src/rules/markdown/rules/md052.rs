use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::path::Path;

/// MD052 / reference-links-images — Reference links and images.
pub struct ReferenceLinksImagesRule;

impl MarkdownRule for ReferenceLinksImagesRule {
    fn id(&self) -> &'static str {
        "MD052"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD052")
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
        let meta = self.official_meta().expect("always Some for MD052");
        let mut diagnostics = Vec::new();
        for link in ctx.inline_links() {
            if link.kind.is_collapsed_reference() {
                let range = ctx.diagnostic_range(link.full_range);
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range,
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: None,
                });
            }
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_collapsed_reference_links_and_images() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ref][] and ![alt][]\n");

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.rule_id == "MD052"));
    }

    #[test]
    fn ignores_full_references_and_code_spans() {
        let rule = ReferenceLinksImagesRule;
        let content = concat!(
            "[ref][label] and ![alt][image]\n",
            "`[ref][]`\n",
            "[label]: https://example.com\n",
            "[image]: https://example.org/image.png\n",
        );
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }
}
