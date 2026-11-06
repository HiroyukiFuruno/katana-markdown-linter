use super::scan::{inside_code_span, skip_ascii_whitespace};
mod attributes;
use super::types::{InlineCodeSpan, InlineHtmlElement};
use crate::rules::markdown::document::{LineInfo, SourceRange};
use attributes::parse_attributes;
pub(in crate::rules::markdown) fn extract_inline_html_elements<'a>(
    lines: &[LineInfo<'a>],
    code_line_flags: &[bool],
    code_spans: &[InlineCodeSpan],
) -> Vec<InlineHtmlElement<'a>> {
    let mut elements = Vec::new();
    for (line_index, line) in lines.iter().enumerate() {
        if code_line_flags[line_index] {
            continue;
        }
        elements.extend(html_elements_on_line(line_index, line, code_spans));
    }
    elements
}
fn html_elements_on_line<'a>(
    line_index: usize,
    line: &LineInfo<'a>,
    code_spans: &[InlineCodeSpan],
) -> Vec<InlineHtmlElement<'a>> {
    let mut elements = Vec::new();
    let mut cursor = 0;
    while cursor < line.text.len() {
        let Some(relative_start) = line.text[cursor..].find('<') else {
            break;
        };
        let tag_start = cursor + relative_start;
        let absolute_start = line.content_range.start + tag_start;
        if inside_code_span(code_spans, absolute_start) {
            cursor = tag_start + 1;
            continue;
        }
        if let Some(element) = parse_html_element(line_index, line, tag_start) {
            cursor = element.full_range.end - line.content_range.start;
            elements.push(element);
        } else {
            cursor = tag_start + 1;
        }
    }
    elements
}

fn parse_html_element<'a>(
    line_index: usize,
    line: &LineInfo<'a>,
    tag_start: usize,
) -> Option<InlineHtmlElement<'a>> {
    let (closing, name_start, name_end) = parse_tag_name(line.text, tag_start)?;
    let tag_end = unquoted_tag_end(line.text, name_end)?;
    let attributes = if closing {
        Vec::new()
    } else {
        parse_attributes(line, name_end, tag_end)
    };
    Some(InlineHtmlElement {
        line: line_index,
        name: &line.text[name_start..name_end],
        name_range: SourceRange {
            start: line.content_range.start + name_start,
            end: line.content_range.start + name_end,
        },
        attributes,
        full_range: SourceRange {
            start: line.content_range.start + tag_start,
            end: line.content_range.start + tag_end + 1,
        },
        closing,
    })
}

fn parse_tag_name(text: &str, tag_start: usize) -> Option<(bool, usize, usize)> {
    let content_start = tag_start + 1;
    let mut cursor = content_start;
    let closing = text.as_bytes().get(cursor) == Some(&b'/');
    if closing {
        cursor += 1;
    }
    cursor = skip_ascii_whitespace(text, cursor, text.len());
    let name_start = cursor;
    let name_end = consume_name(text, name_start, text.len())?;
    if !tag_name_boundary(text, name_end) {
        return None;
    }
    Some((closing, name_start, name_end))
}

fn unquoted_tag_end(line: &str, start: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut cursor = start;
    let mut quote = None;
    while cursor < bytes.len() {
        match (quote, bytes[cursor]) {
            (None, b'"' | b'\'') => quote = Some(bytes[cursor]),
            (Some(current), byte) if current == byte => quote = None,
            (None, b'>') => return Some(cursor),
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn consume_name(line: &str, start: usize, end: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    if start >= end || !bytes[start].is_ascii_alphabetic() {
        return None;
    }
    let mut cursor = start + 1;
    while cursor < end
        && (bytes[cursor].is_ascii_alphanumeric() || matches!(bytes[cursor], b'-' | b'_' | b':'))
    {
        cursor += 1;
    }
    Some(cursor)
}

fn tag_name_boundary(line: &str, name_end: usize) -> bool {
    let Some(byte) = line.as_bytes().get(name_end) else {
        return true;
    };
    match *byte {
        b'/' => name_end > 0 && line.as_bytes()[name_end - 1] != b':',
        b'>' => true,
        other => other.is_ascii_whitespace(),
    }
}
