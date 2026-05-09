mod docs;
mod examples;
mod parameters;
mod parse;
mod tokens;
mod types;

use std::path::Path;

pub use types::{UpstreamCatalog, UpstreamRuleDocument, UpstreamRuleExample, UpstreamRuleProperty};

pub struct UpstreamDocumentService;

impl UpstreamDocumentService {
    pub fn parse_rule_document(source: &str) -> Result<UpstreamRuleDocument, String> {
        parse::parse_rule_document(source)
    }

    pub fn load_catalog_from_dir(doc_dir: &Path) -> Result<UpstreamCatalog, String> {
        parse::load_catalog_from_dir(doc_dir)
    }

    pub fn get_rule_documentation(rule_id: &str, locale: crate::Locale) -> Result<String, String> {
        Self::get_rule_documentation_from_dir(rule_id, locale, Path::new("upstream_docs"))
    }

    pub fn get_rule_documentation_from_dir(
        rule_id: &str,
        locale: crate::Locale,
        base_dir: &Path,
    ) -> Result<String, String> {
        docs::get_rule_documentation_from_dir(rule_id, locale, base_dir)
    }
}

#[cfg(test)]
mod tests;
