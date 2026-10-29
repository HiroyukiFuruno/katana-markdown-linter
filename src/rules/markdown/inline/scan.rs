use super::types::InlineCodeSpan;

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
        if bytes[cursor] == needle && !is_escaped(line, cursor) {
            return Some(cursor);
        }
        cursor += 1;
    }
    None
}

pub(super) fn is_escaped(line: &str, offset: usize) -> bool {
    line.as_bytes()[..offset]
        .iter()
        .rev()
        .take_while(|byte| **byte == b'\\')
        .count()
        % 2
        == 1
}

/// Returns true if `offset` (document-absolute byte position) falls inside any code span.
/// Code spans are sorted by `full_range.start`, so binary search (O(log s)) is used.
pub(super) fn inside_code_span(code_spans: &[InlineCodeSpan], offset: usize) -> bool {
    let idx = code_spans.partition_point(|span| span.full_range.start <= offset);
    idx > 0 && offset < code_spans[idx - 1].full_range.end
}
