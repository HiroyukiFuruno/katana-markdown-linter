use crate::{fix, lint, LintOptions, MarkdownLintConfig, RuleConfig};
use glob::glob;
use serde::Serialize;
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
    let command = if fix_mode { "fix" } else { "check" };
    let mut report = CliReport {
        command,
        files: Vec::new(),
        errors: Vec::new(),
        summary: CliSummary::default(),
    };
    let files = expand_inputs(&cli.inputs)?;

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
        let config_errors = config
            .validate(&crate::rules::markdown::MarkdownLinterOps::get_user_configurable_rules());
        if !config_errors.is_empty() {
            for error in config_errors {
                report
                    .errors
                    .push(CliError::config(&path, error.to_string()));
            }
            continue;
        }

        let options = options_from_config(&config);
        let results = match lint(&content, &options) {
            Ok(results) => results,
            Err(err) => {
                report.errors.push(CliError::rule(&path, err.to_string()));
                continue;
            }
        };

        if fix_mode {
            let fixed = match fix(&content, &options) {
                Ok(fixed) => fixed,
                Err(err) => {
                    report.errors.push(CliError::rule(&path, err.to_string()));
                    continue;
                }
            };
            let applied_fixes = fixed.applied_fixes;
            let fixed_content = fixed.content;
            let changed = fixed_content != content;
            if changed {
                if let Err(err) = fs::write(&path, &fixed_content) {
                    report.errors.push(CliError::filesystem(&path, err));
                    continue;
                }
            }

            let diagnostics = lint(&fixed_content, &options).map_err(|err| err.to_string())?;
            report.files.push(FileReport {
                path: path.display().to_string(),
                diagnostics,
                applied_fixes,
                changed,
            });
        } else {
            report.files.push(FileReport {
                path: path.display().to_string(),
                diagnostics: results,
                applied_fixes: 0,
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

    match cli.format {
        OutputFormat::Text => print_text_report(&report, fix_mode),
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
            );
        }
    }

    Ok(exit_code)
}

fn print_text_report(report: &CliReport, fix_mode: bool) {
    for error in &report.errors {
        if let Some(path) = &error.path {
            eprintln!("{}: {} error: {}", path, error.kind, error.message);
        } else {
            eprintln!("{} error: {}", error.kind, error.message);
        }
    }

    for file in &report.files {
        if fix_mode && file.applied_fixes > 0 {
            println!(
                "{}: fixed {} issue{}",
                file.path,
                file.applied_fixes,
                plural(file.applied_fixes)
            );
        }

        for result in &file.diagnostics {
            println!(
                "{}:{}:{} {} {}",
                file.path, result.line, result.column, result.rule_id, result.message
            );
        }
    }

    if report.files.is_empty() && report.errors.is_empty() {
        println!("No Markdown files found");
    }
}

