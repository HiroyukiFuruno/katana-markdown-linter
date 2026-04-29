use crate::rules::markdown::DocumentContext;

#[derive(Clone, Copy)]
struct ListItemContext {
    content_indent: usize,
}

pub(crate) struct ListContext;

impl ListContext {
    pub(crate) fn is_paragraph_continuation(ctx: &DocumentContext<'_>, line_index: usize) -> bool {
        let Some(line) = ctx.lines().get(line_index) else {
            return false;
        };
        let leading = leading_space_count(line.text);
        if leading == 0 || line.text.trim().is_empty() || list_content_indent(line.text).is_some() {
            return false;
        }

        let mut stack = Vec::new();
        for previous_index in 0..line_index {
            Self::update_stack(ctx, previous_index, &mut stack);
        }

        stack.iter().rev().any(|item| {
            leading >= item.content_indent && leading < item.content_indent.saturating_add(4)
        })
    }

    fn update_stack(
        ctx: &DocumentContext<'_>,
        line_index: usize,
        stack: &mut Vec<ListItemContext>,
    ) {
        let line = ctx.lines()[line_index].text;
        if ctx.is_code_line(line_index) || line.trim().is_empty() {
            return;
        }

        let leading = leading_space_count(line);
        if let Some(content_indent) = list_content_indent(line) {
            stack.retain(|item| item.content_indent <= leading);
            stack.push(ListItemContext { content_indent });
            return;
        }

        stack.retain(|item| item.content_indent <= leading);
    }
}

fn list_content_indent(line: &str) -> Option<usize> {
    let leading = leading_space_count(line);
    let trimmed = &line[leading..];
    let marker_width = list_marker_width(trimmed)?;
    let spacing = leading_space_count(&trimmed[marker_width..]);
    (spacing > 0).then_some(leading + marker_width + spacing)
}

fn list_marker_width(trimmed: &str) -> Option<usize> {
    unordered_marker_width(trimmed).or_else(|| ordered_marker_width(trimmed))
}

fn unordered_marker_width(trimmed: &str) -> Option<usize> {
    let bytes = trimmed.as_bytes();
    matches!(bytes, [b'-' | b'*' | b'+', byte, ..] if byte.is_ascii_whitespace()).then_some(1)
}

fn ordered_marker_width(trimmed: &str) -> Option<usize> {
    let digit_count = trimmed
        .as_bytes()
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if digit_count == 0 {
        return None;
    }
    let marker = *trimmed.as_bytes().get(digit_count)?;
    let space = *trimmed.as_bytes().get(digit_count + 1)?;
    ((marker == b'.' || marker == b')') && space.is_ascii_whitespace()).then_some(digit_count + 1)
}

fn leading_space_count(line: &str) -> usize {
    line.bytes().take_while(|byte| *byte == b' ').count()
}
