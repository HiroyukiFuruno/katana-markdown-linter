use super::scan::{find_unescaped, skip_ascii_whitespace};
use super::types::ReferenceDefinition;
use crate::rules::markdown::document::{LineInfo, SourceRange};

pub(crate) fn extract_reference_definitions<'a>(
    lines: &[LineInfo<'a>],
    code_line_flags: &[bool],
) -> Vec<ReferenceDefinition<'a>> {
    lines
        .iter()
        .enumerate()
        .filter(|(idx, _)| !code_line_flags[*idx])
        .filter_map(|(idx, line)| reference_definition_on_line(idx, line))
        .collect()
}

pub(super) fn reference_definition_on_line<'a>(
    line_index: usize,
    line: &LineInfo<'a>,
) -> Option<ReferenceDefinition<'a>> {
    let indent = line.text.len() - line.text.trim_start_matches(' ').len();
    if indent > 3 || line.text.as_bytes().get(indent) != Some(&b'[') {
        return None;
    }
    let label_start = indent + 1;
    let label_close = find_unescaped(line.text, label_start, b']')?;
    if line.text.as_bytes().get(label_close + 1) != Some(&b':') {
        return None;
    }

    let raw_destination_start = skip_ascii_whitespace(line.text, label_close + 2, line.text.len());
    if raw_destination_start >= line.text.len() {
        return None;
    }
    let (destination_start, destination_end) =
        if line.text.as_bytes().get(raw_destination_start) == Some(&b'<') {
            let destination_start = raw_destination_start + 1;
            let destination_end = line.text[destination_start..]
                .find('>')
                .map(|relative| destination_start + relative)?;
            (destination_start, destination_end)
        } else {
            let destination_end = line.text[raw_destination_start..]
                .find(char::is_whitespace)
                .map_or(line.text.len(), |relative| raw_destination_start + relative);
            (raw_destination_start, destination_end)
        };

    Some(ReferenceDefinition {
        line: line_index,
        label: &line.text[label_start..label_close],
        destination: &line.text[destination_start..destination_end],
        label_range: SourceRange {
            start: line.content_range.start + label_start,
            end: line.content_range.start + label_close,
        },
        destination_range: SourceRange {
            start: line.content_range.start + destination_start,
            end: line.content_range.start + destination_end,
        },
        full_range: line.content_range,
    })
}
