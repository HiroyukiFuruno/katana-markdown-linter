use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleDocument {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub properties: Vec<UpstreamRuleProperty>,
    pub examples: Vec<UpstreamRuleExample>,
    pub fixable: Option<bool>,
    pub source_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleExample {
    pub language: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleProperty {
    pub key: String,
    pub value_type: String,
    pub default_value: Option<String>,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamCatalog {
    pub source: String,
    pub rules: Vec<UpstreamRuleDocument>,
}

pub fn parse_rule_document(source: &str) -> Result<UpstreamRuleDocument, String> {
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
        if in_parameters {
            if line.starts_with("- `") || line.starts_with("  ") {
                parameter_lines.push(line.to_string());
            } else if !line.trim().is_empty() {
                in_parameters = false;
            }
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

pub fn load_catalog_from_dir(doc_dir: &Path) -> Result<UpstreamCatalog, String> {
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

fn parse_fenced_examples(source: &str) -> Vec<UpstreamRuleExample> {
    let mut examples = Vec::new();
    let mut in_fence = false;
    let mut language = None;
    let mut content = String::new();

    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("```") {
            if in_fence {
                examples.push(UpstreamRuleExample {
                    language: language.take(),
                    content: content.trim_end_matches('\n').to_string(),
                });
                content.clear();
                in_fence = false;
            } else {
                let lang = rest.trim();
                language = if lang.is_empty() {
                    None
                } else {
                    Some(lang.to_string())
                };
                in_fence = true;
            }
            continue;
        }
        if in_fence {
            content.push_str(line);
            content.push('\n');
        }
    }
    examples
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

pub(super) fn normalize_type(value: &str) -> String {
    let value = value.trim().trim_matches('`');
    match value {
        "number" => "integer".to_string(),
        "string array" | "string[]" => "array".to_string(),
        other => other.to_string(),
    }
}

fn is_rule_doc(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    file_name.len() == 8
        && file_name.starts_with("md")
        && file_name.ends_with(".md")
        && file_name[2..5].chars().all(|ch| ch.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rule_document_parameters() {
        let document = parse_rule_document(
            r#"# `MD060` - Table column style

Tags: `table`

Aliases: `table-column-style`

Parameters:

- `aligned_delimiter`: Aligned delimiter columns (`boolean`, default `false`)
- `style`: Table column style (`string`, default `any`, values `aligned` /
  `any` / `compact` / `tight`)
"#,
        )
        .expect("document should parse");

        assert_eq!(document.id, "MD060");
        assert_eq!(document.name, "table-column-style");
        assert_eq!(document.tags, vec!["table"]);
        assert_eq!(document.properties.len(), 2);
        assert_eq!(
            document.properties[1].values,
            vec!["aligned", "any", "compact", "tight"]
        );
    }

    #[test]
    fn parses_rule_document_fenced_examples() {
        let document = parse_rule_document(
            r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

This is not a fixture.

```markdown
# Heading

text
```

```json
{ "MD999": false }
```
"#,
        )
        .expect("document should parse");

        assert_eq!(document.examples.len(), 2);
        assert_eq!(document.examples[0].language.as_deref(), Some("markdown"));
        assert_eq!(document.examples[0].content, "# Heading\n\ntext");
        assert_eq!(document.examples[1].language.as_deref(), Some("json"));
    }
}
