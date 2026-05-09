use crate::rules::markdown::DocumentContext;
use std::collections::HashSet;

const INDENTED_CODE_PREFIX: &str = "    ";

pub(super) fn indented_code_line_indexes(ctx: &DocumentContext<'_>) -> HashSet<usize> {
    let mut indexes = HashSet::new();
    let mut in_block = false;
    for (line_index, line) in ctx.lines().iter().enumerate() {
        if ctx.is_code_line(line_index) {
            in_block = false;
            continue;
        }
        if line.text.trim().is_empty() {
            continue;
        }
        if !line.text.starts_with(INDENTED_CODE_PREFIX) {
            in_block = false;
            continue;
        }
        if is_list_marker_line(&line.text[INDENTED_CODE_PREFIX.len()..])
            || is_definition_list_continuation(ctx, line_index)
        {
            in_block = false;
            continue;
        }
        if in_block || previous_line_allows_indented_code(ctx, line_index) {
            indexes.insert(line_index);
            in_block = true;
        } else {
            in_block = false;
        }
    }
    indexes
}

fn previous_line_allows_indented_code(ctx: &DocumentContext<'_>, line_index: usize) -> bool {
    line_index == 0 || ctx.lines()[line_index - 1].text.trim().is_empty()
}

fn is_definition_list_continuation(ctx: &DocumentContext<'_>, line_index: usize) -> bool {
    if line_index == 0 {
        return false;
    }
    for previous_index in (0..line_index).rev() {
        let previous = ctx.lines()[previous_index].text;
        if previous.trim().is_empty()
            || previous.starts_with(INDENTED_CODE_PREFIX)
            || ctx.is_code_line(previous_index)
        {
            continue;
        }
        return previous.trim_start().starts_with(':');
    }
    false
}

fn is_list_marker_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("- ")
        || trimmed.starts_with("* ")
        || trimmed.starts_with("+ ")
        || trimmed == "-"
        || trimmed == "*"
        || trimmed == "+"
    {
        return true;
    }
    let digit_count = trimmed
        .bytes()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    digit_count > 0
        && trimmed
            .get(digit_count..)
            .is_some_and(|rest| rest.starts_with(". ") || rest.starts_with(") "))
}
