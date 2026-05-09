use crate::rules::markdown::RulePropertyType;
use std::collections::BTreeMap;

pub(super) fn local_rule_map() -> BTreeMap<&'static str, crate::rules::markdown::OfficialRuleMeta> {
    let mut map = BTreeMap::new();
    for rule in crate::rules::markdown::MarkdownLinterOps::user_configurable_rules() {
        if let Some(meta) = rule.official_meta() {
            map.insert(meta.code, meta);
        }
    }
    for rule in local_metadata_stubs() {
        if let Some(meta) = rule.official_meta() {
            let replace = map
                .get(meta.code)
                .map(|current| meta.properties.len() > current.properties.len())
                .unwrap_or(true);
            if replace {
                map.insert(meta.code, meta);
            }
        }
    }
    map
}

fn local_metadata_stubs() -> Vec<Box<dyn crate::rules::markdown::MarkdownRule>> {
    use crate::rules::markdown::stubs::*;
    vec![
        Box::new(RuleMD001),
        Box::new(RuleMD003),
        Box::new(RuleMD004),
        Box::new(RuleMD005),
        Box::new(RuleMD007),
        Box::new(RuleMD011),
        Box::new(RuleMD012),
        Box::new(RuleMD013),
        Box::new(RuleMD014),
        Box::new(RuleMD020),
        Box::new(RuleMD021),
        Box::new(RuleMD022),
        Box::new(RuleMD023),
        Box::new(RuleMD024),
        Box::new(RuleMD025),
        Box::new(RuleMD026),
        Box::new(RuleMD027),
        Box::new(RuleMD028),
        Box::new(RuleMD029),
        Box::new(RuleMD030),
        Box::new(RuleMD031),
        Box::new(RuleMD032),
        Box::new(RuleMD033),
        Box::new(RuleMD034),
        Box::new(RuleMD035),
        Box::new(RuleMD036),
        Box::new(RuleMD040),
        Box::new(RuleMD041),
        Box::new(RuleMD042),
        Box::new(RuleMD043),
        Box::new(RuleMD044),
        Box::new(RuleMD045),
        Box::new(RuleMD046),
        Box::new(RuleMD047),
        Box::new(RuleMD048),
        Box::new(RuleMD049),
        Box::new(RuleMD050),
        Box::new(RuleMD051),
        Box::new(RuleMD052),
        Box::new(RuleMD053),
        Box::new(RuleMD054),
        Box::new(RuleMD055),
        Box::new(RuleMD056),
        Box::new(RuleMD058),
        Box::new(RuleMD059),
        Box::new(RuleMD060),
    ]
}

pub(super) fn property_type_name(prop_type: RulePropertyType) -> &'static str {
    match prop_type {
        RulePropertyType::Boolean => "boolean",
        RulePropertyType::Number => "integer",
        RulePropertyType::NumberOrNumberArray => "integer|integer[]",
        RulePropertyType::String => "string",
        RulePropertyType::StringArray => "array",
        RulePropertyType::Enum(_) => "string",
    }
}

pub(super) fn normalize_default(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .replace(' ', "")
        .replace("\\\"", "\"")
}

pub(super) fn normalize_summary(value: &str) -> String {
    value.trim().trim_end_matches('.').to_ascii_lowercase()
}
