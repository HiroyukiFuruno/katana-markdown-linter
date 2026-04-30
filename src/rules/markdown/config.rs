use crate::rules::markdown::{MarkdownRule, RulePropertyType};
use crate::{Error, LintOptions, RuleConfig};
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

    pub fn to_lint_options(&self) -> LintOptions {
        let mut options = LintOptions::default();
        let rule_map = crate::rules::markdown::MarkdownLinterOps::user_configurable_rule_meta_map();
        let default_enabled = self
            .raw
            .as_object()
            .and_then(|root| root.get("default"))
            .and_then(Value::as_bool)
            .unwrap_or(true);

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
            let enabled = match value {
                Value::Bool(enabled) => *enabled,
                Value::Object(properties) => properties
                    .get("enabled")
                    .and_then(Value::as_bool)
                    .unwrap_or(default_enabled),
                _ => default_enabled,
            };
            let entry = options.rules.entry(rule_id.to_string()).or_default();
            entry.enabled = enabled;
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
                    .map(|(property, value)| {
                        let value = value
                            .as_str()
                            .map(ToOwned::to_owned)
                            .unwrap_or_else(|| value.to_string());
                        (property.clone(), value)
                    })
                    .collect();
            }
        }

        options
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

            let Some((canonical_rule_id, meta)) = resolve_rule_key(rule_id, rule_map) else {
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
                                Some(canonical_rule_id.to_string()),
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
                                Some(canonical_rule_id.to_string()),
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
        RulePropertyType::NumberOrNumberArray => {
            if value.is_number()
                || value
                    .as_array()
                    .map(|items| items.iter().all(Value::is_number))
                    .unwrap_or(false)
            {
                None
            } else {
                Some(ConfigErrorKind::InvalidType {
                    expected: "number or number array",
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

fn resolve_rule_key<'a>(
    rule_id: &str,
    rule_map: &'a HashMap<&'static str, crate::rules::markdown::OfficialRuleMeta>,
) -> Option<(&'static str, &'a crate::rules::markdown::OfficialRuleMeta)> {
    if let Some(meta) = rule_map.get(rule_id) {
        return Some((meta.code, meta));
    }
    rule_map
        .values()
        .find(|meta| meta.aliases.contains(&rule_id))
        .map(|meta| (meta.code, meta))
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

    pub fn kind_code(&self) -> &'static str {
        self.kind.code()
    }

    pub fn message_id(&self) -> &'static str {
        self.kind.message_id()
    }

    pub fn message_params(&self) -> crate::i18n::MessageParams {
        let mut params = crate::i18n::MessageParams::new();
        if let Some(rule_id) = &self.rule_id {
            params.insert("rule_id".to_string(), rule_id.clone());
        }
        if let Some(property) = &self.property {
            params.insert("property".to_string(), property.clone());
        }
        match &self.kind {
            ConfigErrorKind::InvalidType { expected, actual } => {
                params.insert("expected".to_string(), (*expected).to_string());
                params.insert("actual".to_string(), (*actual).to_string());
            }
            ConfigErrorKind::InvalidEnumValue { allowed, actual } => {
                params.insert("allowed".to_string(), allowed.join(", "));
                params.insert("actual".to_string(), actual.clone());
            }
            ConfigErrorKind::InvalidRoot
            | ConfigErrorKind::UnknownRule
            | ConfigErrorKind::UnknownProperty => {}
        }
        params.insert("message".to_string(), self.message.clone());
        params
    }

    pub fn localized_message(&self, locale: crate::i18n::Locale) -> String {
        crate::i18n::render_message(
            locale,
            self.message_id(),
            &self.message_params(),
            &self.to_string(),
        )
    }

    pub fn expected(&self) -> Option<&'static str> {
        match &self.kind {
            ConfigErrorKind::InvalidType { expected, .. } => Some(expected),
            _ => None,
        }
    }

    pub fn actual(&self) -> Option<&str> {
        match &self.kind {
            ConfigErrorKind::InvalidType { actual, .. } => Some(actual),
            ConfigErrorKind::InvalidEnumValue { actual, .. } => Some(actual),
            _ => None,
        }
    }

    pub fn allowed(&self) -> Vec<&'static str> {
        match &self.kind {
            ConfigErrorKind::InvalidEnumValue { allowed, .. } => allowed.clone(),
            _ => Vec::new(),
        }
    }
}

