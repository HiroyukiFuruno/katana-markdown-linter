use crate::{fix, lint, LintOptions, MarkdownLintConfig};
use glob::glob;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Check,
    Fix,
    InitConfig,
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub command: Command,
    pub config: Option<PathBuf>,
    pub format: OutputFormat,
    pub inputs: Vec<String>,
}

pub fn run_from_env() -> i32 {
    match run(parse_args(env::args().skip(1).collect())) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{err}");
            2
        }
    }
}

pub fn run(cli: Cli) -> Result<i32, String> {
    match cli.command {
        Command::InitConfig => {
            let path = cli
                .config
                .unwrap_or_else(|| PathBuf::from(".markdownlint.json"));
            MarkdownLintConfig::create_default_file(&path).map_err(|err| err.to_string())?;
            Ok(0)
        }
        Command::Check => {
            let exit = run_check_like(false, &cli)?;
            Ok(exit)
        }
        Command::Fix => {
            let exit = run_check_like(true, &cli)?;
            Ok(exit)
        }
    }
}

fn run_check_like(fix_mode: bool, cli: &Cli) -> Result<i32, String> {
    let mut had_violation = false;
    let mut had_io_error = false;
    let mut had_config_error = false;
    let files = expand_inputs(&cli.inputs)?;

    for path in files {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                had_io_error = true;
                eprintln!("{}: {err}", path.display());
                continue;
            }
        };

        let config = load_effective_config(&path, cli.config.as_deref())?;
        let options = LintOptions::default();
        let results = match lint(&content, &options) {
            Ok(results) => results,
            Err(err) => {
                had_config_error = true;
                eprintln!("{}: {err}", path.display());
                continue;
            }
        };

        let _ = config;

        if fix_mode {
            let fixed = match fix(&content, &options) {
                Ok(fixed) => fixed,
                Err(err) => {
                    had_config_error = true;
                    eprintln!("{}: {err}", path.display());
                    continue;
                }
            };
            if fixed.content != content {
                fs::write(&path, fixed.content).map_err(|err| err.to_string())?;
                had_violation = true;
            }
        } else if !results.is_empty() {
            had_violation = true;
            match cli.format {
                OutputFormat::Text => {
                    for result in results {
                        println!(
                            "{}:{}:{} {} {}",
                            path.display(),
                            result.line,
                            result.column,
                            result.rule_id,
                            result.message
                        );
                    }
                }
                OutputFormat::Json => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&results).map_err(|e| e.to_string())?
                    );
                }
            }
        }
    }

    if had_io_error || had_config_error {
        return Ok(2);
    }
    if had_violation {
        return Ok(1);
    }
    Ok(0)
}

fn load_effective_config(
    path: &Path,
    explicit: Option<&Path>,
) -> Result<MarkdownLintConfig, String> {
    if let Some(path) = explicit {
        return MarkdownLintConfig::load(path).map_err(|err| err.to_string());
    }

    let mut current = path.parent();
    while let Some(dir) = current {
        let json = dir.join(".markdownlint.json");
        if json.exists() {
            return MarkdownLintConfig::load(&json).map_err(|err| err.to_string());
        }
        let jsonc = dir.join(".markdownlint.jsonc");
        if jsonc.exists() {
            return MarkdownLintConfig::load(&jsonc).map_err(|err| err.to_string());
        }
        current = dir.parent();
    }

    Ok(MarkdownLintConfig::default())
}

fn expand_inputs(inputs: &[String]) -> Result<Vec<PathBuf>, String> {
    if inputs.is_empty() {
        return Ok(vec![env::current_dir().map_err(|err| err.to_string())?]);
    }

    let mut paths = Vec::new();
    for input in inputs {
        if has_glob_chars(input) {
            for entry in glob(input).map_err(|err| err.to_string())? {
                match entry {
                    Ok(path) => paths.push(path),
                    Err(err) => return Err(err.to_string()),
                }
            }
        } else {
            paths.push(PathBuf::from(input));
        }
    }
    Ok(paths)
}

fn has_glob_chars(input: &str) -> bool {
    input.contains('*') || input.contains('?') || input.contains('[')
}

pub fn parse_args(args: Vec<String>) -> Cli {
    let mut command = Command::Check;
    let mut config = None;
    let mut format = OutputFormat::Text;
    let mut inputs = Vec::new();
    let mut iter = args.into_iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "check" => command = Command::Check,
            "fix" => command = Command::Fix,
            "init-config" | "init" => command = Command::InitConfig,
            "--config" => {
                if let Some(value) = iter.next() {
                    config = Some(PathBuf::from(value));
                }
            }
            "--format" => {
                if let Some(value) = iter.next() {
                    if value == "json" {
                        format = OutputFormat::Json;
                    }
                }
            }
            other if other.starts_with('-') => {}
            other => inputs.push(other.to_string()),
        }
    }

    Cli {
        command,
        config,
        format,
        inputs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_check_with_json_format_and_config() {
        let cli = parse_args(vec![
            "check".to_string(),
            "--config".to_string(),
            ".markdownlint.jsonc".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "docs/*.md".to_string(),
        ]);
        assert_eq!(cli.command, Command::Check);
        assert_eq!(cli.format, OutputFormat::Json);
        assert_eq!(cli.inputs, vec!["docs/*.md".to_string()]);
        assert_eq!(cli.config, Some(PathBuf::from(".markdownlint.jsonc")));
    }
}
