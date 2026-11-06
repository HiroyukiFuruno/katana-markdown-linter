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

    format_files(&mut report, files, cli, &mut config_session);

    report.recompute_summary();
    let exit_code = if report.errors.is_empty() { 0 } else { 2 };
    output_report(&report, true, cli, locale)?;
    Ok(exit_code)
}

fn format_files(
    report: &mut CliReport,
    files: Vec<std::path::PathBuf>,
    cli: &Cli,
    session: &mut ConfigSession,
) {
    let mut policies = HashMap::new();
    for path in files {
        match format_file(&path, cli, session, &mut policies) {
            Ok(file) => report.files.push(file),
            Err(errors) => report.errors.extend(errors),
        }
    }
}

fn format_file(
    path: &Path,
    cli: &Cli,
    session: &mut ConfigSession,
    policies: &mut HashMap<Option<std::path::PathBuf>, LintOptions>,
) -> Result<FileReport, Vec<CliError>> {
    let content = fs::read_to_string(path).map_err(|err| vec![CliError::filesystem(path, err)])?;
    let config = session
        .load_for_path(path)
        .map_err(|err| vec![CliError::config(path, err)])?;
    if !config.validation_errors.is_empty() {
        return Err(config
            .validation_errors
            .iter()
            .cloned()
            .map(|error| CliError::config_validation(path, error))
            .collect());
    }
    let options = policies
        .entry(config.source.clone())
        .or_insert_with(|| MarkdownFormatter::layout_lint_options_from(&config.lint_options));
    let formatted = MarkdownFormatter::format_markdown_with_lint_options(
        &content,
        &FormatOptions::default(),
        options,
    )
    .map_err(|err| vec![CliError::rule(path, err.to_string())])?;
    let changed = write_formatted_file(path, &content, &formatted.content, cli.diff)?;

    Ok(formatted_file_report(
        path,
        formatted.applied_operations,
        changed,
    ))
}

fn formatted_file_report(path: &Path, applied_fixes: usize, changed: bool) -> FileReport {
    FileReport {
        path: path.display().to_string(),
        diagnostics: Vec::new(),
        applied_fixes,
        fix_details: Vec::new(),
        changed,
    }
}

fn write_formatted_file(
    path: &Path,
    content: &str,
    formatted: &str,
    diff: bool,
) -> Result<bool, Vec<CliError>> {
    if formatted == content {
        return Ok(false);
    }
    if diff {
        print_diff(path, content, formatted);
    }
    fs::write(path, formatted).map_err(|err| vec![CliError::filesystem(path, err)])?;
    Ok(true)
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
            return stdin_fmt_error(
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
        return stdin_fmt_error(cli, locale, errors);
    }
    format_stdin(cli, locale, &content, &config.lint_options)
}

fn format_stdin(
    cli: &Cli,
    locale: Locale,
    content: &str,
    options: &LintOptions,
) -> Result<i32, String> {
    let lint_options = MarkdownFormatter::layout_lint_options_from(options);
    let formatted = match format_stdin_content(content, &lint_options) {
        Ok(formatted) => formatted,
        Err(message) => {
            return stdin_fmt_error(
                cli,
                locale,
                vec![CliError::rule(Path::new("<stdin>"), message)],
            )
        }
    };
    if cli.diff {
        print_diff(Path::new("<stdin>"), content, &formatted);
    } else {
        print!("{formatted}");
    }
    Ok(0)
}

fn stdin_fmt_error(cli: &Cli, locale: Locale, errors: Vec<CliError>) -> Result<i32, String> {
    let mut report = CliReport {
        command: "fmt",
        summary: CliSummary::default(),
        files: Vec::new(),
        errors,
    };
    report.recompute_summary();
    output_report(&report, true, cli, locale)?;
    Ok(2)
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