fn plural(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn options_from_config(config: &MarkdownLintConfig) -> LintOptions {
    let mut options = LintOptions::default();
    let default_enabled = config
        .raw
        .as_object()
        .and_then(|root| root.get("default"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);

    for rule in crate::rules::markdown::MarkdownLinterOps::get_user_configurable_rules() {
        if let Some(meta) = rule.official_meta() {
            options.rules.insert(
                meta.code.to_string(),
                RuleConfig {
                    enabled: default_enabled,
                    properties: std::collections::HashMap::new(),
                },
            );
        }
    }

    let Some(root) = config.raw.as_object() else {
        return options;
    };

    for (key, value) in root {
        if key == "default" {
            continue;
        }
        let enabled = match value {
            serde_json::Value::Bool(enabled) => *enabled,
            serde_json::Value::Object(properties) => properties
                .get("enabled")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(default_enabled),
            _ => default_enabled,
        };
        options.rules.entry(key.clone()).or_default().enabled = enabled;
    }

    options
}

#[derive(Debug, Clone, Serialize)]
struct CliReport {
    command: &'static str,
    summary: CliSummary,
    files: Vec<FileReport>,
    errors: Vec<CliError>,
}

impl CliReport {
    fn recompute_summary(&mut self) {
        let mut summary = CliSummary {
            total_files: self.files.len(),
            ..CliSummary::default()
        };

        for file in &self.files {
            if !file.diagnostics.is_empty() {
                summary.files_with_issues += 1;
            }
            summary.total_issues += file.diagnostics.len();
            summary.fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.fix.is_some())
                .count();
            summary.applied_fixes += file.applied_fixes;
        }

        self.summary = summary;
    }
}

#[derive(Debug, Clone, Default, Serialize)]
struct CliSummary {
    total_files: usize,
    files_with_issues: usize,
    total_issues: usize,
    fixable_issues: usize,
    applied_fixes: usize,
}

#[derive(Debug, Clone, Serialize)]
struct FileReport {
    path: String,
    diagnostics: Vec<crate::LintResult>,
    applied_fixes: usize,
    changed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct CliError {
    kind: &'static str,
    path: Option<String>,
    message: String,
}

impl CliError {
    fn filesystem(path: &Path, err: impl std::fmt::Display) -> Self {
        Self {
            kind: "filesystem",
            path: Some(path.display().to_string()),
            message: err.to_string(),
        }
    }

    fn config(path: &Path, message: String) -> Self {
        Self {
            kind: "config",
            path: Some(path.display().to_string()),
            message,
        }
    }

    fn rule(path: &Path, message: String) -> Self {
        Self {
            kind: "rule",
            path: Some(path.display().to_string()),
            message,
        }
    }
}

fn load_effective_config(
    path: &Path,
    explicit: Option<&Path>,
) -> Result<MarkdownLintConfig, String> {
    if let Some(path) = explicit {
        if !path.exists() {
            return Err(format!("config file not found: {}", path.display()));
        }
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
        return markdown_files_in_dir(&env::current_dir().map_err(|err| err.to_string())?);
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

    let mut expanded = Vec::new();
    for path in paths {
        if path.is_dir() {
            expanded.extend(markdown_files_in_dir(&path)?);
        } else {
            expanded.push(path);
        }
    }
    expanded.sort();
    expanded.dedup();
    Ok(expanded)
}

fn has_glob_chars(input: &str) -> bool {
    input.contains('*') || input.contains('?') || input.contains('[')
}

fn markdown_files_in_dir(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect_markdown_files(dir, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect_markdown_files(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|err| format!("{}: {err}", dir.display()))? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, paths)?;
        } else if is_markdown_file(&path) {
            paths.push(path);
        }
    }
    Ok(())
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            let extension = extension.to_ascii_lowercase();
            extension == "md" || extension == "markdown"
        })
        .unwrap_or(false)
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
    use std::time::{SystemTime, UNIX_EPOCH};

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

    #[test]
    fn check_returns_one_when_violations_exist() {
        let dir = test_dir("check-violation");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        fs::write(&file, "#Title\n").expect("test file should be written");

        let exit = run(Cli {
            command: Command::Check,
            config: None,
            format: OutputFormat::Text,
            inputs: vec![file.display().to_string()],
        })
        .expect("check should run");

        assert_eq!(exit, 1);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn fix_returns_zero_when_fixable_violations_are_resolved() {
        let dir = test_dir("fix-violation");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        let config = dir.join(".markdownlint.json");
        fs::write(&file, "#Title\n").expect("test file should be written");
        fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
            .expect("config should be written");

        let exit = run(Cli {
            command: Command::Fix,
            config: Some(config),
            format: OutputFormat::Text,
            inputs: vec![file.display().to_string()],
        })
        .expect("fix should run");

        assert_eq!(exit, 0);
        assert_eq!(
            fs::read_to_string(&file).expect("fixed file should be readable"),
            "# Title\n"
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn explicit_config_can_disable_default_rules() {
        let dir = test_dir("config-disable");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        let config = dir.join(".markdownlint.jsonc");
        fs::write(&file, "#Title\n").expect("test file should be written");
        fs::write(
            &config,
            "{\n  // Disable all rules\n  \"default\": false,\n}\n",
        )
        .expect("config should be written");

        let exit = run(Cli {
            command: Command::Check,
            config: Some(config),
            format: OutputFormat::Json,
            inputs: vec![file.display().to_string()],
        })
        .expect("check should run");

        assert_eq!(exit, 0);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn directory_input_discovers_markdown_files() {
        let dir = test_dir("directory-input");
        fs::create_dir_all(&dir).expect("test dir should be created");
        fs::write(dir.join("bad.md"), "#Title\n").expect("markdown file should be written");
        fs::write(dir.join("ignored.txt"), "#Title\n").expect("text file should be written");

        let files = expand_inputs(&[dir.display().to_string()]).expect("input should expand");

        assert_eq!(files, vec![dir.join("bad.md")]);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn missing_explicit_config_is_config_error() {
        let dir = test_dir("missing-config");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("good.md");
        fs::write(&file, "# Title\n").expect("test file should be written");

        let exit = run(Cli {
            command: Command::Check,
            config: Some(dir.join("missing.json")),
            format: OutputFormat::Json,
            inputs: vec![file.display().to_string()],
        })
        .expect("check should run");

        assert_eq!(exit, 2);
        let _ = fs::remove_dir_all(dir);
    }

    fn test_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        env::temp_dir().join(format!(
            "katana-markdown-linter-cli-{name}-{}-{nanos}",
            std::process::id()
        ))
    }
}
