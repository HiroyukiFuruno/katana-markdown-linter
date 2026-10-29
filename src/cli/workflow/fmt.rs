use super::super::args::Cli;
use super::super::reporter::{
    output_report, print_diff, CliError, CliReport, CliSummary, FileReport,
};
use crate::config::ConfigSession;
use crate::i18n::Locale;
use crate::{FormatOptions, LintOptions, MarkdownFormatter};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub(super) fn run_fmt(cli: &Cli, locale: Locale) -> Result<i32, String> {
    let mut config_session = ConfigSession::new(cli.config.as_deref());
    let mut report = CliReport {
        command: "fmt",
        files: Vec::new(),
        errors: Vec::new(),
        summary: CliSummary::default(),
    };
    if cli.stdin {
        return run_stdin_fmt(cli, locale, &mut config_session);
    }

    let files = match super::super::input::expand_inputs(cli) {
        Ok(files) => files,
        Err(err) => {
            report.errors.push(CliError::from_input_expand_error(err));
            report.recompute_summary();
            output_report(&report, true, cli, locale)?;
            return Ok(2);
        }
    };

    let mut layout_policies = HashMap::new();
    for path in files {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                report.errors.push(CliError::filesystem(&path, err));
                continue;
            }
        };

        let config = match config_session.load_for_path(&path) {
            Ok(config) => config,
            Err(err) => {
                report.errors.push(CliError::config(&path, err));
                continue;
            }
        };
        let errors = config.validation_errors.clone();
        if !errors.is_empty() {
            for error in errors {
                report
                    .errors
                    .push(CliError::config_validation(&path, error));
            }
            continue;
        }

        let lint_options = layout_policies
            .entry(config.source.clone())
            .or_insert_with(|| MarkdownFormatter::layout_lint_options_from(&config.lint_options));
        let formatted = match MarkdownFormatter::format_markdown_with_lint_options(
            &content,
            &FormatOptions::default(),
            lint_options,
        ) {
            Ok(formatted) => formatted,
            Err(err) => {
                report.errors.push(CliError::rule(&path, err.to_string()));
                continue;
            }
        };
        let changed = formatted.content != content;
        if changed {
            if cli.diff {
                print_diff(&path, &content, &formatted.content);
            }
            if let Err(err) = fs::write(&path, &formatted.content) {
                report.errors.push(CliError::filesystem(&path, err));
                continue;
            }
        }

        report.files.push(FileReport {
            path: path.display().to_string(),
            diagnostics: Vec::new(),
            applied_fixes: formatted.applied_operations,
            fix_details: Vec::new(),
            changed,
        });
    }

    report.recompute_summary();
    let exit_code = if report.errors.is_empty() { 0 } else { 2 };
    output_report(&report, true, cli, locale)?;
    Ok(exit_code)
}

fn run_stdin_fmt(
    cli: &Cli,
    locale: Locale,
    config_session: &mut ConfigSession,
) -> Result<i32, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    let config = match config_session.load_for_path(Path::new("<stdin>")) {
        Ok(config) => config,
        Err(err) => {
            let mut report = CliReport {
                command: "fmt",
                summary: CliSummary::default(),
                files: Vec::new(),
                errors: vec![CliError::config(Path::new("<stdin>"), err)],
            };
            report.recompute_summary();
            output_report(&report, true, cli, locale)?;
            return Ok(2);
        }
    };
    let errors = config.validation_errors.clone();
    if !errors.is_empty() {
        let mut report = CliReport {
            command: "fmt",
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: errors
                .into_iter()
                .map(|error| CliError::config_validation(Path::new("<stdin>"), error))
                .collect(),
        };
        report.recompute_summary();
        output_report(&report, true, cli, locale)?;
        return Ok(2);
    }

    let lint_options = MarkdownFormatter::layout_lint_options_from(&config.lint_options);
    let formatted = match format_stdin_content(&content, &lint_options) {
        Ok(formatted) => formatted,
        Err(message) => {
            let mut report = CliReport {
                command: "fmt",
                summary: CliSummary::default(),
                files: Vec::new(),
                errors: vec![CliError::rule(Path::new("<stdin>"), message)],
            };
            report.recompute_summary();
            output_report(&report, true, cli, locale)?;
            return Ok(2);
        }
    };
    if cli.diff {
        print_diff(Path::new("<stdin>"), &content, &formatted);
    } else {
        print!("{formatted}");
    }
    Ok(0)
}

pub(super) fn format_stdin_content(
    content: &str,
    lint_options: &LintOptions,
) -> Result<String, String> {
    Ok(MarkdownFormatter::format_markdown_with_lint_options(
        content,
        &FormatOptions::default(),
        lint_options,
    )
    .map_err(|err| err.to_string())?
    .content)
}
