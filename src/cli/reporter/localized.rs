use super::error::CliError;
use super::types::{CliReport, CliSummary};
use crate::i18n::{Locale, LocalizedDiagnostic, MessageParams};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(in crate::cli) struct LocalizedCliReport {
    command: &'static str,
    summary: CliSummary,
    files: Vec<LocalizedFileReport>,
    errors: Vec<LocalizedCliError>,
}

impl LocalizedCliReport {
    pub(in crate::cli) fn from_report(report: &CliReport, locale: Locale) -> Self {
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
                    fix_details: file.fix_details.clone(),
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
    fix_details: Vec<crate::FixDetail>,
    changed: bool,
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
