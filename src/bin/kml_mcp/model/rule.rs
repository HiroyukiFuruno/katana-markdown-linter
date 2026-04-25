use katana_markdown_linter::{catalog::RuleCatalogEntry, Locale};
use rmcp::schemars;
use serde::Serialize;

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct RuleListResponse {
    pub(crate) locale: String,
    pub(crate) count: usize,
    pub(crate) rules: Vec<RuleMetadata>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct RuleMetadata {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) english_description: String,
    pub(crate) docs_url: String,
    pub(crate) fixable: bool,
    pub(crate) implemented_check: bool,
    pub(crate) lifecycle: String,
    pub(crate) locale: String,
}

impl RuleMetadata {
    pub(crate) fn from_entry(rule: RuleCatalogEntry, lifecycle: &str, locale: Locale) -> Self {
        let locale_code = locale.code();
        let description = rule.localized_description(locale_code);
        Self {
            id: rule.id,
            name: rule.name,
            description,
            english_description: rule.description,
            docs_url: rule.docs_url,
            fixable: rule.fixable,
            implemented_check: rule.implemented_check,
            lifecycle: lifecycle.to_string(),
            locale: locale_code.to_string(),
        }
    }
}
