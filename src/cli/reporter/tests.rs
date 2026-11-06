use super::localized::LocalizedCliReport;
use super::render::render_text_report;
use super::*;
use crate::cli::args::Cli;
use crate::i18n::Locale;
use crate::{LintResult, Severity};
use std::path::Path;

fn diagnostic_report(path: &str, unsafe_fix: bool) -> CliReport {
    let mut report = CliReport {
        command: "check",
        summary: CliSummary::default(),
        files: vec![FileReport {
            path: path.into(),
            diagnostics: vec![heading_diagnostic(unsafe_fix)],
            applied_fixes: 0,
            fix_details: Vec::new(),
            changed: false,
        }],
        errors: Vec::new(),
    };
    report.recompute_summary();
    report
}

fn heading_diagnostic(unsafe_fix: bool) -> LintResult {
    const HEADING_END_COLUMN: usize = 6;
    let message = "No space after hash on atx style heading";
    LintResult {
        rule_id: "MD018".into(),
        rule_name: "no-missing-space-atx".into(),
        message: message.into(),
        message_id: "rule.generic".into(),
        message_params: crate::i18n::MessageCatalog::diagnostic_message_params(
            "MD018",
            "no-missing-space-atx",
            message,
        ),
        severity: Severity::Warning,
        line: 1,
        column: 1,
        end_line: 1,
        end_column: HEADING_END_COLUMN,
        fix: unsafe_fix.then(|| crate::Fix {
            range: crate::Range {
                start_line: 1,
                start_column: 1,
                end_line: 1,
                end_column: HEADING_END_COLUMN,
            },
            replacement: "# Heading".into(),
            safety: crate::FixSafety::Unsafe,
        }),
    }
}

#[test]
fn renders_text_report_contract_with_statistics_and_verbose() {
    let report = diagnostic_report("<stdin>", false);

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
    let report = diagnostic_report("README.md", true);

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
    let report = diagnostic_report("README.md", false);

    let output = render_text_report(&report, false, &Cli::default(), Locale::Ja);

    assert!(output.contains("README.md:1:1 MD018 ATX 見出し"));
}

#[test]
fn json_report_keeps_shape_and_adds_localized_message_metadata() {
    let report = diagnostic_report("README.md", true);

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
