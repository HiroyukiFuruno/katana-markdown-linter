use crate::rules::markdown::document::{LineInfo, SourceRange};
use crate::rules::markdown::DocumentContext;
use std::collections::HashSet;

pub(super) struct ShortcutReference<'a> {
    pub(super) label: &'a str,
    pub(super) range: SourceRange,
}

pub(super) fn shortcut_references<'a>(
    ctx: &DocumentContext<'a>,
    indented_code_lines: &HashSet<usize>,
) -> Vec<ShortcutReference<'a>> {
    let definition_lines = ctx
        .reference_definitions()
        .iter()
        .map(|definition| definition.line)
        .collect::<HashSet<_>>();
    let mut references = Vec::new();
    for (line_index, line) in ctx.lines().iter().enumerate() {
        if ctx.is_code_line(line_index)
            || indented_code_lines.contains(&line_index)
            || definition_lines.contains(&line_index)
        {
            continue;
        }
        references.extend(shortcut_references_on_line(ctx, line));
    }
    references
}

fn shortcut_references_on_line<'a>(
    ctx: &DocumentContext<'_>,
    line: &LineInfo<'a>,
) -> Vec<ShortcutReference<'a>> {
    let bytes = line.text.as_bytes();
    let mut references = Vec::new();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\\' {
            cursor += 2;
            continue;
        }
        if bytes[cursor] == b'[' && cursor > 0 && bytes[cursor - 1] == b']' {
            cursor += 1;
            continue;
        }
        let (full_start, label_open) = match bytes[cursor] {
            b'!' if bytes.get(cursor + 1) == Some(&b'[') => (cursor, cursor + 1),
            b'[' => (cursor, cursor),
            _ => {
                cursor += 1;
                continue;
            }
        };
        let Some(label_close) = matching_bracket(line.text, label_open) else {
            cursor = label_open + 1;
            continue;
        };
        let after_label = label_close + 1;
        if matches!(bytes.get(after_label), Some(b'(' | b'[' | b':')) {
            cursor = after_label;
            continue;
        }
        let label_start = label_open + 1;
        if label_start == label_close {
            cursor = after_label;
            continue;
        }
        let label = &line.text[label_start..label_close];
        if is_changelog_version_heading(line.text, label_open, after_label, label) {
            cursor = after_label;
            continue;
        }
        let range = SourceRange {
            start: line.content_range.start + full_start,
            end: line.content_range.start + after_label,
        };
        if !ctx.is_inside_inline_code(range) {
            references.push(ShortcutReference { label, range });
        }
        cursor = after_label;
    }
    references
}

fn is_changelog_version_heading(
    line: &str,
    label_open: usize,
    after_label: usize,
    label: &str,
) -> bool {
    is_heading_prefix(&line[..label_open])
        && is_version_label(label)
        && line[after_label..].trim_start().starts_with('-')
}

fn is_heading_prefix(prefix: &str) -> bool {
    let trimmed = prefix.trim_start();
    let hashes = trimmed.bytes().take_while(|byte| *byte == b'#').count();
    (1..=6).contains(&hashes) && trimmed[hashes..].trim().is_empty()
}

fn is_version_label(label: &str) -> bool {
    let raw = label.strip_prefix('v').unwrap_or(label);
    let core = raw.split(['-', '+']).next().unwrap_or(raw);
    let parts = core.split('.').collect::<Vec<_>>();
    parts.len() >= 2
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

fn matching_bracket(line: &str, open_bracket: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut cursor = open_bracket + 1;
    let mut depth = 1usize;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\\' {
            cursor += 2;
            continue;
        }
        match bytes[cursor] {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}
