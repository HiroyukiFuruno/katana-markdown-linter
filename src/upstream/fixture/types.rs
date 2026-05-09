use super::super::document::UpstreamRuleProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureMatrix {
    pub upstream_source: String,
    pub summary: RuleFixtureMatrixSummary,
    pub rules: Vec<RuleFixtureEntry>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureMatrixSummary {
    pub total_rules: usize,
    pub rules_with_examples: usize,
    pub rules_with_fix_metadata: usize,
    pub rules_with_parameters: usize,
    pub manual_required: usize,
    pub missing_fixtures: usize,
    pub stale_fixtures: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureEntry {
    pub rule_id: String,
    pub aliases: Vec<String>,
    pub tags: Vec<String>,
    pub parameters: Vec<UpstreamRuleProperty>,
    pub fixable: Option<bool>,
    pub check_pass: Vec<RuleFixtureCase>,
    pub check_fail: Vec<RuleFixtureCase>,
    pub fix: Vec<RuleFixtureCase>,
    pub config_valid: Vec<RuleFixtureCase>,
    pub config_invalid: Vec<RuleFixtureCase>,
    pub edge: Vec<RuleFixtureCase>,
    pub manual_required: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureCase {
    pub name: String,
    pub source: String,
    pub expected: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftReport {
    pub upstream_source: String,
    pub summary: RuleFixtureDriftSummary,
    pub items: Vec<RuleFixtureDriftItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftSummary {
    pub upstream_rules: usize,
    pub matrix_rules: usize,
    pub missing_fixtures: usize,
    pub stale_fixtures: usize,
    pub manual_required: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleFixtureDriftItem {
    pub rule_id: String,
    pub drift_type: RuleFixtureDriftType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleFixtureDriftType {
    MissingFixture,
    StaleFixture,
    ManualRequired,
}
