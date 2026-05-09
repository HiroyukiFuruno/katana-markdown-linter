use super::error::ConfigErrorKind;
use crate::rules::markdown::RulePropertyType;
use serde_json::Value;
use std::collections::HashMap;

pub(super) fn validate_property_value(
    prop_type: RulePropertyType,
    value: &Value,
) -> Option<ConfigErrorKind> {
    match prop_type {
        RulePropertyType::Boolean => expected_type(value.is_boolean(), "boolean", value),
        RulePropertyType::Number => expected_type(value.is_number(), "number", value),
        RulePropertyType::NumberOrNumberArray => expected_type(
            value.is_number()
                || value
                    .as_array()
                    .map(|items| items.iter().all(Value::is_number))
                    .unwrap_or(false),
            "number or number array",
            value,
        ),
        RulePropertyType::String => expected_type(value.is_string(), "string", value),
        RulePropertyType::StringArray => expected_type(
            value
                .as_array()
                .map(|items| items.iter().all(Value::is_string))
                .unwrap_or(false),
            "string array",
            value,
        ),
        RulePropertyType::Enum(options) => validate_enum_value(options, value),
    }
}

pub(super) fn resolve_rule_key<'a>(
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

pub(super) fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn expected_type(valid: bool, expected: &'static str, value: &Value) -> Option<ConfigErrorKind> {
    if valid {
        return None;
    }
    Some(ConfigErrorKind::InvalidType {
        expected,
        actual: value_kind(value),
    })
}

fn validate_enum_value(options: &'static [&'static str], value: &Value) -> Option<ConfigErrorKind> {
    let Some(actual) = value.as_str() else {
        return Some(ConfigErrorKind::InvalidType {
            expected: "string",
            actual: value_kind(value),
        });
    };
    if options.contains(&actual) {
        return None;
    }
    Some(ConfigErrorKind::InvalidEnumValue {
        allowed: options.to_vec(),
        actual: actual.to_string(),
    })
}
