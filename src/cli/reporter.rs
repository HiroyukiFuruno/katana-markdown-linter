use super::args::{Cli, OutputFormat};
use super::input::InputExpandError;
use crate::i18n::{Locale, LocalizedDiagnostic, MessageParams};
use crate::FixSafety;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct LocalizedRuleMeta {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) english_description: String,
    pub(crate) docs_url: String,
    pub(crate) fixable: bool,
    pub(crate) locale: &'static str,
}

impl LocalizedRuleMeta {
    pub(crate) fn from_rule(rule: &crate::RuleMeta, locale: Locale) -> Self {
        Self {
            id: rule.id.clone(),
            name: rule.name.clone(),
            description: rule.localized_description(locale.code()),
            english_description: rule.description.clone(),
            docs_url: rule.docs_url.clone(),
            fixable: rule.fixable,
            locale: locale.code(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CliReport {
    pub(crate) command: &'static str,
    pub(crate) summary: CliSummary,
    pub(crate) files: Vec<FileReport>,
    pub(crate) errors: Vec<CliError>,
}

impl CliReport {
    pub(crate) fn recompute_summary(&mut self) {
        let mut summary = CliSummary {
            total_files: self.files.len(),
            ..CliSummary::default()
        };

        for file in &self.files {
            if !file.diagnostics.is_empty() {
                summary.files_with_issues += 1;
            }
            summary.total_issues += file.diagnostics.len();
            summary.fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.fix.is_some())
                .count();
            summary.safe_fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic
                        .fix
                        .as_ref()
                        .is_some_and(|fix| fix.safety == FixSafety::Safe)
                })
                .count();
            summary.unsafe_fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic
                        .fix
                        .as_ref()
                        .is_some_and(|fix| fix.safety == FixSafety::Unsafe)
                })
                .count();
            summary.applied_fixes += file.applied_fixes;
        }

