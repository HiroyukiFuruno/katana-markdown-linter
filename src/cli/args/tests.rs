use super::*;

#[test]
fn parses_check_with_json_format_and_config() {
    let cli = CliArgsParser::parse_args(vec![
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
    assert_eq!(
        cli.config,
        Some(std::path::PathBuf::from(".markdownlint.jsonc"))
    );
}

#[test]
fn parses_explicit_file_inputs() {
    let cli = CliArgsParser::parse_args(vec![
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
    let check_fix = CliArgsParser::parse_args(vec![
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

    let unsafe_fix = CliArgsParser::parse_args(vec![
        "fix".to_string(),
        "--unsafe".to_string(),
        "--yes".to_string(),
    ]);
    assert_eq!(unsafe_fix.command, Command::Fix);
    assert!(unsafe_fix.unsafe_fixes);
    assert!(unsafe_fix.yes);

    assert_eq!(
        CliArgsParser::parse_args(vec!["fmt".to_string()]).command,
        Command::Fmt
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["lsp".to_string()]).command,
        Command::Lsp
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["help".to_string()]).command,
        Command::Help(None)
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["--help".to_string()]).command,
        Command::Help(None)
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["-h".to_string()]).command,
        Command::Help(None)
    );
    let localized_global_help = CliArgsParser::parse_args(vec![
        "--locale".to_string(),
        "ja".to_string(),
        "help".to_string(),
    ]);
    assert_eq!(localized_global_help.command, Command::Help(None));
    assert_eq!(localized_global_help.locale.as_deref(), Some("ja"));

    let localized_command_help = CliArgsParser::parse_args(vec![
        "check".to_string(),
        "--help".to_string(),
        "--locale".to_string(),
        "ja".to_string(),
    ]);
    assert_eq!(
        localized_command_help.command,
        Command::Help(Some(HelpTopic::Check))
    );
    assert_eq!(localized_command_help.locale.as_deref(), Some("ja"));
    assert_eq!(
        CliArgsParser::parse_args(vec!["check".to_string(), "--help".to_string()]).command,
        Command::Help(Some(HelpTopic::Check))
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["help".to_string(), "fmt".to_string()]).command,
        Command::Help(Some(HelpTopic::Fmt))
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["rule".to_string(), "MD013".to_string()]).command,
        Command::Rule(Some("MD013".to_string()))
    );
    assert_eq!(
        CliArgsParser::parse_args(vec![
            "rule".to_string(),
            "--output".to_string(),
            "json".to_string()
        ])
        .format,
        OutputFormat::Json
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["config".to_string(), "file".to_string()]).command,
        Command::Config(ConfigCommand::File)
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["config".to_string(), "schema".to_string()]).command,
        Command::Config(ConfigCommand::Schema)
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["version".to_string()]).command,
        Command::Version
    );
    assert_eq!(
        CliArgsParser::parse_args(vec!["-v".to_string()]).command,
        Command::Version
    );
}

#[test]
fn parses_include_exclude_and_ignore_options() {
    let cli = CliArgsParser::parse_args(vec![
        "check".to_string(),
        "--include".to_string(),
        "**/*.md".to_string(),
        "--exclude".to_string(),
        "**/skip.md".to_string(),
        "--no-ignore".to_string(),
        "--include-ignored".to_string(),
        "--include-reserved".to_string(),
        "--force-exclude".to_string(),
    ]);

    assert_eq!(cli.include, vec!["**/*.md"]);
    assert_eq!(cli.exclude, vec!["**/skip.md"]);
    assert!(!cli.respect_gitignore);
    assert!(cli.include_ignored);
    assert!(cli.include_reserved);
    assert!(cli.force_exclude);
}

#[test]
fn parses_locale_options() {
    let long_ja = CliArgsParser::parse_args(vec![
        "check".to_string(),
        "--locale".to_string(),
        "ja".to_string(),
    ]);
    assert_eq!(long_ja.locale.as_deref(), Some("ja"));

    let long_en = CliArgsParser::parse_args(vec![
        "check".to_string(),
        "--locale".to_string(),
        "en".to_string(),
    ]);
    assert_eq!(long_en.locale.as_deref(), Some("en"));

    let typo_alias = CliArgsParser::parse_args(vec![
        "check".to_string(),
        "--local".to_string(),
        "ja".to_string(),
    ]);
    assert_eq!(typo_alias.locale.as_deref(), Some("ja"));

    let short_en = CliArgsParser::parse_args(vec![
        "check".to_string(),
        "-l".to_string(),
        "en".to_string(),
    ]);
    assert_eq!(short_en.locale.as_deref(), Some("en"));

    let short_ja = CliArgsParser::parse_args(vec![
        "check".to_string(),
        "-l".to_string(),
        "ja".to_string(),
    ]);
    assert_eq!(short_ja.locale.as_deref(), Some("ja"));
}
