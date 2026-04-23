use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
pub enum Severity {
    Error,
    #[default]
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct Range {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Fix {
    pub range: Range,
    pub replacement: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LintResult {
    pub rule_id: String,
    pub rule_name: String,
    pub message: String,
    pub severity: Severity,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub fix: Option<Fix>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct FixResult {
    pub content: String,
    pub applied_fixes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct RuleConfig {
    pub enabled: bool,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct LintOptions {
    pub rules: HashMap<String, RuleConfig>,
    pub default_severity: Severity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuleMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub docs_url: String,
    pub fixable: bool,
}
