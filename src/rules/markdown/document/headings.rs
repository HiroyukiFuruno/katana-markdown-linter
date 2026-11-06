use super::types::{DocumentContext, Heading, LineInfo, SourceRange};

const MAX_HEADING_INDENT: usize = 3;
const MAX_ATX_HEADING_LEVEL: usize = 6;

pub(super) fn extract_headings<'a>(ctx: &DocumentContext<'a>) -> Vec<Heading<'a>> {
    ctx.lines()
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            let heading = parse_heading(index, line)?;
            (!ctx.is_literal_block_line(index)).then_some(heading)
        })
        .collect()
}

pub(in crate::rules::markdown) fn parse_heading<'a>(
    line_index: usize,
    line: &LineInfo<'a>,
) -> Option<Heading<'a>> {
    let (indent, level) = heading_marker(line.text)?;
    let trimmed = &line.text[indent..];
    let marker_start = line.content_range.start + indent;
    let body = trimmed[level..].trim_start_matches([' ', '\t']);
    let text_start = line.content_range.end - body.len();
    let text_end = heading_text_end(line, text_start);
    Some(Heading {
        line: line_index,
        level,
        text: &line.text
            [text_start - line.content_range.start..text_end - line.content_range.start],
        marker_range: SourceRange {
            start: marker_start,
            end: marker_start + level,
        },
        text_range: SourceRange {
            start: text_start,
            end: text_end,
        },
    })
}

fn heading_marker(text: &str) -> Option<(usize, usize)> {
    let indent = text.len() - text.trim_start_matches(' ').len();
    if indent > MAX_HEADING_INDENT {
        return None;
    }
    let trimmed = &text[indent..];
    let level = trimmed.bytes().take_while(|byte| *byte == b'#').count();
    if !(1..=MAX_ATX_HEADING_LEVEL).contains(&level) {
        return None;
    }
    if trimmed
        .as_bytes()
        .get(level)
        .is_some_and(|byte| !matches!(byte, b' ' | b'\t'))
    {
        return None;
    }
    Some((indent, level))
}

fn heading_text_end(line: &LineInfo<'_>, text_start: usize) -> usize {
    let body = line.text[text_start - line.content_range.start..].trim_end_matches([' ', '\t']);
    let without_hashes = body.trim_end_matches('#');
    let text = if without_hashes.len() < body.len()
        && (without_hashes.is_empty() || without_hashes.ends_with([' ', '\t']))
    {
        without_hashes.trim_end_matches([' ', '\t'])
    } else {
        body
    };
    text_start + text.len()
}
