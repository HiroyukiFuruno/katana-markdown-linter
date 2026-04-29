//! `katana-markdown-linter` library.

pub mod catalog;
pub mod cli;
pub mod config;
pub mod fix;
pub mod formatter;
pub mod i18n;
pub mod lsp;
pub mod parser;
pub mod rules;
pub mod types;
pub mod upstream;

pub use config::{ConfigError, ConfigErrorKind, MarkdownLintConfig};
pub use formatter::{format_markdown, layout_lint_options, FormatOptions, FormatResult};
pub use i18n::{
    has_rule_description_translation, localized_rule_description, resolve_locale_code,
    resolve_locale_code_or, supported_locales, Locale, LocaleError, LocalizedDiagnostic,
};
pub use types::{
    Fix, FixDetail, FixResult, FixSafety, LintOptions, LintResult, Range, RuleConfig, RuleMeta,
    Severity,
};

use std::path::Path;
use std::sync::OnceLock;

/// Runs linting for the provided Markdown content.
pub fn lint(content: &str, options: &LintOptions) -> Result<Vec<LintResult>, Error> {
    lint_for_path(Path::new("<memory>"), content, options)
}

pub(crate) fn lint_for_path(
    file_path: &Path,
    content: &str,
    options: &LintOptions,
) -> Result<Vec<LintResult>, Error> {
    let severity_map = build_severity_map(options);
    let diags = rules::markdown::MarkdownLinterOps::evaluate_all(
        file_path,
        content,
        true,
        &severity_map,
        &options.rules,
    );
    Ok(diags.into_iter().map(Into::into).collect())
}

/// Applies available fixes to the provided Markdown content.
pub fn fix(content: &str, options: &LintOptions) -> Result<FixResult, Error> {
    const MAX_FIX_PASSES: usize = 8;

    let mut content = content.to_string();
    let mut applied_fixes = 0;
    let mut all_details = Vec::new();
    let severity_map = build_fix_severity_map(options);

    for _ in 0..MAX_FIX_PASSES {
        let diags = rules::markdown::MarkdownLinterOps::evaluate_all(
            Path::new("<memory>"),
            &content,
            true,
            &severity_map,
            &options.rules,
        );
        let results = diags
            .into_iter()
            .map(Into::into)
            .collect::<Vec<LintResult>>();
        if !results.iter().any(|result| result.fix.is_some()) {
            break;
        }

        let fixed = fix_with_results(&content, &results);
        if fixed.applied_fixes == 0 || fixed.content == content {
            break;
        }

        applied_fixes += fixed.applied_fixes;
        all_details.extend(fixed.details);
        content = fixed.content;
    }

    Ok(FixResult {
        content,
        applied_fixes,
        details: all_details,
    })
}

/// Applies available fixes from already computed lint results.
pub fn fix_with_results(content: &str, results: &[LintResult]) -> FixResult {
    fix::apply(results, content, false)
}

/// Applies available fixes from already computed lint results, including unsafe fixes.
///
/// Callers are responsible for presenting an explicit confirmation flow before using this helper
/// with user-authored content.
pub fn fix_with_results_including_unsafe(content: &str, results: &[LintResult]) -> FixResult {
    fix::apply(results, content, true)
}

/// Returns the set of available rules.
pub fn available_rules() -> Vec<RuleMeta> {
    static AVAILABLE_RULES: OnceLock<Vec<RuleMeta>> = OnceLock::new();
    AVAILABLE_RULES
        .get_or_init(|| catalog::RuleCatalog::build().to_rule_meta())
        .clone()
}

/// Returns the set of available rules with descriptions localized by language code.
pub fn localized_available_rules(language_code: &str) -> Vec<RuleMeta> {
    available_rules()
        .into_iter()
        .map(|rule| rule.localized(language_code))
        .collect()
}

