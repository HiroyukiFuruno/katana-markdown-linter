use crate::RuleMeta;
use serde::Serialize;

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

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct RuleCatalog {
    pub active: Vec<RuleCatalogEntry>,
    pub deprecated: Vec<RuleCatalogEntry>,
    pub removed: Vec<RuleCatalogEntry>,
}

impl RuleCatalog {
    pub fn build() -> Self {
        let available = crate::rules::markdown::MarkdownLinterOps::get_user_configurable_rules();
        let implemented_ids = crate::rules::markdown::MarkdownLinterOps::get_official_rules()
            .into_iter()
            .map(|rule| rule.id().to_string())
            .collect::<std::collections::HashSet<_>>();

        let mut active: Vec<RuleCatalogEntry> = available
            .into_iter()
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
        self.active
            .iter()
            .map(|entry| RuleMeta {
                id: entry.id.clone(),
                name: entry.name.clone(),
                description: entry.description.clone(),
                docs_url: entry.docs_url.clone(),
                fixable: entry.fixable,
            })
            .collect()
    }
}
