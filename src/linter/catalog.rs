use crate::{catalog, rules, RuleMeta};
use std::sync::OnceLock;

pub struct RuleCatalogService;

impl RuleCatalogService {
    pub fn available_rules() -> Vec<RuleMeta> {
        static AVAILABLE_RULES: OnceLock<Vec<RuleMeta>> = OnceLock::new();
        AVAILABLE_RULES
            .get_or_init(|| catalog::RuleCatalog::build().to_rule_meta())
            .clone()
    }

    pub fn localized_available_rules(language_code: &str) -> Vec<RuleMeta> {
        Self::available_rules()
            .into_iter()
            .map(|rule| rule.localized(language_code))
            .collect()
    }

    pub fn implemented_rules() -> Vec<RuleMeta> {
        let configurable_meta =
            crate::rules::markdown::MarkdownLinterOps::user_configurable_rule_meta_map();

        rules::markdown::MarkdownLinterOps::official_rules()
            .iter()
            .filter_map(|rule| rule.official_meta())
            .map(|value| {
                let aliases = configurable_meta
                    .get(value.code)
                    .map(|meta| {
                        meta.aliases
                            .iter()
                            .map(|alias| alias.to_string())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                RuleMeta {
                    id: value.code.to_string(),
                    name: value.title.to_string(),
                    description: value.description.to_string(),
                    docs_url: value.docs_url.to_string(),
                    fixable: value.is_fixable,
                    aliases,
                }
            })
            .collect()
    }

    pub fn missing_rules() -> Vec<RuleMeta> {
        let configurable_meta =
            crate::rules::markdown::MarkdownLinterOps::user_configurable_rule_meta_map();

        catalog::RuleCatalog::build()
            .missing_check_rules()
            .into_iter()
            .map(|entry| {
                let aliases = configurable_meta
                    .get(entry.id.as_str())
                    .map(|meta| {
                        meta.aliases
                            .iter()
                            .map(|alias| alias.to_string())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                RuleMeta {
                    id: entry.id.clone(),
                    name: entry.name.clone(),
                    description: entry.description.clone(),
                    docs_url: entry.docs_url.clone(),
                    fixable: entry.fixable,
                    aliases,
                }
            })
            .collect()
    }

    pub fn rule_catalog() -> catalog::RuleCatalog {
        catalog::RuleCatalog::build()
    }

    pub fn localized_rule_catalog(language_code: &str) -> catalog::RuleCatalog {
        catalog::RuleCatalog::build().localized(language_code)
    }
}
