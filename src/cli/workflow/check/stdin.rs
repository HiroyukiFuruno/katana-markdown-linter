use super::super::common::apply_fixes_until_stable;
use crate::cli::args::{Cli, OutputFormat};
use crate::cli::reporter::{
    output_report, print_diff, CliError, CliReport, CliSummary, FileReport,
};
use crate::config::ConfigLoader;
use crate::i18n::Locale;
use crate::MarkdownLinter;
use std::io::{self, Read};
use std::path::Path;

pub(super) fn run_stdin_check_like(
    command: &'static str,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> Result<i32, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    let config =
        match ConfigLoader::load_effective_config(Path::new("<stdin>"), cli.config.as_deref()) {
            Ok(config) => config,
            Err(err) => {
                let mut report = CliReport {
                    command,
                    summary: CliSummary::default(),
                    files: Vec::new(),
                    errors: vec![CliError::config(Path::new("<stdin>"), err)],
                };
                report.recompute_summary();
                output_report(&report, fix_mode, cli, locale)?;
                return Ok(2);
            }
        };
    let config_errors = config.validate_against_schema();
    if !config_errors.is_empty() {
        let mut report = CliReport {
            command,
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: config_errors
                .into_iter()
                .map(|error| CliError::config_validation(Path::new("<stdin>"), error))
                .collect(),
        };
        report.recompute_summary();
        output_report(&report, fix_mode, cli, locale)?;
        return Ok(2);
    }
    let options = config.to_lint_options();
    if fix_mode {
        return run_stdin_fix(command, cli, locale, content, options);
    }

    let diagnostics = match MarkdownLinter::lint(&content, &options) {
        Ok(diagnostics) => diagnostics,
        Err(err) => {
            let mut report = CliReport {
                command,
                summary: CliSummary::default(),
                files: Vec::new(),
                errors: vec![CliError::rule(Path::new("<stdin>"), err.to_string())],
            };
            report.recompute_summary();
            output_report(&report, false, cli, locale)?;
            return Ok(2);
        }
    };
    let mut report = CliReport {
        command,
        summary: CliSummary::default(),
        files: vec![FileReport {
            path: "<stdin>".to_string(),
            diagnostics,
            applied_fixes: 0,
            fix_details: Vec::new(),
            changed: false,
        }],
        errors: Vec::new(),
    };
    report.recompute_summary();
    output_report(&report, false, cli, locale)?;
    Ok(if report.summary.total_issues > 0 {
        1
    } else {
        0
    })
}

fn run_stdin_fix(
    command: &'static str,
    cli: &Cli,
    locale: Locale,
    content: String,
    options: crate::LintOptions,
) -> Result<i32, String> {
    if cli.unsafe_fixes && !cli.yes {
        return Err(
            "unsafe fixes with --stdin require --yes because stdin is used for content".to_string(),
        );
    }
    let results = MarkdownLinter::lint(&content, &options).map_err(|err| err.to_string())?;
    let fixed = apply_fixes_until_stable(
        &content,
        Path::new("<stdin>"),
        results,
        &options,
        cli.unsafe_fixes,
    )?;
    if matches!(cli.format, OutputFormat::Json) {
        let mut report = CliReport {
            command,
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "<stdin>".to_string(),
                diagnostics: fixed.diagnostics,
                applied_fixes: fixed.applied_fixes,
                fix_details: fixed.fix_details,
                changed: fixed.content != content,
            }],
            errors: Vec::new(),
        };
        report.recompute_summary();
        let exit = if report.summary.total_issues > 0 {
            1
        } else {
            0
        };
        output_report(&report, true, cli, locale)?;
        return Ok(exit);
    }
    if cli.diff {
        print_diff(Path::new("<stdin>"), &content, &fixed.content);
    } else {
        print!("{}", fixed.content);
    }
    Ok(if fixed.diagnostics.is_empty() { 0 } else { 1 })
}
