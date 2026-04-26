use super::super::scan::skip_ascii_whitespace;

pub(super) struct ParsedInlineDestination {
    pub(super) destination_start: usize,
    pub(super) destination_end: usize,
    pub(super) full_end: usize,
}

pub(super) fn parse_inline_destination(
    line: &str,
    open_paren: usize,
) -> Option<ParsedInlineDestination> {
    let close = closing_paren(line, open_paren + 1)?;
    let raw_destination_start = skip_ascii_whitespace(line, open_paren + 1, close);
    let (destination_start, destination_end) =
        if line.as_bytes().get(raw_destination_start) == Some(&b'<') {
            let destination_start = raw_destination_start + 1;
            let destination_end = line[destination_start..close]
                .find('>')
                .map(|relative| destination_start + relative)
                .unwrap_or(destination_start);
            (destination_start, destination_end)
        } else {
            (
                raw_destination_start,
                destination_end_before_title(line, raw_destination_start, close),
            )
        };
    Some(ParsedInlineDestination {
        destination_start,
        destination_end,
        full_end: close + 1,
    })
}

fn destination_end_before_title(line: &str, start: usize, close: usize) -> usize {
    let bytes = line.as_bytes();
    let mut cursor = start;
    let mut nested_parens = 0usize;
    while cursor < close {
        match bytes[cursor] {
            b'(' => nested_parens += 1,
            b')' if nested_parens > 0 => nested_parens -= 1,
            byte if byte.is_ascii_whitespace() && nested_parens == 0 => break,
            _ => {}
        }
        cursor += 1;
    }
    cursor
}

fn closing_paren(line: &str, start: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut cursor = start;
    let mut nested_parens = 0usize;
    let mut quote = None;
    while cursor < bytes.len() {
        let byte = bytes[cursor];
        if byte == b'\\' {
            cursor += 2;
            continue;
        }
        match (quote, byte) {
            (Some(current), close) if current == close => quote = None,
            (Some(_), _) => {}
            (None, b'"' | b'\'') => quote = Some(byte),
            (None, b'(') => nested_parens += 1,
            (None, b')') if nested_parens == 0 => return Some(cursor),
            (None, b')') => nested_parens -= 1,
            _ => {}
        }
        cursor += 1;
    }
    None
}