        if summary.unsafe_fixable_issues > 0 {
            summary
                .unsafe_fix_status
                .get_or_insert_with(|| "unsafe_mode_not_enabled".to_string());
        }
        self.summary = summary;
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct CliSummary {
    pub(crate) total_files: usize,
    pub(crate) files_with_issues: usize,
    pub(crate) total_issues: usize,
    pub(crate) fixable_issues: usize,
    pub(crate) safe_fixable_issues: usize,
    pub(crate) unsafe_fixable_issues: usize,
    pub(crate) applied_fixes: usize,
    pub(crate) unsafe_fix_status: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct FileReport {
    pub(crate) path: String,
    pub(crate) diagnostics: Vec<crate::LintResult>,
    pub(crate) applied_fixes: usize,
    pub(crate) fix_details: Vec<crate::FixDetail>,
    pub(crate) changed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct LocalizedCliReport {
    command: &'static str,
    summary: CliSummary,
    files: Vec<LocalizedFileReport>,
    errors: Vec<LocalizedCliError>,
}

impl LocalizedCliReport {
    pub(crate) fn from_report(report: &CliReport, locale: Locale) -> Self {
        Self {
            command: report.command,
            summary: report.summary.clone(),
            files: report
                .files
                .iter()
                .map(|file| LocalizedFileReport {
                    path: file.path.clone(),
                    diagnostics: file
                        .diagnostics
                        .iter()
                        .map(|diagnostic| LocalizedDiagnostic::from_result(diagnostic, locale))
                        .collect(),
                    applied_fixes: file.applied_fixes,
                    fix_details: file.fix_details.clone(),
                    changed: file.changed,
                })
                .collect(),
            errors: report
                .errors
                .iter()
                .map(|error| LocalizedCliError::from_error(error, locale))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct LocalizedFileReport {
    path: String,
    diagnostics: Vec<LocalizedDiagnostic>,
    applied_fixes: usize,
    fix_details: Vec<crate::FixDetail>,
    changed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CliError {
    pub(crate) kind: &'static str,
    pub(crate) path: Option<String>,
    pub(crate) message: String,
    pub(crate) message_id: String,
    pub(crate) message_params: MessageParams,
}

impl CliError {
    pub(crate) fn filesystem(path: &Path, err: impl std::fmt::Display) -> Self {
        let message = err.to_string();
        Self {
            kind: "filesystem",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "filesystem.error".to_string(),
        }
    }

    pub(crate) fn filesystem_message(message: String) -> Self {
        Self {
            kind: "filesystem",
            path: None,
            message_params: message_params(&message),
            message,
            message_id: "filesystem.error".to_string(),
        }
    }

    pub(crate) fn glob(message: String) -> Self {
        Self {
            kind: "glob",
            path: None,
            message_params: message_params(&message),
            message,
            message_id: "glob.error".to_string(),
        }
    }

    pub(crate) fn config(path: &Path, message: String) -> Self {
        Self {
            kind: "config",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "config.error".to_string(),
        }
    }

    pub(crate) fn config_validation(path: &Path, error: crate::ConfigError) -> Self {
        Self {
            kind: "config",
            path: Some(path.display().to_string()),
            message_params: error.message_params(),
            message: error.to_string(),
            message_id: error.message_id().to_string(),
        }
    }

    pub(crate) fn rule(path: &Path, message: String) -> Self {
        Self {
            kind: "rule",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "rule.error".to_string(),
        }
    }

    pub(crate) fn from_input_expand_error(error: InputExpandError) -> Self {
        match error {
            InputExpandError::Filesystem(message) => Self::filesystem_message(message),
            InputExpandError::Glob(message) => Self::glob(message),
        }
    }

    pub(crate) fn localized_message(&self, locale: Locale) -> String {
        crate::i18n::render_message(
            locale,
            self.message_id.as_str(),
            &self.message_params,
            self.message.as_str(),
        )
    }
}

#[derive(Debug, Clone, Serialize)]
struct LocalizedCliError {
    kind: &'static str,
    path: Option<String>,
    message: String,
    message_id: String,
    message_params: MessageParams,
}

impl LocalizedCliError {
    fn from_error(error: &CliError, locale: Locale) -> Self {
        Self {
            kind: error.kind,
            path: error.path.clone(),
            message: error.localized_message(locale),
            message_id: error.message_id.clone(),
            message_params: error.message_params.clone(),
        }
    }
}

pub(crate) fn message_params(message: &str) -> MessageParams {
    let mut params = MessageParams::new();
    params.insert("message".to_string(), message.to_string());
    params
}

pub(crate) fn print_text_report(report: &CliReport, fix_mode: bool, cli: &Cli, locale: Locale) {
    for error in &report.errors {
        if let Some(path) = &error.path {
            eprintln!(
                "{}: {} error: {}",
                path,
                error.kind,
                error.localized_message(locale)
            );
        } else {
            eprintln!("{} error: {}", error.kind, error.localized_message(locale));
        }
    }
    print!("{}", render_text_report(report, fix_mode, cli, locale));
}

pub(crate) fn output_report(
    report: &CliReport,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> Result<(), String> {
    match cli.format {
        OutputFormat::Text => print_text_report(report, fix_mode, cli, locale),
        OutputFormat::Json => {
            let report = LocalizedCliReport::from_report(report, locale);
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
            );
        }
    }
    Ok(())
}

pub(crate) fn render_text_report(
    report: &CliReport,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> String {
    let mut output = String::new();
    for file in &report.files {
        if fix_mode && file.applied_fixes > 0 && !cli.quiet {
            let mut params = MessageParams::new();
            params.insert("path".to_string(), file.path.clone());
            params.insert("count".to_string(), file.applied_fixes.to_string());
            let (message_id, fallback) = if report.command == "fmt" {
                (
                    "format.formatted_count",
                    format!(
                        "{}: formatted {} operation{}",
                        file.path,
                        file.applied_fixes,
                        plural(file.applied_fixes)
                    ),
                )
            } else {
                (
                    "fix.fixed_count",
                    format!(
                        "{}: fixed {} issue{}",
                        file.path,
                        file.applied_fixes,
                        plural(file.applied_fixes)
                    ),
                )
            };
            output.push_str(&crate::i18n::render_message(
                locale, message_id, &params, &fallback,
            ));
            output.push('\n');
        }

        for result in &file.diagnostics {
            if cli.quiet {
                continue;
            }
            let message = crate::i18n::render_message(
                locale,
                result.message_id.as_str(),
                &result.message_params,
                result.message.as_str(),
            );
            output.push_str(&format!(
                "{}:{}:{} {} {}",
                file.path, result.line, result.column, result.rule_id, message
            ));
            output.push('\n');
        }
    }

    if report.files.is_empty() && report.errors.is_empty() {
        output.push_str(&crate::i18n::render_message(
            locale,
            "summary.no_files",
            &MessageParams::new(),
            "No Markdown files found",
        ));
        output.push('\n');
    }
    if cli.statistics {
        let mut params = MessageParams::new();
        params.insert("files".to_string(), report.summary.total_files.to_string());
        params.insert(
            "files_with_issues".to_string(),
            report.summary.files_with_issues.to_string(),
        );
        params.insert(
            "issues".to_string(),
            report.summary.total_issues.to_string(),
        );
        params.insert(
            "fixable".to_string(),
            report.summary.fixable_issues.to_string(),
        );
        params.insert(
            "safe_fixable".to_string(),
            report.summary.safe_fixable_issues.to_string(),
        );
        params.insert(
            "unsafe_fixable".to_string(),
            report.summary.unsafe_fixable_issues.to_string(),
        );
        params.insert(
            "fixed".to_string(),
            report.summary.applied_fixes.to_string(),
        );
        let fallback = format!(
            "files: {}, files_with_issues: {}, issues: {}, fixable: {}, safe_fixable: {}, unsafe_fixable: {}, fixed: {}",
            report.summary.total_files,
            report.summary.files_with_issues,
            report.summary.total_issues,
            report.summary.fixable_issues,
            report.summary.safe_fixable_issues,
            report.summary.unsafe_fixable_issues,
            report.summary.applied_fixes
        );
        output.push_str(&crate::i18n::render_message(
            locale,
            "summary.statistics",
            &params,
            &fallback,
        ));
        output.push('\n');
    }
    if cli.verbose {
        output.push_str(&format!("command: {}\n", report.command));
    }
    output
}

pub(crate) fn print_diff(path: &Path, before: &str, after: &str) {
    if before == after {
        return;
    }
    println!("--- {}", path.display());
    println!("+++ {}", path.display());
    println!("@@");
    println!("-{}", before);
    println!("+{}", after);
}

pub(crate) fn plural(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Locale;
    use crate::{LintResult, Severity};

    #[test]
    fn renders_text_report_contract_with_statistics_and_verbose() {
        let mut report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "<stdin>".to_string(),
                diagnostics: vec![LintResult {
                    rule_id: "MD018".to_string(),
                    rule_name: "no-missing-space-atx".to_string(),
                    message: "No space after hash on atx style heading".to_string(),
                    message_id: "rule.generic".to_string(),
                    message_params: crate::i18n::diagnostic_message_params(
                        "MD018",
                        "no-missing-space-atx",
                        "No space after hash on atx style heading",
                    ),
                    severity: Severity::Warning,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 6,
                    fix: None,
                }],
                applied_fixes: 0,
                fix_details: Vec::new(),
                changed: false,
            }],
            errors: Vec::new(),
        };
        report.recompute_summary();

        let output = render_text_report(
            &report,
            false,
            &Cli {
                statistics: true,
                verbose: true,
                ..Cli::default()
            },
            Locale::En,
        );

        assert!(output.contains("<stdin>:1:1 MD018 No space after hash"));
        assert!(output.contains("files: 1, files_with_issues: 1, issues: 1"));
        assert!(output.contains("command: check"));
    }

    #[test]
    fn quiet_text_report_suppresses_diagnostics_but_keeps_statistics() {
        let mut report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "README.md".to_string(),
                diagnostics: vec![LintResult {
                    rule_id: "MD018".to_string(),
                    rule_name: "no-missing-space-atx".to_string(),
                    message: "No space after hash on atx style heading".to_string(),
                    message_id: "rule.generic".to_string(),
                    message_params: crate::i18n::diagnostic_message_params(
                        "MD018",
                        "no-missing-space-atx",
                        "No space after hash on atx style heading",
                    ),
                    severity: Severity::Warning,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 6,
                    fix: Some(crate::Fix {
                        range: crate::Range {
                            start_line: 1,
                            start_column: 1,
                            end_line: 1,
                            end_column: 6,
                        },
                        replacement: "# Heading".to_string(),
                        safety: crate::FixSafety::Unsafe,
                    }),
                }],
                applied_fixes: 0,
                fix_details: Vec::new(),
                changed: false,
            }],
            errors: Vec::new(),
        };
        report.recompute_summary();

        let output = render_text_report(
            &report,
            false,
            &Cli {
                quiet: true,
                statistics: true,
                ..Cli::default()
            },
            Locale::En,
        );

        assert!(!output.contains("MD018"));
        assert!(output.contains("issues: 1"));
    }

    #[test]
    fn text_report_uses_selected_japanese_locale() {
        let mut report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "README.md".to_string(),
                diagnostics: vec![LintResult {
                    rule_id: "MD018".to_string(),
                    rule_name: "no-missing-space-atx".to_string(),
                    message: "No space after hash on atx style heading".to_string(),
                    message_id: "rule.generic".to_string(),
                    message_params: crate::i18n::diagnostic_message_params(
                        "MD018",
                        "no-missing-space-atx",
                        "No space after hash on atx style heading",
                    ),
                    severity: Severity::Warning,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 6,
                    fix: None,
                }],
                applied_fixes: 0,
                fix_details: Vec::new(),
                changed: false,
            }],
            errors: Vec::new(),
        };
        report.recompute_summary();

