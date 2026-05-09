use super::types::{BlockRange, FenceKind, FenceLineMarker, LineInfo, SourceRange};

const MIN_FENCE_MARKER_LENGTH: usize = 3;
const UNORDERED_LIST_MARKER_LEN: usize = 2;

pub(super) fn extract_code_blocks(lines: &[LineInfo<'_>]) -> Vec<BlockRange> {
    let mut blocks = Vec::new();
    let mut open: Option<(usize, FenceKind, usize)> = None;
    for (index, line) in lines.iter().enumerate() {
        let Some(marker) = fence_line_marker(line.text) else {
            continue;
        };
        if let Some((start, start_kind, start_length)) = open {
            if start_kind == marker.kind && marker.length >= start_length {
                blocks.push(BlockRange {
                    start_line: start,
                    end_line: index,
                    range: SourceRange {
                        start: lines[start].content_range.start,
                        end: line.full_range.end,
                    },
                    fence: marker.kind,
                });
                open = None;
            }
        } else {
            open = Some((index, marker.kind, marker.length));
        }
    }
    if let Some((start, kind, _)) = open {
        push_unclosed_block(&mut blocks, lines, start, kind);
    }
    blocks
}

pub(in crate::rules::markdown) fn fence_line_marker(line: &str) -> Option<FenceLineMarker> {
    let (rest, offset) = strip_blockquote_prefix(line);
    direct_fence_marker(rest, offset).or_else(|| list_item_fence_marker(rest, offset))
}

fn push_unclosed_block(
    blocks: &mut Vec<BlockRange>,
    lines: &[LineInfo<'_>],
    start: usize,
    kind: FenceKind,
) {
    let Some(last) = lines.last() else {
        return;
    };
    blocks.push(BlockRange {
        start_line: start,
        end_line: lines.len() - 1,
        range: SourceRange {
            start: lines[start].content_range.start,
            end: last.full_range.end,
        },
        fence: kind,
    });
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
