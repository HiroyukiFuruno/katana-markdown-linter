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
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            for link_text in markdown_link_texts(line.text) {
                if !contains_prohibited_text(link_text.text, &prohibited) {
                    continue;
                }
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: line.number,
                        start_column: link_text.start + 1,
                        end_line: line.number,
                        end_column: link_text.end + 1,
                    },
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

fn markdown_link_texts(line: &str) -> impl Iterator<Item = MarkdownLinkText<'_>> {
    MarkdownLinkTextIterator { line, cursor: 0 }
}

struct MarkdownLinkText<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

struct MarkdownLinkTextIterator<'a> {
    line: &'a str,
    cursor: usize,
}

impl<'a> Iterator for MarkdownLinkTextIterator<'a> {
    type Item = MarkdownLinkText<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.line.as_bytes();
        while self.cursor < self.line.len() {
            let start = find_next_link_text_start(self.line, self.cursor)?;
            if bytes[start] == b'`' {
                self.cursor = cursor_after_code_span(self.line, start);
                continue;
            }
            self.cursor = start + 1;
            if is_image_marker(self.line, start) {
                continue;
            }

            let end = find_closing_bracket(self.line, self.cursor)?;
            self.cursor = end + 1;
            if is_markdown_link_destination(self.line, self.cursor) {
                return Some(MarkdownLinkText {
                    text: &self.line[start + 1..end],
                    start: start + 1,
                    end,
                });
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

fn cursor_after_code_span(line: &str, start: usize) -> usize {
    let bytes = line.as_bytes();
    let marker_len = bytes[start..]
        .iter()
        .take_while(|byte| **byte == b'`')
        .count();
    let content_start = start + marker_len;
    let marker = "`".repeat(marker_len);
    line[content_start..]
        .find(&marker)
        .map(|offset| content_start + offset + marker_len)
        .unwrap_or(line.len())
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
