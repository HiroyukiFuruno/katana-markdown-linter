use super::Diagnostic;
use katana_markdown_linter::{LintResult, Locale};
use rmcp::schemars;
use serde::Serialize;

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct CheckTextResponse {
    pub(crate) issue_count: usize,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

impl CheckTextResponse {
    pub(crate) fn from_results(results: Vec<LintResult>, locale: Locale) -> Self {
        Self {
            issue_count: results.len(),
            diagnostics: results
                .into_iter()
                .map(|result| Diagnostic::from_result(result, locale))
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct FixTextResponse {
    pub(crate) content: String,
    pub(crate) applied_fixes: usize,
    pub(crate) remaining_issue_count: usize,
    pub(crate) remaining_diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct FileCheckResponse {
    pub(crate) path: String,
    pub(crate) issue_count: usize,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct DirectoryCheckResponse {
    pub(crate) path: String,
    pub(crate) file_count: usize,
    pub(crate) issue_count: usize,
    pub(crate) files: Vec<FileCheckResponse>,
    pub(crate) errors: Vec<WorkspaceToolError>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct FileFixPreviewResponse {
    pub(crate) path: String,
    pub(crate) changed: bool,
    pub(crate) applied_fixes: usize,
    pub(crate) diff: String,
    pub(crate) remaining_issue_count: usize,
    pub(crate) remaining_diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct FileFixApplyResponse {
    pub(crate) path: String,
    pub(crate) changed: bool,
    pub(crate) applied_fixes: usize,
    pub(crate) remaining_issue_count: usize,
    pub(crate) remaining_diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct WorkspaceToolError {
    pub(crate) path: String,
    pub(crate) message: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct RuleDocResponse {
    pub(crate) rule_id: String,
    pub(crate) locale: String,
    pub(crate) content: String,
}
