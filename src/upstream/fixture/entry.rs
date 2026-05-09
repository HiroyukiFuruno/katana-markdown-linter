use super::types::{RuleFixtureCase, RuleFixtureEntry};
use crate::upstream::document::{UpstreamRuleDocument, UpstreamRuleProperty};

pub(super) fn rule_fixture_entry(rule: &UpstreamRuleDocument) -> RuleFixtureEntry {
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
