use super::tokens::parse_backtick_list;
use super::types::UpstreamRuleProperty;

pub(super) fn parse_parameter_block(lines: &[String]) -> Vec<UpstreamRuleProperty> {
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
    Some(UpstreamRuleProperty {
        key: key.to_string(),
        value_type,
        default_value,
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
