mod scan;
mod types;

use scan::CliArgScan;
use std::iter::Peekable;
use std::path::PathBuf;
use std::vec::IntoIter;
pub use types::{Cli, Command, ConfigCommand, HelpTopic, OutputFormat};

#[cfg(test)]
mod tests;

pub struct CliArgsParser;
type Args = Peekable<IntoIter<String>>;

impl CliArgsParser {
    pub fn parse_args(args: Vec<String>) -> Cli {
        if args.is_empty() || CliArgScan::requests_help(&args) {
            return Cli {
                command: Command::Help(CliArgScan::help_topic(&args)),
                locale: CliArgScan::locale_arg(&args),
                ..Cli::default()
            };
        }
        if CliArgScan::requests_version(&args) {
            return Cli {
                command: Command::Version,
                ..Cli::default()
            };
        }
        let mut cli = Cli::default();
        let mut iter = args.into_iter().peekable();
        while let Some(arg) = iter.next() {
            if !parse_command(&mut cli, &arg, &mut iter)
                && !parse_value_option(&mut cli, &arg, &mut iter)
                && !parse_flag(&mut cli, &arg)
                && !arg.starts_with('-')
            {
                cli.inputs.push(arg);
            }
        }
        cli
    }
}

fn parse_command(cli: &mut Cli, arg: &str, iter: &mut Args) -> bool {
    cli.command = match arg {
        "check" => Command::Check,
        "fix" => Command::Fix,
        "fmt" => Command::Fmt,
        "lsp" => Command::Lsp,
        "version" | "--version" | "-V" | "-v" => Command::Version,
        "rule" => Command::Rule(iter.next_if(|value| !value.starts_with('-'))),
        "config" => {
            let sub = iter.next_if(|value| !value.starts_with('-'));
            Command::Config(match sub.as_deref() {
                Some("file") => ConfigCommand::File,
                Some("schema") => ConfigCommand::Schema,
                _ => ConfigCommand::Get,
            })
        }
        "init-config" | "init" => Command::InitConfig,
        _ => return false,
    };
    true
}

fn parse_value_option(cli: &mut Cli, arg: &str, iter: &mut Args) -> bool {
    match arg {
        "--config" => cli.config = iter.next().map(PathBuf::from).or(cli.config.take()),
        "--file" => cli.inputs.extend(iter.next()),
        "--format" | "--output" => {
            if iter.next().as_deref() == Some("json") {
                cli.format = OutputFormat::Json;
            }
        }
        "--locale" | "--local" | "-l" => cli.locale = iter.next().or(cli.locale.take()),
        "--include" => cli.include.extend(iter.next()),
        "--exclude" => cli.exclude.extend(iter.next()),
        _ => return false,
    }
    true
}

fn parse_flag(cli: &mut Cli, arg: &str) -> bool {
    match arg {
        "--fix" => cli.check_fix = true,
        "--stdin" => cli.stdin = true,
        "--respect-gitignore" => cli.respect_gitignore = true,
        "--no-ignore" => cli.respect_gitignore = false,
        "--include-ignored" => cli.include_ignored = true,
        "--include-reserved" => cli.include_reserved = true,
        "--force-exclude" => cli.force_exclude = true,
        "--statistics" => cli.statistics = true,
        "--quiet" => cli.quiet = true,
        "--verbose" => cli.verbose = true,
        "--diff" => cli.diff = true,
        "--unsafe" => cli.unsafe_fixes = true,
        "--yes" | "-y" => cli.yes = true,
        "--ignore-config-errors" => cli.ignore_config_errors = true,
        _ => return false,
    }
    true
}
