pub(super) fn leading_spaces(line: &str) -> &str {
    let count = line.len() - line.trim_start_matches(' ').len();
    &line[..count]
}