/// Returns the set of rules that are currently executed by the linter.
pub fn implemented_rules() -> Vec<RuleMeta> {
    rules::markdown::MarkdownLinterOps::official_rules()
        .iter()
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

/// Returns a structured catalog with descriptions localized by language code.
pub fn localized_rule_catalog(language_code: &str) -> catalog::RuleCatalog {
    catalog::RuleCatalog::build().localized(language_code)
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
            safety: fix_safety_for_rule(&value.rule_id),
        });
        Self {
            message_id: crate::i18n::diagnostic_message_id(&value.rule_id, &value.message),
            message_params: crate::i18n::diagnostic_message_params(
                &value.rule_id,
                value
                    .official_meta
                    .as_ref()
                    .map(|meta| meta.title)
                    .unwrap_or_default(),
                &value.message,
            ),
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

fn fix_safety_for_rule(rule_id: &str) -> FixSafety {
    if is_unsafe_fix_rule(rule_id) {
        FixSafety::Unsafe
    } else {
        FixSafety::Safe
    }
}

fn is_unsafe_fix_rule(rule_id: &str) -> bool {
    matches!(rule_id, "MD036")
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
) -> std::collections::HashMap<&str, Option<rules::markdown::DiagnosticSeverity>> {
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
            (rule_id.as_str(), severity)
        })
        .collect()
}

fn build_fix_severity_map(
    options: &LintOptions,
) -> std::collections::HashMap<&'static str, Option<rules::markdown::DiagnosticSeverity>> {
    rules::markdown::MarkdownLinterOps::official_rules()
        .iter()
        .map(|rule| {
            let severity = if is_safe_fix_rule(rule.id()) {
                options
                    .rules
                    .get(rule.id())
                    .map(|rule_config| rule_config.enabled)
                    .unwrap_or(true)
                    .then_some(match options.default_severity {
                        Severity::Error => rules::markdown::DiagnosticSeverity::Error,
                        Severity::Warning => rules::markdown::DiagnosticSeverity::Warning,
                        Severity::Info => rules::markdown::DiagnosticSeverity::Info,
                    })
            } else {
                None
            };
            (rule.id(), severity)
        })
        .collect()
}

