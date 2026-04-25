use crate::model::{Diagnostic, FileCheckResponse, RuleMetadata};
use crate::workspace::{FileLint, Workspace};
use katana_markdown_linter::{rule_catalog, LintResult, Locale};

pub(super) fn request_locale(locale: Option<&str>) -> Locale {
    locale
        .map(katana_markdown_linter::resolve_locale_code)
        .unwrap_or(Locale::En)
}

pub(super) fn diagnostics(results: Vec<LintResult>, locale: Locale) -> Vec<Diagnostic> {
    results
        .into_iter()
        .map(|result| Diagnostic::from_result(result, locale))
        .collect()
}

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

pub(super) fn catalog_rules(locale: Locale) -> Vec<RuleMetadata> {
    let catalog = rule_catalog();
    catalog
        .active
        .into_iter()
        .map(|rule| RuleMetadata::from_entry(rule, "active", locale))
        .chain(
            catalog
                .deprecated
                .into_iter()
                .map(|rule| RuleMetadata::from_entry(rule, "deprecated", locale)),
        )
        .chain(
            catalog
                .removed
                .into_iter()
                .map(|rule| RuleMetadata::from_entry(rule, "removed", locale)),
        )
        .collect()
}
