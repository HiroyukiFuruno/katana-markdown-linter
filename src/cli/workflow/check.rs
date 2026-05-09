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
use crate::config::ConfigLoader;
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
    let mut report = CliReport {
        command,
        files: Vec::new(),
        errors: Vec::new(),
        summary: CliSummary::default(),
    };
    if cli.stdin {
        return run_stdin_check_like(command, fix_mode, cli, locale);
    }

    let files = match super::super::input::expand_inputs(cli) {
        Ok(files) => files,
        Err(err) => {
            report.errors.push(CliError::from_input_expand_error(err));
            report.recompute_summary();
            output_report(&report, fix_mode, cli, locale)?;
            return Ok(2);
        }
    };

    let unsafe_policy = if fix_mode {
        resolve_unsafe_fix_policy(cli, &files, locale)?
    } else {
        UnsafeFixPolicy::default()
    };
    if unsafe_policy.declined {
        report.summary.unsafe_fix_status = Some("confirmation_declined".to_string());
        output_report(&report, fix_mode, cli, locale)?;
        return Ok(1);
    }

    let config_errors = collect_config_validation_errors(&files, cli.config.as_deref());
    if !config_errors.is_empty() {
        report.errors.extend(config_errors);
        if !cli.ignore_config_errors {
            report.recompute_summary();
            output_report(&report, fix_mode, cli, locale)?;
            return Ok(2);
        }
    }

    for path in files {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                report.errors.push(CliError::filesystem(&path, err));
                continue;
            }
        };

        let config = match ConfigLoader::load_effective_config(&path, cli.config.as_deref()) {
            Ok(config) => config,
            Err(err) => {
                report.errors.push(CliError::config(&path, err));
                continue;
            }
        };

        let options = config.to_lint_options();
        let results = match MarkdownLinter::lint_for_path(&path, &content, &options) {
            Ok(results) => results,
            Err(err) => {
                report.errors.push(CliError::rule(&path, err.to_string()));
                continue;
            }
        };

        if fix_mode {
            let FixedContent {
                content: fixed_content,
                diagnostics,
                applied_fixes,
                fix_details,
            } = apply_fixes_until_stable(
                &content,
                &path,
                results,
                &options,
                unsafe_policy.include_unsafe,
            )?;
            let changed = match write_fixed_content(&path, &content, &fixed_content, cli.diff) {
                Ok(changed) => changed,
                Err(err) => {
                    report.errors.push(CliError::filesystem(&path, err));
                    continue;
                }
            };

            report.files.push(FileReport {
                path: path.display().to_string(),
                diagnostics,
                applied_fixes,
                fix_details,
                changed,
            });
        } else {
            report.files.push(FileReport {
                path: path.display().to_string(),
                diagnostics: results,
                applied_fixes: 0,
                fix_details: Vec::new(),
                changed: false,
            });
        }
    }

    report.recompute_summary();
    let exit_code = if has_blocking_errors(&report, cli.ignore_config_errors) {
        2
    } else if report.summary.total_issues > 0 {
        1
    } else {
        0
    };

    output_report(&report, fix_mode, cli, locale)?;

    Ok(exit_code)
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
