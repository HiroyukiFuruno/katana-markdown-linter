use crate::i18n::{Locale, LocalizedDiagnostic, MessageParams};
use crate::{
    fix_with_results, fix_with_results_including_unsafe, lint, FixSafety, LintOptions,
    MarkdownLintConfig,
};
use glob::{glob, Pattern};
use ignore::{WalkBuilder, WalkState};
use serde::Serialize;
use std::env;
use std::fs;
use std::io::{self, BufRead, IsTerminal, Read, Write};
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
    pub locale: Option<String>,
    pub unsafe_fixes: bool,
    pub yes: bool,
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
            locale: None,
            unsafe_fixes: false,
            yes: false,
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
            let exit = run_check_like(cli.check_fix, &cli, locale)?;
            Ok(exit)
        }
        Command::Fix | Command::Fmt => {
            let exit = run_check_like(true, &cli, locale)?;
            Ok(exit)
        }
        Command::Rule(rule_id) => run_rule(rule_id.as_deref(), cli.format, locale),
        Command::Config(ref command) => run_config(command.clone(), &cli),
        Command::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(0)
        }
    }
}

fn run_check_like(fix_mode: bool, cli: &Cli, locale: Locale) -> Result<i32, String> {
    let command = if fix_mode { "fix" } else { "check" };
    let mut report = CliReport {
        command,
        files: Vec::new(),
        errors: Vec::new(),
        summary: CliSummary::default(),
    };
    if cli.stdin {
        return run_stdin_check_like(fix_mode, cli, locale);
    }

    let files = match expand_inputs(cli) {
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
        let config_errors = config.validate_cached_rules();
        if !config_errors.is_empty() {
            for error in config_errors {
                report
                    .errors
                    .push(CliError::config_validation(&path, error));
            }
            continue;
        }

        let options = config.to_lint_options();
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
            } = apply_fixes_until_stable(
                &content,
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

    output_report(&report, fix_mode, cli, locale)?;

    Ok(exit_code)
}

fn run_stdin_check_like(fix_mode: bool, cli: &Cli, locale: Locale) -> Result<i32, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    let config = match load_effective_config(Path::new("<stdin>"), cli.config.as_deref()) {
        Ok(config) => config,
        Err(err) => {
            let mut report = CliReport {
                command: "check",
                summary: CliSummary::default(),
                files: Vec::new(),
                errors: vec![CliError::config(Path::new("<stdin>"), err)],
            };
            report.recompute_summary();
            output_report(&report, fix_mode, cli, locale)?;
            return Ok(2);
        }
    };
    let options = config.to_lint_options();
    if fix_mode {
        if cli.unsafe_fixes && !cli.yes {
            return Err(
                "unsafe fixes with --stdin require --yes because stdin is used for content"
                    .to_string(),
            );
        }
        let results = lint(&content, &options).map_err(|err| err.to_string())?;
        let fixed = apply_fixes_until_stable(&content, results, &options, cli.unsafe_fixes)?;
        if cli.diff {
            print_diff(Path::new("<stdin>"), &content, &fixed.content);
        } else {
            print!("{}", fixed.content);
        }
        return Ok(0);
    }

    let diagnostics = match lint(&content, &options) {
        Ok(diagnostics) => diagnostics,
        Err(err) => {
            let mut report = CliReport {
                command: "check",
                summary: CliSummary::default(),
                files: Vec::new(),
                errors: vec![CliError::rule(Path::new("<stdin>"), err.to_string())],
            };
            report.recompute_summary();
            output_report(&report, false, cli, locale)?;
            return Ok(2);
        }
    };
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
    output_report(&report, false, cli, locale)?;
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct UnsafeFixPolicy {
    include_unsafe: bool,
    declined: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnsafeFixCandidateSummary {
    path: String,
    rule_id: String,
    count: usize,
}

fn resolve_unsafe_fix_policy(
    cli: &Cli,
    files: &[PathBuf],
    locale: Locale,
) -> Result<UnsafeFixPolicy, String> {
    if !cli.unsafe_fixes {
        return Ok(UnsafeFixPolicy::default());
    }
    let candidates = collect_unsafe_fix_candidates(files, cli)?;
    if candidates.is_empty() {
        // Do not broaden later fix passes unless the initial diagnostic set exposed an unsafe
        // candidate that the user could review before any write.
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
    let approved = prompt_unsafe_confirmation(&mut stdin, &mut stderr)?;
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
        let diagnostics = lint(&content, &options).map_err(|err| err.to_string())?;
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

fn prompt_unsafe_confirmation(
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

fn apply_fixes_until_stable(
    content: &str,
    initial_results: Vec<crate::LintResult>,
    options: &LintOptions,
    include_unsafe: bool,
) -> Result<FixedContent, String> {
    const MAX_FIX_PASSES: usize = 8;

    let mut content = content.to_string();
    let mut diagnostics = initial_results;
    let mut applied_fixes = 0;

    for _ in 0..MAX_FIX_PASSES {
        if !diagnostics
            .iter()
            .any(|diagnostic| is_applicable_fix(diagnostic, include_unsafe))
        {
            break;
        }

        let fixed = if include_unsafe {
            fix_with_results_including_unsafe(&content, &diagnostics)
        } else {
            fix_with_results(&content, &diagnostics)
        };
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

fn is_applicable_fix(diagnostic: &crate::LintResult, include_unsafe: bool) -> bool {
    diagnostic
        .fix
        .as_ref()
        .is_some_and(|fix| include_unsafe || fix.safety == FixSafety::Safe)
}

fn run_rule(rule_id: Option<&str>, format: OutputFormat, locale: Locale) -> Result<i32, String> {
    print!("{}", render_rule(rule_id, format, locale)?);
    Ok(0)
}

fn render_rule(
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

#[derive(Debug, Clone, Serialize)]
struct LocalizedRuleMeta {
    id: String,
    name: String,
    description: String,
    english_description: String,
    docs_url: String,
    fixable: bool,
    locale: &'static str,
}

impl LocalizedRuleMeta {
    fn from_rule(rule: &crate::RuleMeta, locale: Locale) -> Self {
        Self {
            id: rule.id.clone(),
            name: rule.name.clone(),
            description: rule.localized_description(locale.code()),
            english_description: rule.description.clone(),
            docs_url: rule.docs_url.clone(),
            fixable: rule.fixable,
            locale: locale.code(),
        }
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

fn print_text_report(report: &CliReport, fix_mode: bool, cli: &Cli, locale: Locale) {
    for error in &report.errors {
        if let Some(path) = &error.path {
            eprintln!(
                "{}: {} error: {}",
                path,
                error.kind,
                error.localized_message(locale)
            );
        } else {
            eprintln!("{} error: {}", error.kind, error.localized_message(locale));
        }
    }
    print!("{}", render_text_report(report, fix_mode, cli, locale));
}

fn output_report(
    report: &CliReport,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> Result<(), String> {
    match cli.format {
        OutputFormat::Text => print_text_report(report, fix_mode, cli, locale),
        OutputFormat::Json => {
            let report = LocalizedCliReport::from_report(report, locale);
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
            );
        }
    }
    Ok(())
}

fn render_text_report(report: &CliReport, fix_mode: bool, cli: &Cli, locale: Locale) -> String {
    let mut output = String::new();
    for file in &report.files {
        if fix_mode && file.applied_fixes > 0 && !cli.quiet {
            let mut params = MessageParams::new();
            params.insert("path".to_string(), file.path.clone());
            params.insert("count".to_string(), file.applied_fixes.to_string());
            let fallback = format!(
                "{}: fixed {} issue{}",
                file.path,
                file.applied_fixes,
                plural(file.applied_fixes)
            );
            output.push_str(&crate::i18n::render_message(
                locale,
                "fix.fixed_count",
                &params,
                &fallback,
            ));
            output.push('\n');
        }

        for result in &file.diagnostics {
            if cli.quiet {
                continue;
            }
            let message = crate::i18n::render_message(
                locale,
                result.message_id.as_str(),
                &result.message_params,
                result.message.as_str(),
            );
            output.push_str(&format!(
                "{}:{}:{} {} {}",
                file.path, result.line, result.column, result.rule_id, message
            ));
            output.push('\n');
        }
    }

    if report.files.is_empty() && report.errors.is_empty() {
        output.push_str(&crate::i18n::render_message(
            locale,
            "summary.no_files",
            &MessageParams::new(),
            "No Markdown files found",
        ));
        output.push('\n');
    }
    if cli.statistics {
        let mut params = MessageParams::new();
        params.insert("files".to_string(), report.summary.total_files.to_string());
        params.insert(
            "files_with_issues".to_string(),
            report.summary.files_with_issues.to_string(),
        );
        params.insert(
            "issues".to_string(),
            report.summary.total_issues.to_string(),
        );
        params.insert(
            "fixable".to_string(),
            report.summary.fixable_issues.to_string(),
        );
        params.insert(
            "safe_fixable".to_string(),
            report.summary.safe_fixable_issues.to_string(),
        );
        params.insert(
            "unsafe_fixable".to_string(),
            report.summary.unsafe_fixable_issues.to_string(),
        );
        params.insert(
            "fixed".to_string(),
            report.summary.applied_fixes.to_string(),
        );
        let fallback = format!(
            "files: {}, files_with_issues: {}, issues: {}, fixable: {}, safe_fixable: {}, unsafe_fixable: {}, fixed: {}",
            report.summary.total_files,
            report.summary.files_with_issues,
            report.summary.total_issues,
            report.summary.fixable_issues,
            report.summary.safe_fixable_issues,
            report.summary.unsafe_fixable_issues,
            report.summary.applied_fixes
        );
        output.push_str(&crate::i18n::render_message(
            locale,
            "summary.statistics",
            &params,
            &fallback,
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
            summary.safe_fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic
                        .fix
                        .as_ref()
                        .is_some_and(|fix| fix.safety == FixSafety::Safe)
                })
                .count();
            summary.unsafe_fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic
                        .fix
                        .as_ref()
                        .is_some_and(|fix| fix.safety == FixSafety::Unsafe)
                })
                .count();
            summary.applied_fixes += file.applied_fixes;
        }

        if summary.unsafe_fixable_issues > 0 {
            summary
                .unsafe_fix_status
                .get_or_insert_with(|| "unsafe_mode_not_enabled".to_string());
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
    safe_fixable_issues: usize,
    unsafe_fixable_issues: usize,
    applied_fixes: usize,
    unsafe_fix_status: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct FileReport {
    path: String,
    diagnostics: Vec<crate::LintResult>,
    applied_fixes: usize,
    changed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct LocalizedCliReport {
    command: &'static str,
    summary: CliSummary,
    files: Vec<LocalizedFileReport>,
    errors: Vec<LocalizedCliError>,
}

impl LocalizedCliReport {
    fn from_report(report: &CliReport, locale: Locale) -> Self {
        Self {
            command: report.command,
            summary: report.summary.clone(),
            files: report
                .files
                .iter()
                .map(|file| LocalizedFileReport {
                    path: file.path.clone(),
                    diagnostics: file
                        .diagnostics
                        .iter()
                        .map(|diagnostic| LocalizedDiagnostic::from_result(diagnostic, locale))
                        .collect(),
                    applied_fixes: file.applied_fixes,
                    changed: file.changed,
                })
                .collect(),
            errors: report
                .errors
                .iter()
                .map(|error| LocalizedCliError::from_error(error, locale))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct LocalizedFileReport {
    path: String,
    diagnostics: Vec<LocalizedDiagnostic>,
    applied_fixes: usize,
    changed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct CliError {
    kind: &'static str,
    path: Option<String>,
    message: String,
    message_id: String,
    message_params: MessageParams,
}

impl CliError {
    fn filesystem(path: &Path, err: impl std::fmt::Display) -> Self {
        let message = err.to_string();
        Self {
            kind: "filesystem",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "filesystem.error".to_string(),
        }
    }

    fn filesystem_message(message: String) -> Self {
        Self {
            kind: "filesystem",
            path: None,
            message_params: message_params(&message),
            message,
            message_id: "filesystem.error".to_string(),
        }
    }

    fn glob(message: String) -> Self {
        Self {
            kind: "glob",
            path: None,
            message_params: message_params(&message),
            message,
            message_id: "glob.error".to_string(),
        }
    }

    fn config(path: &Path, message: String) -> Self {
        Self {
            kind: "config",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "config.error".to_string(),
        }
    }

    fn config_validation(path: &Path, error: crate::ConfigError) -> Self {
        Self {
            kind: "config",
            path: Some(path.display().to_string()),
            message_params: error.message_params(),
            message: error.to_string(),
            message_id: error.message_id().to_string(),
        }
    }

    fn rule(path: &Path, message: String) -> Self {
        Self {
            kind: "rule",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "rule.error".to_string(),
        }
    }

    fn from_input_expand_error(error: InputExpandError) -> Self {
        match error {
            InputExpandError::Filesystem(message) => Self::filesystem_message(message),
            InputExpandError::Glob(message) => Self::glob(message),
        }
    }

    fn localized_message(&self, locale: Locale) -> String {
        crate::i18n::render_message(
            locale,
            self.message_id.as_str(),
            &self.message_params,
            self.message.as_str(),
        )
    }
}

#[derive(Debug, Clone, Serialize)]
struct LocalizedCliError {
    kind: &'static str,
    path: Option<String>,
    message: String,
    message_id: String,
    message_params: MessageParams,
}

impl LocalizedCliError {
    fn from_error(error: &CliError, locale: Locale) -> Self {
        Self {
            kind: error.kind,
            path: error.path.clone(),
            message: error.localized_message(locale),
            message_id: error.message_id.clone(),
            message_params: error.message_params.clone(),
        }
    }
}

fn message_params(message: &str) -> MessageParams {
    let mut params = MessageParams::new();
    params.insert("message".to_string(), message.to_string());
    params
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum InputExpandError {
    Filesystem(String),
    Glob(String),
}

impl From<String> for InputExpandError {
    fn from(value: String) -> Self {
        Self::Filesystem(value)
    }
}

fn expand_inputs(cli: &Cli) -> Result<Vec<PathBuf>, InputExpandError> {
    let inputs = &cli.inputs;
    if inputs.is_empty() {
        return filter_paths(
            markdown_files_in_dir(
                &env::current_dir().map_err(|err| InputExpandError::Filesystem(err.to_string()))?,
                cli.respect_gitignore,
            )
            .map_err(InputExpandError::Filesystem)?,
            cli,
            false,
        );
    }

    let mut paths = Vec::new();
    for input in inputs {
        if has_glob_chars(input) {
            for entry in glob(input).map_err(|err| InputExpandError::Glob(err.to_string()))? {
                match entry {
                    Ok(path) => paths.push(path),
                    Err(err) => return Err(InputExpandError::Glob(err.to_string())),
                }
            }
        } else {
            paths.push(PathBuf::from(input));
        }
    }

    let mut expanded = Vec::new();
    for path in paths {
        if path.is_dir() {
            expanded.extend(
                markdown_files_in_dir(&path, cli.respect_gitignore)
                    .map_err(InputExpandError::Filesystem)?,
            );
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

fn filter_paths(
    paths: Vec<PathBuf>,
    cli: &Cli,
    explicit: bool,
) -> Result<Vec<PathBuf>, InputExpandError> {
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

fn compile_patterns(patterns: &[String]) -> Result<Vec<Pattern>, InputExpandError> {
    patterns
        .iter()
        .map(|pattern| Pattern::new(pattern).map_err(|err| InputExpandError::Glob(err.to_string())))
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
    let mut locale = None;
    let mut unsafe_fixes = false;
    let mut yes = false;
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
            "--force-exclude" => force_exclude = true,
            "--statistics" => statistics = true,
            "--quiet" => quiet = true,
            "--verbose" => verbose = true,
            "--diff" => diff = true,
            "--unsafe" => unsafe_fixes = true,
            "--yes" | "-y" => yes = true,
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
        locale,
        unsafe_fixes,
        yes,
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

        let unsafe_fix = parse_args(vec![
            "fix".to_string(),
            "--unsafe".to_string(),
            "--yes".to_string(),
        ]);
        assert_eq!(unsafe_fix.command, Command::Fix);
        assert!(unsafe_fix.unsafe_fixes);
        assert!(unsafe_fix.yes);

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
    fn parses_locale_options() {
        let long_ja = parse_args(vec![
            "check".to_string(),
            "--locale".to_string(),
            "ja".to_string(),
        ]);
        assert_eq!(long_ja.locale.as_deref(), Some("ja"));

        let long_en = parse_args(vec![
            "check".to_string(),
            "--locale".to_string(),
            "en".to_string(),
        ]);
        assert_eq!(long_en.locale.as_deref(), Some("en"));

        let typo_alias = parse_args(vec![
            "check".to_string(),
            "--local".to_string(),
            "ja".to_string(),
        ]);
        assert_eq!(typo_alias.locale.as_deref(), Some("ja"));

        let short_en = parse_args(vec![
            "check".to_string(),
            "-l".to_string(),
            "en".to_string(),
        ]);
        assert_eq!(short_en.locale.as_deref(), Some("en"));

        let short_ja = parse_args(vec![
            "check".to_string(),
            "-l".to_string(),
            "ja".to_string(),
        ]);
        assert_eq!(short_ja.locale.as_deref(), Some("ja"));
    }

    #[test]
    fn renders_rule_list_and_detail_contract() {
        let list =
            render_rule(None, OutputFormat::Text, Locale::En).expect("rule list should render");
        assert!(list.contains("MD013 line-length"));
        assert!(list.contains("Line length"));

        let detail = render_rule(Some("MD013"), OutputFormat::Text, Locale::En)
            .expect("rule detail should render");
        assert!(detail.contains("MD013 line-length"));
        assert!(detail.contains("Line length"));
        assert!(detail.contains("markdownlint"));

        let json = render_rule(Some("MD013"), OutputFormat::Json, Locale::En)
            .expect("rule detail json should render");
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("rule detail should be json");
        assert_eq!(value["id"], "MD013");
        assert_eq!(value["locale"], "en");
        assert_eq!(value["description"], value["english_description"]);
    }

    #[test]
    fn rule_output_uses_selected_japanese_locale() {
        let list =
            render_rule(None, OutputFormat::Text, Locale::Ja).expect("rule list should render");
        assert!(list.contains("MD003 heading-style - 見出しのスタイルを統一してください"));

        let json = render_rule(Some("MD003"), OutputFormat::Json, Locale::Ja)
            .expect("rule detail json should render");
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("rule detail should be json");
        assert_eq!(value["locale"], "ja");
        assert_eq!(value["description"], "見出しのスタイルを統一してください");
        assert_eq!(value["english_description"], "Heading style");
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
                    message_id: "rule.generic".to_string(),
                    message_params: crate::i18n::diagnostic_message_params(
                        "MD018",
                        "no-missing-space-atx",
                        "No space after hash on atx style heading",
                    ),
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
            Locale::En,
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
                    message_id: "rule.generic".to_string(),
                    message_params: crate::i18n::diagnostic_message_params(
                        "MD018",
                        "no-missing-space-atx",
                        "No space after hash on atx style heading",
                    ),
                    severity: Severity::Warning,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 6,
                    fix: Some(crate::Fix {
                        range: crate::Range {
                            start_line: 1,
                            start_column: 1,
                            end_line: 1,
                            end_column: 6,
                        },
                        replacement: "# Heading".to_string(),
                        safety: crate::FixSafety::Unsafe,
                    }),
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
            Locale::En,
        );

        assert!(!output.contains("MD018"));
        assert!(output.contains("issues: 1"));
    }

    #[test]
    fn text_report_uses_selected_japanese_locale() {
        let mut report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "README.md".to_string(),
                diagnostics: vec![LintResult {
                    rule_id: "MD018".to_string(),
                    rule_name: "no-missing-space-atx".to_string(),
                    message: "No space after hash on atx style heading".to_string(),
                    message_id: "rule.generic".to_string(),
                    message_params: crate::i18n::diagnostic_message_params(
                        "MD018",
                        "no-missing-space-atx",
                        "No space after hash on atx style heading",
                    ),
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

        let output = render_text_report(&report, false, &Cli::default(), Locale::Ja);

        assert!(output.contains("README.md:1:1 MD018 ATX 見出し"));
    }

    #[test]
    fn json_report_keeps_shape_and_adds_localized_message_metadata() {
        let mut report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: vec![FileReport {
                path: "README.md".to_string(),
                diagnostics: vec![LintResult {
                    rule_id: "MD018".to_string(),
                    rule_name: "no-missing-space-atx".to_string(),
                    message: "No space after hash on atx style heading".to_string(),
                    message_id: "rule.generic".to_string(),
                    message_params: crate::i18n::diagnostic_message_params(
                        "MD018",
                        "no-missing-space-atx",
                        "No space after hash on atx style heading",
                    ),
                    severity: Severity::Warning,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 6,
                    fix: Some(crate::Fix {
                        range: crate::Range {
                            start_line: 1,
                            start_column: 1,
                            end_line: 1,
                            end_column: 6,
                        },
                        replacement: "# Heading".to_string(),
                        safety: crate::FixSafety::Unsafe,
                    }),
                }],
                applied_fixes: 0,
                changed: false,
            }],
            errors: Vec::new(),
        };
        report.recompute_summary();

        let json = serde_json::to_value(LocalizedCliReport::from_report(&report, Locale::Ja))
            .expect("localized report should serialize");

        assert_eq!(
            json["files"][0]["diagnostics"][0]["message_id"],
            "rule.generic"
        );
        assert_eq!(
            json["files"][0]["diagnostics"][0]["message_params"]["rule_id"],
            "MD018"
        );
        assert!(json["files"][0]["diagnostics"][0]["message"]
            .as_str()
            .expect("message should be string")
            .contains("ATX 見出し"));
        assert_eq!(
            json["files"][0]["diagnostics"][0]["fix"]["safety"],
            "unsafe"
        );
        assert_eq!(json["summary"]["unsafe_fixable_issues"], 1);
        assert_eq!(
            json["summary"]["unsafe_fix_status"],
            "unsafe_mode_not_enabled"
        );
    }

    #[test]
    fn json_report_localizes_config_errors_with_stable_metadata() {
        let report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: vec![CliError::config(
                Path::new("<stdin>"),
                "config file not found: missing.json".to_string(),
            )],
        };

        let json = serde_json::to_value(LocalizedCliReport::from_report(&report, Locale::Ja))
            .expect("localized report should serialize");

        assert_eq!(json["errors"][0]["message_id"], "config.error");
        assert!(json["errors"][0]["message"]
            .as_str()
            .expect("message should be string")
            .contains("設定エラー"));
    }

    #[test]
    fn json_report_localizes_structured_config_validation_errors() {
        let config = MarkdownLintConfig {
            raw: serde_json::json!({ "MD999": true }),
        };
        let error = config
            .validate_cached_rules()
            .into_iter()
            .next()
            .expect("config should be invalid");
        let report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: vec![CliError::config_validation(Path::new("README.md"), error)],
        };

        let json = serde_json::to_value(LocalizedCliReport::from_report(&report, Locale::Ja))
            .expect("localized report should serialize");

        assert_eq!(json["errors"][0]["message_id"], "config.unknown_rule");
        assert_eq!(json["errors"][0]["message_params"]["rule_id"], "MD999");
        assert_eq!(
            json["errors"][0]["message"],
            "未知の markdownlint rule です: MD999"
        );
    }

    #[test]
    fn json_report_localizes_glob_errors_with_stable_metadata() {
        let report = CliReport {
            command: "check",
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: vec![CliError::glob("invalid glob pattern".to_string())],
        };

        let json = serde_json::to_value(LocalizedCliReport::from_report(&report, Locale::Ja))
            .expect("localized report should serialize");

        assert_eq!(json["errors"][0]["kind"], "glob");
        assert_eq!(json["errors"][0]["message_id"], "glob.error");
        assert!(json["errors"][0]["message"]
            .as_str()
            .expect("message should be string")
            .contains("glob エラー"));
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
    fn default_fix_does_not_apply_unsafe_candidates() {
        let dir = test_dir("unsafe-default");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        let config = dir.join(".markdownlint.json");
        fs::write(&file, "**Important**\n\nText\n").expect("test file should be written");
        fs::write(&config, "{ \"default\": false, \"MD036\": true }\n")
            .expect("config should be written");

        let exit = run(Cli {
            command: Command::Fix,
            config: Some(config),
            inputs: vec![file.display().to_string()],
            ..Cli::default()
        })
        .expect("fix should run");

        assert_eq!(exit, 1);
        assert_eq!(
            fs::read_to_string(&file).expect("file should be readable"),
            "**Important**\n\nText\n"
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn unsafe_fix_requires_yes_in_non_interactive_mode() {
        let dir = test_dir("unsafe-needs-yes");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        let config = dir.join(".markdownlint.json");
        fs::write(&file, "**Important**\n\nText\n").expect("test file should be written");
        fs::write(&config, "{ \"default\": false, \"MD036\": true }\n")
            .expect("config should be written");

        let result = run(Cli {
            command: Command::Fix,
            config: Some(config),
            inputs: vec![file.display().to_string()],
            unsafe_fixes: true,
            ..Cli::default()
        });

        assert!(result
            .expect_err("unsafe non-interactive run should fail")
            .contains("--yes"));
        assert_eq!(
            fs::read_to_string(&file).expect("file should be readable"),
            "**Important**\n\nText\n"
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn unsafe_yes_applies_unsafe_fix() {
        let dir = test_dir("unsafe-yes");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        let config = dir.join(".markdownlint.json");
        fs::write(&file, "**Important**\n\nText\n").expect("test file should be written");
        fs::write(&config, "{ \"default\": false, \"MD036\": true }\n")
            .expect("config should be written");

        let exit = run(Cli {
            command: Command::Fix,
            config: Some(config),
            inputs: vec![file.display().to_string()],
            unsafe_fixes: true,
            yes: true,
            ..Cli::default()
        })
        .expect("unsafe fix should run with --yes");

        assert_eq!(exit, 0);
        assert_eq!(
            fs::read_to_string(&file).expect("file should be readable"),
            "# Important\n\nText\n"
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn unsafe_confirmation_accepts_only_explicit_yes() {
        let mut yes = std::io::Cursor::new(b"Y".to_vec());
        let mut output = Vec::new();
        assert!(prompt_unsafe_confirmation(&mut yes, &mut output).expect("prompt should run"));

        let mut default_yes = std::io::Cursor::new(b"\n".to_vec());
        let mut output = Vec::new();
        assert!(
            prompt_unsafe_confirmation(&mut default_yes, &mut output).expect("prompt should run")
        );

        let mut no = std::io::Cursor::new(b"n".to_vec());
        let mut output = Vec::new();
        assert!(!prompt_unsafe_confirmation(&mut no, &mut output).expect("prompt should run"));

        let mut eof = std::io::Cursor::new(Vec::<u8>::new());
        let mut output = Vec::new();
        assert!(!prompt_unsafe_confirmation(&mut eof, &mut output).expect("prompt should run"));
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
