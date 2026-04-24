use crate::{fix_with_results, lint, LintOptions, MarkdownLintConfig, RuleConfig};
use glob::{glob, Pattern};
use ignore::{WalkBuilder, WalkState};
use serde::Serialize;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Check,
    Fix,
    Fmt,
    InitConfig,
    Rule(Option<String>),
    Config(ConfigCommand),
    Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigCommand {
    File,
    Get,
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub command: Command,
    pub config: Option<PathBuf>,
    pub format: OutputFormat,
    pub inputs: Vec<String>,
    pub check_fix: bool,
    pub stdin: bool,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub respect_gitignore: bool,
    pub force_exclude: bool,
    pub statistics: bool,
    pub quiet: bool,
    pub verbose: bool,
    pub diff: bool,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            command: Command::Check,
            config: None,
            format: OutputFormat::Text,
            inputs: Vec::new(),
            check_fix: false,
            stdin: false,
            include: Vec::new(),
            exclude: Vec::new(),
            respect_gitignore: true,
            force_exclude: false,
            statistics: false,
            quiet: false,
            verbose: false,
            diff: false,
        }
    }
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
            let exit = run_check_like(cli.check_fix, &cli)?;
            Ok(exit)
        }
        Command::Fix | Command::Fmt => {
            let exit = run_check_like(true, &cli)?;
            Ok(exit)
        }
        Command::Rule(rule_id) => run_rule(rule_id.as_deref(), cli.format),
        Command::Config(ref command) => run_config(command.clone(), &cli),
        Command::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(0)
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
    if cli.stdin {
        return run_stdin_check_like(fix_mode, cli);
    }

    let files = expand_inputs(cli)?;

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
        let config_errors = config.validate_cached_rules();
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
            let FixedContent {
                content: fixed_content,
                diagnostics,
                applied_fixes,
            } = apply_fixes_until_stable(&content, results, &options)?;
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
        OutputFormat::Text => print_text_report(&report, fix_mode, cli),
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
            );
        }
    }

    Ok(exit_code)
}

fn run_stdin_check_like(fix_mode: bool, cli: &Cli) -> Result<i32, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    let config = load_effective_config(Path::new("<stdin>"), cli.config.as_deref())?;
    let options = options_from_config(&config);
    if fix_mode {
        let results = lint(&content, &options).map_err(|err| err.to_string())?;
        let fixed = apply_fixes_until_stable(&content, results, &options)?;
        if cli.diff {
            print_diff(Path::new("<stdin>"), &content, &fixed.content);
        } else {
            print!("{}", fixed.content);
        }
        return Ok(0);
    }

    let diagnostics = lint(&content, &options).map_err(|err| err.to_string())?;
    let mut report = CliReport {
        command: "check",
        summary: CliSummary::default(),
        files: vec![FileReport {
            path: "<stdin>".to_string(),
            diagnostics,
            applied_fixes: 0,
            changed: false,
        }],
        errors: Vec::new(),
    };
    report.recompute_summary();
    match cli.format {
        OutputFormat::Text => print_text_report(&report, false, cli),
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
        ),
    }
    Ok(if report.summary.total_issues > 0 {
        1
    } else {
        0
    })
}

struct FixedContent {
    content: String,
    diagnostics: Vec<crate::LintResult>,
    applied_fixes: usize,
}

fn apply_fixes_until_stable(
    content: &str,
    initial_results: Vec<crate::LintResult>,
    options: &LintOptions,
) -> Result<FixedContent, String> {
    const MAX_FIX_PASSES: usize = 8;

    let mut content = content.to_string();
    let mut diagnostics = initial_results;
    let mut applied_fixes = 0;

    for _ in 0..MAX_FIX_PASSES {
        if !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.fix.is_some())
        {
            break;
        }

        let fixed = fix_with_results(&content, &diagnostics);
        if fixed.applied_fixes == 0 || fixed.content == content {
            break;
        }

        applied_fixes += fixed.applied_fixes;
        content = fixed.content;
        diagnostics = lint(&content, options).map_err(|err| err.to_string())?;
    }

    Ok(FixedContent {
        content,
        diagnostics,
        applied_fixes,
    })
}

fn run_rule(rule_id: Option<&str>, format: OutputFormat) -> Result<i32, String> {
    print!("{}", render_rule(rule_id, format)?);
    Ok(0)
}