impl ConfigErrorKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidRoot => "invalid_root",
            Self::UnknownRule => "unknown_rule",
            Self::UnknownProperty => "unknown_property",
            Self::InvalidType { .. } => "invalid_type",
            Self::InvalidEnumValue { .. } => "invalid_enum_value",
        }
    }

    pub fn message_id(&self) -> &'static str {
        match self {
            Self::InvalidRoot => "config.invalid_root",
            Self::UnknownRule => "config.unknown_rule",
            Self::UnknownProperty => "config.unknown_property",
            Self::InvalidType { .. } => "config.invalid_type",
            Self::InvalidEnumValue { .. } => "config.invalid_enum_value",
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
    fn constructors_and_missing_file_load_return_default_config() {
        let missing_path = std::env::temp_dir().join(format!(
            "katana-markdown-linter-missing-{}.json",
            std::process::id()
        ));
        let _ = fs::remove_file(&missing_path);

        assert_eq!(MarkdownLintConfig::new().raw, json!({ "default": true }));
        assert_eq!(
            MarkdownLintConfig::create_default().raw,
            json!({ "default": true })
        );
        assert_eq!(
            MarkdownLintConfig::load(&missing_path)
                .expect("missing config should load default")
                .raw,
            json!({ "default": true })
        );
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
        let unknown = errors
            .iter()
            .find(|error| matches!(error.kind, ConfigErrorKind::UnknownRule))
            .expect("unknown rule error should exist");
        assert_eq!(unknown.kind_code(), "unknown_rule");
        assert_eq!(unknown.message_id(), "config.unknown_rule");
        assert_eq!(
            unknown.localized_message(crate::i18n::Locale::Ja),
            "未知の markdownlint rule です: MD999"
        );
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD001")
                && error.property.as_deref() == Some("front_matter_title")
                && matches!(error.kind, ConfigErrorKind::InvalidType { .. })
        }));
    }

    #[test]
    fn validate_accepts_official_aliases_and_number_or_array_properties() {
        let config = MarkdownLintConfig {
            raw: json!({
                "default": false,
                "heading-increment": true,
                "first-line-h1": { "allow_preamble": true },
                "first-line-heading": false,
                "no-duplicate-heading": false,
                "no-inline-html": false,
                "MD022": {
                    "lines_above": [1, 1, 1, 1, 1, 1],
                    "lines_below": 1
                }
            }),
        };

        let rules = crate::rules::markdown::MarkdownLinterOps::get_user_configurable_rules();
        let errors = config.validate(&rules);
        assert_eq!(errors, Vec::new());

        let options = config.to_lint_options();
        assert_eq!(
            options.rules.get("MD001").map(|rule| rule.enabled),
            Some(true)
        );
        assert_eq!(
            options
                .rules
                .get("MD041")
                .and_then(|rule| rule.properties.get("allow_preamble")),
            Some(&"true".to_string())
        );
        assert_eq!(
            options.rules.get("MD024").map(|rule| rule.enabled),
            Some(false)
        );
        assert_eq!(
            options.rules.get("MD033").map(|rule| rule.enabled),
            Some(false)
        );
    }

    #[test]
    fn validate_covers_root_rule_property_and_enum_errors() {
        let rules = crate::rules::markdown::MarkdownLinterOps::get_user_configurable_rules();

        let invalid_root = MarkdownLintConfig { raw: json!(true) };
        let root_errors = invalid_root.validate(&rules);
        assert_eq!(root_errors.len(), 1);
        assert_eq!(
            root_errors[0].to_string(),
            "config root must be a JSON object"
        );

        let config = MarkdownLintConfig {
            raw: json!({
                "default": "true",
                "MD001": {
                    "front_matter_title": "^title$",
                    "unknown": true
                },
                "MD003": { "style": "invalid" },
                "MD004": { "style": "dash" },
                "MD007": {
                    "indent": 2,
                    "start_indented": false
                },
                "MD013": 80,
                "MD033": {
                    "allowed_elements": ["br"],
                    "table_allowed_elements": [1]
                },
                "MD043": {
                    "headings": null,
                    "match_case": null
                },
                "MD051": {
                    "ignore_case": true,
                    "ignored_pattern": 1
                }
            }),
        };

        let errors = config.validate(&rules);
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("default")
                && matches!(
                    error.kind,
                    ConfigErrorKind::InvalidType {
                        actual: "string",
                        ..
                    }
                )
        }));
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD001")
                && error.property.as_deref() == Some("unknown")
                && matches!(error.kind, ConfigErrorKind::UnknownProperty)
                && error.to_string() == "MD001.unknown: unknown rule property"
        }));
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD003")
                && matches!(error.kind, ConfigErrorKind::InvalidEnumValue { .. })
        }));
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD013")
                && error.property.is_none()
                && matches!(
                    error.kind,
                    ConfigErrorKind::InvalidType {
                        actual: "number",
                        ..
                    }
                )
                && error.to_string() == "MD013: rule config must be a boolean or object"
        }));
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD033")
                && error.property.as_deref() == Some("table_allowed_elements")
                && matches!(
                    error.kind,
                    ConfigErrorKind::InvalidType {
                        actual: "array",
                        ..
                    }
                )
        }));
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD043")
                && error.property.as_deref() == Some("headings")
                && matches!(
                    error.kind,
                    ConfigErrorKind::InvalidType { actual: "null", .. }
                )
        }));
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD043")
                && error.property.as_deref() == Some("match_case")
                && matches!(
                    error.kind,
                    ConfigErrorKind::InvalidType { actual: "null", .. }
                )
        }));
        assert!(errors.iter().any(|error| {
            error.rule_id.as_deref() == Some("MD051")
                && error.property.as_deref() == Some("ignored_pattern")
                && matches!(
                    error.kind,
                    ConfigErrorKind::InvalidType {
                        actual: "number",
                        ..
                    }
                )
        }));
    }

    #[test]
    fn to_lint_options_matches_markdownlint_config_semantics() {
        let config = MarkdownLintConfig {
            raw: json!({
                "default": false,
                "MD003": {
                    "enabled": false,
                    "style": "atx"
                },
                "MD007": {
                    "enabled": true,
                    "indent": 4,
                    "start_indented": false
                },
                "MD013": true,
                "MD033": {
                    "enabled": true,
                    "allowed_elements": ["br"]
                }
            }),
        };

        let options = config.to_lint_options();

        assert_eq!(
            options.rules.get("MD001").map(|rule| rule.enabled),
            Some(false)
        );
        assert_eq!(
            options.rules.get("MD013").map(|rule| rule.enabled),
            Some(true)
        );

        let md003 = options.rules.get("MD003").expect("MD003 should exist");
        assert!(!md003.enabled);
        assert_eq!(md003.properties.get("style"), Some(&"atx".to_string()));

        let md007 = options.rules.get("MD007").expect("MD007 should exist");
        assert!(md007.enabled);
        assert_eq!(md007.properties.get("indent"), Some(&"4".to_string()));
        assert_eq!(
            md007.properties.get("start_indented"),
            Some(&"false".to_string())
        );
        assert!(!md007.properties.contains_key("enabled"));

        let md033 = options.rules.get("MD033").expect("MD033 should exist");
        assert_eq!(
            md033.properties.get("allowed_elements"),
            Some(&"[\"br\"]".to_string())
        );
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
    fn parse_config_text_keeps_strings_while_stripping_comments() {
        let parsed = parse_config_text(
            "{\n  \"default\": true,\n  \"url\": \"https://example.com//not-comment\",\n  /* block comment */\n  \"escaped\": \"quote: \\\"//not-comment\\\"\",\r\n  // crlf comment\r\n  \"items\": [\"a\",],\n}",
        )
        .expect("jsonc should parse");

        assert_eq!(parsed["url"], json!("https://example.com//not-comment"));
        assert_eq!(parsed["escaped"], json!("quote: \"//not-comment\""));
        assert_eq!(parsed["items"], json!(["a"]));
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

    #[test]
    fn create_default_file_loads_existing_config() {
        let path = std::env::temp_dir().join(format!(
            "katana-markdown-linter-existing-{}.json",
            std::process::id()
        ));
        fs::write(&path, r#"{ "default": false }"#).expect("test config should write");

        let config = MarkdownLintConfig::create_default_file(&path)
            .expect("existing config should be loaded");
        assert_eq!(config.raw, json!({ "default": false }));

        let _ = fs::remove_file(&path);
    }
}
