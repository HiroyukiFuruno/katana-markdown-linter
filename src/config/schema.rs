use crate::rules::markdown::{
    MarkdownLintConfig, MarkdownLinterOps, OfficialRuleMeta, RuleProperty, RulePropertyType,
};
use serde_json::{json, Map, Value};

pub const MARKDOWNLINT_CONFIG_SCHEMA_ID: &str =
    "https://schemas.katana.tools/kml/markdownlint.schema.json";

pub fn markdownlint_config_schema() -> Value {
    ConfigSchemaGenerator::new().build()
}

impl MarkdownLintConfig {
    pub fn schema() -> Value {
        markdownlint_config_schema()
    }

    pub fn validate_against_schema(&self) -> Vec<crate::ConfigError> {
        self.validate_cached_rules()
    }
}

struct ConfigSchemaGenerator {
    rules: Vec<OfficialRuleMeta>,
}

impl ConfigSchemaGenerator {
    fn new() -> Self {
        let mut rules = MarkdownLinterOps::user_configurable_rule_meta_map()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        rules.sort_by_key(|rule| rule.code);
        Self { rules }
    }

    fn build(&self) -> Value {
        let mut properties = Map::new();
        properties.insert(
            "default".to_string(),
            json!({
                "type": "boolean",
                "default": true,
                "description": "Enable all markdownlint rules by default."
            }),
        );

        for rule in &self.rules {
            properties.insert(rule.code.to_string(), self.rule_schema(rule));
        }

        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": MARKDOWNLINT_CONFIG_SCHEMA_ID,
            "title": "katana-markdown-linter configuration",
            "description": "JSON schema for .markdownlint.json and .markdownlint.jsonc files consumed by kml.",
            "type": "object",
            "additionalProperties": false,
            "properties": properties
        })
    }

    fn rule_schema(&self, rule: &OfficialRuleMeta) -> Value {
        json!({
            "description": format!("{} See {}.", rule.description, rule.docs_url),
            "anyOf": [
                {
                    "type": "boolean",
                    "description": format!("Enable or disable {}.", rule.code)
                },
                {
                    "type": "object",
                    "additionalProperties": false,
                    "description": rule.description,
                    "properties": self.rule_properties(rule)
                }
            ]
        })
    }

    fn rule_properties(&self, rule: &OfficialRuleMeta) -> Map<String, Value> {
        let mut properties = Map::new();
        properties.insert(
            "enabled".to_string(),
            json!({
                "type": "boolean",
                "description": format!("Enable or disable {}.", rule.code)
            }),
        );
        for property in rule.properties {
            properties.insert(property.key.to_string(), property_schema(property));
        }
        properties
    }
}

fn property_schema(property: &RuleProperty) -> Value {
    let mut schema = Map::new();
    schema.insert(
        "description".to_string(),
        Value::String(property.description.to_string()),
    );
    schema.insert("default".to_string(), default_value(property));

    match property.prop_type {
        RulePropertyType::Boolean => {
            schema.insert("type".to_string(), Value::String("boolean".to_string()));
        }
        RulePropertyType::Number => {
            schema.insert("type".to_string(), Value::String("number".to_string()));
        }
        RulePropertyType::String => {
            schema.insert("type".to_string(), Value::String("string".to_string()));
        }
        RulePropertyType::StringArray => {
            schema.insert("type".to_string(), Value::String("array".to_string()));
            schema.insert("items".to_string(), json!({ "type": "string" }));
        }
        RulePropertyType::Enum(options) => {
            schema.insert("type".to_string(), Value::String("string".to_string()));
            schema.insert("enum".to_string(), json!(options));
        }
    }

    Value::Object(schema)
}

fn default_value(property: &RuleProperty) -> Value {
    match property.prop_type {
        RulePropertyType::Boolean => Value::Bool(property.default_value == "true"),
        RulePropertyType::Number => property
            .default_value
            .parse::<i64>()
            .map(|value| json!(value))
            .unwrap_or(Value::Null),
        RulePropertyType::String | RulePropertyType::Enum(_) => {
            Value::String(property.default_value.to_string())
        }
        RulePropertyType::StringArray => {
            serde_json::from_str(property.default_value).unwrap_or_else(|_| json!([]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_contains_rule_property_metadata() {
        let schema = markdownlint_config_schema();

        assert_eq!(schema["properties"]["default"]["type"], "boolean");
        assert_eq!(
            schema["properties"]["MD007"]["anyOf"][1]["properties"]["indent"]["default"],
            2
        );
        assert_eq!(
            schema["properties"]["MD033"]["anyOf"][1]["properties"]["allowed_elements"]["items"]
                ["type"],
            "string"
        );
    }
}