        let output = render_text_report(&report, false, &Cli::default(), Locale::Ja);

        assert!(output.contains("README.md:1:1 MD018 ATX 見出し"));
    }

    #[test]
    fn json_report_keeps_shape_and_adds_localized_message_metadata() {
        let mut report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "README.md".to_string(),
                diagnostics: vec![LintResult {
                    rule_id: "MD018".to_string(),
                    rule_name: "no-missing-space-atx".to_string(),
                    message: "No space after hash on atx style heading".to_string(),
                    message_id: "rule.generic".to_string(),
                    message_params: crate::i18n::diagnostic_message_params(
                        "MD018",
                        "no-missing-space-atx",
                        "No space after hash on atx style heading",
                    ),
                    severity: Severity::Warning,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 6,
                    fix: Some(crate::Fix {
                        range: crate::Range {
                            start_line: 1,
                            start_column: 1,
                            end_line: 1,
                            end_column: 6,
                        },
                        replacement: "# Heading".to_string(),
                        safety: crate::FixSafety::Unsafe,
                    }),
                }],
                applied_fixes: 0,
                fix_details: Vec::new(),
                changed: false,
            }],
            errors: Vec::new(),
        };
        report.recompute_summary();

        let json = serde_json::to_value(LocalizedCliReport::from_report(&report, Locale::Ja))
            .expect("localized report should serialize");

        assert_eq!(
            json["files"][0]["diagnostics"][0]["message_id"],
            "rule.generic"
        );
        assert_eq!(
            json["files"][0]["diagnostics"][0]["message_params"]["rule_id"],
            "MD018"
        );
        assert!(json["files"][0]["diagnostics"][0]["message"]
            .as_str()
            .expect("message should be string")
            .contains("ATX 見出し"));
        assert_eq!(
            json["files"][0]["diagnostics"][0]["fix"]["safety"],
            "unsafe"
        );
        assert_eq!(json["summary"]["unsafe_fixable_issues"], 1);
        assert_eq!(
            json["summary"]["unsafe_fix_status"],
            "unsafe_mode_not_enabled"
        );
    }

    #[test]
    fn json_report_localizes_config_errors_with_stable_metadata() {
        let report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: vec![CliError::config(
                Path::new("<stdin>"),
                "config file not found: missing.json".to_string(),
            )],
        };

        let json = serde_json::to_value(LocalizedCliReport::from_report(&report, Locale::Ja))
            .expect("localized report should serialize");

        assert_eq!(json["errors"][0]["message_id"], "config.error");
        assert!(json["errors"][0]["message"]
            .as_str()
            .expect("message should be string")
            .contains("設定エラー"));
    }

    #[test]
    fn json_report_localizes_structured_config_validation_errors() {
        let config = crate::MarkdownLintConfig {
            raw: serde_json::json!({ "MD999": true }),
        };
        let error = config
            .validate_cached_rules()
            .into_iter()
            .next()
            .expect("config should be invalid");
        let report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: vec![CliError::config_validation(Path::new("README.md"), error)],
        };

        let json = serde_json::to_value(LocalizedCliReport::from_report(&report, Locale::Ja))
            .expect("localized report should serialize");

        assert_eq!(json["errors"][0]["message_id"], "config.unknown_rule");
        assert_eq!(json["errors"][0]["message_params"]["rule_id"], "MD999");
        assert_eq!(
            json["errors"][0]["message"],
            "未知の markdownlint rule です: MD999"
        );
    }

    #[test]
    fn json_report_localizes_glob_errors_with_stable_metadata() {
        let report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: vec![CliError::glob("invalid glob pattern".to_string())],
        };

        let json = serde_json::to_value(LocalizedCliReport::from_report(&report, Locale::Ja))
            .expect("localized report should serialize");

        assert_eq!(json["errors"][0]["kind"], "glob");
        assert_eq!(json["errors"][0]["message_id"], "glob.error");
        assert!(json["errors"][0]["message"]
            .as_str()
            .expect("message should be string")
            .contains("glob エラー"));
    }
}
