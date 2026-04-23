use crate::rules::markdown::{MarkdownRule, RulePropertyType};
use crate::Error;
use serde_json::{Map, Value};
use std::fmt;
use std::fs;
use std::path::Path;

/// Why: markdownlint config helper needs a stable JSON representation.
#[derive(Debug, Clone, PartialEq)]
pub struct MarkdownLintConfig {
    pub raw: Value,
}

impl Default for MarkdownLintConfig {
    fn default() -> Self {
        Self {
            raw: serde_json::json!({ "default": true }),
        }
    }
}

impl MarkdownLintConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(path: &Path) -> Result<Self, Error> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let raw = fs::read_to_string(path)
            .map_err(|err| Error::new(format!("failed to read config: {err}")))?;
        let parsed = serde_json::from_str(&raw)
            .map_err(|err| Error::new(format!("failed to parse config: {err}")))?;
        Ok(Self { raw: parsed })
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let json_str = serde_json::to_string_pretty(&self.raw)
            .map_err(|err| Error::new(format!("failed to serialize config: {err}")))?;
        fs::write(path, json_str)
            .map_err(|err| Error::new(format!("failed to write config: {err}")))
    }

    pub fn create_default() -> Self {
        Self::default()
    }

    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) {
        if let Some(obj) = self.raw.as_object_mut() {
            obj.insert(rule_id.to_string(), Value::Bool(enabled));
        }
    }

    pub fn set_rule_property(&mut self, rule_id: &str, prop_key: &str, value: Value) {
        if let Some(obj) = self.raw.as_object_mut() {
            if !obj.contains_key(rule_id) || !obj[rule_id].is_object() {
                obj.insert(rule_id.to_string(), Value::Object(Map::new()));
            }
            if let Some(rule_obj) = obj.get_mut(rule_id).and_then(|v| v.as_object_mut()) {
                rule_obj.insert(prop_key.to_string(), value);
            }
        }
    }

    pub fn get_rule_property(&self, rule_id: &str, prop_key: &str) -> Option<&Value> {
        self.raw
            .get(rule_id)
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get(prop_key))
    }

    pub fn validate(&self, rules: &[Box<dyn MarkdownRule>]) -> Vec<ConfigError> {
        let mut errors = Vec::new();
        let mut rule_map = std::collections::HashMap::new();
        for rule in rules {
            if let Some(meta) = rule.official_meta() {
                rule_map.insert(meta.code, meta);
            }
        }

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
            if rule_id == "default" {
                if !rule_value.is_boolean() {
                    errors.push(ConfigError::new(
                        Some(rule_id.clone()),
                        None,
                        ConfigErrorKind::InvalidType {
                            expected: "boolean",
                            actual: value_kind(rule_value),
                        },
                        "default must be a boolean",
                    ));
                }
                continue;
            }

            let Some(meta) = rule_map.get(rule_id.as_str()) else {
                errors.push(ConfigError::new(
                    Some(rule_id.clone()),
                    None,
                    ConfigErrorKind::UnknownRule,
                    "unknown markdownlint rule",
                ));
                continue;
            };

            match rule_value {
                Value::Bool(_) => continue,
                Value::Object(properties) => {
                    for (prop_key, prop_value) in properties {
                        let Some(prop_meta) = meta.properties.iter().find(|p| p.key == prop_key) else {
                            errors.push(ConfigError::new(
                                Some(rule_id.clone()),
                                Some(prop_key.clone()),
                                ConfigErrorKind::UnknownProperty,
                                "unknown rule property",
                            ));
                            continue;
                        };

                        if let Some(kind_err) = validate_property_value(prop_meta.prop_type, prop_value)
                        {
                            errors.push(ConfigError::new(
                                Some(rule_id.clone()),
                                Some(prop_key.clone()),
                                kind_err,
                                "invalid rule property value",
                            ));
                        }
                    }
                }
                other => {
                    errors.push(ConfigError::new(
                        Some(rule_id.clone()),
                        None,
                        ConfigErrorKind::InvalidType {
                            expected: "boolean or object",
                            actual: value_kind(other),
                        },
                        "rule config must be a boolean or object",
                    ));
                }
            }
        }

        errors
    }
}

fn validate_property_value(
    prop_type: RulePropertyType,
    value: &Value,
) -> Option<ConfigErrorKind> {
    match prop_type {
        RulePropertyType::Boolean => {
            if value.is_boolean() {
                None
            } else {
                Some(ConfigErrorKind::InvalidType {
                    expected: "boolean",
                    actual: value_kind(value),
                })
            }
        }
        RulePropertyType::Number => {
            if value.is_number() {
                None
            } else {
                Some(ConfigErrorKind::InvalidType {
                    expected: "number",
                    actual: value_kind(value),
                })
            }
        }
        RulePropertyType::String => {
            if value.is_string() {
                None
            } else {
                Some(ConfigErrorKind::InvalidType {
                    expected: "string",
                    actual: value_kind(value),
                })
            }
        }
        RulePropertyType::StringArray => {
            if value
                .as_array()
                .map(|items| items.iter().all(Value::is_string))
                .unwrap_or(false)
            {
                None
            } else {
                Some(ConfigErrorKind::InvalidType {
                    expected: "string array",
                    actual: value_kind(value),
                })
            }
        }
        RulePropertyType::Enum(options) => {
            if let Some(actual) = value.as_str() {
                if options.contains(&actual) {
                    None
                } else {
                    Some(ConfigErrorKind::InvalidEnumValue {
                        allowed: options.to_vec(),
                        actual: actual.to_string(),
                    })
                }
            } else {
                Some(ConfigErrorKind::InvalidType {
                    expected: "string",
                    actual: value_kind(value),
                })
            }
        }
    }
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigErrorKind {
    InvalidRoot,
    UnknownRule,
    UnknownProperty,
    InvalidType {
        expected: &'static str,
        actual: &'static str,
    },
    InvalidEnumValue {
        allowed: Vec<&'static str>,
        actual: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    pub rule_id: Option<String>,
    pub property: Option<String>,
    pub kind: ConfigErrorKind,
    pub message: String,
}

impl ConfigError {
    fn new(
        rule_id: Option<String>,
        property: Option<String>,
        kind: ConfigErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_id,
            property,
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.rule_id, &self.property) {
            (Some(rule_id), Some(property)) => {
                write!(f, "{rule_id}.{property}: {}", self.message)
            }
            (Some(rule_id), None) => write!(f, "{rule_id}: {}", self.message),
            _ => f.write_str(&self.message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid_json_object() {
        let config = MarkdownLintConfig::default();
        let root = config.raw.as_object().expect("default config must be object");
        assert_eq!(root.get("default").and_then(Value::as_bool), Some(true));
    }

    #[test]
    fn validate_reports_unknown_rules_and_type_mismatches() {
        let mut config = MarkdownLintConfig::default();
        config.set_rule_enabled("MD999", true);
        config.set_rule_property("MD001", "front_matter_title", Value::Number(1.into()));

        let rules = crate::rules::markdown::MarkdownLinterOps::get_user_configurable_rules();
        let errors = config.validate(&rules);

        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD999")
                && matches!(error.kind, ConfigErrorKind::UnknownRule)
        }));
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD001")
                && error.property.as_deref() == Some("front_matter_title")
                && matches!(error.kind, ConfigErrorKind::InvalidType { .. })
        }));
    }
}
