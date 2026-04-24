//! `katana-markdown-linter` library.

pub mod catalog;
pub mod cli;
pub mod config;
pub mod fix;
pub mod parser;
pub mod rules;
pub mod types;
pub mod upstream;

pub use config::{ConfigError, ConfigErrorKind, MarkdownLintConfig};
pub use types::{Fix, FixResult, LintOptions, LintResult, Range, RuleConfig, RuleMeta, Severity};

use std::path::Path;

/// Runs linting for the provided Markdown content.
pub fn lint(content: &str, options: &LintOptions) -> Result<Vec<LintResult>, Error> {
    let file_path = Path::new("<memory>");
    let severity_map = build_severity_map(options);
    let diags =
        rules::markdown::MarkdownLinterOps::evaluate_all(file_path, content, true, &severity_map);
    Ok(diags.into_iter().map(Into::into).collect())
}

/// Applies available fixes to the provided Markdown content.
pub fn fix(content: &str, options: &LintOptions) -> Result<FixResult, Error> {
    let results = lint(content, options)?;
    Ok(fix_with_results(content, &results))
}

/// Applies available fixes from already computed lint results.
pub fn fix_with_results(content: &str, results: &[LintResult]) -> FixResult {
    fix::apply(results, content)
}

/// Returns the set of available rules.
pub fn available_rules() -> Vec<RuleMeta> {
    catalog::RuleCatalog::build().to_rule_meta()
}

/// Returns the set of rules that are currently executed by the linter.
pub fn implemented_rules() -> Vec<RuleMeta> {
    rules::markdown::MarkdownLinterOps::get_official_rules()
        .into_iter()
        .filter_map(|rule| rule.official_meta())
        .map(Into::into)
        .collect()
}

/// Returns the set of official rules that are exposed to configuration but not yet linted.
pub fn missing_rules() -> Vec<RuleMeta> {
    catalog::RuleCatalog::build()
        .missing_check_rules()
        .into_iter()
        .map(|entry| RuleMeta {
            id: entry.id.clone(),
            name: entry.name.clone(),
            description: entry.description.clone(),
            docs_url: entry.docs_url.clone(),
            fixable: entry.fixable,
        })
        .collect()
}

