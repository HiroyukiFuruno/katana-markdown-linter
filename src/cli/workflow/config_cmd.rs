use super::super::args::{Cli, ConfigCommand, OutputFormat};
use super::super::reporter::LocalizedRuleMeta;
use crate::i18n::Locale;
use crate::MarkdownLintConfig;
use std::io::{BufRead, Write};
use std::path::PathBuf;

pub(super) fn run_rule(
    rule_id: Option<&str>,
    format: OutputFormat,
    locale: Locale,
) -> Result<i32, String> {
    print!("{}", render_rule(rule_id, format, locale)?);
    Ok(0)
}

pub(super) fn render_rule(
    rule_id: Option<&str>,
    format: OutputFormat,
    locale: Locale,
) -> Result<String, String> {
    let rules = crate::available_rules();
    if let Some(rule_id) = rule_id {
        let Some(rule) = rules.iter().find(|rule| rule.id == rule_id) else {
            return Err(format!("unknown rule: {rule_id}"));
        };
        let output = LocalizedRuleMeta::from_rule(rule, locale);
        return match format {
            OutputFormat::Text => Ok(format!(
                "{} {}\n{}\n{}\n",
                output.id, output.name, output.description, output.docs_url
            )),
            OutputFormat::Json => Ok(format!(
                "{}\n",
                serde_json::to_string_pretty(&output).map_err(|err| err.to_string())?
            )),
        };
    }

    match format {
        OutputFormat::Text => {
            let mut output = String::new();
            for rule in rules {
                let rule = LocalizedRuleMeta::from_rule(&rule, locale);
                output.push_str(&format!(
                    "{} {} - {}\n",
                    rule.id, rule.name, rule.description
                ));
            }
            Ok(output)
        }
        OutputFormat::Json => {
            let rules = rules
                .iter()
                .map(|rule| LocalizedRuleMeta::from_rule(rule, locale))
                .collect::<Vec<_>>();
            Ok(format!(
                "{}\n",
                serde_json::to_string_pretty(&rules).map_err(|err| err.to_string())?
            ))
        }
    }
}

pub(super) fn run_config(command: ConfigCommand, cli: &Cli) -> Result<i32, String> {
    print!("{}", render_config(command, cli)?);
    Ok(0)
}

pub(super) fn render_config(command: ConfigCommand, cli: &Cli) -> Result<String, String> {
    let path = cli
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from(".markdownlint.json"));
    match command {
        ConfigCommand::File => match cli.format {
            OutputFormat::Text => Ok(format!("{}\n", path.display())),
            OutputFormat::Json => Ok(format!(
                "{}\n",
                serde_json::to_string_pretty(&serde_json::json!({ "path": path }))
                    .map_err(|err| err.to_string())?
            )),
        },
        ConfigCommand::Get => {
            let config = MarkdownLintConfig::load(&path).map_err(|err| err.to_string())?;
            match cli.format {
                OutputFormat::Text | OutputFormat::Json => Ok(format!(
                    "{}\n",
                    serde_json::to_string_pretty(&config.raw).map_err(|err| err.to_string())?
                )),
            }
        }
    }
}

pub(super) fn prompt_unsafe_confirmation(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
) -> Result<bool, String> {
    write!(writer, "Apply unsafe fixes? [Y/n] ").map_err(|err| err.to_string())?;
    writer.flush().map_err(|err| err.to_string())?;

    let mut answer = String::new();
    match reader
        .read_line(&mut answer)
        .map_err(|err| err.to_string())?
    {
        0 => Ok(false),
        _ => {
            let answer = answer.trim();
            Ok(answer.is_empty()
                || answer.eq_ignore_ascii_case("y")
                || answer.eq_ignore_ascii_case("yes"))
        }
    }
}
