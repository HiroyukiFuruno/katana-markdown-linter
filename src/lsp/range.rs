use crate::fix::LineOffsetIndex;
use serde_json::{json, Value};
use std::ops::Range;

pub(super) fn selected_line_offsets(
    index: &LineOffsetIndex<'_>,
    range: &Value,
) -> Option<Range<usize>> {
    let (start_line, start_character) = request_position(range.get("start")?)?;
    let (end_line, end_character) = request_position(range.get("end")?)?;
    if (start_line, start_character) >= (end_line, end_character) {
        return None;
    }

    let start = index.line_start(start_line)?;
    let end = if end_character == 0 {
        index.offset_for_position(end_line.checked_add(1)?, 1)?
    } else {
        index.line_end_including_newline(end_line)?
    };
    (start < end).then_some(start..end)
}

fn request_position(position: &Value) -> Option<(usize, u64)> {
    let line = usize::try_from(position.get("line")?.as_u64()?).ok()?;
    Some((line, position.get("character")?.as_u64()?))
}

pub(super) fn lsp_range(index: &LineOffsetIndex<'_>, offsets: Range<usize>) -> Value {
    json!({
        "start": lsp_position(index, offsets.start),
        "end": lsp_position(index, offsets.end),
    })
}

fn lsp_position(index: &LineOffsetIndex<'_>, offset: usize) -> Value {
    let (line, character) = index.utf16_position(offset);
    json!({ "line": line, "character": character })
}
