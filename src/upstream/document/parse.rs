use super::examples::parse_fenced_examples;
use super::parameters::parse_parameter_block;
use super::tokens::parse_backtick_list;
use super::types::{UpstreamCatalog, UpstreamRuleDocument};
use std::fs;
use std::path::Path;

const RULE_DOC_FILE_NAME_LEN: usize = 8;
const RULE_DOC_NUMBER_END: usize = 5;

pub(super) fn parse_rule_document(source: &str) -> Result<UpstreamRuleDocument, String> {
    let mut lines = source.lines();
    let header = lines
        .next()
        .ok_or_else(|| "rule document is empty".to_string())?;
    let (id, summary) = parse_header(header)?;
    let mut tags = Vec::new();
    let mut aliases = Vec::new();
    let mut parameter_lines = Vec::new();
    let mut in_parameters = false;

    for line in source.lines().skip(1) {
        if let Some(value) = line.strip_prefix("Tags:") {
            tags = parse_backtick_list(value);
            continue;
        }
        if let Some(value) = line.strip_prefix("Aliases:") {
            aliases = parse_backtick_list(value);
            continue;
        }
        if line.trim() == "Parameters:" {
            in_parameters = true;
            continue;
        }
        if !in_parameters {
            continue;
        }
        if line.starts_with("- `") || line.starts_with("  ") {
            parameter_lines.push(line.to_string());
            continue;
        }
        if !line.trim().is_empty() {
            in_parameters = false;
        }
    }

    let properties = parse_parameter_block(&parameter_lines);
    let examples = parse_fenced_examples(source);
    let name = aliases
        .first()
        .cloned()
        .unwrap_or_else(|| summary.to_lowercase());
    let fixable = if source.contains("Fixable: Some violations can be fixed")
        || source.contains("automatically fixable")
        || source.contains("can be fixed")
    {
        Some(true)
    } else {
        None
    };

    Ok(UpstreamRuleDocument {
        id,
        name,
        summary,
        tags,
        aliases,
        properties,
        examples,
        fixable,
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

fn parse_header(header: &str) -> Result<(String, String), String> {
    let Some(rest) = header.strip_prefix("# `") else {
        return Err(format!("invalid rule header: {header}"));
    };
    let Some((id, after_id)) = rest.split_once("` - ") else {
        return Err(format!("invalid rule header: {header}"));
    };
    Ok((id.to_string(), after_id.trim().to_string()))
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
