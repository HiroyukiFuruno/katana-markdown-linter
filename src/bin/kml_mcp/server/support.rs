use crate::model::FileCheckResponse;
pub(super) use crate::shared::{catalog_rules, diagnostics, request_locale};
use crate::workspace::{FileLint, Workspace};
use katana_markdown_linter::Locale;

pub(super) fn file_check_response(
    workspace: &Workspace,
    file: FileLint,
    locale: Locale,
) -> FileCheckResponse {
    let diagnostics = diagnostics(file.diagnostics, locale);
    FileCheckResponse {
        path: workspace.display_relative(&file.path),
        issue_count: diagnostics.len(),
        diagnostics,
    }
}
