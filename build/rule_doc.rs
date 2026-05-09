#[derive(Debug, Clone)]
pub(super) struct UpstreamRuleDocument {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) aliases: Vec<String>,
    pub(super) summary: String,
    pub(super) properties: Vec<UpstreamRuleProperty>,
    pub(super) fixable: bool,
}

#[derive(Debug, Clone)]
pub(super) struct UpstreamRuleProperty {
    pub(super) key: String,
    pub(super) value_type: String,
    pub(super) default_value: Option<String>,
    pub(super) description: String,
    pub(super) values: Vec<String>,
}

pub(super) fn parse_rule_document(source: &str) -> Result<UpstreamRuleDocument, String> {
    let mut lines = source.lines();
    let header = lines
        .next()
        .ok_or_else(|| "rule document is empty".to_string())?;
    let (id, summary) = parse_header(header)?;

    let mut aliases = Vec::new();
    let mut parameter_lines = Vec::new();
    let mut in_parameters = false;

    for line in source.lines().skip(1) {
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

    let name = aliases
        .first()
        .cloned()
        .unwrap_or_else(|| summary.to_lowercase());
    let fixable = source.contains("Fixable: Some violations can be fixed")
        || source.contains("automatically fixable")
        || source.contains("can be fixed");

    Ok(UpstreamRuleDocument {
        id,
        name,
        aliases,
        summary,
        properties: parse_parameter_block(&parameter_lines),
        fixable,
    })
}

fn parse_header(header: &str) -> Result<(String, String), String> {
    let rest = header
        .strip_prefix("# `")
        .ok_or_else(|| format!("invalid rule header: {header}"))?;
    let (id, after_id) = rest
        .split_once("` - ")
        .ok_or_else(|| format!("invalid rule header: {header}"))?;
    Ok((id.to_string(), after_id.trim().to_string()))
}

fn parse_backtick_list(value: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut remaining = value;
    while let Some(start) = remaining.find('`') {
        let after_start = &remaining[start + 1..];
        let Some(end) = after_start.find('`') else {
            break;
        };
        values.push(after_start[..end].to_string());
        remaining = &after_start[end + 1..];
    }
    values
}

fn parse_parameter_block(lines: &[String]) -> Vec<UpstreamRuleProperty> {
    let mut bullets = Vec::new();
    for line in lines {
        if line.starts_with("- `") {
            bullets.push(line.clone());
            continue;
        }

        let Some(last) = bullets.last_mut() else {
            continue;
        };
        last.push(' ');
        last.push_str(line.trim());
    }
    bullets
        .iter()
        .filter_map(|line| parse_parameter_line(line))
        .collect()
}

fn parse_parameter_line(line: &str) -> Option<UpstreamRuleProperty> {
    let rest = line.strip_prefix("- `")?;
    let (key, after_key) = rest.split_once("`:")?;
    let value_type = extract_parenthesized(after_key)
        .and_then(|inside| inside.split(',').next().map(str::trim).map(normalize_type))
        .unwrap_or_else(|| "unknown".to_string());
    let default_value = after_key
        .split("default `")
        .nth(1)
        .and_then(|rest| rest.split('`').next())
        .map(str::to_string);
    let values = after_key
        .split("values ")
        .nth(1)
        .map(parse_values)
        .unwrap_or_default();
    let description = after_key.split('(').next().unwrap_or("").trim().to_string();

    Some(UpstreamRuleProperty {
        key: key.to_string(),
        value_type,
        default_value,
        description,
        values,
    })
}

fn extract_parenthesized(value: &str) -> Option<&str> {
    let start = value.rfind('(')?;
    let end = value[start + 1..].find(')')?;
    Some(&value[start + 1..start + 1 + end])
}

fn parse_values(value: &str) -> Vec<String> {
    let backtick_values = parse_backtick_list(value);
    if !backtick_values.is_empty() {
        return backtick_values;
    }
    value
        .split('/')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}

fn normalize_type(value: &str) -> String {
    let value = value.trim().trim_matches('`');
    match value {
        "number" => "integer".to_string(),
        "string array" | "string[]" => "array".to_string(),
        other => other.to_string(),
    }
}
