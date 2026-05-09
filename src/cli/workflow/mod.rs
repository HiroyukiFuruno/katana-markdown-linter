mod check;
mod common;
mod config_cmd;
mod fmt;
mod help;

use super::args::{Cli, Command};
use crate::i18n::Locale;
use crate::MarkdownLintConfig;
use std::path::PathBuf;

pub struct CliWorkflow;

impl CliWorkflow {
    pub fn run_from_env() -> i32 {
        match Self::run(super::args::CliArgsParser::parse_args(
            std::env::args().skip(1).collect(),
        )) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("{err}");
                2
            }
        }
    }

    pub fn run(cli: Cli) -> Result<i32, String> {
        let locale = Locale::resolve(cli.locale.as_deref()).map_err(|err| err.to_string())?;
        match cli.command {
            Command::InitConfig => {
                let path = cli
                    .config
                    .unwrap_or_else(|| PathBuf::from(".markdownlint.json"));
                MarkdownLintConfig::create_default_file(&path).map_err(|err| err.to_string())?;
                Ok(0)
            }
            Command::Check => {
                let exit = check::run_check_like("check", cli.check_fix, &cli, locale)?;
                Ok(exit)
            }
            Command::Fix => {
                let exit = check::run_check_like("fix", true, &cli, locale)?;
                Ok(exit)
            }
            Command::Fmt => fmt::run_fmt(&cli, locale),
            Command::Help(topic) => Ok(help::run_help(topic, locale)),
            Command::Lsp => {
                crate::lsp::LspServerRunner::run_stdio()?;
                Ok(0)
            }
            Command::Rule(rule_id) => config_cmd::run_rule(rule_id.as_deref(), cli.format, locale),
            Command::Config(ref command) => config_cmd::run_config(command.clone(), &cli),
            Command::Version => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                Ok(0)
            }
        }
    }
}

#[cfg(test)]
mod tests;
