use super::args::{Cli, Command, ConfigCommand, OutputFormat};
use super::input::expand_inputs;
use super::reporter::{
    output_report, plural, print_diff, CliError, CliReport, CliSummary, FileReport,
    LocalizedRuleMeta,
};
use crate::i18n::Locale;
use crate::{
    fix_with_results, fix_with_results_including_unsafe, format_markdown, lint, lint_for_path,
    FixSafety, FormatOptions, LintOptions, MarkdownLintConfig,
};
use std::fs;
use std::io::{self, BufRead, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};

pub fn run_from_env() -> i32 {
    match run(super::args::parse_args(std::env::args().skip(1).collect())) {
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
            let exit = run_check_like("check", cli.check_fix, &cli, locale)?;
            Ok(exit)
        }
        Command::Fix => {
            let exit = run_check_like("fix", true, &cli, locale)?;
            Ok(exit)
        }
        Command::Fmt => run_fmt(&cli, locale),
        Command::Rule(rule_id) => run_rule(rule_id.as_deref(), cli.format, locale),
        Command::Config(ref command) => run_config(command.clone(), &cli),
        Command::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(0)
        }
    }
}

fn run_fmt(cli: &Cli, locale: Locale) -> Result<i32, String> {
    let mut report = CliReport {
        command: "fmt",
        files: Vec::new(),
        errors: Vec::new(),
        summary: CliSummary::default(),
    };
    if cli.stdin {
        return run_stdin_fmt(cli, locale);
    }

    let files = match expand_inputs(cli) {
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

        match validate_effective_config(&path, cli.config.as_deref()) {
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

        let formatted =
            format_markdown(&content, &FormatOptions::default()).map_err(|err| err.to_string())?;
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
            changed,
        });
    }

    report.recompute_summary();
    let exit_code = if report.errors.is_empty() { 0 } else { 2 };
    output_report(&report, true, cli, locale)?;
    Ok(exit_code)
}

fn run_check_like(
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

fn run_stdin_fmt(cli: &Cli, locale: Locale) -> Result<i32, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    match validate_effective_config(Path::new("<stdin>"), cli.config.as_deref()) {
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

fn format_stdin_content(content: &str) -> Result<String, String> {
    Ok(format_markdown(content, &FormatOptions::default())
        .map_err(|err| err.to_string())?
        .content)
}

fn run_stdin_check_like(
    command: &'static str,
    fix_mode: bool,
    cli: &Cli,
    locale: Locale,
) -> Result<i32, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|err| err.to_string())?;
    let config = match load_effective_config(Path::new("<stdin>"), cli.config.as_deref()) {
        Ok(config) => config,
        Err(err) => {
            let mut report = CliReport {
                command,
                summary: CliSummary::default(),
                files: Vec::new(),
                errors: vec![CliError::config(Path::new("<stdin>"), err)],
            };
            report.recompute_summary();
            output_report(&report, fix_mode, cli, locale)?;
            return Ok(2);
        }
    };
    let config_errors = config.validate_cached_rules();
    if !config_errors.is_empty() {
        let mut report = CliReport {
            command,
            summary: CliSummary::default(),
            files: Vec::new(),
            errors: config_errors
                .into_iter()
                .map(|error| CliError::config_validation(Path::new("<stdin>"), error))
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
            Path::new("<stdin>"),
            results,
            &options,
            cli.unsafe_fixes,
        )?;
        if matches!(cli.format, OutputFormat::Json) {
            let mut report = CliReport {
                command,
                summary: CliSummary::default(),
                files: vec![FileReport {
                    path: "<stdin>".to_string(),
                    diagnostics: fixed.diagnostics,
                    applied_fixes: fixed.applied_fixes,
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
            print_diff(Path::new("<stdin>"), &content, &fixed.content);
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
                errors: vec![CliError::rule(Path::new("<stdin>"), err.to_string())],
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
    file_path: &Path,
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
        diagnostics = lint_for_path(file_path, &content, options).map_err(|err| err.to_string())?;
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

fn validate_effective_config(
    path: &Path,
    explicit: Option<&Path>,
) -> Result<Vec<crate::ConfigError>, String> {
    Ok(load_effective_config(path, explicit)?.validate_cached_rules())
}

#[cfg(test)]
mod tests {
    use super::{
        format_stdin_content, prompt_unsafe_confirmation, render_config, render_rule, run,
    };
    use crate::cli::args::{Cli, Command, ConfigCommand, OutputFormat};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "katana-markdown-linter-cli-{name}-{}-{nanos}",
            std::process::id()
        ))
    }

    #[test]
    fn renders_rule_list_and_detail_contract() {
        use crate::i18n::Locale;
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
        use crate::i18n::Locale;
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
    fn fmt_applies_layout_formatting_and_is_idempotent() {
        let dir = test_dir("fmt-layout");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        fs::write(&file, "# Title\r\nText\n\n\n-  item").expect("test file should be written");

        let exit = run(Cli {
            command: Command::Fmt,
            inputs: vec![file.display().to_string()],
            ..Cli::default()
        })
        .expect("fmt should run");

        assert_eq!(exit, 0);
        assert_eq!(
            fs::read_to_string(&file).expect("fixed file should be readable"),
            "# Title\n\nText\n\n- item\n"
        );

        let second_exit = run(Cli {
            command: Command::Fmt,
            inputs: vec![file.display().to_string()],
            ..Cli::default()
        })
        .expect("fmt should run again");

        assert_eq!(second_exit, 0);
        assert_eq!(
            fs::read_to_string(&file).expect("fixed file should be readable"),
            "# Title\n\nText\n\n- item\n"
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn fmt_does_not_apply_non_layout_safe_fixes_or_return_one_for_lint_issues() {
        let dir = test_dir("fmt-non-layout");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("bad.md");
        fs::write(&file, "#Title\n").expect("test file should be written");

        let exit = run(Cli {
            command: Command::Fmt,
            inputs: vec![file.display().to_string()],
            ..Cli::default()
        })
        .expect("fmt should run");

        assert_eq!(exit, 0);
        assert_eq!(
            fs::read_to_string(&file).expect("fixed file should be readable"),
            "#Title\n"
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn fmt_stdin_contract_formats_markdown_only() {
        let formatted =
            format_stdin_content("# Title\r\nText\n\n\n").expect("stdin format should run");

        assert_eq!(formatted, "# Title\n\nText\n");
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
}
