use super::scan::{inside_code_span, skip_ascii_whitespace};
use super::types::{InlineCodeSpan, InlineHtmlAttribute, InlineHtmlElement};
use crate::rules::markdown::document::{LineInfo, SourceRange};
pub(crate) fn extract_inline_html_elements<'a>(
    lines: &[LineInfo<'a>],
    code_line_flags: &[bool],
    code_spans: &[InlineCodeSpan],
) -> Vec<InlineHtmlElement<'a>> {
    let mut elements = Vec::new();
    for (line_index, line) in lines.iter().enumerate() {
        if code_line_flags.get(line_index).copied().unwrap_or(false) {
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
        if inside_code_span(code_spans, line_index, absolute_start) {
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
    let content_start = tag_start + 1;
    let mut cursor = content_start;
    let closing = line.text.as_bytes().get(cursor) == Some(&b'/');
    if closing {
        cursor += 1;
    }
    cursor = skip_ascii_whitespace(line.text, cursor, line.text.len());
    let name_start = cursor;
    let name_end = consume_name(line.text, name_start, line.text.len())?;
    if !tag_name_boundary(line.text, name_end) {
        return None;
    }
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

fn parse_attributes<'a>(
    line: &LineInfo<'a>,
    start: usize,
    tag_end: usize,
) -> Vec<InlineHtmlAttribute<'a>> {
    let mut attributes = Vec::new();
    let mut cursor = start;
    while cursor < tag_end {
        cursor = skip_ascii_whitespace(line.text, cursor, tag_end);
        if cursor >= tag_end || line.text.as_bytes().get(cursor) == Some(&b'/') {
            break;
        }
        let name_start = cursor;
        let Some(name_end) = consume_name(line.text, name_start, tag_end) else {
            cursor += 1;
            continue;
        };
        cursor = skip_ascii_whitespace(line.text, name_end, tag_end);
        let (value, value_range, full_end) = parse_attribute_value(line, cursor, tag_end);
        attributes.push(InlineHtmlAttribute {
            name: &line.text[name_start..name_end],
            value,
            name_range: SourceRange {
                start: line.content_range.start + name_start,
                end: line.content_range.start + name_end,
            },
            value_range,
            full_range: SourceRange {
                start: line.content_range.start + name_start,
                end: line.content_range.start + full_end,
            },
        });
        cursor = full_end;
    }
    attributes
}

fn parse_attribute_value<'a>(
    line: &LineInfo<'a>,
    cursor: usize,
    tag_end: usize,
) -> (Option<&'a str>, Option<SourceRange>, usize) {
    if line.text.as_bytes().get(cursor) != Some(&b'=') {
        return (None, None, cursor);
    }
    let value_start = skip_ascii_whitespace(line.text, cursor + 1, tag_end);
    let Some(quote) = line.text.as_bytes().get(value_start).copied() else {
        return (None, None, value_start);
    };
    if quote != b'"' && quote != b'\'' {
        let value_end = consume_unquoted_value(line.text, value_start, tag_end);
        return (None, None, value_end);
    }
    let content_start = value_start + 1;
    let Some(close_offset) = line.text[content_start..tag_end].find(quote as char) else {
        return (None, None, tag_end);
    };
    let content_end = content_start + close_offset;
    (
        Some(&line.text[content_start..content_end]),
        Some(SourceRange {
            start: line.content_range.start + content_start,
            end: line.content_range.start + content_end,
        }),
        content_end + 1,
    )
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

fn consume_unquoted_value(line: &str, start: usize, end: usize) -> usize {
    let bytes = line.as_bytes();
    let mut cursor = start;
    while cursor < end && !bytes[cursor].is_ascii_whitespace() && bytes[cursor] != b'/' {
        cursor += 1;
    }
    cursor
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
