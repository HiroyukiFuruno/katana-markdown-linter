use super::error::{ConfigError, ConfigErrorKind};
use super::property::{resolve_rule_key, validate_property_value, value_kind};
use super::types::MarkdownLintConfig;
use crate::rules::markdown::MarkdownRule;
use std::collections::HashMap;

impl MarkdownLintConfig {
    pub fn validate(&self, rules: &[Box<dyn MarkdownRule>]) -> Vec<ConfigError> {
        self.validate_rule_iter(rules.iter().map(|rule| rule.as_ref()))
    }

    pub fn validate_cached_rules(&self) -> Vec<ConfigError> {
        self.validate_meta_map(
            crate::rules::markdown::MarkdownLinterOps::user_configurable_rule_meta_map(),
        )
    }

    fn validate_rule_iter<'a>(
        &self,
        rules: impl IntoIterator<Item = &'a dyn MarkdownRule>,
    ) -> Vec<ConfigError> {
        let mut rule_map = HashMap::new();
        for rule in rules {
            if let Some(meta) = rule.official_meta() {
                rule_map.insert(meta.code, meta);
            }
        }
        self.validate_meta_map(&rule_map)
    }

    fn validate_meta_map(
        &self,
        rule_map: &HashMap<&'static str, crate::rules::markdown::OfficialRuleMeta>,
    ) -> Vec<ConfigError> {
        let mut errors = Vec::new();
        let Some(root) = self.raw.as_object() else {
            errors.push(ConfigError::new(
                None,
                None,
                ConfigErrorKind::InvalidRoot,
                "config root must be a JSON object",
            ));
            return errors;
        };

        for (rule_id, rule_value) in root {
            collect_rule_errors(rule_id, rule_value, rule_map, &mut errors);
        }
        errors
    }
}

fn collect_rule_errors(
    rule_id: &str,
    rule_value: &serde_json::Value,
    rule_map: &HashMap<&'static str, crate::rules::markdown::OfficialRuleMeta>,
    errors: &mut Vec<ConfigError>,
) {
    if rule_id == "default" {
        collect_default_errors(rule_id, rule_value, errors);
        return;
    }

    let Some((canonical_rule_id, meta)) = resolve_rule_key(rule_id, rule_map) else {
        errors.push(ConfigError::new(
            Some(rule_id.to_string()),
            None,
            ConfigErrorKind::UnknownRule,
            "unknown markdownlint rule",
        ));
        return;
    };

    match rule_value {
        serde_json::Value::Bool(_) => {}
        serde_json::Value::Object(properties) => {
            for (prop_key, prop_value) in properties {
                collect_property_error(canonical_rule_id, meta, prop_key, prop_value, errors);
            }
        }
        other => errors.push(ConfigError::new(
            Some(rule_id.to_string()),
            None,
            ConfigErrorKind::InvalidType {
                expected: "boolean or object",
                actual: value_kind(other),
            },
            "rule config must be a boolean or object",
        )),
    }
}

fn collect_default_errors(
    rule_id: &str,
    rule_value: &serde_json::Value,
    errors: &mut Vec<ConfigError>,
) {
    if rule_value.is_boolean() {
        return;
    }
    errors.push(ConfigError::new(
        Some(rule_id.to_string()),
        None,
        ConfigErrorKind::InvalidType {
            expected: "boolean",
            actual: value_kind(rule_value),
        },
        "default must be a boolean",
    ));
}

fn collect_property_error(
    rule_id: &'static str,
    meta: &crate::rules::markdown::OfficialRuleMeta,
    prop_key: &str,
    prop_value: &serde_json::Value,
    errors: &mut Vec<ConfigError>,
) {
    let Some(prop_meta) = meta
        .properties
        .iter()
        .find(|property| property.key == prop_key)
    else {
        errors.push(ConfigError::new(
            Some(rule_id.to_string()),
            Some(prop_key.to_string()),
            ConfigErrorKind::UnknownProperty,
            "unknown rule property",
        ));
        return;
    };

    if let Some(kind_err) = validate_property_value(prop_meta.prop_type, prop_value) {
        errors.push(ConfigError::new(
            Some(rule_id.to_string()),
            Some(prop_key.to_string()),
            kind_err,
            "invalid rule property value",
        ));
    }
}
