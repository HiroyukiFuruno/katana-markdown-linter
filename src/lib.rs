//! `katana-markdown-linter` library scaffold.

pub mod config;
pub mod fix;
pub mod parser;
pub mod rules;
pub mod types;

pub use types::{Fix, FixResult, LintOptions, LintResult, Range, RuleConfig, RuleMeta, Severity};

/// Runs linting for the provided Markdown content.
pub fn lint(_content: &str, _options: &LintOptions) -> Result<Vec<LintResult>, Error> {
    Ok(Vec::new())
}

/// Applies available fixes to the provided Markdown content.
pub fn fix(_content: &str, _options: &LintOptions) -> Result<FixResult, Error> {
    Ok(FixResult::default())
}

/// Returns the set of available rules.
pub fn available_rules() -> Vec<RuleMeta> {
    Vec::new()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}