/// Returns a structured catalog of active, deprecated, and removed rules.
pub fn rule_catalog() -> catalog::RuleCatalog {
    catalog::RuleCatalog::build()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MarkdownLintConfig;
    use serde_json::json;

    #[test]
    fn available_rules_exposes_official_rules() {
        let rules = available_rules();
        assert!(rules.iter().any(|rule| rule.id == "MD001"));
        assert!(rules.iter().any(|rule| rule.id == "MD060"));
        let ids = rules.iter().map(|rule| rule.id.clone()).collect::<Vec<_>>();
        let mut sorted = ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(ids, sorted);
    }

    #[test]
    fn rule_catalog_exposes_active_and_empty_lifecycle_buckets() {
        let catalog = rule_catalog();
        assert!(catalog.active_rules().any(|rule| rule.id == "MD001"));
        assert!(catalog.deprecated.is_empty());
        assert!(catalog.removed.is_empty());
    }

    #[test]
    fn lint_reports_heading_increment_violation() {
        let content = "# title\n\n### skipped heading";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD001"));
    }

    #[test]
    fn lint_reports_regex_based_rule_violations() {
        let content = "> no space\n\n\nReversed [link](text[\n\nhttps://example.com";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD020"));
        assert!(results.iter().any(|result| result.rule_id == "MD011"));
        assert!(results.iter().any(|result| result.rule_id == "MD034"));
    }

    #[test]
    fn lint_reports_atx_heading_spacing_violations() {
        let content = "#Title\n##  Title";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD018"));
        assert!(results.iter().any(|result| result.rule_id == "MD019"));
    }

    #[test]
    fn lint_reports_blockquote_spacing_variants() {
        let content = "> no space\n>  too many";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD020"));
        assert!(results.iter().any(|result| result.rule_id == "MD021"));
    }

    #[test]
    fn lint_reports_line_length_violation() {
        let content = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefgh";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD013"));
    }

    #[test]
    fn lint_reports_list_indentation_violation() {
        let content = "   - item";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD007"));
    }

    #[test]
    fn lint_reports_list_indent_rule_violation() {
        let content = "- item\n  - nested\n   - inconsistent";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD005"));
    }

    #[test]
    fn lint_reports_multiple_blank_lines() {
        let content = "first\n\n\nsecond\n";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD012"));
    }

    #[test]
    fn lint_reports_blockquote_spacing_violation() {
        let content = ">  quote";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD027"));
    }

    #[test]
    fn lint_reports_table_rules() {
        let content = "| a | b |\n| 1 | 2 | 3 |\n| 1  | 2 |\nclick here";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD056"));
        assert!(results.iter().any(|result| result.rule_id == "MD058"));
        assert!(results.iter().any(|result| result.rule_id == "MD059"));
        assert!(results.iter().any(|result| result.rule_id == "MD060"));
    }

    #[test]
    fn lint_reports_style_and_link_variants() {
        let content = "No heading here.\n\n```rust\ncode\n```\n~~~\ncode\n~~~\n    indented\n*em* and _em_\n**strong** and __strong__\nmarkdownlint and github\nlink [fragment](#frag)\n[ref][]\n[dup]: https://example.com\n[dup]: https://example.com/2\ninline [link](https://example.com)\n| a | b \n";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD043"));
        assert!(results.iter().any(|result| result.rule_id == "MD044"));
        assert!(results.iter().any(|result| result.rule_id == "MD046"));
        assert!(results.iter().any(|result| result.rule_id == "MD048"));
        assert!(results.iter().any(|result| result.rule_id == "MD049"));
        assert!(results.iter().any(|result| result.rule_id == "MD050"));
        assert!(results.iter().any(|result| result.rule_id == "MD051"));
        assert!(results.iter().any(|result| result.rule_id == "MD052"));
        assert!(results.iter().any(|result| result.rule_id == "MD053"));
        assert!(results.iter().any(|result| result.rule_id == "MD054"));
        assert!(results.iter().any(|result| result.rule_id == "MD055"));
    }

    #[test]
    fn lint_reports_spacing_inside_emphasis_and_code() {
        let content = "This is * spaced * text and ` code ` span.";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD037"));
        assert!(results.iter().any(|result| result.rule_id == "MD038"));
    }

    #[test]
    fn fix_keeps_unmodified_content_when_no_fixes_apply() {
        let content = "# title\n\nParagraph\n";
        let options = LintOptions::default();
        let result = fix(content, &options).expect("fix should succeed");
        assert_eq!(result.content, content);
        assert_eq!(result.applied_fixes, 0);
    }

    #[test]
    fn fix_applies_list_marker_normalization() {
        let content = "- item\n+ item";
        let mut options = LintOptions::default();
        options.rules.insert(
            "MD004".to_string(),
            RuleConfig {
                enabled: true,
                properties: std::collections::HashMap::new(),
            },
        );
        let result = fix(content, &options).expect("fix should succeed");
        assert_ne!(result.content, content);
        let results = lint(&result.content, &options).expect("re-lint should succeed");
        assert!(!results.iter().any(|result| result.rule_id == "MD004"));
    }

    #[test]
    fn fix_with_results_matches_fix_output() {
        let content = "#Title\n\n- item\n+ item";
        let mut options = LintOptions::default();
        options.rules.insert(
            "MD004".to_string(),
            RuleConfig {
                enabled: true,
                properties: std::collections::HashMap::new(),
            },
        );
        let results = lint(content, &options).expect("lint should succeed");
        let direct = fix(content, &options).expect("fix should succeed");
        let reused = fix_with_results(content, &results);
        assert_eq!(direct, reused);
    }

    #[test]
    fn config_validate_is_exposed_from_root() {
        let config = MarkdownLintConfig {
            raw: json!({
            "default": true,
            "MD999": true
            }),
        };
        let errors = config.validate(&[]);
        assert!(errors
            .iter()
            .any(|error| matches!(error.kind, ConfigErrorKind::UnknownRule)));
    }

    #[test]
    fn missing_rules_exposes_stubbed_official_rules() {
        let rules = missing_rules();
        assert!(!rules.iter().any(|rule| rule.id == "MD001"));
        assert!(!rules.iter().any(|rule| rule.id == "MD024"));
        assert!(!rules.iter().any(|rule| rule.id == "MD030"));
        assert!(!rules.iter().any(|rule| rule.id == "MD031"));
        assert!(!rules.iter().any(|rule| rule.id == "MD005"));
        assert!(!rules.iter().any(|rule| rule.id == "MD056"));
    }

    #[test]
    fn missing_rules_are_empty() {
        assert!(missing_rules().is_empty());
    }
}
