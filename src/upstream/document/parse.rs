use super::examples::parse_fenced_examples;
use super::types::{UpstreamCatalog, UpstreamRuleDocument, UpstreamRuleProperty};
use std::fs;
use std::path::Path;

const RULE_DOC_FILE_NAME_LEN: usize = 8;
const RULE_DOC_NUMBER_END: usize = 5;

pub(super) fn parse_rule_document(source: &str) -> Result<UpstreamRuleDocument, String> {
    let mut tags = Vec::new();
    let parsed =
        super::common::parse_document(source, Some(&mut tags), |property, _description| {
            UpstreamRuleProperty {
                key: property.key,
                value_type: property.value_type,
                default_value: property.default_value,
                values: property.values,
            }
        })?;
    Ok(UpstreamRuleDocument {
        id: parsed.id,
        name: parsed.name,
        summary: parsed.summary,
        tags,
        aliases: parsed.aliases,
        properties: parsed.properties,
        examples: parse_fenced_examples(source),
        fixable: parsed.fixable.then_some(true),
        source_path: None,
    })
}

pub(super) fn load_catalog_from_dir(doc_dir: &Path) -> Result<UpstreamCatalog, String> {
    let mut rules = Vec::new();
    for entry in fs::read_dir(doc_dir).map_err(|err| format!("{}: {err}", doc_dir.display()))? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        if !is_rule_doc(&path) {
            continue;
        }
        let source =
            fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
        let mut document = parse_rule_document(&source)?;
        document.source_path = Some(path);
        rules.push(document);
    }
    rules.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(UpstreamCatalog {
        source: "DavidAnson/markdownlint default branch".to_string(),
        rules,
    })
}

fn is_rule_doc(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    file_name.len() == RULE_DOC_FILE_NAME_LEN
        && file_name.starts_with("md")
        && file_name.ends_with(".md")
        && file_name[2..RULE_DOC_NUMBER_END]
            .chars()
            .all(|ch| ch.is_ascii_digit())
}
