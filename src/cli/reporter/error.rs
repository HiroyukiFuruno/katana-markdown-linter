use super::super::input::InputExpandError;
use crate::i18n::{Locale, MessageParams};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CliError {
    pub(crate) kind: &'static str,
    pub(crate) path: Option<String>,
    pub(crate) message: String,
    pub(crate) message_id: String,
    pub(crate) message_params: MessageParams,
}

impl CliError {
    pub(in crate::cli) fn filesystem(path: &Path, err: impl std::fmt::Display) -> Self {
        let message = err.to_string();
        Self {
            kind: "filesystem",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "filesystem.error".to_string(),
        }
    }

    pub(in crate::cli) fn filesystem_message(message: String) -> Self {
        Self {
            kind: "filesystem",
            path: None,
            message_params: message_params(&message),
            message,
            message_id: "filesystem.error".to_string(),
        }
    }

    pub(in crate::cli) fn glob(message: String) -> Self {
        Self {
            kind: "glob",
            path: None,
            message_params: message_params(&message),
            message,
            message_id: "glob.error".to_string(),
        }
    }

    pub(in crate::cli) fn config(path: &Path, message: String) -> Self {
        Self {
            kind: "config",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "config.error".to_string(),
        }
    }

    pub(in crate::cli) fn config_validation(path: &Path, error: crate::ConfigError) -> Self {
        Self {
            kind: "config",
            path: Some(path.display().to_string()),
            message_params: error.message_params(),
            message: error.to_string(),
            message_id: error.message_id().to_string(),
        }
    }

    pub(in crate::cli) fn rule(path: &Path, message: String) -> Self {
        Self {
            kind: "rule",
            path: Some(path.display().to_string()),
            message_params: message_params(&message),
            message,
            message_id: "rule.error".to_string(),
        }
    }

    pub(in crate::cli) fn from_input_expand_error(error: InputExpandError) -> Self {
        match error {
            InputExpandError::Filesystem(message) => Self::filesystem_message(message),
            InputExpandError::Glob(message) => Self::glob(message),
        }
    }

    pub(in crate::cli) fn localized_message(&self, locale: Locale) -> String {
        crate::i18n::MessageCatalog::render_message(
            locale,
            self.message_id.as_str(),
            &self.message_params,
            self.message.as_str(),
        )
    }
}

fn message_params(message: &str) -> MessageParams {
    let mut params = MessageParams::new();
    params.insert("message".to_string(), message.to_string());
    params
}
