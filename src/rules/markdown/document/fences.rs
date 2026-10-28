use super::types::{FenceKind, FenceLineMarker};

const MIN_FENCE_MARKER_LENGTH: usize = 3;
const UNORDERED_LIST_MARKER_LEN: usize = 2;

pub(in crate::rules::markdown) fn fence_line_marker(line: &str) -> Option<FenceLineMarker> {
    let (rest, offset) = strip_blockquote_prefix(line);
    direct_fence_marker(rest, offset).or_else(|| list_item_fence_marker(rest, offset))
}

fn strip_blockquote_prefix(mut rest: &str) -> (&str, usize) {
    let mut offset = 0;
    loop {
        let spaces = leading_space_count(rest);
        rest = &rest[spaces..];
        offset += spaces;
        let Some(after_marker) = rest.strip_prefix('>') else {
            return (rest, offset);
        };
        rest = after_marker;
        offset += 1;
        if rest.starts_with(' ') {
            rest = &rest[1..];
            offset += 1;
        }
    }
}

fn list_item_fence_marker(rest: &str, offset: usize) -> Option<FenceLineMarker> {
    let marker_len = list_marker_len(rest)?;
    let after_marker = &rest[marker_len..];
    let spaces = leading_space_count(after_marker);
    direct_fence_marker(&after_marker[spaces..], offset + marker_len + spaces)
}

fn direct_fence_marker(rest: &str, offset: usize) -> Option<FenceLineMarker> {
    let (kind, length) = fence_marker(rest)?;
    Some(FenceLineMarker {
        kind,
        length,
        info_start: offset + length,
    })
}

fn fence_marker(trimmed: &str) -> Option<(FenceKind, usize)> {
    let mut chars = trimmed.chars();
    let marker = chars.next()?;
    if marker != '`' && marker != '~' {
        return None;
    }
    let length = trimmed.chars().take_while(|ch| *ch == marker).count();
    if length < MIN_FENCE_MARKER_LENGTH {
        return None;
    }
    if trimmed.starts_with("```") {
        Some((FenceKind::Backtick, length))
    } else if trimmed.starts_with("~~~") {
        Some((FenceKind::Tilde, length))
    } else {
        None
    }
}

fn list_marker_len(rest: &str) -> Option<usize> {
    let bytes = rest.as_bytes();
    if let [b'-' | b'*' | b'+', b' ', ..] = bytes {
        return Some(UNORDERED_LIST_MARKER_LEN);
    }
    let digit_count = bytes
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if digit_count == 0 {
        return None;
    }
    let marker = *bytes.get(digit_count)?;
    let space = *bytes.get(digit_count + 1)?;
    ((marker == b'.' || marker == b')') && space == b' ').then_some(digit_count + 2)
}

fn leading_space_count(input: &str) -> usize {
    input.bytes().take_while(|byte| *byte == b' ').count()
}
