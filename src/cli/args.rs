mod scan;
mod types;

use scan::CliArgScan;
use std::path::PathBuf;
pub use types::{Cli, Command, ConfigCommand, HelpTopic, OutputFormat};

#[cfg(test)]
mod tests;

pub struct CliArgsParser;

impl CliArgsParser {
    pub fn parse_args(args: Vec<String>) -> Cli {
        if args.is_empty() {
            return Cli {
                command: Command::Help(None),
                ..Cli::default()
            };
        }

        let early_locale = CliArgScan::locale_arg(&args);
        if CliArgScan::requests_help(&args) {
            return Cli {
                command: Command::Help(CliArgScan::help_topic(&args)),
                locale: early_locale,
                ..Cli::default()
            };
        }

        if CliArgScan::requests_version(&args) {
            return Cli {
                command: Command::Version,
                ..Cli::default()
            };
        }

        let mut command = Command::Check;
        let mut config = None;
        let mut format = OutputFormat::Text;
        let mut inputs = Vec::new();
        let mut check_fix = false;
        let mut stdin = false;
        let mut include = Vec::new();
        let mut exclude = Vec::new();
        let mut respect_gitignore = true;
        let mut include_ignored = false;
        let mut include_reserved = false;
        let mut force_exclude = false;
        let mut statistics = false;
        let mut quiet = false;
        let mut verbose = false;
        let mut diff = false;
        let mut locale = None;
        let mut unsafe_fixes = false;
        let mut yes = false;
        let mut ignore_config_errors = false;
        let mut iter = args.into_iter().peekable();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "check" => command = Command::Check,
                "fix" => command = Command::Fix,
                "fmt" => command = Command::Fmt,
                "lsp" => command = Command::Lsp,
                "version" | "--version" | "-V" | "-v" => command = Command::Version,
                "rule" => {
                    let rule_id = iter
                        .next_if(|value| !value.starts_with('-'))
                        .map(|value| value.to_string());
                    command = Command::Rule(rule_id);
                }
                "config" => {
                    let sub = iter
                        .next_if(|value| !value.starts_with('-'))
                        .map(|value| value.to_string());
                    command = Command::Config(match sub.as_deref() {
                        Some("file") => ConfigCommand::File,
                        Some("schema") => ConfigCommand::Schema,
                        Some("get") | None => ConfigCommand::Get,
                        Some(value) if value.starts_with('-') => ConfigCommand::Get,
                        Some(_) => ConfigCommand::Get,
                    });
                }
                "init-config" | "init" => command = Command::InitConfig,
                "--fix" => check_fix = true,
                "--stdin" => stdin = true,
                "--config" => {
                    if let Some(value) = iter.next() {
                        config = Some(PathBuf::from(value));
                    }
                }
                "--file" => {
                    if let Some(value) = iter.next() {
                        inputs.push(value);
                    }
                }
                "--format" => set_json_format_if_requested(&mut format, iter.next()),
                "--output" => set_json_format_if_requested(&mut format, iter.next()),
                "--locale" | "--local" | "-l" => {
                    if let Some(value) = iter.next() {
                        locale = Some(value);
                    }
                }
                "--include" => {
                    if let Some(value) = iter.next() {
                        include.push(value);
                    }
                }
                "--exclude" => {
                    if let Some(value) = iter.next() {
                        exclude.push(value);
                    }
                }
                "--respect-gitignore" => respect_gitignore = true,
                "--no-ignore" => respect_gitignore = false,
                "--include-ignored" => include_ignored = true,
                "--include-reserved" => include_reserved = true,
                "--force-exclude" => force_exclude = true,
                "--statistics" => statistics = true,
                "--quiet" => quiet = true,
                "--verbose" => verbose = true,
                "--diff" => diff = true,
                "--unsafe" => unsafe_fixes = true,
                "--yes" | "-y" => yes = true,
                "--ignore-config-errors" => ignore_config_errors = true,
                other if other.starts_with('-') => {}
                other => inputs.push(other.to_string()),
            }
        }

        Cli {
            command,
            config,
            format,
            inputs,
            check_fix,
            stdin,
            include,
            exclude,
            respect_gitignore,
            include_ignored,
            include_reserved,
            force_exclude,
            statistics,
            quiet,
            verbose,
            diff,
            locale,
            unsafe_fixes,
            yes,
            ignore_config_errors,
        }
    }
}

fn set_json_format_if_requested(format: &mut OutputFormat, value: Option<String>) {
    if value.as_deref() == Some("json") {
        *format = OutputFormat::Json;
    }
}
