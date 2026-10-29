use super::scan::{find_unescaped, is_escaped};
use crate::rules::markdown::document::{DocumentContext, LineInfo, SourceRange};

pub(in crate::rules::markdown) struct ReversedLink<'a> {
    pub(in crate::rules::markdown) start: usize,
    pub(in crate::rules::markdown) end: usize,
    pub(in crate::rules::markdown) text: &'a str,
    pub(in crate::rules::markdown) destination: &'a str,
}

pub(in crate::rules::markdown) fn reversed_links_on_line<'a>(
    line: &LineInfo<'a>,
    ctx: &DocumentContext<'_>,
) -> Vec<ReversedLink<'a>> {
    let mut links = Vec::new();
    let bytes = line.text.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] != b'(' || is_escaped(line.text, cursor) {
            cursor += 1;
            continue;
        }
        let Some(close_paren) = find_unescaped(line.text, cursor + 1, b')') else {
            break;
        };
        if bytes.get(close_paren + 1) != Some(&b'[') || is_escaped(line.text, close_paren + 1) {
            cursor += 1;
            continue;
        }
        let Some(close_bracket) = find_unescaped(line.text, close_paren + 2, b']') else {
            break;
        };
        let text = &line.text[cursor + 1..close_paren];
        let destination = &line.text[close_paren + 2..close_bracket];
        let source_start = line.content_range.start + cursor;
        let source_end = line.content_range.start + close_bracket + 1;
        if !ctx.overlaps_inline_code(SourceRange {
            start: source_start,
            end: source_end,
        }) && !text.trim().is_empty()
            && !destination.starts_with('^')
        {
            links.push(ReversedLink {
                start: cursor,
                end: close_bracket + 1,
                text,
                destination,
            });
        }
        cursor = close_bracket + 1;
    }
    links
}
