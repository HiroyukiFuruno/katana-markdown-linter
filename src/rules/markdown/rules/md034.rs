use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD034 / no-bare-urls — No bare URLs.
pub struct NoBareUrlsRule;

impl MarkdownRule for NoBareUrlsRule {
    fn id(&self) -> &'static str {
        "MD034"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD034")
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
        let meta = self.official_meta().expect("always Some for MD034");
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            if let Some((start, end)) = bare_url_range(line.text) {
                let url = &line.text[start..end];
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: line.number,
                        start_column: start + 1,
                        end_line: line.number,
                        end_column: end + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: line.number,
                        start_column: start + 1,
                        end_line: line.number,
                        end_column: end + 1,
                        replacement: format!("<{url}>"),
                    }),
                });
            }
        }
        diagnostics
    }
}

fn bare_url_range(line: &str) -> Option<(usize, usize)> {
    let mut search_start = 0;
    while let Some(start) = next_url_start(line, search_start) {
        if is_ignored_url(line, start) {
            search_start = start + 1;
            continue;
        }

        return url_range(line, start);
    }

    None
}

fn next_url_start(line: &str, search_start: usize) -> Option<usize> {
    let remainder = &line[search_start..];
    match (remainder.find("http://"), remainder.find("https://")) {
        (Some(http), Some(https)) => Some(search_start + http.min(https)),
        (Some(http), None) => Some(search_start + http),
        (None, Some(https)) => Some(search_start + https),
        (None, None) => None,
    }
}

fn is_ignored_url(line: &str, start: usize) -> bool {
    if start > 0 && line.as_bytes()[start - 1] == b'<' {
        return true;
    }
    if line[..start].ends_with("](") || line[..start].ends_with(")[") {
        return true;
    }
    if is_link_reference_definition_destination(line, start) {
        return true;
    }
    if is_inside_code_span(line, start) {
        return true;
    }

    is_inside_html_attribute_value(line, start)
}

fn is_inside_html_attribute_value(line: &str, start: usize) -> bool {
    let Some(tag_start) = line[..start].rfind('<') else {
        return false;
    };
    if line[tag_start..start].contains('>') {
        return false;
    }

    let bytes = line.as_bytes();
    let mut quote = None;
    for byte in &bytes[tag_start..start] {
        match (quote, byte) {
            (None, b'"' | b'\'') => quote = Some(*byte),
            (Some(current), close) if current == *close => quote = None,
            _ => {}
        }
    }
    quote.is_some()
}

fn is_link_reference_definition_destination(line: &str, start: usize) -> bool {
    let before = &line[..start];
    let Some(definition_marker) = before.find("]:") else {
        return false;
    };
    let label = before[..definition_marker].trim_start();
    label.starts_with('[')
        && !label[1..].contains('[')
        && before[definition_marker + 2..].trim().is_empty()
}

fn url_range(line: &str, start: usize) -> Option<(usize, usize)> {
    let mut end = line.len();
    for (offset, ch) in line[start..].char_indices() {
        if ch.is_whitespace() || ch == '<' || ch == '>' {
            end = start + offset;
            break;
        }
    }
    while end > start && matches!(line.as_bytes()[end - 1], b'.' | b',' | b';' | b':') {
        end -= 1;
    }
    (end > start).then_some((start, end))
}

fn is_inside_code_span(line: &str, offset: usize) -> bool {
    let bytes = line.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] != b'`' {
            cursor += 1;
            continue;
        }
        let marker_len = bytes[cursor..]
            .iter()
            .take_while(|byte| **byte == b'`')
            .count();
        let content_start = cursor + marker_len;
        let Some(close_relative) = line[content_start..].find(&"`".repeat(marker_len)) else {
            return false;
        };
        let close = content_start + close_relative;
        if content_start <= offset && offset < close {
            return true;
        }
        cursor = close + marker_len;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_bare_url_with_angle_brackets() {
        let rule = NoBareUrlsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "See http://example.com now.");
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("url should be fixable");
        assert_eq!(fix.replacement, "<http://example.com>");
    }

    #[test]
    fn ignores_url_inside_inline_link_destination() {
        let rule = NoBareUrlsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[Example](https://example.com)");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_url_inside_reversed_link_destination() {
        let rule = NoBareUrlsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "(Example)[https://example.com]");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_url_inside_html_attribute_values() {
        let rule = NoBareUrlsRule;
        let content = concat!(
            "<p align=\"center\">\n",
            "  <a href=\"#\"><img src=\"https://img.shields.io/badge/license-MIT-blue.svg\" alt=\"License: MIT\"></a>\n",
            "  <img src=\"data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22%3E\" alt=\"inline\">\n",
            "</p>\n"
        );
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_urls_inside_code_spans_and_fenced_code() {
        let rule = NoBareUrlsRule;
        let content = concat!(
            "`https://example.com`\n",
            "```\n",
            "https://example.org\n",
            "```\n",
        );
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn detects_bare_url_after_html_attribute_value() {
        let rule = NoBareUrlsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "<img src=\"https://example.com/image.png\"> See https://example.com.",
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].range.start_column, 47);
    }

    #[test]
    fn ignores_url_inside_link_reference_definition() {
        let rule = NoBareUrlsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[normal]: https://github.com\n");

        assert!(diagnostics.is_empty());
    }
}
