use super::super::scan::skip_ascii_whitespace;
use super::super::types::InlineHtmlAttribute;
use super::consume_name;
use crate::rules::markdown::document::{LineInfo, SourceRange};

pub(super) fn parse_attributes<'a>(
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
        let (attribute, full_end) = parse_attribute(line, name_start, name_end, cursor, tag_end);
        attributes.push(attribute);
        cursor = full_end;
    }
    attributes
}

fn parse_attribute<'a>(
    line: &LineInfo<'a>,
    name_start: usize,
    name_end: usize,
    cursor: usize,
    tag_end: usize,
) -> (InlineHtmlAttribute<'a>, usize) {
    let (value, value_range, full_end) = parse_attribute_value(line, cursor, tag_end);
    let attribute = InlineHtmlAttribute {
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
    };
    (attribute, full_end)
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

fn consume_unquoted_value(line: &str, start: usize, end: usize) -> usize {
    let bytes = line.as_bytes();
    let mut cursor = start;
    while cursor < end && !bytes[cursor].is_ascii_whitespace() && bytes[cursor] != b'/' {
        cursor += 1;
    }
    cursor
}
