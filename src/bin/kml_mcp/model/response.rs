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
pub(crate) struct RuleDocResponse {
    pub(crate) rule_id: String,
    pub(crate) locale: String,
    pub(crate) content: String,
}
