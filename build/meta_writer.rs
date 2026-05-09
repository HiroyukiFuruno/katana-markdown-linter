use super::rule_doc::{UpstreamRuleDocument, UpstreamRuleProperty};

pub(super) fn generated_meta(rules: &[UpstreamRuleDocument]) -> String {
    let mut output = String::new();
    output.push_str("pub fn get_official_meta(code: &str) -> Option<crate::rules::markdown::OfficialRuleMeta> {\n");
    output.push_str("    match code {\n");

    for rule in rules {
        write_rule_meta(&mut output, rule);
    }

    output.push_str("        _ => None,\n");
    output.push_str("    }\n");
    output.push_str("}\n");
    output
}

fn write_rule_meta(output: &mut String, rule: &UpstreamRuleDocument) {
    let id_lower = rule.id.to_lowercase();
    output.push_str(&format!(
        "        \"{}\" => Some(crate::rules::markdown::OfficialRuleMeta {{\n",
        rule.id
    ));
    output.push_str(&format!("            code: \"{}\",\n", rule.id));
    output.push_str(&format!("            title: \"{}\",\n", rule.name));
    output.push_str(&format!(
        "            description: \"{}\",\n",
        escape_for_rust_str(&rule.summary)
    ));
    output.push_str(&format!("            docs_url: \"https://github.com/DavidAnson/markdownlint/blob/main/doc/{}.md\",\n", id_lower));
    output.push_str("            aliases: &[\n");
    for alias in &rule.aliases {
        output.push_str(&format!(
            "                \"{}\",\n",
            escape_for_rust_str(alias)
        ));
    }
    output.push_str("            ],\n");
    output.push_str("            parity: crate::rules::markdown::RuleParityStatus::Official,\n");
    output.push_str(&format!("            is_fixable: {},\n", rule.fixable));
    output.push_str("            properties: &[\n");
    for property in &rule.properties {
        write_property(output, property);
    }
    output.push_str("            ],\n");
    output.push_str("        }),\n");
}

fn write_property(output: &mut String, property: &UpstreamRuleProperty) {
    output.push_str("                crate::rules::markdown::RuleProperty {\n");
    output.push_str(&format!("                    key: \"{}\",\n", property.key));
    output.push_str(&format!(
        "                    description: \"{}\",\n",
        escape_for_rust_str(&property.description)
    ));

    let default_value = property.default_value.as_deref().unwrap_or("");
    let normalized_default =
        escape_for_rust_str(&default_value.trim().trim_matches('"').replace(' ', ""));
    output.push_str(&format!(
        "                    default_value: \"{}\",\n",
        normalized_default
    ));
    output.push_str(&format!(
        "                    prop_type: {},\n",
        property_type(property)
    ));
    output.push_str("                },\n");
}

fn property_type(property: &UpstreamRuleProperty) -> String {
    if !property.values.is_empty() {
        return enum_property_type(&property.values);
    }

    match property.value_type.as_str() {
        "boolean" => "crate::rules::markdown::RulePropertyType::Boolean".to_string(),
        "integer" => "crate::rules::markdown::RulePropertyType::Number".to_string(),
        "array" => "crate::rules::markdown::RulePropertyType::StringArray".to_string(),
        "integer|integer[]" | "number|number[]" => {
            "crate::rules::markdown::RulePropertyType::NumberOrNumberArray".to_string()
        }
        _ => "crate::rules::markdown::RulePropertyType::String".to_string(),
    }
}

fn enum_property_type(values: &[String]) -> String {
    let values = values
        .iter()
        .map(|value| format!("\"{}\"", escape_for_rust_str(value)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("crate::rules::markdown::RulePropertyType::Enum(&[{values}])")
}

fn escape_for_rust_str(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
