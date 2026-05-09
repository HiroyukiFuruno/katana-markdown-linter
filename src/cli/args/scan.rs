use super::types::HelpTopic;

pub(super) struct CliArgScan;

impl CliArgScan {
    pub(super) fn requests_help(args: &[String]) -> bool {
        matches!(
            Self::command_tokens(args).first().map(String::as_str),
            Some("help")
        ) || args
            .iter()
            .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
    }

    pub(super) fn help_topic(args: &[String]) -> Option<HelpTopic> {
        let tokens = Self::command_tokens(args);
        if matches!(tokens.first().map(String::as_str), Some("help")) {
            return tokens
                .get(1)
                .and_then(|value| HelpTopic::from_command(value));
        }

        tokens
            .iter()
            .find_map(|value| HelpTopic::from_command(value))
    }

    pub(super) fn requests_version(args: &[String]) -> bool {
        matches!(
            Self::command_tokens(args).first().map(String::as_str),
            Some("version")
        ) || args
            .iter()
            .any(|arg| matches!(arg.as_str(), "--version" | "-V" | "-v"))
    }

    pub(super) fn locale_arg(args: &[String]) -> Option<String> {
        args.windows(2).find_map(|window| {
            matches!(window[0].as_str(), "--locale" | "--local" | "-l").then(|| window[1].clone())
        })
    }

    fn command_tokens(args: &[String]) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut skip_next = false;
        for arg in args {
            if skip_next {
                skip_next = false;
                continue;
            }
            if Self::option_takes_value(arg) {
                skip_next = true;
                continue;
            }
            if arg.starts_with('-') {
                continue;
            }
            tokens.push(arg.clone());
        }
        tokens
    }

    fn option_takes_value(arg: &str) -> bool {
        matches!(
            arg,
            "--config"
                | "--file"
                | "--format"
                | "--output"
                | "--locale"
                | "--local"
                | "-l"
                | "--include"
                | "--exclude"
        )
    }
}
