use super::types::InlineCodeSpan;
use crate::rules::markdown::document::BlockRange;

pub(super) fn skip_ascii_whitespace(line: &str, start: usize, end: usize) -> usize {
    let mut cursor = start;
    while cursor < end
        && line
            .as_bytes()
            .get(cursor)
            .is_some_and(u8::is_ascii_whitespace)
    {
        cursor += 1;
    }
    cursor
}

pub(super) fn find_unescaped(line: &str, start: usize, needle: u8) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut cursor = start;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\\' {
            cursor += 2;
            continue;
        }
        if bytes[cursor] == needle {
            return Some(cursor);
        }
        cursor += 1;
    }
    None
}

pub(super) fn inside_code_span(
    code_spans: &[InlineCodeSpan],
    line_index: usize,
    offset: usize,
) -> bool {
    code_spans.iter().any(|span| {
        span.line == line_index && span.full_range.start <= offset && offset < span.full_range.end
    })
}

pub(super) fn line_in_blocks(line_index: usize, blocks: &[BlockRange]) -> bool {
    blocks
        .iter()
        .any(|block| (block.start_line..=block.end_line).contains(&line_index))
}
