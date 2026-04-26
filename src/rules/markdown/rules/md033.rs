use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

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
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        _config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD033");
        let mut diagnostics = Vec::new();
        let mut reported_line = None;
        for element in ctx.inline_html_elements() {
            if !is_likely_html_tag(element.name) || reported_line == Some(element.line) {
                continue;
            }
            let line = &ctx.lines()[element.line];
            RuleHelpers::push_diag(
                &mut diagnostics,
                ctx.file_path(),
                element.line,
                line.text,
                &meta,
                DiagnosticSeverity::Warning,
            );
            reported_line = Some(element.line);
        }
        diagnostics
    }
}

fn is_likely_html_tag(content: &str) -> bool {
    matches!(
        content.to_lowercase().as_str(),
        "br" | "hr"
            | "div"
            | "span"
            | "p"
            | "b"
            | "i"
            | "em"
            | "strong"
            | "a"
            | "img"
            | "table"
            | "tr"
            | "td"
            | "th"
            | "ul"
            | "ol"
            | "li"
            | "pre"
            | "code"
            | "blockquote"
            | "details"
            | "summary"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_html_ignores_fenced_code_blocks() {
        let rule = NoInlineHtmlRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```html\n<span>code</span>\n```\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn inline_html_ignores_long_and_unclosed_code_spans() {
        let rule = NoInlineHtmlRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "``<span>``\n`<img>\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn inline_html_reports_real_html_outside_code_spans() {
        let rule = NoInlineHtmlRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "`<span>` and <img src=\"x\">\n");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "MD033");
    }
}
