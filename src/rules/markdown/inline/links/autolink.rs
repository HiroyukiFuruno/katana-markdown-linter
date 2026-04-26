use super::super::scan::inside_code_span;
use super::super::types::{InlineCodeSpan, InlineLink, InlineLinkKind};
use crate::rules::markdown::document::{LineInfo, SourceRange};

pub(super) fn autolinks_on_line<'a>(
    line_index: usize,
    line: &LineInfo<'a>,
    code_spans: &[InlineCodeSpan],
) -> Vec<InlineLink<'a>> {
    let mut links = Vec::new();
    let mut cursor = 0;
    while let Some(relative_start) = line.text[cursor..].find('<') {
        let start = cursor + relative_start;
        let absolute_start = line.content_range.start + start;
        cursor = start + 1;
        if inside_code_span(code_spans, line_index, absolute_start) {
            continue;
        }
        let Some(close_relative) = line.text[cursor..].find('>') else {
            break;
        };
        let close = cursor + close_relative;
        let destination = &line.text[cursor..close];
        if !(destination.starts_with("http://") || destination.starts_with("https://")) {
            continue;
        }
        if destination.chars().any(char::is_whitespace) {
            continue;
        }
        links.push(InlineLink {
            line: line_index,
            kind: InlineLinkKind::AutoLink,
            text: None,
            label: None,
            destination: Some(destination),
            text_range: None,
            label_range: None,
            destination_range: Some(SourceRange {
                start: line.content_range.start + cursor,
                end: line.content_range.start + close,
            }),
            full_range: SourceRange {
                start: absolute_start,
                end: line.content_range.start + close + 1,
            },
        });
        cursor = close + 1;
    }
    links
}
