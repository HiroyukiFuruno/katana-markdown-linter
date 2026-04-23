use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Warning
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Range {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    pub range: Range,
    pub replacement: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FixResult {
    pub content: String,
    pub applied_fixes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuleConfig {
    pub enabled: bool,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintOptions {
    pub rules: HashMap<String, RuleConfig>,
    pub default_severity: Severity,
}

impl Default for LintOptions {
    fn default() -> Self {
        Self {
            rules: HashMap::new(),
            default_severity: Severity::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub docs_url: String,
    pub fixable: bool,
}
