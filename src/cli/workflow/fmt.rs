use super::super::args::Cli;
use super::super::reporter::{
    output_report, print_diff, CliError, CliReport, CliSummary, FileReport,
};
use crate::config::ConfigLoader;
use crate::i18n::Locale;
use crate::{FormatOptions, MarkdownFormatter};
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub(super) fn run_fmt(cli: &Cli, locale: Locale) -> Result<i32, String> {
    let mut report = CliReport {
        command: "fmt",
        files: Vec::new(),
        errors: Vec::new(),
        summary: CliSummary::default(),
    };
    if cli.stdin {
        return run_stdin_fmt(cli, locale);
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

    for path in files {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                report.errors.push(CliError::filesystem(&path, err));
                continue;
            }
        };

        match ConfigLoader::validate_effective_config(&path, cli.config.as_deref()) {
            Ok(errors) if errors.is_empty() => {}
            Ok(errors) => {
                for error in errors {
                    report
                        .errors
                        .push(CliError::config_validation(&path, error));
                }
                continue;
            }
            Err(err) => {
                report.errors.push(CliError::config(&path, err));
                continue;
            }
        }

        let formatted = MarkdownFormatter::format_markdown(&content, &FormatOptions::default())
            .map_err(|err| err.to_string())?;
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

fn run_stdin_fmt(cli: &Cli, locale: Locale) -> Result<i32, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    match ConfigLoader::validate_effective_config(Path::new("<stdin>"), cli.config.as_deref()) {
        Ok(errors) if errors.is_empty() => {}
        Ok(errors) => {
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
    }

    let formatted = format_stdin_content(&content)?;
    if cli.diff {
        print_diff(Path::new("<stdin>"), &content, &formatted);
    } else {
        print!("{formatted}");
    }
    Ok(0)
}

pub(super) fn format_stdin_content(content: &str) -> Result<String, String> {
    Ok(
        MarkdownFormatter::format_markdown(content, &FormatOptions::default())
            .map_err(|err| err.to_string())?
            .content,
    )
}
