use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    SourceRange,
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
        for (line_index, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(line_index) {
                continue;
            }
            if contains_html_tag_outside_inline_code(ctx, line_index, line.text) {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    ctx.file_path(),
                    line_index,
                    line.text,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}

fn contains_html_tag_outside_inline_code(
    ctx: &DocumentContext<'_>,
    line_index: usize,
    line: &str,
) -> bool {
    let line_start = ctx.lines()[line_index].content_range.start;
    let has_inline_code = line.contains('`');
    let mut offset = 0;
    while let Some(tag_start_offset) = line[offset..].find('<') {
        let tag_start = offset + tag_start_offset;
        let tag_content_start = tag_start + 1;
        let Some(tag_end_offset) = line[tag_content_start..].find('>') else {
            return false;
        };
        let tag_content_end = tag_content_start + tag_end_offset;
        let tag_end = tag_content_end + 1;
        let tag_content = &line[tag_content_start..tag_content_end];
        let tag_range = SourceRange {
            start: line_start + tag_start,
            end: line_start + tag_end,
        };

        if is_likely_html_tag(tag_content)
            && !(has_inline_code && ctx.is_inside_inline_code(tag_range))
        {
            return true;
        }
        offset = tag_end;
    }
    false
}

fn is_likely_html_tag(content: &str) -> bool {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return false;
    }
    let tag_name = trimmed
        .trim_start_matches('/')
        .split(|it: char| it.is_whitespace() || it == '/')
        .next()
        .unwrap_or("");
    matches!(
        tag_name.to_lowercase().as_str(),
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
