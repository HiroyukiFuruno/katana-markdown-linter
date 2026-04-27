use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD059 / link-text — Prohibited link text.
pub struct ProhibitedLinkTextRule;

impl MarkdownRule for ProhibitedLinkTextRule {
    fn id(&self) -> &'static str {
        "MD059"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD059")
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
        let meta = self.official_meta().expect("always Some for MD059");
        let mut diagnostics = Vec::new();
        let prohibited = ["click here", "here", "link", "more"];
        for link in ctx.inline_links() {
            if link.kind.is_image() {
                continue;
            }
            let Some(text) = link.text else {
                continue;
            };
            if !contains_prohibited_text(text, &prohibited) {
                continue;
            }
            let range = ctx.diagnostic_range(
                link.text_range
                    .expect("markdown link text should have a source range"),
            );
            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: range.start_line,
                    start_column: range.start_column,
                    end_line: range.end_line,
                    end_column: range.end_column,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: None,
            });
        }
        diagnostics
    }
}

fn contains_prohibited_text(link_text: &str, prohibited: &[&str]) -> bool {
    let normalized_text = normalize_link_text(link_text);
    prohibited.iter().any(|needle| normalized_text == *needle)
}

fn normalize_link_text(link_text: &str) -> String {
    let mut result = String::new();
    for word in link_text.split_whitespace() {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(word);
    }
    result.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_prohibited_markdown_link_text() {
        let rule = ProhibitedLinkTextRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "See [click here](#target).");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "MD059");
    }

    #[test]
    fn reports_each_prohibited_link_text_on_the_same_line() {
        let rule = ProhibitedLinkTextRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "See [link](https://example.com) and [more](https://example.org).",
        );

        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].range.start_column, 6);
        assert_eq!(diagnostics[0].range.end_column, 10);
        assert_eq!(diagnostics[1].range.start_column, 38);
        assert_eq!(diagnostics[1].range.end_column, 42);
    }

    #[test]
    fn reports_exact_prohibited_markdown_link_text() {
        let rule = ProhibitedLinkTextRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "See [link](https://github.com).");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "MD059");
    }

    #[test]
    fn ignores_descriptive_markdown_link_text() {
        let rule = ProhibitedLinkTextRule;
        let content = concat!(
            "[Normal link](https://github.com)\n",
            "[Email link](mailto:test@example.com)\n",
            "[Normal link][normal]\n",
            "\n",
            "[normal]: https://github.com\n",
        );
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_markdown_autolink() {
        let rule = ProhibitedLinkTextRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "<https://github.com>");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_html_anchor_text() {
        let rule = ProhibitedLinkTextRule;
        let content = concat!(
            "<p align=\"center\">\n",
            "  English | <a href=\"#\">click here</a> | <a href=\"#\">日本語</a>\n",
            "</p>\n"
        );
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_markdown_link_text_inside_code() {
        let rule = ProhibitedLinkTextRule;
        let content = "`[link](https://github.com)`\n```\n[link](https://github.com)\n```\n";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_markdown_link_text_inside_long_code_span() {
        let rule = ProhibitedLinkTextRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "``[link](https://github.com)``");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_markdown_link_text_after_unclosed_code_span_marker() {
        let rule = ProhibitedLinkTextRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "`[link](https://github.com)");

        assert!(diagnostics.is_empty());
    }
}
