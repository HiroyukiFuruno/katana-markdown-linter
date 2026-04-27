use super::document::{UpstreamCatalog, UpstreamRuleDocument, UpstreamRuleProperty};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureMatrix {
    pub upstream_source: String,
    pub summary: RuleFixtureMatrixSummary,
    pub rules: Vec<RuleFixtureEntry>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureMatrixSummary {
    pub total_rules: usize,
    pub rules_with_examples: usize,
    pub rules_with_fix_metadata: usize,
    pub rules_with_parameters: usize,
    pub manual_required: usize,
    pub missing_fixtures: usize,
    pub stale_fixtures: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureEntry {
    pub rule_id: String,
    pub aliases: Vec<String>,
    pub tags: Vec<String>,
    pub parameters: Vec<UpstreamRuleProperty>,
    pub fixable: Option<bool>,
    pub check_pass: Vec<RuleFixtureCase>,
    pub check_fail: Vec<RuleFixtureCase>,
    pub fix: Vec<RuleFixtureCase>,
    pub config_valid: Vec<RuleFixtureCase>,
    pub config_invalid: Vec<RuleFixtureCase>,
    pub edge: Vec<RuleFixtureCase>,
    pub manual_required: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureCase {
    pub name: String,
    pub source: String,
    pub expected: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftReport {
    pub upstream_source: String,
    pub summary: RuleFixtureDriftSummary,
    pub items: Vec<RuleFixtureDriftItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftSummary {
    pub upstream_rules: usize,
    pub matrix_rules: usize,
    pub missing_fixtures: usize,
    pub stale_fixtures: usize,
    pub manual_required: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftItem {
    pub rule_id: String,
    pub drift_type: RuleFixtureDriftType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleFixtureDriftType {
    MissingFixture,
    StaleFixture,
    ManualRequired,
}

pub fn build_fixture_matrix(catalog: &UpstreamCatalog) -> RuleFixtureMatrix {
    let mut rules = catalog
        .rules
        .iter()
        .map(rule_fixture_entry)
        .collect::<Vec<_>>();
    rules.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));

    let mut summary = RuleFixtureMatrixSummary {
        total_rules: rules.len(),
        ..RuleFixtureMatrixSummary::default()
    };
    for entry in &rules {
        if !entry.check_fail.is_empty() || !entry.check_pass.is_empty() {
            summary.rules_with_examples += 1;
        }
        if entry.fixable.is_some() {
            summary.rules_with_fix_metadata += 1;
        }
        if !entry.parameters.is_empty() {
            summary.rules_with_parameters += 1;
        }
        if !entry.manual_required.is_empty() {
            summary.manual_required += 1;
        }
    }

    let mut matrix = RuleFixtureMatrix {
        upstream_source: catalog.source.clone(),
        summary,
        rules,
    };
    let drift = compare_fixture_matrix(catalog, &matrix);
    matrix.summary.missing_fixtures = drift.summary.missing_fixtures;
    matrix.summary.stale_fixtures = drift.summary.stale_fixtures;
    matrix
}

pub fn render_fixture_matrix_summary(matrix: &RuleFixtureMatrix) -> String {
    let mut out = String::new();
    out.push_str("# Rule Fixture Matrix\n\n");
    out.push_str(&format!("- total rules: {}\n", matrix.summary.total_rules));
    out.push_str(&format!(
        "- rules with examples: {}\n",
        matrix.summary.rules_with_examples
    ));
    out.push_str(&format!(
        "- rules with fix metadata: {}\n",
        matrix.summary.rules_with_fix_metadata
    ));
    out.push_str(&format!(
        "- rules with parameters: {}\n",
        matrix.summary.rules_with_parameters
    ));
    out.push_str(&format!(
        "- manual required: {}\n\n",
        matrix.summary.manual_required
    ));
    out.push_str(&format!(
        "- missing fixtures: {}\n",
        matrix.summary.missing_fixtures
    ));
    out.push_str(&format!(
        "- stale fixtures: {}\n\n",
        matrix.summary.stale_fixtures
    ));
    out.push_str("| Rule | Aliases | Parameters | Examples | Fixable | Manual Required |\n");
    out.push_str("| --- | --- | ---: | ---: | --- | --- |\n");
    for rule in &matrix.rules {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            rule.rule_id,
            rule.aliases.join(", ").replace('|', "\\|"),
            rule.parameters.len(),
            rule.check_fail.len() + rule.check_pass.len(),
            rule.fixable
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            rule.manual_required.join("<br>").replace('|', "\\|")
        ));
    }
    out
}

pub fn write_fixture_matrix_files(
    matrix: &RuleFixtureMatrix,
    output_dir: &Path,
) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|err| format!("{}: {err}", output_dir.display()))?;
    let json = serde_json::to_string_pretty(matrix).map_err(|err| err.to_string())?;
    fs::write(output_dir.join("rule-fixture-matrix.json"), json)
        .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    fs::write(
        output_dir.join("rule-fixture-matrix.md"),
        render_fixture_matrix_summary(matrix),
    )
    .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    Ok(())
}

pub fn compare_fixture_matrix(
    catalog: &UpstreamCatalog,
    matrix: &RuleFixtureMatrix,
) -> RuleFixtureDriftReport {
    let upstream_rules = catalog
        .rules
        .iter()
        .map(|rule| (rule.id.as_str(), rule))
        .collect::<BTreeMap<_, _>>();
    let matrix_rules = matrix
        .rules
        .iter()
        .map(|rule| (rule.rule_id.as_str(), rule))
        .collect::<BTreeMap<_, _>>();
    let mut items = Vec::new();

    for rule_id in upstream_rules.keys() {
        if !matrix_rules.contains_key(rule_id) {
            items.push(RuleFixtureDriftItem {
                rule_id: (*rule_id).to_string(),
                drift_type: RuleFixtureDriftType::MissingFixture,
                message: "upstream rule has no fixture matrix entry".to_string(),
            });
        }
    }

    for entry in &matrix.rules {
        if !upstream_rules.contains_key(entry.rule_id.as_str()) {
            items.push(RuleFixtureDriftItem {
                rule_id: entry.rule_id.clone(),
                drift_type: RuleFixtureDriftType::StaleFixture,
                message: "fixture matrix entry is not present in upstream docs".to_string(),
            });
        }
        if !entry.manual_required.is_empty() {
            items.push(RuleFixtureDriftItem {
                rule_id: entry.rule_id.clone(),
                drift_type: RuleFixtureDriftType::ManualRequired,
                message: entry.manual_required.join("; "),
            });
        }
    }

    let mut summary = RuleFixtureDriftSummary {
        upstream_rules: upstream_rules.len(),
        matrix_rules: matrix_rules.len(),
        ..RuleFixtureDriftSummary::default()
    };
    for item in &items {
        match item.drift_type {
            RuleFixtureDriftType::MissingFixture => summary.missing_fixtures += 1,
            RuleFixtureDriftType::StaleFixture => summary.stale_fixtures += 1,
            RuleFixtureDriftType::ManualRequired => summary.manual_required += 1,
        }
    }

    RuleFixtureDriftReport {
        upstream_source: catalog.source.clone(),
        summary,
        items,
    }
}

fn rule_fixture_entry(rule: &UpstreamRuleDocument) -> RuleFixtureEntry {
    let example_cases = rule
        .examples
        .iter()
        .enumerate()
        .map(|(index, example)| RuleFixtureCase {
            name: format!("official_example_{}", index + 1),
            source: example.content.clone(),
            expected: None,
        })
        .collect::<Vec<_>>();
    let config_valid = rule
        .properties
        .iter()
        .map(|property| RuleFixtureCase {
            name: format!("{}_valid", property.key),
            source: format!(
                "{{ \"{}\": {{ \"{}\": {} }} }}",
                rule.id,
                property.key,
                config_value_for(property)
            ),
            expected: None,
        })
        .collect::<Vec<_>>();
    let config_invalid = rule
        .properties
        .iter()
        .map(|property| RuleFixtureCase {
            name: format!("{}_invalid_type", property.key),
            source: format!("{{ \"{}\": {{ \"{}\": null }} }}", rule.id, property.key),
            expected: Some("invalid type".to_string()),
        })
        .collect::<Vec<_>>();
    let mut manual_required = Vec::new();
    if example_cases.is_empty() {
        manual_required
            .push("official document has no fenced examples for check fixtures".to_string());
    }
    if rule.fixable.unwrap_or(false) {
        manual_required.push("fix before/after fixture must be reviewed manually".to_string());
    }
    if rule.properties.is_empty() {
        manual_required.push("config fixture has no documented parameters".to_string());
    }

    RuleFixtureEntry {
        rule_id: rule.id.clone(),
        aliases: rule.aliases.clone(),
        tags: rule.tags.clone(),
        parameters: rule.properties.clone(),
        fixable: rule.fixable,
        check_pass: Vec::new(),
        check_fail: example_cases,
        fix: Vec::new(),
        config_valid,
        config_invalid,
        edge: Vec::new(),
        manual_required,
    }
}

fn config_value_for(property: &UpstreamRuleProperty) -> String {
    if let Some(default_value) = &property.default_value {
        return normalize_config_literal(default_value, &property.value_type);
    }
    match property.value_type.as_str() {
        "boolean" => "true".to_string(),
        "integer" => "1".to_string(),
        "array" => "[]".to_string(),
        _ => "\"value\"".to_string(),
    }
}

fn normalize_config_literal(value: &str, value_type: &str) -> String {
    let value = value.trim();
    let starts_with_number = value.chars().next().is_some_and(|ch| ch.is_ascii_digit());
    let is_json_literal =
        value.starts_with('[') || starts_with_number || matches!(value, "true" | "false");
    match value_type {
        "boolean" | "integer" | "array" if is_json_literal => value.to_string(),
        _ if value.starts_with('"') => value.to_string(),
        _ => format!("\"{}\"", value.replace('"', "\\\"")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::upstream::document::parse_rule_document;
    use std::fs;

    #[test]
    fn builds_fixture_matrix_with_required_schema_fields() {
        let catalog = UpstreamCatalog {
            source: "test".to_string(),
            rules: vec![parse_rule_document(
                r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

Parameters:

- `enabled`: Enable example (`boolean`, default `true`)

Fixable: Some violations can be fixed by tooling.

```markdown
bad
```
"#,
            )
            .expect("document should parse")],
        };

        let matrix = build_fixture_matrix(&catalog);
        let json = serde_json::to_value(&matrix).expect("matrix should serialize");
        let rule = &matrix.rules[0];

        assert_eq!(matrix.summary.total_rules, 1);
        assert_eq!(matrix.summary.rules_with_examples, 1);
        assert_eq!(matrix.summary.rules_with_parameters, 1);
        assert_eq!(matrix.summary.rules_with_fix_metadata, 1);
        assert_eq!(rule.rule_id, "MD999");
        assert_eq!(rule.aliases, vec!["example-rule"]);
        assert_eq!(rule.tags, vec!["test"]);
        assert_eq!(rule.parameters[0].key, "enabled");
        assert_eq!(rule.fixable, Some(true));
        assert_eq!(rule.check_fail.len(), 1);
        assert_eq!(
            rule.config_valid[0].source,
            r#"{ "MD999": { "enabled": true } }"#
        );
        assert_eq!(
            rule.config_invalid[0].expected.as_deref(),
            Some("invalid type")
        );

        for key in [
            "rule_id",
            "aliases",
            "tags",
            "parameters",
            "fixable",
            "check_pass",
            "check_fail",
            "fix",
            "config_valid",
            "config_invalid",
            "edge",
            "manual_required",
        ] {
            assert!(
                json["rules"][0].get(key).is_some(),
                "missing schema key {key}"
            );
        }
    }

    #[test]
    fn fixture_matrix_reports_missing_stale_and_manual_required_items() {
        let catalog = UpstreamCatalog {
            source: "test".to_string(),
            rules: vec![parse_rule_document(
                r#"# `MD001` - Heading increment

Tags: `headings`

Aliases: `heading-increment`
"#,
            )
            .expect("document should parse")],
        };
        let matrix = RuleFixtureMatrix {
            upstream_source: "test".to_string(),
            summary: RuleFixtureMatrixSummary::default(),
            rules: vec![RuleFixtureEntry {
                rule_id: "MD999".to_string(),
                aliases: Vec::new(),
                tags: Vec::new(),
                parameters: Vec::new(),
                fixable: None,
                check_pass: Vec::new(),
                check_fail: Vec::new(),
                fix: Vec::new(),
                config_valid: Vec::new(),
                config_invalid: Vec::new(),
                edge: Vec::new(),
                manual_required: vec!["review needed".to_string()],
            }],
        };

        let drift = compare_fixture_matrix(&catalog, &matrix);

        assert_eq!(drift.summary.missing_fixtures, 1);
        assert_eq!(drift.summary.stale_fixtures, 1);
        assert_eq!(drift.summary.manual_required, 1);
        assert!(drift
            .items
            .iter()
            .any(|item| item.drift_type == RuleFixtureDriftType::MissingFixture));
        assert!(drift
            .items
            .iter()
            .any(|item| item.drift_type == RuleFixtureDriftType::StaleFixture));
    }

    #[test]
    fn renders_and_writes_fixture_matrix_reports() {
        let catalog = UpstreamCatalog {
            source: "test".to_string(),
            rules: vec![parse_rule_document(
                r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

```markdown
bad
```
"#,
            )
            .expect("document should parse")],
        };
        let matrix = build_fixture_matrix(&catalog);
        let markdown = render_fixture_matrix_summary(&matrix);
        let dir = std::env::temp_dir().join(format!(
            "katana-markdown-linter-fixture-matrix-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);

        write_fixture_matrix_files(&matrix, &dir).expect("matrix files should be written");

        assert!(markdown.contains("Rule Fixture Matrix"));
        assert!(markdown.contains("MD999"));
        assert!(dir.join("rule-fixture-matrix.json").exists());
        assert!(dir.join("rule-fixture-matrix.md").exists());
        let _ = fs::remove_dir_all(dir);
    }
}
