use super::localized::LocalizedCliReport;
use super::text::fixed_count_message;
use super::types::CliReport;
use crate::cli::args::{Cli, OutputFormat};
use crate::i18n::{Locale, MessageParams};

pub(in crate::cli) fn print_text_report(
    report: &CliReport,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) {
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
    if has_config_validation_errors(report) && !cli.ignore_config_errors {
        eprintln!(
            "{}",
            crate::i18n::MessageCatalog::render_message(
                locale,
                "config.fix_or_ignore_advice",
                &MessageParams::new(),
                "Fix the markdownlint config, or rerun with --ignore-config-errors to ignore invalid config entries."
            )
        );
    }
    print!("{}", render_text_report(report, fix_mode, cli, locale));
}

fn has_config_validation_errors(report: &CliReport) -> bool {
    report
        .errors
        .iter()
        .any(|error| error.kind == "config" && error.message_id != "config.error")
}

pub(in crate::cli) fn output_report(
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

pub(in crate::cli) fn render_text_report(
    report: &CliReport,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> String {
    let mut output = String::new();
    for file in &report.files {
        render_file_report(&mut output, report, file, fix_mode, cli, locale);
    }

    if report.files.is_empty() && report.errors.is_empty() {
        output.push_str(&crate::i18n::MessageCatalog::render_message(
            locale,
            "summary.no_files",
            &MessageParams::new(),
            "No Markdown files found",
        ));
        output.push('\n');
    }
    if cli.statistics {
        render_statistics(&mut output, report, locale);
    }
    if cli.verbose {
        output.push_str(&format!("command: {}\n", report.command));
    }
    output
}

fn render_file_report(
    output: &mut String,
    report: &CliReport,
    file: &super::types::FileReport,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) {
    if fix_mode && file.applied_fixes > 0 && !cli.quiet {
        let (message_id, fallback) = fixed_count_message(report, file);
        let mut params = MessageParams::new();
        params.insert("path".to_string(), file.path.clone());
        params.insert("count".to_string(), file.applied_fixes.to_string());
        output.push_str(&crate::i18n::MessageCatalog::render_message(
            locale, message_id, &params, &fallback,
        ));
        output.push('\n');
    }

    for result in &file.diagnostics {
        if cli.quiet {
            continue;
        }
        let message = crate::i18n::MessageCatalog::render_message(
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

fn render_statistics(output: &mut String, report: &CliReport, locale: Locale) {
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
    output.push_str(&crate::i18n::MessageCatalog::render_message(
        locale,
        "summary.statistics",
        &params,
        &fallback,
    ));
    output.push('\n');
}
