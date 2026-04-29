use katana_markdown_linter::{ConfigError, Locale, MarkdownLintConfig};
use rmcp::schemars;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct ConfigValidateResponse {
    pub(crate) valid: bool,
    pub(crate) error_count: usize,
    pub(crate) errors: Vec<ConfigValidationError>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct ConfigValidationError {
    pub(crate) rule_id: Option<String>,
    pub(crate) property: Option<String>,
    pub(crate) kind: String,
    pub(crate) message: String,
    pub(crate) message_id: String,
    pub(crate) message_params: std::collections::BTreeMap<String, String>,
    pub(crate) expected: Option<String>,
    pub(crate) actual: Option<String>,
    pub(crate) allowed: Vec<String>,
}

impl ConfigValidationError {
    pub(crate) fn from_error(error: ConfigError, locale: Locale) -> Self {
        Self {
            rule_id: error.rule_id.clone(),
            property: error.property.clone(),
            kind: error.kind_code().to_string(),
            message: error.localized_message(locale),
            message_id: error.message_id().to_string(),
            message_params: error.message_params(),
            expected: error.expected().map(str::to_string),
            actual: error.actual().map(str::to_string),
            allowed: error.allowed().into_iter().map(str::to_string).collect(),
        }
    }
}

pub(crate) fn validate_config(raw: Value, locale: Locale) -> Vec<ConfigValidationError> {
    MarkdownLintConfig { raw }
        .validate_against_schema()
        .into_iter()
        .map(|error| ConfigValidationError::from_error(error, locale))
        .collect()
}
