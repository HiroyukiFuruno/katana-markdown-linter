use std::path::PathBuf;

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
    pub include_ignored: bool,
    pub include_reserved: bool,
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
            include_ignored: false,
            include_reserved: false,
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
            "--include-ignored" => include_ignored = true,
            "--include-reserved" => include_reserved = true,
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
        assert_eq!(
            cli.config,
            Some(std::path::PathBuf::from(".markdownlint.jsonc"))
        );
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
}
