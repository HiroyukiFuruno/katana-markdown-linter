use crate::{
    fix::{FixEngine, DEFAULT_MAX_FIX_PASSES},
    LintOptions, MarkdownLinter,
};
use std::path::Path;

pub(super) struct FixedContent {
    pub(super) content: String,
    pub(super) diagnostics: Vec<crate::LintResult>,
    pub(super) applied_fixes: usize,
    pub(super) fix_details: Vec<crate::FixDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct UnsafeFixPolicy {
    pub(super) include_unsafe: bool,
    pub(super) declined: bool,
}

pub(super) fn apply_fixes_until_stable(
    content: &str,
    file_path: &Path,
    initial_results: Vec<crate::LintResult>,
    options: &LintOptions,
    include_unsafe: bool,
) -> Result<FixedContent, String> {
    let engine = FixEngine::run(
        content,
        initial_results,
        DEFAULT_MAX_FIX_PASSES,
        include_unsafe,
        |current| MarkdownLinter::lint_for_path(file_path, current, options),
    )
    .map_err(|err| err.to_string())?;

    Ok(FixedContent {
        content: engine.content,
        diagnostics: engine.diagnostics,
        applied_fixes: engine.applied_fixes,
        fix_details: engine.details,
    })
}
