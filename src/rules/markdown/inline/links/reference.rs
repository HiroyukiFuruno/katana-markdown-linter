use super::super::scan::find_unescaped;
use super::super::types::{InlineLink, InlineLinkKind};
use crate::rules::markdown::document::{LineInfo, SourceRange};

pub(super) fn reference_link<'a>(
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
