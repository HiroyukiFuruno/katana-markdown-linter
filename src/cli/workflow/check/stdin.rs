use super::super::common::apply_fixes_until_stable;
use crate::cli::args::{Cli, OutputFormat};
use crate::cli::reporter::{output_report, print_diff, CliError, FileReport};
use crate::config::ConfigSession;
use crate::i18n::Locale;
use crate::MarkdownLinter;
use std::io::{self, Read};
use std::path::Path;

pub(super) fn run_stdin_check_like(
    command: &'static str,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
    config_session: &mut ConfigSession,
) -> Result<i32, String> {
    let content = read_stdin()?;
    let config = match config_session.load_for_path(Path::new("<stdin>")) {
        Ok(config) => config,
        Err(err) => {
            return stdin_error(
                command,
                fix_mode,
                cli,
                locale,
                vec![CliError::config(Path::new("<stdin>"), err)],
            )
        }
    };
    if !config.validation_errors.is_empty() {
        let errors = config
            .validation_errors
            .iter()
            .cloned()
            .map(|error| CliError::config_validation(Path::new("<stdin>"), error))
            .collect();
        return stdin_error(command, fix_mode, cli, locale, errors);
    }
    let options = config.lint_options.clone();
    match fix_mode {
        true => run_stdin_fix(command, cli, locale, content, options),
        false => stdin_lint(command, cli, locale, &content, &options),
    }
}

fn read_stdin() -> Result<String, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    Ok(content)
}

fn stdin_lint(
    command: &'static str,
    cli: &Cli,
    locale: Locale,
    content: &str,
    options: &crate::LintOptions,
) -> Result<i32, String> {
    let diagnostics = match MarkdownLinter::lint(content, options) {
        Ok(diagnostics) => diagnostics,
        Err(err) => {
            return stdin_error(
                command,
                false,
                cli,
                locale,
                vec![CliError::rule(Path::new("<stdin>"), err.to_string())],
            )
        }
    };
    let file = FileReport {
        path: "<stdin>".into(),
        diagnostics,
        applied_fixes: 0,
        fix_details: Vec::new(),
        changed: false,
    };
    stdin_file_report(command, false, cli, locale, file)
}

fn stdin_error(
    command: &'static str,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
    errors: Vec<CliError>,
) -> Result<i32, String> {
    let mut report = super::empty_report(command);
    report.errors = errors;
    report.recompute_summary();
    output_report(&report, fix_mode, cli, locale)?;
    Ok(2)
}

fn stdin_file_report(
    command: &'static str,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
    file: FileReport,
) -> Result<i32, String> {
    let mut report = super::empty_report(command);
    report.files.push(file);
    report.recompute_summary();
    let exit = i32::from(report.summary.total_issues > 0);
    output_report(&report, fix_mode, cli, locale)?;
    Ok(exit)
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
    let fixed = match apply_fixes_until_stable(
        &content,
        Path::new("<stdin>"),
        results,
        &options,
        cli.unsafe_fixes,
    ) {
        Ok(fixed) => fixed,
        Err(message) => {
            return stdin_error(
                command,
                true,
                cli,
                locale,
                vec![CliError::rule(Path::new("<stdin>"), message)],
            )
        }
    };
    output_stdin_fix(command, cli, locale, &content, fixed)
}

fn output_stdin_fix(
    command: &'static str,
    cli: &Cli,
    locale: Locale,
    content: &str,
    fixed: super::super::common::FixedContent,
) -> Result<i32, String> {
    if matches!(cli.format, OutputFormat::Json) {
        let file = FileReport {
            path: "<stdin>".to_string(),
            diagnostics: fixed.diagnostics,
            applied_fixes: fixed.applied_fixes,
            fix_details: fixed.fix_details,
            changed: fixed.content != content,
        };
        return stdin_file_report(command, true, cli, locale, file);
    }
    if cli.diff {
        print_diff(Path::new("<stdin>"), content, &fixed.content);
    } else {
        print!("{}", fixed.content);
    }
    Ok(if fixed.diagnostics.is_empty() { 0 } else { 1 })
}
