#[path = "build/meta_writer.rs"]
mod meta_writer;
#[path = "build/rule_doc.rs"]
mod rule_doc;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=upstream_docs");

    let docs_dir = Path::new("upstream_docs");
    if !docs_dir.exists() {
        return;
    }

    let mut rules = load_rules(docs_dir);
    rules.sort_by(|a, b| a.id.cmp(&b.id));

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_meta.rs");
    fs::write(dest_path, meta_writer::generated_meta(&rules)).unwrap();
}

fn load_rules(docs_dir: &Path) -> Vec<rule_doc::UpstreamRuleDocument> {
    let mut rules = Vec::new();
    for entry in fs::read_dir(docs_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        let Ok(document) = rule_doc::parse_rule_document(&source) else {
            continue;
        };
        rules.push(document);
    }
    rules
}
