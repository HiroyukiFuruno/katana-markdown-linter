use serde::Serialize;

use super::{render_message, Locale};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalizedDiagnostic {
    pub rule_id: String,
    pub rule_name: String,
    pub message: String,
    pub message_id: String,
    pub message_params: super::MessageParams,
    pub severity: crate::Severity,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub fix: Option<crate::Fix>,
}

impl LocalizedDiagnostic {
    pub fn from_result(result: &crate::LintResult, locale: Locale) -> Self {
        let mut localized = result.clone();
        localized.message = render_message(
            locale,
            result.message_id.as_str(),
            &result.message_params,
            result.message.as_str(),
        );
        Self {
            rule_id: localized.rule_id,
            rule_name: localized.rule_name,
            message: localized.message,
            message_id: localized.message_id,
            message_params: localized.message_params,
            severity: localized.severity,
            line: localized.line,
            column: localized.column,
            end_line: localized.end_line,
            end_column: localized.end_column,
            fix: localized.fix,
        }
    }
}
