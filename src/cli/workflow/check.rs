mod config;
mod stdin;
mod unsafe_fixes;

use self::config::{collect_config_validation_errors, has_blocking_errors};
use self::stdin::run_stdin_check_like;
use self::unsafe_fixes::resolve_unsafe_fix_policy;
use super::super::args::Cli;
use super::super::reporter::{
    output_report, print_diff, CliError, CliReport, CliSummary, FileReport,
};
use super::common::{apply_fixes_until_stable, FixedContent, UnsafeFixPolicy};
use crate::config::ConfigSession;
use crate::i18n::Locale;
use crate::MarkdownLinter;
use std::fs;
use std::path::Path;

pub(super) fn run_check_like(
    command: &'static str,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> Result<i32, String> {
    let mut session = ConfigSession::new(cli.config.as_deref());
    if cli.stdin {
        return run_stdin_check_like(command, fix_mode, cli, locale, &mut session);
    }
    let mut report = empty_report(command);
    let files = match super::super::input::expand_inputs(cli) {
        Ok(files) => files,
        Err(err) => {
            report.errors.push(CliError::from_input_expand_error(err));
            return finish_report(&mut report, fix_mode, cli, locale);
        }
    };
    let policy = if fix_mode {
        resolve_unsafe_fix_policy(cli, &files, locale)?
    } else {
        UnsafeFixPolicy::default()
    };
    if policy.declined {
        report.summary.unsafe_fix_status = Some("confirmation_declined".to_string());
        output_report(&report, fix_mode, cli, locale)?;
        return Ok(1);
    }
    process_files(&mut report, files, fix_mode, cli, &mut session, &policy);
    finish_report(&mut report, fix_mode, cli, locale)
}

fn process_files(
    report: &mut CliReport,
    files: Vec<std::path::PathBuf>,
    fix_mode: bool,
    cli: &Cli,
    session: &mut ConfigSession,
    policy: &UnsafeFixPolicy,
) {
    report
        .errors
        .extend(collect_config_validation_errors(&files, session));
    if report.errors.is_empty() || cli.ignore_config_errors {
        for path in files {
            match check_file(&path, fix_mode, cli, session, policy) {
                Ok(file) => report.files.push(file),
                Err(error) => report.errors.push(error),
            }
        }
    }
}

fn empty_report(command: &'static str) -> CliReport {
    CliReport {
        command,
        files: Vec::new(),
        errors: Vec::new(),
        summary: CliSummary::default(),
    }
}

fn finish_report(
    report: &mut CliReport,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> Result<i32, String> {
    report.recompute_summary();
    let exit = if has_blocking_errors(report, cli.ignore_config_errors) {
        2
    } else if report.summary.total_issues > 0 {
        1
    } else {
        0
    };
    output_report(report, fix_mode, cli, locale)?;
    Ok(exit)
}

fn check_file(
    path: &Path,
    fix_mode: bool,
    cli: &Cli,
    session: &mut ConfigSession,
    policy: &UnsafeFixPolicy,
) -> Result<FileReport, CliError> {
    let content = fs::read_to_string(path).map_err(|err| CliError::filesystem(path, err))?;
    let config = session
        .load_for_path(path)
        .map_err(|err| CliError::config(path, err))?;
    let options = &config.lint_options;
    let diagnostics = MarkdownLinter::lint_for_path(path, &content, options)
        .map_err(|err| CliError::rule(path, err.to_string()))?;
    if fix_mode {
        let fixed =
            apply_fixes_until_stable(&content, path, diagnostics, options, policy.include_unsafe)
                .map_err(|err| CliError::rule(path, err))?;
        return fixed_file_report(path, &content, fixed, cli.diff);
    }
    Ok(FileReport {
        path: path.display().to_string(),
        diagnostics,
        applied_fixes: 0,
        fix_details: Vec::new(),
        changed: false,
    })
}

fn fixed_file_report(
    path: &Path,
    content: &str,
    fixed: FixedContent,
    diff: bool,
) -> Result<FileReport, CliError> {
    let changed = write_fixed_content(path, content, &fixed.content, diff)
        .map_err(|err| CliError::filesystem(path, err))?;
    Ok(FileReport {
        path: path.display().to_string(),
        diagnostics: fixed.diagnostics,
        applied_fixes: fixed.applied_fixes,
        fix_details: fixed.fix_details,
        changed,
    })
}

fn write_fixed_content(
    path: &Path,
    content: &str,
    fixed_content: &str,
    diff: bool,
) -> Result<bool, std::io::Error> {
    if fixed_content == content {
        return Ok(false);
    }
    if diff {
        print_diff(path, content, fixed_content);
    }
    fs::write(path, fixed_content)?;
    Ok(true)
}
