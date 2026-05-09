use std::fs;
use std::path::Path;

const RULE_CODE_PREFIX: &str = "code: \"";
const OFFICIAL_META_START: &str = "fn official_meta(&self) -> Option<OfficialRuleMeta> {";

fn main() {
    let rules_dir = Path::new("../../src/rules/markdown/rules");
    for entry in fs::read_dir(rules_dir).unwrap() {
        process_entry(entry.unwrap().path());
    }
}

fn process_entry(path: std::path::PathBuf) {
    if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
        return;
    }
    let content = fs::read_to_string(&path).unwrap();
    let Some(code) = rule_code(&content) else {
        return;
    };
    let Some(start_idx) = content.find(OFFICIAL_META_START) else {
        return;
    };
    let Some(end_idx) = official_meta_end(&content, start_idx) else {
        return;
    };
    let new_content = format!(
        "{}fn official_meta(&self) -> Option<OfficialRuleMeta> {{\n        crate::rules::markdown::catalog::get_official_meta(\"{}\")\n    }}{}",
        &content[..start_idx],
        code,
        &content[end_idx + 1..]
    );
    fs::write(&path, new_content).unwrap();
}

fn rule_code(content: &str) -> Option<&str> {
    let code_idx = content.find("code: \"MD")?;
    let start = code_idx + RULE_CODE_PREFIX.len();
    let end = content[start..].find('"')?;
    Some(&content[start..start + end])
}

fn official_meta_end(content: &str, start_idx: usize) -> Option<usize> {
    let mut brace_count = 0;
    let body_start = start_idx + OFFICIAL_META_START.len() - 1;
    for (offset, char) in content[body_start..].char_indices() {
        match char {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    return Some(body_start + offset);
                }
            }
            _ => {}
        }
    }
    None
}
