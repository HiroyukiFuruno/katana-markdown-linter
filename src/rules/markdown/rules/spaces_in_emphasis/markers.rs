const MAX_EMPHASIS_MARKER_LEN: usize = 2;
const OPENING_CONTEXT_CHARS: &str = "([{\"'";
const PUNCTUATION_CONTEXT_CHARS: &str = ".!?,;:";

#[derive(Clone, Copy)]
pub(super) struct EmphasisMarker {
    pub(super) start: usize,
    pub(super) len: usize,
    pub(super) kind: char,
}

pub(super) fn emphasis_markers(line: &str) -> Vec<EmphasisMarker> {
    let mut markers = Vec::new();
    let mut chars = line.char_indices().peekable();
    while let Some((start, kind)) = chars.next() {
        if !is_emphasis_marker(kind) {
            continue;
        }
        let len = marker_run_len(kind, &mut chars);
        if len <= MAX_EMPHASIS_MARKER_LEN {
            markers.push(EmphasisMarker { start, len, kind });
        }
    }
    markers
}

fn marker_run_len(kind: char, chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>) -> usize {
    let mut len = 1;
    while let Some(&(_, next_kind)) = chars.peek() {
        if next_kind != kind {
            break;
        }
        len += 1;
        chars.next();
    }
    len
}

fn is_emphasis_marker(kind: char) -> bool {
    kind == '*' || kind == '_'
}

pub(super) fn valid_start(line: &str, markers: &[EmphasisMarker], marker_index: usize) -> bool {
    if closes_existing_emphasis(line, markers, marker_index) {
        return false;
    }
    let marker = markers[marker_index];
    let Some(previous) = line[..marker.start].chars().next_back() else {
        return true;
    };
    previous.is_whitespace()
        || OPENING_CONTEXT_CHARS.contains(previous)
        || PUNCTUATION_CONTEXT_CHARS.contains(previous)
}

fn closes_existing_emphasis(line: &str, markers: &[EmphasisMarker], marker_index: usize) -> bool {
    let marker = markers[marker_index];
    if !has_non_whitespace_before(line, marker.start) {
        return false;
    }
    markers[..marker_index]
        .iter()
        .rev()
        .find(|candidate| candidate.kind == marker.kind && candidate.len == marker.len)
        .is_some_and(|candidate| has_non_whitespace_after(line, candidate.start + candidate.len))
}

fn has_non_whitespace_before(line: &str, marker_start: usize) -> bool {
    line[..marker_start]
        .chars()
        .next_back()
        .is_some_and(|char| !char.is_whitespace())
}

fn has_non_whitespace_after(line: &str, marker_end: usize) -> bool {
    line[marker_end..]
        .chars()
        .next()
        .is_some_and(|char| !char.is_whitespace())
}

pub(super) fn matching_end_marker(
    markers: &[EmphasisMarker],
    marker_index: usize,
) -> Option<EmphasisMarker> {
    let marker = markers[marker_index];
    markers
        .iter()
        .skip(marker_index + 1)
        .find(|candidate| candidate.kind == marker.kind && candidate.len == marker.len)
        .copied()
}
