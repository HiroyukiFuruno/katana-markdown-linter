use crate::RuleMeta;
use serde::Serialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RuleLifecycleState {
    Active,
    Deprecated,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuleCatalogEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub docs_url: String,
    pub fixable: bool,
    pub implemented_check: bool,
    pub lifecycle: RuleLifecycleState,
}

impl RuleCatalogEntry {
    pub fn localized_description(&self, language_code: &str) -> String {
        crate::i18n::localized_rule_description(&self.id, &self.description, language_code)
    }

    pub fn localized(&self, language_code: &str) -> Self {
        let mut entry = self.clone();
        entry.description = self.localized_description(language_code);
        entry
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct RuleCatalog {
    pub active: Vec<RuleCatalogEntry>,
    pub deprecated: Vec<RuleCatalogEntry>,
    pub removed: Vec<RuleCatalogEntry>,
}

impl RuleCatalog {
    pub fn build() -> Self {
        static CATALOG: OnceLock<RuleCatalog> = OnceLock::new();
        CATALOG.get_or_init(Self::build_uncached).clone()
    }

    fn build_uncached() -> Self {
        let available = crate::rules::markdown::MarkdownLinterOps::user_configurable_rules();
        let implemented_ids = crate::rules::markdown::MarkdownLinterOps::official_rules()
            .iter()
            .map(|rule| rule.id())
            .collect::<std::collections::HashSet<_>>();

        let mut active: Vec<RuleCatalogEntry> = available
            .iter()
            .filter_map(|rule| {
                rule.official_meta().map(|meta| RuleCatalogEntry {
                    implemented_check: implemented_ids.contains(meta.code),
                    lifecycle: RuleLifecycleState::Active,
                    id: meta.code.to_string(),
                    name: meta.title.to_string(),
                    description: meta.description.to_string(),
                    docs_url: meta.docs_url.to_string(),
                    fixable: meta.is_fixable,
                })
            })
            .collect();

        active.sort_by(|a, b| a.id.cmp(&b.id));
        Self {
            active,
            deprecated: Vec::new(),
            removed: Vec::new(),
        }
    }

    pub fn active_rules(&self) -> impl Iterator<Item = &RuleCatalogEntry> {
        self.active.iter()
    }

    pub fn missing_check_rules(&self) -> Vec<&RuleCatalogEntry> {
        self.active
            .iter()
            .filter(|entry| !entry.implemented_check)
            .collect()
    }

    pub fn to_rule_meta(&self) -> Vec<RuleMeta> {
        let configurable_meta =
            crate::rules::markdown::MarkdownLinterOps::user_configurable_rule_meta_map();

        self.active
            .iter()
            .map(|entry| {
                let aliases = configurable_meta
                    .get(entry.id.as_str())
                    .map(|meta| {
                        meta.aliases
                            .iter()
                            .map(|s| s.to_string())
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

    pub fn localized(&self, language_code: &str) -> Self {
        Self {
            active: self
                .active
                .iter()
                .map(|entry| entry.localized(language_code))
                .collect(),
            deprecated: self
                .deprecated
                .iter()
                .map(|entry| entry.localized(language_code))
                .collect(),
            removed: self
                .removed
                .iter()
                .map(|entry| entry.localized(language_code))
                .collect(),
        }
    }
}
