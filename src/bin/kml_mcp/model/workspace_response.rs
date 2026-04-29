use super::Diagnostic;
use rmcp::schemars;
use serde::Serialize;

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
