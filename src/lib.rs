//! `katana-markdown-linter` library.

pub mod config;
pub mod fix;
pub mod parser;
pub mod rules;
pub mod types;

pub use types::{Fix, FixResult, LintOptions, LintResult, Range, RuleConfig, RuleMeta, Severity};

use std::path::Path;

/// Runs linting for the provided Markdown content.
pub fn lint(content: &str, options: &LintOptions) -> Result<Vec<LintResult>, Error> {
    let file_path = Path::new("<memory>");
    let severity_map = build_severity_map(options);
    let diags = rules::markdown::MarkdownLinterOps::evaluate_all(
        file_path,
        content,
        true,
        &severity_map,
    );
    Ok(diags.into_iter().map(Into::into).collect())
}

/// Applies available fixes to the provided Markdown content.
pub fn fix(content: &str, options: &LintOptions) -> Result<FixResult, Error> {
    let results = lint(content, options)?;
    Ok(fix::apply(&results, content))
}

/// Returns the set of available rules.
pub fn available_rules() -> Vec<RuleMeta> {
    rules::markdown::MarkdownLinterOps::get_user_configurable_rules()
        .into_iter()
        .filter_map(|rule| rule.official_meta())
        .map(Into::into)
        .collect()
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

impl From<rules::markdown::MarkdownDiagnostic> for LintResult {
    fn from(value: rules::markdown::MarkdownDiagnostic) -> Self {
        let fix = value.fix_info.map(|fix_info| Fix {
            range: Range {
                start_line: fix_info.start_line,
                start_column: fix_info.start_column,
                end_line: fix_info.end_line,
                end_column: fix_info.end_column,
            },
            replacement: fix_info.replacement,
        });
        Self {
            rule_id: value.rule_id,
            rule_name: value
                .official_meta
                .as_ref()
                .map(|meta| meta.title.to_string())
                .unwrap_or_default(),
            message: value.message,
            severity: match value.severity {
                rules::markdown::DiagnosticSeverity::Error => Severity::Error,
                rules::markdown::DiagnosticSeverity::Warning => Severity::Warning,
                rules::markdown::DiagnosticSeverity::Info => Severity::Info,
            },
            line: value.range.start_line,
            column: value.range.start_column,
            end_line: value.range.end_line,
            end_column: value.range.end_column,
            fix,
        }
    }
}

impl From<rules::markdown::OfficialRuleMeta> for RuleMeta {
    fn from(value: rules::markdown::OfficialRuleMeta) -> Self {
        Self {
            id: value.code.to_string(),
            name: value.title.to_string(),
            description: value.description.to_string(),
            docs_url: value.docs_url.to_string(),
            fixable: value.is_fixable,
        }
    }
}

fn build_severity_map(
    options: &LintOptions,
) -> std::collections::HashMap<String, Option<rules::markdown::DiagnosticSeverity>> {
    options
        .rules
        .iter()
        .map(|(rule_id, rule_config)| {
            let severity = if rule_config.enabled {
                Some(match options.default_severity {
                    Severity::Error => rules::markdown::DiagnosticSeverity::Error,
                    Severity::Warning => rules::markdown::DiagnosticSeverity::Warning,
                    Severity::Info => rules::markdown::DiagnosticSeverity::Info,
                })
            } else {
                None
            };
            (rule_id.clone(), severity)
        })
        .collect()
}
