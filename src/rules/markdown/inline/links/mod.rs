mod autolink;
mod bracket;
mod destination;

use super::reference_definitions::reference_definition_on_line;
use super::scan::{find_unescaped, inside_code_span};
use super::types::{InlineCodeSpan, InlineLink, InlineLinkKind};
use crate::rules::markdown::document::{LineInfo, SourceRange};
use autolink::autolinks_on_line;
use bracket::matching_bracket;
use destination::parse_inline_destination;

pub(crate) fn extract_inline_links<'a>(
    lines: &[LineInfo<'a>],
    code_line_flags: &[bool],
    code_spans: &[InlineCodeSpan],
) -> Vec<InlineLink<'a>> {
    let mut links = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if code_line_flags.get(idx).copied().unwrap_or(false) {
            continue;
        }
        links.extend(markdown_links_on_line(idx, line, code_spans));
        if reference_definition_on_line(idx, line).is_none() {
            links.extend(autolinks_on_line(idx, line, code_spans));
        }
    }
    links.sort_by_key(|link| link.full_range.start);
    links
}

fn markdown_links_on_line<'a>(
    line_index: usize,
    line: &LineInfo<'a>,
    code_spans: &[InlineCodeSpan],
) -> Vec<InlineLink<'a>> {
    let mut links = Vec::new();
    let bytes = line.text.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        let Some((full_start_local, text_open_local, image)) =
            next_link_open(line, code_spans, line_index, cursor)
        else {
            break;
        };
        let text_start_local = text_open_local + 1;
        let Some(text_close_local) = matching_bracket(line.text, text_open_local) else {
            cursor = text_open_local + 1;
            continue;
        };
        let after_text = text_close_local + 1;
        if bytes.get(after_text) == Some(&b'(') {
            if let Some(link) = inline_destination_link(
                line_index,
                line,
                full_start_local,
                text_start_local,
                text_close_local,
                after_text,
                image,
            ) {
                cursor = link.full_range.end - line.content_range.start;
                links.push(link);
                continue;
            }
        } else if bytes.get(after_text) == Some(&b'[') {
            if let Some(link) = reference_link(
                line_index,
                line,
                full_start_local,
                text_start_local,
                text_close_local,
                after_text,
                image,
            ) {
                cursor = link.full_range.end - line.content_range.start;
                links.push(link);
                continue;
            }
        }
        cursor = after_text;
    }
    links
}

fn next_link_open(
    line: &LineInfo<'_>,
    code_spans: &[InlineCodeSpan],
    line_index: usize,
    cursor: usize,
) -> Option<(usize, usize, bool)> {
    let bytes = line.text.as_bytes();
    let mut scan = cursor;
    while scan < bytes.len() {
        let offset = line.content_range.start + scan;
        if inside_code_span(code_spans, line_index, offset) {
            scan += 1;
            continue;
        }
        if bytes[scan] == b'!' && bytes.get(scan + 1) == Some(&b'[') {
            return Some((scan, scan + 1, true));
        }
        if bytes[scan] == b'[' {
            return Some((scan, scan, false));
        }
        scan += 1;
    }
    None
}

fn inline_destination_link<'a>(
    line_index: usize,
    line: &LineInfo<'a>,
    full_start_local: usize,
    text_start_local: usize,
    text_close_local: usize,
    destination_open_local: usize,
    image: bool,
) -> Option<InlineLink<'a>> {
    let destination = parse_inline_destination(line.text, destination_open_local)?;
    Some(InlineLink {
        line: line_index,
        kind: if image {
            InlineLinkKind::ImageInline
        } else {
            InlineLinkKind::Inline
        },
        text: Some(&line.text[text_start_local..text_close_local]),
        label: None,
        destination: Some(&line.text[destination.destination_start..destination.destination_end]),
        text_range: Some(SourceRange {
            start: line.content_range.start + text_start_local,
            end: line.content_range.start + text_close_local,
        }),
        label_range: None,
        destination_range: Some(SourceRange {
            start: line.content_range.start + destination.destination_start,
            end: line.content_range.start + destination.destination_end,
        }),
        full_range: SourceRange {
            start: line.content_range.start + full_start_local,
            end: line.content_range.start + destination.full_end,
        },
    })
}

fn reference_link<'a>(
    line_index: usize,
    line: &LineInfo<'a>,
    full_start_local: usize,
    text_start_local: usize,
    text_close_local: usize,
    label_open_local: usize,
    image: bool,
) -> Option<InlineLink<'a>> {
    let label_start = label_open_local + 1;
    let label_close = find_unescaped(line.text, label_start, b']')?;
    let collapsed = label_start == label_close;
    let label = (!collapsed).then_some(&line.text[label_start..label_close]);
    Some(InlineLink {
        line: line_index,
        kind: reference_kind(image, collapsed),
        text: Some(&line.text[text_start_local..text_close_local]),
        label,
        destination: None,
        text_range: Some(SourceRange {
            start: line.content_range.start + text_start_local,
            end: line.content_range.start + text_close_local,
        }),
        label_range: (!collapsed).then_some(SourceRange {
            start: line.content_range.start + label_start,
            end: line.content_range.start + label_close,
        }),
        destination_range: None,
        full_range: SourceRange {
            start: line.content_range.start + full_start_local,
            end: line.content_range.start + label_close + 1,
        },
    })
}

fn reference_kind(image: bool, collapsed: bool) -> InlineLinkKind {
    match (image, collapsed) {
        (true, true) => InlineLinkKind::ImageReferenceCollapsed,
        (true, false) => InlineLinkKind::ImageReferenceFull,
        (false, true) => InlineLinkKind::ReferenceCollapsed,
        (false, false) => InlineLinkKind::ReferenceFull,
    }
}
