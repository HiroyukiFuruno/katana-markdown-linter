use std::fs;
use std::path::Path;

fn main() {
    let rules_dir = Path::new("../../src/rules/markdown/rules");
    for entry in fs::read_dir(rules_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") { continue; }
        
        let content = fs::read_to_string(&path).unwrap();
        if let Some(code_idx) = content.find("code: \"MD") {
            let start = code_idx + 7;
            let end = content[start..].find('"').unwrap();
            let code = &content[start..start+end];
            
            let start_str = "fn official_meta(&self) -> Option<OfficialRuleMeta> {";
            if let Some(start_idx) = content.find(start_str) {
                let mut brace_count = 0;
                let mut end_idx = 0;
                for (i, c) in content[start_idx + start_str.len() - 1..].char_indices() {
                    if c == '{' { brace_count += 1; }
                    else if c == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            end_idx = start_idx + start_str.len() - 1 + i;
                            break;
                        }
                    }
                }
                
                if end_idx > 0 {
                    let new_content = format!(
                        "{}fn official_meta(&self) -> Option<OfficialRuleMeta> {{\n        crate::rules::markdown::catalog::get_official_meta(\"{}\")\n    }}{}",
                        &content[..start_idx], code, &content[end_idx+1..]
                    );
                    fs::write(&path, new_content).unwrap();
                }
            }
        }
    }
}
