use crate::rules::markdown::document::SourceRange;
use crate::rules::markdown::types::DiagnosticFix;
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
        let mut meta = crate::rules::markdown::catalog::get_official_meta("MD052")?;
        meta.is_fixable = true;
        Some(meta)
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
                let fix_range = ctx.diagnostic_range(SourceRange {
                    start: link.full_range.end - 2,
                    end: link.full_range.end,
                });
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range,
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(DiagnosticFix {
                        start_line: fix_range.start_line,
                        start_column: fix_range.start_column,
                        end_line: fix_range.end_line,
                        end_column: fix_range.end_column,
                        replacement: String::new(),
                    }),
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

    #[test]
    fn fix_removes_trailing_brackets() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ref][]\n");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("fix_info must be Some");
        assert_eq!(fix.replacement, "");
        assert_eq!(fix.start_line, 1);
        assert_eq!(fix.end_line, 1);
        assert_eq!(fix.start_column, fix.end_column - 2);
    }

    #[test]
    fn fix_image_removes_trailing_brackets() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "![alt][]\n");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("fix_info must be Some");
        assert_eq!(fix.replacement, "");
        assert_eq!(fix.start_line, 1);
    }

    #[test]
    fn is_fixable_in_catalog() {
        let rule = ReferenceLinksImagesRule;
        let meta = rule.official_meta().expect("meta must be Some");
        assert!(meta.is_fixable, "MD052 must advertise is_fixable=true");
    }
}
