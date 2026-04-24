use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct UpstreamRuleDocument {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub properties: Vec<UpstreamRuleProperty>,
    pub fixable: bool,
}

#[derive(Debug, Clone)]
struct UpstreamRuleProperty {
    pub key: String,
    pub value_type: String,
    pub default_value: Option<String>,
    pub description: String,
    pub values: Vec<String>,
}

fn parse_rule_document(source: &str) -> Result<UpstreamRuleDocument, String> {
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
        if in_parameters {
            if line.starts_with("- `") || line.starts_with("  ") {
                parameter_lines.push(line.to_string());
            } else if !line.trim().is_empty() {
                in_parameters = false;
            }
        }
    }

    let properties = parse_parameter_block(&parameter_lines);
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
        summary,
        properties,
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
        } else if let Some(last) = bullets.last_mut() {
            last.push(' ');
            last.push_str(line.trim());
        }
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
        .filter(|v| !v.is_empty())
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

fn escape_for_rust_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn main() {
    println!("cargo:rerun-if-changed=upstream_docs");

    let docs_dir = Path::new("upstream_docs");
    if !docs_dir.exists() {
        return;
    }

    let mut rules = Vec::new();
    for entry in fs::read_dir(docs_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        if let Ok(document) = parse_rule_document(&source) {
            rules.push(document);
        }
    }

    rules.sort_by(|a, b| a.id.cmp(&b.id));

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_meta.rs");

    let mut out = String::new();
    out.push_str("pub fn get_official_meta(code: &str) -> Option<crate::rules::markdown::OfficialRuleMeta> {\n");
    out.push_str("    match code {\n");

    for rule in rules {
        let id_lower = rule.id.to_lowercase();
        out.push_str(&format!(
            "        \"{}\" => Some(crate::rules::markdown::OfficialRuleMeta {{\n",
            rule.id
        ));
        out.push_str(&format!("            code: \"{}\",\n", rule.id));
        out.push_str(&format!("            title: \"{}\",\n", rule.name));
        out.push_str(&format!(
            "            description: \"{}\",\n",
            escape_for_rust_str(&rule.summary)
        ));
        out.push_str(&format!("            docs_url: \"https://github.com/DavidAnson/markdownlint/blob/main/doc/{}.md\",\n", id_lower));
        out.push_str("            parity: crate::rules::markdown::RuleParityStatus::Official,\n");
        out.push_str(&format!("            is_fixable: {},\n", rule.fixable));

        out.push_str("            properties: &[\n");
        for prop in rule.properties {
            out.push_str("                crate::rules::markdown::RuleProperty {\n");
            out.push_str(&format!("                    key: \"{}\",\n", prop.key));
            out.push_str(&format!(
                "                    description: \"{}\",\n",
                escape_for_rust_str(&prop.description)
            ));

            let def_val = prop.default_value.as_deref().unwrap_or("");
            let norm_def = escape_for_rust_str(&def_val.trim().trim_matches('"').replace(' ', ""));
            out.push_str(&format!(
                "                    default_value: \"{}\",\n",
                norm_def
            ));

            let prop_type = if !prop.values.is_empty() {
                let mut vals = String::new();
                for (i, v) in prop.values.iter().enumerate() {
                    if i > 0 {
                        vals.push_str(", ");
                    }
                    vals.push_str(&format!("\"{}\"", escape_for_rust_str(v)));
                }
                format!(
                    "crate::rules::markdown::RulePropertyType::Enum(&[{}])",
                    vals
                )
            } else {
                match prop.value_type.as_str() {
                    "boolean" => "crate::rules::markdown::RulePropertyType::Boolean".to_string(),
                    "integer" => "crate::rules::markdown::RulePropertyType::Number".to_string(),
                    "array" => "crate::rules::markdown::RulePropertyType::StringArray".to_string(),
                    _ => "crate::rules::markdown::RulePropertyType::String".to_string(),
                }
            };
            out.push_str(&format!("                    prop_type: {},\n", prop_type));
            out.push_str("                },\n");
        }
        out.push_str("            ],\n");
        out.push_str("        }),\n");
    }

    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    fs::write(dest_path, out).unwrap();
}
