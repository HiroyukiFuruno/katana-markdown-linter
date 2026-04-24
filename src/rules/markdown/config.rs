use crate::rules::markdown::{MarkdownRule, RulePropertyType};
use crate::Error;
use serde_json::{Map, Value};
use std::collections::HashMap;
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
        let parsed = parse_config_text(&raw)?;
        Ok(Self { raw: parsed })
    }

    pub fn create_default_file(path: &Path) -> Result<Self, Error> {
        if path.exists() {
            return Self::load(path);
        }

        let config = Self::default();
        config.save(path)?;
        Ok(config)
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
        let mut rule_map = std::collections::HashMap::new();
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
                        let Some(prop_meta) = meta.properties.iter().find(|p| p.key == prop_key)
                        else {
                            errors.push(ConfigError::new(
                                Some(rule_id.clone()),
                                Some(prop_key.clone()),
                                ConfigErrorKind::UnknownProperty,
                                "unknown rule property",
                            ));
                            continue;
                        };

                        if let Some(kind_err) =
                            validate_property_value(prop_meta.prop_type, prop_value)
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

fn validate_property_value(prop_type: RulePropertyType, value: &Value) -> Option<ConfigErrorKind> {
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

fn parse_config_text(raw: &str) -> Result<Value, Error> {
    let without_comments = strip_jsonc_comments(raw);
    let normalized = strip_trailing_commas(&without_comments);
    serde_json::from_str(&normalized)
        .map_err(|err| Error::new(format!("failed to parse config: {err}")))
}

fn strip_jsonc_comments(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut escape = false;

    while let Some(ch) = chars.next() {
        if in_string {
            output.push(ch);
            if escape {
                escape = false;
                continue;
            }
            match ch {
                '\\' => escape = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                output.push(ch);
            }
            '/' if matches!(chars.peek(), Some('/')) => {
                chars.next();
                for comment_ch in chars.by_ref() {
                    if comment_ch == '\n' {
                        output.push('\n');
                        break;
                    }
                    if comment_ch == '\r' {
                        output.push('\r');
                        if matches!(chars.peek(), Some('\n')) {
                            output.push('\n');
                            chars.next();
                        }
                        break;
                    }
                }
            }
            '/' if matches!(chars.peek(), Some('*')) => {
                chars.next();
                let mut prev = '\0';
                for comment_ch in chars.by_ref() {
                    if prev == '*' && comment_ch == '/' {
                        break;
                    }
                    prev = comment_ch;
                }
            }
            _ => output.push(ch),
        }
    }

    output
}

fn strip_trailing_commas(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    let mut escape = false;

    while i < chars.len() {
        let ch = chars[i];
        if in_string {
            output.push(ch);
            if escape {
                escape = false;
            } else {
                match ch {
                    '\\' => escape = true,
                    '"' => in_string = false,
                    _ => {}
                }
            }
            i += 1;
            continue;
        }

        if ch == '"' {
            in_string = true;
            output.push(ch);
            i += 1;
            continue;
        }

        if ch == ',' {
            let mut j = i + 1;
            let mut trailing = false;
            while let Some(next) = chars.get(j) {
                if next.is_whitespace() {
                    j += 1;
                    continue;
                }
                if *next == '}' || *next == ']' {
                    trailing = true;
                    break;
                }
                break;
            }
            if trailing {
                i += 1;
                continue;
            }
        }

        output.push(ch);
        i += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    #[test]
    fn default_config_is_valid_json_object() {
        let config = MarkdownLintConfig::default();
        let root = config
            .raw
            .as_object()
            .expect("default config must be object");
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

    #[test]
    fn load_supports_jsonc_comments_and_trailing_commas() {
        let path = std::env::temp_dir().join(format!(
            "katana-markdown-linter-jsonc-{}.jsonc",
            std::process::id()
        ));
        fs::write(
            &path,
            r#"{
  // default config
  "default": true,
  "MD001": {
    "front_matter_title": "^title[:=]$",
  },
}"#,
        )
        .expect("test config should write");

        let config = MarkdownLintConfig::load(&path).expect("jsonc should load");
        assert_eq!(
            config.get_rule_property("MD001", "front_matter_title"),
            Some(&json!("^title[:=]$"))
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn create_default_file_writes_default_when_missing() {
        let path = std::env::temp_dir().join(format!(
            "katana-markdown-linter-default-{}.json",
            std::process::id()
        ));
        let _ = fs::remove_file(&path);

        let config = MarkdownLintConfig::create_default_file(&path)
            .expect("default config should be created");
        assert_eq!(config.raw, json!({ "default": true }));

        let loaded = MarkdownLintConfig::load(&path).expect("created file should load");
        assert_eq!(loaded.raw, config.raw);

        let _ = fs::remove_file(&path);
    }
}
