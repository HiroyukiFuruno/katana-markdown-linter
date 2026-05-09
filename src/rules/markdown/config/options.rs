use super::property::{resolve_rule_key, validate_property_value};
use super::types::MarkdownLintConfig;
use crate::{LintOptions, RuleConfig};
use serde_json::Value;
use std::collections::HashMap;

impl MarkdownLintConfig {
    pub fn to_lint_options(&self) -> LintOptions {
        let mut options = LintOptions::default();
        let rule_map = crate::rules::markdown::MarkdownLinterOps::user_configurable_rule_meta_map();
        let default_enabled = self
            .raw
            .as_object()
            .and_then(|root| root.get("default"))
            .and_then(Value::as_bool)
            .unwrap_or(true);

        seed_default_rules(&mut options, default_enabled);
        let Some(root) = self.raw.as_object() else {
            return options;
        };

        for (key, value) in root {
            if key == "default" {
                continue;
            }
            let Some((rule_id, meta)) = resolve_rule_key(key, rule_map) else {
                continue;
            };
            let entry = options.rules.entry(rule_id.to_string()).or_default();
            entry.enabled = rule_enabled(value, default_enabled);
            if let Value::Object(properties) = value {
                entry.properties = properties
                    .iter()
                    .filter(|(property, _)| property.as_str() != "enabled")
                    .filter(|(property, value)| {
                        meta.properties
                            .iter()
                            .find(|meta| meta.key == property.as_str())
                            .is_some_and(|property_meta| {
                                validate_property_value(property_meta.prop_type, value).is_none()
                            })
                    })
                    .map(|(property, value)| (property.clone(), property_value(value)))
                    .collect();
            }
        }

        options
    }
}

fn seed_default_rules(options: &mut LintOptions, default_enabled: bool) {
    for rule in crate::rules::markdown::MarkdownLinterOps::user_configurable_rules() {
        if let Some(meta) = rule.official_meta() {
            options.rules.insert(
                meta.code.to_string(),
                RuleConfig {
                    enabled: default_enabled,
                    properties: HashMap::new(),
                },
            );
        }
    }
}

fn rule_enabled(value: &Value, default_enabled: bool) -> bool {
    match value {
        Value::Bool(enabled) => *enabled,
        Value::Object(properties) => properties
            .get("enabled")
            .and_then(Value::as_bool)
            .unwrap_or(default_enabled),
        _ => default_enabled,
    }
}

fn property_value(value: &Value) -> String {
    value
        .as_str()
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| value.to_string())
}