fn is_safe_fix_rule(rule_id: &str) -> bool {
    matches!(
        rule_id,
        "MD003"
            | "MD004"
            | "MD005"
            | "MD007"
            | "MD009"
            | "MD010"
            | "MD011"
            | "MD012"
            | "MD014"
            | "MD018"
            | "MD019"
            | "MD020"
            | "MD021"
            | "MD022"
            | "MD023"
            | "MD025"
            | "MD026"
            | "MD027"
            | "MD029"
            | "MD030"
            | "MD031"
            | "MD032"
            | "MD034"
            | "MD035"
            | "MD037"
            | "MD038"
            | "MD039"
            | "MD040"
            | "MD044"
            | "MD046"
            | "MD047"
            | "MD048"
            | "MD049"
            | "MD050"
            | "MD051"
            | "MD052"
            | "MD053"
            | "MD054"
            | "MD055"
            | "MD056"
            | "MD058"
            | "MD060"
    )
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
    fn rule_meta_exposes_localized_description_helper() {
        let rule = available_rules()
            .into_iter()
            .find(|rule| rule.id == "MD003")
            .expect("MD003 should be in public catalog");

        assert_eq!(
            rule.localized_description("ja-JP"),
            "見出しのスタイルを統一してください"
        );
        assert_eq!(
            rule.localized_description("fr-FR"),
            "Conservez un style de titre cohérent"
        );
        assert_eq!(resolve_locale_code_or("sv", Locale::Ja), Locale::Ja);
    }

    #[test]
    fn localized_catalog_api_preserves_metadata_and_localizes_descriptions() {
        let english = rule_catalog();
        let japanese = localized_rule_catalog("ja-JP");
        let en_md003 = english
            .active
            .iter()
            .find(|rule| rule.id == "MD003")
            .expect("MD003 should exist");
        let ja_md003 = japanese
            .active
            .iter()
            .find(|rule| rule.id == "MD003")
            .expect("MD003 should exist");

        assert_eq!(en_md003.id, ja_md003.id);
        assert_eq!(en_md003.docs_url, ja_md003.docs_url);
        assert_eq!(ja_md003.description, "見出しのスタイルを統一してください");

        let rules = localized_available_rules("ja");
        assert!(rules
            .iter()
            .any(|rule| rule.id == "MD003"
                && rule.description == "見出しのスタイルを統一してください"));
    }

    #[test]
    fn rule_catalog_exposes_active_and_empty_lifecycle_buckets() {
        let catalog = rule_catalog();
        assert!(catalog.active_rules().any(|rule| rule.id == "MD001"));
        assert!(catalog.deprecated.is_empty());
        assert!(catalog.removed.is_empty());
    }

    #[test]
    fn cached_rule_registries_match_owned_compatibility_apis() {
        let official = rules::markdown::MarkdownLinterOps::official_rules();
        let same_official = rules::markdown::MarkdownLinterOps::official_rules();
        assert!(std::ptr::eq(official, same_official));
        assert_eq!(
            official.len(),
            rules::markdown::MarkdownLinterOps::get_official_rules().len()
        );

        let configurable = rules::markdown::MarkdownLinterOps::user_configurable_rules();
        let same_configurable = rules::markdown::MarkdownLinterOps::user_configurable_rules();
        assert!(std::ptr::eq(configurable, same_configurable));
        assert_eq!(
            configurable.len(),
            rules::markdown::MarkdownLinterOps::get_user_configurable_rules().len()
        );

        let meta_map = rules::markdown::MarkdownLinterOps::user_configurable_rule_meta_map();
        let same_meta_map = rules::markdown::MarkdownLinterOps::user_configurable_rule_meta_map();
        assert!(std::ptr::eq(meta_map, same_meta_map));
        assert_eq!(meta_map.len(), configurable.len());
        assert!(meta_map.contains_key("MD060"));
        assert!(!meta_map.contains_key("md-broken-link"));
    }

    #[test]
    fn cached_config_validation_matches_owned_rule_validation() {
        let config = MarkdownLintConfig {
            raw: json!({
                "default": true,
                "MD013": false,
                "MD999": true
            }),
        };
        let owned = rules::markdown::MarkdownLinterOps::get_user_configurable_rules();
        assert_eq!(config.validate(&owned), config.validate_cached_rules());
    }

    #[test]
    fn lint_reports_heading_increment_violation() {
        let content = "# title\n\n### skipped heading";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD001"));
    }

    #[test]
    fn implemented_rules_and_error_display_are_public_api() {
        let rules = implemented_rules();
        assert!(rules.iter().any(|rule| rule.id == "MD001"));
        assert!(rules.iter().any(|rule| !rule.docs_url.is_empty()));

        let error = Error::new("sample error");
        assert_eq!(error.to_string(), "sample error");
    }

    #[test]
    fn lint_maps_configured_severity_variants() {
        let mut options = LintOptions {
            default_severity: Severity::Error,
            ..LintOptions::default()
        };
        options.rules.insert(
            "MD001".to_string(),
            RuleConfig {
                enabled: true,
                properties: std::collections::HashMap::new(),
            },
        );

        let error_results =
            lint("# title\n\n### skipped heading", &options).expect("lint should succeed");
        assert!(error_results.iter().any(|result| {
            result.rule_id == "MD001" && matches!(result.severity, Severity::Error)
        }));

        options.default_severity = Severity::Info;
        let info_results =
            lint("# title\n\n### skipped heading", &options).expect("lint should succeed");
        assert!(info_results.iter().any(|result| {
            result.rule_id == "MD001" && matches!(result.severity, Severity::Info)
        }));

        options
            .rules
            .get_mut("MD001")
            .expect("MD001 config should exist")
            .enabled = false;
        let disabled_results =
            lint("# title\n\n### skipped heading", &options).expect("lint should succeed");
        assert!(!disabled_results
            .iter()
            .any(|result| result.rule_id == "MD001"));
    }

    #[test]
    fn lint_reports_regex_based_rule_violations() {
        let content = "\n\n\nReversed (link)[https://example.com]\n\nhttps://example.com";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
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
    fn lint_reports_closed_atx_spacing_variants() {
        let content = "#Title#\n##  Title  ##";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD020"));
        assert!(results.iter().any(|result| result.rule_id == "MD021"));
    }

    #[test]
    fn lint_reports_blockquote_spacing_variants() {
        let content = ">  too many";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD027"));
    }

    #[test]
    fn lint_reports_line_length_violation() {
        let content =
            "abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefgh";
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
        let content = "Intro\n| a | b |\n| --- | --- |\n| 1 | 2 | 3 |\n\n|x| y |\n|---|---|\n| z | q |\n[click here](#target)";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD056"));
        assert!(results.iter().any(|result| result.rule_id == "MD058"));
        assert!(results.iter().any(|result| result.rule_id == "MD059"));
        assert!(results.iter().any(|result| result.rule_id == "MD060"));
    }

    #[test]
    fn lint_reports_style_and_link_variants() {
        let content = "No heading here.\n\n```rust\ncode\n```\n~~~\ncode\n~~~\n    indented\n*em* and _em_\n**strong** and __strong__\nmarkdownlint and github\nlink [fragment](#frag)\n[ref][]\n[dup]: https://example.com\n[dup]: https://example.com/2\ninline [link](https://example.com)\n| a | b |\n|---|---\n  c | d\n";
        let options = LintOptions::default();
        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD046"));
        assert!(results.iter().any(|result| result.rule_id == "MD048"));
        assert!(results.iter().any(|result| result.rule_id == "MD049"));
        assert!(results.iter().any(|result| result.rule_id == "MD050"));
        assert!(results.iter().any(|result| result.rule_id == "MD051"));
        assert!(results.iter().any(|result| result.rule_id == "MD052"));
        assert!(results.iter().any(|result| result.rule_id == "MD053"));
        assert!(results.iter().any(|result| result.rule_id == "MD055"));
    }

    #[test]
    fn lint_reports_configured_proper_names_and_link_style() {
        let content = "markdownlint and github\n[inline](target)\n[ref][]\n[ref]: target\n";
        let mut options = LintOptions::default();
        options.rules.insert(
            "MD044".to_string(),
            RuleConfig {
                enabled: true,
                properties: [(
                    "names".to_string(),
                    "[\"Markdownlint\",\"GitHub\"]".to_string(),
                )]
                .into_iter()
                .collect(),
            },
        );
        options.rules.insert(
            "MD054".to_string(),
            RuleConfig {
                enabled: true,
                properties: [("collapsed".to_string(), "false".to_string())]
                    .into_iter()
                    .collect(),
            },
        );

        let results = lint(content, &options).expect("lint should succeed");
        assert!(results.iter().any(|result| result.rule_id == "MD044"));
        assert!(results.iter().any(|result| result.rule_id == "MD054"));
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
    fn fix_applies_multiple_bare_url_fixes_idempotently() {
        let content = "# Title\n\nSee https://example.com and (https://example.org).\n";
        let options = LintOptions::default();
        let result = fix(content, &options).expect("fix should succeed");

        assert_eq!(result.applied_fixes, 2);
        assert_eq!(
            result.content,
            "# Title\n\nSee <https://example.com> and (<https://example.org>).\n"
        );

        let second = fix(&result.content, &options).expect("second fix should succeed");
        assert_eq!(second.applied_fixes, 0);
        assert_eq!(second.content, result.content);
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
