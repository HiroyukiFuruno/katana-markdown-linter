pub(super) fn matching_bracket(line: &str, open_bracket: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut cursor = open_bracket + 1;
    let mut depth = 1usize;
    while cursor < bytes.len() {
        if matches!(bytes[cursor], b'[' | b']') && super::super::scan::is_escaped(line, cursor) {
            cursor += 1;
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
