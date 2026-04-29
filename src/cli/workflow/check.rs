use super::super::args::Cli;
use super::super::reporter::{
    output_report, plural, print_diff, CliError, CliReport, CliSummary, FileReport,
};
use super::common::{
    apply_fixes_until_stable, load_effective_config, FixedContent, UnsafeFixPolicy,
};
use crate::i18n::Locale;
use crate::{lint, lint_for_path, FixSafety};
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

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

    for path in files {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                report.errors.push(CliError::filesystem(&path, err));
                continue;
            }
        };

        let config = match load_effective_config(&path, cli.config.as_deref()) {
            Ok(config) => config,
            Err(err) => {
                report.errors.push(CliError::config(&path, err));
                continue;
            }
        };
        let config_errors = config.validate_against_schema();
        if !config_errors.is_empty() {
            for error in config_errors {
                report
                    .errors
                    .push(CliError::config_validation(&path, error));
            }
            continue;
        }

        let options = config.to_lint_options();
        let results = match lint_for_path(&path, &content, &options) {
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
            let changed = fixed_content != content;
            if changed {
                if cli.diff {
                    print_diff(&path, &content, &fixed_content);
                }
                if let Err(err) = fs::write(&path, &fixed_content) {
                    report.errors.push(CliError::filesystem(&path, err));
                    continue;
                }
            }

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
    let exit_code = if !report.errors.is_empty() {
        2
    } else if report.summary.total_issues > 0 {
        1
    } else {
        0
    };

    output_report(&report, fix_mode, cli, locale)?;

    Ok(exit_code)
}

fn run_stdin_check_like(
    command: &'static str,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> Result<i32, String> {
    use std::io::Read;
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    let config = match load_effective_config(std::path::Path::new("<stdin>"), cli.config.as_deref())
    {
        Ok(config) => config,
        Err(err) => {
            let mut report = CliReport {
                command,
                summary: CliSummary::default(),
                files: Vec::new(),
                errors: vec![CliError::config(std::path::Path::new("<stdin>"), err)],
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
                .map(|error| CliError::config_validation(std::path::Path::new("<stdin>"), error))
                .collect(),
        };
        report.recompute_summary();
        output_report(&report, fix_mode, cli, locale)?;
        return Ok(2);
    }
    let options = config.to_lint_options();
    if fix_mode {
        if cli.unsafe_fixes && !cli.yes {
            return Err(
                "unsafe fixes with --stdin require --yes because stdin is used for content"
                    .to_string(),
            );
        }
        let results = lint(&content, &options).map_err(|err| err.to_string())?;
        let fixed = apply_fixes_until_stable(
            &content,
            std::path::Path::new("<stdin>"),
            results,
            &options,
            cli.unsafe_fixes,
        )?;
        if matches!(cli.format, super::super::args::OutputFormat::Json) {
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
            print_diff(std::path::Path::new("<stdin>"), &content, &fixed.content);
        } else {
            print!("{}", fixed.content);
        }
        return Ok(if fixed.diagnostics.is_empty() { 0 } else { 1 });
    }

    let diagnostics = match lint(&content, &options) {
        Ok(diagnostics) => diagnostics,
        Err(err) => {
            let mut report = CliReport {
                command,
                summary: CliSummary::default(),
                files: Vec::new(),
                errors: vec![CliError::rule(
                    std::path::Path::new("<stdin>"),
                    err.to_string(),
                )],
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

pub(super) fn resolve_unsafe_fix_policy(
    cli: &Cli,
    files: &[PathBuf],
    locale: Locale,
) -> Result<UnsafeFixPolicy, String> {
    if !cli.unsafe_fixes {
        return Ok(UnsafeFixPolicy::default());
    }
    let candidates = collect_unsafe_fix_candidates(files, cli)?;
    if candidates.is_empty() {
        return Ok(UnsafeFixPolicy {
            include_unsafe: false,
            declined: false,
        });
    }
    let mut stderr = io::stderr();
    write_unsafe_fix_summary(&mut stderr, &candidates, locale)?;
    if cli.yes {
        writeln!(stderr, "Unsafe fixes approved by --yes.").map_err(|err| err.to_string())?;
        return Ok(UnsafeFixPolicy {
            include_unsafe: true,
            declined: false,
        });
    }
    if !io::stdin().is_terminal() {
        return Err("unsafe fixes require --yes in non-interactive mode".to_string());
    }

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let approved = super::config_cmd::prompt_unsafe_confirmation(&mut stdin, &mut stderr)?;
    if approved {
        writeln!(stderr, "Unsafe fixes approved.").map_err(|err| err.to_string())?;
    } else {
        writeln!(stderr, "Unsafe fixes declined; no files changed.")
            .map_err(|err| err.to_string())?;
    }
    Ok(UnsafeFixPolicy {
        include_unsafe: approved,
        declined: !approved,
    })
}

struct UnsafeFixCandidateSummary {
    path: String,
    rule_id: String,
    count: usize,
}

fn collect_unsafe_fix_candidates(
    files: &[PathBuf],
    cli: &Cli,
) -> Result<Vec<UnsafeFixCandidateSummary>, String> {
    let mut summaries = Vec::new();
    for path in files {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let config = match load_effective_config(path, cli.config.as_deref()) {
            Ok(config) => config,
            Err(_) => continue,
        };
        if !config.validate_cached_rules().is_empty() {
            continue;
        }
        let options = config.to_lint_options();
        let diagnostics = lint_for_path(path, &content, &options).map_err(|err| err.to_string())?;
        let mut by_rule = std::collections::BTreeMap::<String, usize>::new();
        for diagnostic in diagnostics {
            if diagnostic
                .fix
                .as_ref()
                .is_some_and(|fix| fix.safety == FixSafety::Unsafe)
            {
                *by_rule.entry(diagnostic.rule_id).or_default() += 1;
            }
        }
        summaries.extend(
            by_rule
                .into_iter()
                .map(|(rule_id, count)| UnsafeFixCandidateSummary {
                    path: path.display().to_string(),
                    rule_id,
                    count,
                }),
        );
    }
    Ok(summaries)
}

fn write_unsafe_fix_summary(
    mut writer: impl Write,
    candidates: &[UnsafeFixCandidateSummary],
    _locale: Locale,
) -> Result<(), String> {
    let total = candidates
        .iter()
        .map(|candidate| candidate.count)
        .sum::<usize>();
    writeln!(
        writer,
        "Unsafe fixes requested: {total} candidate{}",
        plural(total)
    )
    .map_err(|err| err.to_string())?;
    for candidate in candidates {
        writeln!(
            writer,
            "- {} {}: {} candidate{}",
            candidate.path,
            candidate.rule_id,
            candidate.count,
            plural(candidate.count)
        )
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}
