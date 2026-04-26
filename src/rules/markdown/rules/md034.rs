#[cfg(test)]
mod tests;

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
            for (start, end) in bare_url_ranges(line.text) {
                let absolute_start = line.content_range.start + start;
                if is_ignored_url(ctx, i, line.text, start, absolute_start) {
                    continue;
                }
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

fn bare_url_ranges(line: &str) -> BareUrlRanges<'_> {
    BareUrlRanges {
        line,
        search_start: 0,
    }
}

struct BareUrlRanges<'a> {
    line: &'a str,
    search_start: usize,
}

impl Iterator for BareUrlRanges<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(start) = next_url_start(self.line, self.search_start) {
            self.search_start = start + 1;
            if let Some((_, end)) = url_range(self.line, start) {
                self.search_start = end;
                return Some((start, end));
            }
        }

        None
    }
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

fn is_ignored_url(
    ctx: &DocumentContext<'_>,
    line_index: usize,
    line: &str,
    start: usize,
    absolute_start: usize,
) -> bool {
    if start > 0 && line.as_bytes()[start - 1] == b'<' {
        return true;
    }
    if line[..start].ends_with("](") || line[..start].ends_with(")[") {
        return true;
    }
    if ctx.inline_code_spans().iter().any(|span| {
        span.line == line_index
            && span.full_range.start <= absolute_start
            && absolute_start < span.full_range.end
    }) {
        return true;
    }
    if ctx.inline_links().iter().any(|link| {
        link.line == line_index
            && link.full_range.start <= absolute_start
            && absolute_start < link.full_range.end
    }) {
        return true;
    }
    if ctx.reference_definitions().iter().any(|definition| {
        definition.line == line_index
            && definition.full_range.start <= absolute_start
            && absolute_start < definition.full_range.end
    }) {
        return true;
    }

    ctx.html_attribute_at(absolute_start).is_some()
}

fn url_range(line: &str, start: usize) -> Option<(usize, usize)> {
    let mut end = line.len();
    for (offset, ch) in line[start..].char_indices() {
        if ch.is_whitespace() || ch == '<' || ch == '>' {
            end = start + offset;
            break;
        }
    }
    loop {
        if end <= start {
            break;
        }
        match line.as_bytes()[end - 1] {
            b'.' | b',' | b';' | b':' => end -= 1,
            b')' if has_unmatched_closing_delimiter(&line[start..end], b'(', b')') => end -= 1,
            b']' if has_unmatched_closing_delimiter(&line[start..end], b'[', b']') => end -= 1,
            _ => break,
        }
    }
    (end > start).then_some((start, end))
}

fn has_unmatched_closing_delimiter(text: &str, open: u8, close: u8) -> bool {
    let open_count = text.as_bytes().iter().filter(|byte| **byte == open).count();
    let close_count = text
        .as_bytes()
        .iter()
        .filter(|byte| **byte == close)
        .count();
    close_count > open_count
}