fn render_rule(rule_id: Option<&str>, format: OutputFormat) -> Result<String, String> {
    let rules = crate::available_rules();
    if let Some(rule_id) = rule_id {
        let Some(rule) = rules.iter().find(|rule| rule.id == rule_id) else {
            return Err(format!("unknown rule: {rule_id}"));
        };
        return match format {
            OutputFormat::Text => Ok(format!(
                "{} {}\n{}\n{}\n",
                rule.id, rule.name, rule.description, rule.docs_url
            )),
            OutputFormat::Json => Ok(format!(
                "{}\n",
                serde_json::to_string_pretty(rule).map_err(|err| err.to_string())?
            )),
        };
    }

    match format {
        OutputFormat::Text => {
            let mut output = String::new();
            for rule in rules {
                output.push_str(&format!("{} {}\n", rule.id, rule.name));
            }
            Ok(output)
        }
        OutputFormat::Json => Ok(format!(
            "{}\n",
            serde_json::to_string_pretty(&rules).map_err(|err| err.to_string())?
        )),
    }
}

fn run_config(command: ConfigCommand, cli: &Cli) -> Result<i32, String> {
    print!("{}", render_config(command, cli)?);
    Ok(0)
}

fn render_config(command: ConfigCommand, cli: &Cli) -> Result<String, String> {
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

fn print_text_report(report: &CliReport, fix_mode: bool, cli: &Cli) {
    for error in &report.errors {
        if let Some(path) = &error.path {
            eprintln!("{}: {} error: {}", path, error.kind, error.message);
        } else {
            eprintln!("{} error: {}", error.kind, error.message);
        }
    }
    print!("{}", render_text_report(report, fix_mode, cli));
}

fn render_text_report(report: &CliReport, fix_mode: bool, cli: &Cli) -> String {
    let mut output = String::new();
    for file in &report.files {
        if fix_mode && file.applied_fixes > 0 && !cli.quiet {
            output.push_str(&format!(
                "{}: fixed {} issue{}",
                file.path,
                file.applied_fixes,
                plural(file.applied_fixes)
            ));
            output.push('\n');
        }

        for result in &file.diagnostics {
            if cli.quiet {
                continue;
            }
            output.push_str(&format!(
                "{}:{}:{} {} {}",
                file.path, result.line, result.column, result.rule_id, result.message
            ));
            output.push('\n');
        }
    }

    if report.files.is_empty() && report.errors.is_empty() {
        output.push_str("No Markdown files found\n");
    }
    if cli.statistics {
        output.push_str(&format!(
            "files: {}, files_with_issues: {}, issues: {}, fixable: {}, fixed: {}",
            report.summary.total_files,
            report.summary.files_with_issues,
            report.summary.total_issues,
            report.summary.fixable_issues,
            report.summary.applied_fixes
        ));
        output.push('\n');
    }
    if cli.verbose {
        output.push_str(&format!("command: {}\n", report.command));
    }
    output
}

fn print_diff(path: &Path, before: &str, after: &str) {
    if before == after {
        return;
    }
    println!("--- {}", path.display());
    println!("+++ {}", path.display());
    println!("@@");
    println!("-{}", before);
    println!("+{}", after);
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

    for rule in crate::rules::markdown::MarkdownLinterOps::user_configurable_rules() {
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
        let entry = options.rules.entry(key.clone()).or_default();
        entry.enabled = enabled;
        if let serde_json::Value::Object(properties) = value {
            entry.properties = properties
                .iter()
                .filter(|(property, _)| property.as_str() != "enabled")
                .map(|(property, value)| {
                    let value = value
                        .as_str()
                        .map(ToOwned::to_owned)
                        .unwrap_or_else(|| value.to_string());
                    (property.clone(), value)
                })
                .collect();
        }
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

fn expand_inputs(cli: &Cli) -> Result<Vec<PathBuf>, String> {
    let inputs = &cli.inputs;
    if inputs.is_empty() {
        return filter_paths(
            markdown_files_in_dir(
                &env::current_dir().map_err(|err| err.to_string())?,
                cli.respect_gitignore,
            )?,
            cli,
            false,
        );
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
            expanded.extend(markdown_files_in_dir(&path, cli.respect_gitignore)?);
        } else {
            expanded.push(path);
        }
    }
    expanded.sort();
    expanded.dedup();
    filter_paths(expanded, cli, true)
}

fn has_glob_chars(input: &str) -> bool {
    input.contains('*') || input.contains('?') || input.contains('[')
}

fn markdown_files_in_dir(dir: &Path, respect_gitignore: bool) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect_markdown_files(dir, &mut paths, respect_gitignore)?;
    paths.sort();
    Ok(paths)
}

fn collect_markdown_files(
    dir: &Path,
    paths: &mut Vec<PathBuf>,
    respect_gitignore: bool,
) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let walker = WalkBuilder::new(dir)
        .hidden(false)
        .parents(true)
        .ignore(respect_gitignore)
        .git_ignore(respect_gitignore)
        .git_global(respect_gitignore)
        .git_exclude(respect_gitignore)
        .require_git(false)
        .build_parallel();

    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |entry| {
            match entry {
                Ok(entry) => {
                    let path = entry.into_path();
                    if path.is_file() && is_markdown_file(&path) {
                        let _ = tx.send(Ok(path));
                    }
                }
                Err(err) => {
                    let _ = tx.send(Err(err.to_string()));
                }
            }
            WalkState::Continue
        })
    });
    drop(tx);

    for result in rx {
        match result {
            Ok(path) => paths.push(path),
            Err(err) => return Err(format!("{}: {err}", dir.display())),
        }
    }
    Ok(())
}

