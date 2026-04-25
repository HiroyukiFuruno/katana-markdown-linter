use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
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
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            if markdown_link_texts(line.text)
                .any(|link_text| contains_prohibited_text(link_text, &prohibited))
            {
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

fn contains_prohibited_text(link_text: &str, prohibited: &[&str]) -> bool {
    let normalized_text = normalize_link_text(link_text);
    prohibited.iter().any(|needle| normalized_text == *needle)
}

fn normalize_link_text(link_text: &str) -> String {
    link_text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn markdown_link_texts(line: &str) -> impl Iterator<Item = &str> {
    MarkdownLinkTextIterator { line, cursor: 0 }
}

struct MarkdownLinkTextIterator<'a> {
    line: &'a str,
    cursor: usize,
}

impl<'a> Iterator for MarkdownLinkTextIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.line.as_bytes();
        let mut in_code = false;
        while self.cursor < self.line.len() {
            let start = find_next_link_text_start(self.line, self.cursor)?;
            if bytes[start] == b'`' {
                in_code = !in_code;
                self.cursor = start + 1;
                continue;
            }
            self.cursor = start + 1;
            if in_code {
                continue;
            }
            if is_image_marker(self.line, start) {
                continue;
            }

            let end = find_closing_bracket(self.line, self.cursor)?;
            self.cursor = end + 1;
            if is_markdown_link_destination(self.line, self.cursor) {
                return Some(&self.line[start + 1..end]);
            }
        }

        None
    }
}

fn find_next_link_text_start(line: &str, cursor: usize) -> Option<usize> {
    line[cursor..]
        .find(['[', '`'])
        .map(|offset| cursor + offset)
}

fn is_image_marker(line: &str, bracket_start: usize) -> bool {
    bracket_start > 0 && line.as_bytes()[bracket_start - 1] == b'!'
}

fn find_closing_bracket(line: &str, cursor: usize) -> Option<usize> {
    line[cursor..].find(']').map(|offset| cursor + offset)
}

fn is_markdown_link_destination(line: &str, cursor: usize) -> bool {
    matches!(line.as_bytes().get(cursor), Some(b'(' | b'['))
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
}
