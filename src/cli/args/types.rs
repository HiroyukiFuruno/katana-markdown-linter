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
    Help(Option<HelpTopic>),
    InitConfig,
    Lsp,
    Rule(Option<String>),
    Config(ConfigCommand),
    Version,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelpTopic {
    Check,
    Config,
    Fix,
    Fmt,
    InitConfig,
    Lsp,
    Rule,
    Version,
}

impl HelpTopic {
    pub(super) fn from_command(value: &str) -> Option<Self> {
        match value {
            "check" => Some(Self::Check),
            "config" => Some(Self::Config),
            "fix" => Some(Self::Fix),
            "fmt" => Some(Self::Fmt),
            "init" | "init-config" => Some(Self::InitConfig),
            "lsp" => Some(Self::Lsp),
            "rule" => Some(Self::Rule),
            "version" => Some(Self::Version),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigCommand {
    File,
    Get,
    Schema,
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
    pub ignore_config_errors: bool,
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
            ignore_config_errors: false,
        }
    }
}
