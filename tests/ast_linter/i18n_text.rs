use std::path::Path;

pub(crate) fn read_to_string(path: &Path, violations: &mut Vec<String>) -> String {
    match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            violations.push(format!("{}: {err}", path.display()));
            String::new()
        }
    }
}

pub(crate) fn prose_without_code(content: &str) -> String {
    let mut prose = String::new();
    let mut in_code = false;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if !in_code {
            prose.push_str(line.trim());
            prose.push('\n');
        }
    }
    prose
}

pub(crate) fn has_expected_script(locale_code: &str, content: &str) -> bool {
    match locale_code {
        "ja" => content.chars().any(is_japanese),
        "zh-CN" | "zh-TW" => content.chars().any(is_cjk),
        "ko" => content.chars().any(is_hangul),
        _ => true,
    }
}

fn is_japanese(ch: char) -> bool {
    ('\u{3040}'..='\u{30ff}').contains(&ch) || is_cjk(ch)
}

fn is_cjk(ch: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&ch)
}

fn is_hangul(ch: char) -> bool {
    ('\u{ac00}'..='\u{d7af}').contains(&ch)
}
