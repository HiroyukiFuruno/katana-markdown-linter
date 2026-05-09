use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleDocument {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub properties: Vec<UpstreamRuleProperty>,
    pub examples: Vec<UpstreamRuleExample>,
    pub fixable: Option<bool>,
    pub source_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleExample {
    pub language: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRuleProperty {
    pub key: String,
    pub value_type: String,
    pub default_value: Option<String>,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamCatalog {
    pub source: String,
    pub rules: Vec<UpstreamRuleDocument>,
}
