use super::parameters::{parse_parameter_block, ParsedProperty};
use super::tokens::parse_backtick_list;

pub(super) struct ParsedDocument<Property> {
    pub(super) id: String,
    pub(super) summary: String,
    pub(super) name: String,
    pub(super) aliases: Vec<String>,
    pub(super) properties: Vec<Property>,
    pub(super) fixable: bool,
}

pub(super) fn parse_document<Property>(
    source: &str,
    tags: Option<&mut Vec<String>>,
    make_property: impl Fn(ParsedProperty, &str) -> Property,
) -> Result<ParsedDocument<Property>, String> {
    let header = source
        .lines()
        .next()
        .ok_or_else(|| "rule document is empty".to_string())?;
    let (id, summary) = parse_header(header)?;
    let (aliases, parameter_lines) = parse_metadata(source, tags);
    let name = aliases
        .first()
        .cloned()
        .unwrap_or_else(|| summary.to_lowercase());
    let fixable = source.contains("Fixable: Some violations can be fixed")
        || source.contains("automatically fixable")
        || source.contains("can be fixed");
    Ok(ParsedDocument {
        id,
        summary,
        name,
        aliases,
        properties: parse_parameter_block(&parameter_lines, make_property),
        fixable,
    })
}

fn parse_metadata(source: &str, mut tags: Option<&mut Vec<String>>) -> (Vec<String>, Vec<String>) {
    let mut aliases = Vec::new();
    let mut parameter_lines = Vec::new();
    let mut in_parameters = false;

    for line in source.lines().skip(1) {
        if let (Some(tags), Some(value)) = (tags.as_deref_mut(), line.strip_prefix("Tags:")) {
            *tags = parse_backtick_list(value);
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
        collect_parameter_line(line, &mut in_parameters, &mut parameter_lines);
    }
    (aliases, parameter_lines)
}

fn collect_parameter_line(line: &str, in_parameters: &mut bool, parameter_lines: &mut Vec<String>) {
    if !*in_parameters {
        return;
    }
    if line.starts_with("- `") || line.starts_with("  ") {
        parameter_lines.push(line.to_string());
    } else if !line.trim().is_empty() {
        *in_parameters = false;
    }
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