fn filter_paths(paths: Vec<PathBuf>, cli: &Cli, explicit: bool) -> Result<Vec<PathBuf>, String> {
    let includes = compile_patterns(&cli.include)?;
    let excludes = compile_patterns(&cli.exclude)?;
    Ok(paths
        .into_iter()
        .filter(|path| {
            let text = path.to_string_lossy();
            let included =
                includes.is_empty() || includes.iter().any(|pattern| pattern.matches(&text));
            let excluded = excludes.iter().any(|pattern| pattern.matches(&text));
            included && (!excluded || (explicit && !cli.force_exclude))
        })
        .collect())
}

fn compile_patterns(patterns: &[String]) -> Result<Vec<Pattern>, String> {
    patterns
        .iter()
        .map(|pattern| Pattern::new(pattern).map_err(|err| err.to_string()))
        .collect()
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
    let mut check_fix = false;
    let mut stdin = false;
    let mut include = Vec::new();
    let mut exclude = Vec::new();
    let mut respect_gitignore = true;
    let mut force_exclude = false;
    let mut statistics = false;
    let mut quiet = false;
    let mut verbose = false;
    let mut diff = false;
    let mut iter = args.into_iter().peekable();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "check" => command = Command::Check,
            "fix" => command = Command::Fix,
            "fmt" => command = Command::Fmt,
            "version" | "--version" | "-V" => command = Command::Version,
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
            "--format" => {
                if let Some(value) = iter.next() {
                    if value == "json" {
                        format = OutputFormat::Json;
                    }
                }
            }
            "--output" => {
                if let Some(value) = iter.next() {
                    if value == "json" {
                        format = OutputFormat::Json;
                    }
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
            "--force-exclude" => force_exclude = true,
            "--statistics" => statistics = true,
            "--quiet" => quiet = true,
            "--verbose" => verbose = true,
            "--diff" => diff = true,
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
        force_exclude,
        statistics,
        quiet,
        verbose,
        diff,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LintResult, Severity};
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
    fn parses_explicit_file_inputs() {
        let cli = parse_args(vec![
            "check".to_string(),
            "--file".to_string(),
            "README.md".to_string(),
            "--file".to_string(),
            "docs/guide.md".to_string(),
        ]);

        assert_eq!(cli.command, Command::Check);
        assert_eq!(
            cli.inputs,
            vec!["README.md".to_string(), "docs/guide.md".to_string()]
        );
    }

    #[test]
    fn parses_cli_parity_commands_and_options() {
        let check_fix = parse_args(vec![
            "check".to_string(),
            "--fix".to_string(),
            "--output".to_string(),
            "json".to_string(),
            "--statistics".to_string(),
            "--quiet".to_string(),
            "--verbose".to_string(),
            "--diff".to_string(),
            "--stdin".to_string(),
        ]);
        assert_eq!(check_fix.command, Command::Check);
        assert!(check_fix.check_fix);
        assert_eq!(check_fix.format, OutputFormat::Json);
        assert!(check_fix.statistics);
        assert!(check_fix.quiet);
        assert!(check_fix.verbose);
        assert!(check_fix.diff);
        assert!(check_fix.stdin);

        assert_eq!(parse_args(vec!["fmt".to_string()]).command, Command::Fmt);
        assert_eq!(
            parse_args(vec!["rule".to_string(), "MD013".to_string()]).command,
            Command::Rule(Some("MD013".to_string()))
        );
        assert_eq!(
            parse_args(vec![
                "rule".to_string(),
                "--output".to_string(),
                "json".to_string()
            ])
            .format,
            OutputFormat::Json
        );
        assert_eq!(
            parse_args(vec!["config".to_string(), "file".to_string()]).command,
            Command::Config(ConfigCommand::File)
        );
        assert_eq!(
            parse_args(vec!["version".to_string()]).command,
            Command::Version
        );
    }

    #[test]
    fn parses_include_exclude_and_ignore_options() {
        let cli = parse_args(vec![
            "check".to_string(),
            "--include".to_string(),
            "**/*.md".to_string(),
            "--exclude".to_string(),
            "**/skip.md".to_string(),
            "--no-ignore".to_string(),
            "--force-exclude".to_string(),
        ]);

        assert_eq!(cli.include, vec!["**/*.md"]);
        assert_eq!(cli.exclude, vec!["**/skip.md"]);
        assert!(!cli.respect_gitignore);
        assert!(cli.force_exclude);
    }

    #[test]
    fn renders_rule_list_and_detail_contract() {
        let list = render_rule(None, OutputFormat::Text).expect("rule list should render");
        assert!(list.contains("MD013 line-length"));

        let detail =
            render_rule(Some("MD013"), OutputFormat::Text).expect("rule detail should render");
        assert!(detail.contains("MD013 line-length"));
        assert!(detail.contains("Line length"));
        assert!(detail.contains("markdownlint"));

        let json =
            render_rule(Some("MD013"), OutputFormat::Json).expect("rule detail json should render");
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("rule detail should be json");
        assert_eq!(value["id"], "MD013");
    }

    #[test]
    fn renders_config_file_and_get_contract() {
        let dir = test_dir("config-render");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let config = dir.join(".markdownlint.json");
        fs::write(&config, "{ \"default\": false }\n").expect("config should be written");
        let cli = Cli {
            command: Command::Config(ConfigCommand::Get),
            config: Some(config.clone()),
            format: OutputFormat::Json,
            ..Cli::default()
        };

        let file =
            render_config(ConfigCommand::File, &cli).expect("config file should render as json");
        let file_json: serde_json::Value =
            serde_json::from_str(&file).expect("config file should be json");
        assert_eq!(file_json["path"], config.display().to_string());

        let get = render_config(ConfigCommand::Get, &cli).expect("config get should render");
        let get_json: serde_json::Value =
            serde_json::from_str(&get).expect("config get should be json");
        assert_eq!(get_json, serde_json::json!({ "default": false }));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn renders_text_report_contract_with_statistics_and_verbose() {
        let mut report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "<stdin>".to_string(),
                diagnostics: vec![LintResult {
                    rule_id: "MD018".to_string(),
                    rule_name: "no-missing-space-atx".to_string(),
                    message: "No space after hash on atx style heading".to_string(),
                    severity: Severity::Warning,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 6,
                    fix: None,
                }],
                applied_fixes: 0,
                changed: false,
            }],
            errors: Vec::new(),
        };
        report.recompute_summary();

        let output = render_text_report(
            &report,
            false,
            &Cli {
                statistics: true,
                verbose: true,
                ..Cli::default()
            },
        );

        assert!(output.contains("<stdin>:1:1 MD018 No space after hash"));
        assert!(output.contains("files: 1, files_with_issues: 1, issues: 1"));
        assert!(output.contains("command: check"));
    }

    #[test]
    fn quiet_text_report_suppresses_diagnostics_but_keeps_statistics() {
        let mut report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "README.md".to_string(),
                diagnostics: vec![LintResult {
                    rule_id: "MD018".to_string(),
                    rule_name: "no-missing-space-atx".to_string(),
                    message: "No space after hash on atx style heading".to_string(),
                    severity: Severity::Warning,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 6,
                    fix: None,
                }],
                applied_fixes: 0,
                changed: false,
            }],
            errors: Vec::new(),
        };
        report.recompute_summary();

        let output = render_text_report(
            &report,
            false,
            &Cli {
                quiet: true,
                statistics: true,
                ..Cli::default()
            },
        );

        assert!(!output.contains("MD018"));
        assert!(output.contains("issues: 1"));
    }

    #[test]
    fn empty_input_discovers_markdown_files_from_current_dir() {
        let dir = test_dir("default-recursive-input");
        let nested = dir.join("docs");
        fs::create_dir_all(&nested).expect("test dir should be created");
        fs::write(dir.join("README.md"), "#Title\n").expect("markdown file should be written");
        fs::write(nested.join("guide.markdown"), "#Title\n")
            .expect("markdown file should be written");
        fs::write(nested.join("ignored.txt"), "#Title\n").expect("text file should be written");

        let original_dir = env::current_dir().expect("current dir should be available");
        env::set_current_dir(&dir).expect("current dir should be changed");
        let files =
            expand_inputs(&Cli::default()).expect("empty input should expand from current dir");
        env::set_current_dir(original_dir).expect("current dir should be restored");

        assert_eq!(
            files,
            vec![
                dir.join("README.md")
                    .canonicalize()
                    .expect("path should canonicalize"),
                dir.join("docs/guide.markdown")
                    .canonicalize()
                    .expect("path should canonicalize")
            ]
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn check_returns_one_when_violations_exist() {
        let dir = test_dir("check-violation");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        fs::write(&file, "#Title\n").expect("test file should be written");

        let exit = run(Cli {
            command: Command::Check,
            inputs: vec![file.display().to_string()],
            ..Cli::default()
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
            inputs: vec![file.display().to_string()],
            ..Cli::default()
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
    fn check_fix_applies_fixes_and_returns_zero_when_resolved() {
        let dir = test_dir("check-fix");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        let config = dir.join(".markdownlint.json");
        fs::write(&file, "#Title\n").expect("test file should be written");
        fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
            .expect("config should be written");

        let exit = run(Cli {
            command: Command::Check,
            config: Some(config),
            inputs: vec![file.display().to_string()],
            check_fix: true,
            ..Cli::default()
        })
        .expect("check --fix should run");

        assert_eq!(exit, 0);
        assert_eq!(
            fs::read_to_string(&file).expect("fixed file should be readable"),
            "# Title\n"
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn fmt_applies_fixes_like_fix() {
        let dir = test_dir("fmt");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        let config = dir.join(".markdownlint.json");
        fs::write(&file, "#Title\n").expect("test file should be written");
        fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
            .expect("config should be written");

        let exit = run(Cli {
            command: Command::Fmt,
            config: Some(config),
            inputs: vec![file.display().to_string()],
            ..Cli::default()
        })
        .expect("fmt should run");

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
            ..Cli::default()
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

        let files = expand_inputs(&Cli {
            inputs: vec![dir.display().to_string()],
            ..Cli::default()
        })
        .expect("input should expand");

        assert_eq!(files, vec![dir.join("bad.md")]);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn directory_input_respects_gitignore() {
        let dir = test_dir("directory-gitignore");
        let ignored_dir = dir.join("ignored");
        fs::create_dir_all(&ignored_dir).expect("test dir should be created");
        fs::write(dir.join(".gitignore"), "ignored/\n").expect("gitignore should be written");
        fs::write(dir.join("kept.md"), "#Title\n").expect("markdown file should be written");
        fs::write(ignored_dir.join("skipped.md"), "#Title\n")
            .expect("markdown file should be written");

        let files = expand_inputs(&Cli {
            inputs: vec![dir.display().to_string()],
            ..Cli::default()
        })
        .expect("input should expand");

        assert_eq!(files, vec![dir.join("kept.md")]);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn no_ignore_discovers_gitignored_markdown_files() {
        let dir = test_dir("directory-no-ignore");
        let ignored_dir = dir.join("ignored");
        fs::create_dir_all(&ignored_dir).expect("test dir should be created");
        fs::write(dir.join(".gitignore"), "ignored/\n").expect("gitignore should be written");
        fs::write(dir.join("kept.md"), "#Title\n").expect("markdown file should be written");
        fs::write(ignored_dir.join("skipped.md"), "#Title\n")
            .expect("markdown file should be written");

        let files = expand_inputs(&Cli {
            inputs: vec![dir.display().to_string()],
            respect_gitignore: false,
            ..Cli::default()
        })
        .expect("input should expand");

        assert_eq!(files.len(), 2);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn force_exclude_applies_to_explicit_files() {
        let dir = test_dir("force-exclude");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("skip.md");
        fs::write(&file, "#Title\n").expect("markdown file should be written");

        let files = expand_inputs(&Cli {
            inputs: vec![file.display().to_string()],
            exclude: vec!["**/skip.md".to_string()],
            force_exclude: true,
            ..Cli::default()
        })
        .expect("input should expand");

        assert!(files.is_empty());
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
            ..Cli::default()
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
