use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::path::Path;

/// MD039 / no-space-in-links — Spaces inside link text.
pub struct NoSpacesInLinksRule;

impl MarkdownRule for NoSpacesInLinksRule {
    fn id(&self) -> &'static str {
        "MD039"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD039")
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
        let meta = self.official_meta().expect("always Some for MD039");
        let mut diagnostics = Vec::new();
        for link in ctx.inline_links() {
            if link.kind.is_image() {
                continue;
            }
            let Some(text) = link.text else {
                continue;
            };
            let trimmed = text.trim();
            if trimmed.is_empty() || trimmed == text {
                continue;
            }
            let Some(text_range) = link.text_range else {
                continue;
            };
            let full_range = ctx.diagnostic_range(link.full_range);
            let fix_range = ctx.diagnostic_range(text_range);
            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: full_range,
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                    start_line: fix_range.start_line,
                    start_column: fix_range.start_column,
                    end_line: fix_range.end_line,
                    end_column: fix_range.end_column,
                    replacement: trimmed.to_string(),
                }),
            });
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_spaces_inside_inline_link_text() {
        let rule = NoSpacesInLinksRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ text ](https://example.com)");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("link text spaces should be fixable");
        assert_eq!(fix.start_column, 2);
        assert_eq!(fix.end_column, 8);
        assert_eq!(fix.replacement, "text");
    }

    #[test]
    fn ignores_task_checkboxes_and_image_alt_text() {
        let rule = NoSpacesInLinksRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "- [ ] task\n- [x] done\n![ alt ](image.png)\n```\n[ link ](target)\n```",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn fixes_nested_link_text_using_shared_link_range() {
        let rule = NoSpacesInLinksRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ text [inner] ](target)");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("nested link text should be fixable");
        assert_eq!(fix.replacement, "text [inner]");
    }

    #[test]
    fn ignores_links_inside_long_and_unclosed_code_spans() {
        let rule = NoSpacesInLinksRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "``[ text ](target)``\n`[ text ][]");

        assert!(diagnostics.is_empty());
    }
}
