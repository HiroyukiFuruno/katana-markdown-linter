mod jsonc;

use super::*;
use serde_json::{json, Value};
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
            "MD007": { "indent": 2, "start_indented": false },
            "MD013": 80,
            "MD033": {
                "allowed_elements": ["br"],
                "table_allowed_elements": [1]
            },
            "MD043": { "headings": null, "match_case": null },
            "MD051": { "ignore_case": true, "ignored_pattern": 1 }
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
            "MD003": { "enabled": false, "style": "atx" },
            "MD007": { "enabled": true, "indent": 4, "start_indented": false },
            "MD013": true,
            "MD033": { "enabled": true, "allowed_elements": ["br"] }
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
