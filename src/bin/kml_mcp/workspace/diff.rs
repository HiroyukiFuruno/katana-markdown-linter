pub(super) fn unified_line_diff(before: &str, after: &str) -> String {
    if before == after {
        return String::new();
    }

    let before_lines = before.lines().collect::<Vec<_>>();
    let after_lines = after.lines().collect::<Vec<_>>();
    let mut output = String::new();
    output.push_str("--- before\n+++ after\n");
    let max_len = before_lines.len().max(after_lines.len());
    for index in 0..max_len {
        push_line_delta(&mut output, before_lines.get(index), after_lines.get(index));
    }
    output
}

fn push_line_delta(output: &mut String, before: Option<&&str>, after: Option<&&str>) {
    match (before, after) {
        (Some(before), Some(after)) if before == after => push_line(output, ' ', before),
        (Some(before), Some(after)) => {
            push_line(output, '-', before);
            push_line(output, '+', after);
        }
        (Some(before), None) => push_line(output, '-', before),
        (None, Some(after)) => push_line(output, '+', after),
        (None, None) => {}
    }
}

fn push_line(output: &mut String, prefix: char, line: &str) {
    output.push(prefix);
    output.push_str(line);
    output.push('\n');
}
