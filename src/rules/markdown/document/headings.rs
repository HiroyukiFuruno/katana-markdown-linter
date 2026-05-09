use super::blocks::line_in_blocks;
use super::types::{BlockRange, Heading, LineInfo, SourceRange};

const MAX_HEADING_INDENT: usize = 3;
const MAX_ATX_HEADING_LEVEL: usize = 6;

pub(super) fn extract_headings<'a>(
    lines: &[LineInfo<'a>],
    code_blocks: &[BlockRange],
) -> Vec<Heading<'a>> {
    lines
        .iter()
        .enumerate()
        .filter(|(index, _)| !line_in_blocks(*index, code_blocks))
        .filter_map(|(index, line)| parse_heading(index, line))
        .collect()
}

fn parse_heading<'a>(line_index: usize, line: &LineInfo<'a>) -> Option<Heading<'a>> {
    let indent = line.text.len() - line.text.trim_start_matches(' ').len();
    if indent > MAX_HEADING_INDENT {
        return None;
    }
    let trimmed = &line.text[indent..];
    let level = trimmed.bytes().take_while(|byte| *byte == b'#').count();
    if !(1..=MAX_ATX_HEADING_LEVEL).contains(&level) {
        return None;
    }
    if trimmed
        .as_bytes()
        .get(level)
        .is_some_and(|byte| *byte != b' ')
    {
        return None;
    }
    let marker_start = line.content_range.start + indent;
    let text_start = marker_start + level + usize::from(trimmed.len() > level);
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

fn heading_text_end(line: &LineInfo<'_>, text_start: usize) -> usize {
    let mut text_end = line.content_range.end;
    let body = &line.text[text_start - line.content_range.start..];
    if let Some(closing_start) = body.rfind(" #") {
        if body[closing_start + 1..].bytes().all(|byte| byte == b'#') {
            text_end = text_start + closing_start;
        }
    }
    text_end
}
