use crate::{
    fix_with_results, fix_with_results_including_unsafe, lint_for_path, FixSafety, LintOptions,
    MarkdownLintConfig,
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

pub(super) fn load_effective_config(
    path: &Path,
    explicit: Option<&Path>,
) -> Result<MarkdownLintConfig, String> {
    if let Some(path) = explicit {
        if !path.exists() {
            return Err(format!("config file not found: {}", path.display()));
        }
        return MarkdownLintConfig::load(path).map_err(|err| err.to_string());
    }

    let mut current = path.parent();
    while let Some(dir) = current {
        let json = dir.join(".markdownlint.json");
        if json.exists() {
            return MarkdownLintConfig::load(&json).map_err(|err| err.to_string());
        }
        let jsonc = dir.join(".markdownlint.jsonc");
        if jsonc.exists() {
            return MarkdownLintConfig::load(&jsonc).map_err(|err| err.to_string());
        }
        current = dir.parent();
    }

    Ok(MarkdownLintConfig::default())
}

pub(super) fn validate_effective_config(
    path: &Path,
    explicit: Option<&Path>,
) -> Result<Vec<crate::ConfigError>, String> {
    Ok(load_effective_config(path, explicit)?.validate_against_schema())
}

pub(super) fn apply_fixes_until_stable(
    content: &str,
    file_path: &Path,
    initial_results: Vec<crate::LintResult>,
    options: &LintOptions,
    include_unsafe: bool,
) -> Result<FixedContent, String> {
    const MAX_FIX_PASSES: usize = 8;

    let mut content = content.to_string();
    let mut diagnostics = initial_results;
    let mut applied_fixes = 0;
    let mut fix_details = Vec::new();

    for _ in 0..MAX_FIX_PASSES {
        if !diagnostics
            .iter()
            .any(|diagnostic| is_applicable_fix(diagnostic, include_unsafe))
        {
            break;
        }

        let fixed = if include_unsafe {
            fix_with_results_including_unsafe(&content, &diagnostics)
        } else {
            fix_with_results(&content, &diagnostics)
        };
        if fixed.applied_fixes == 0 || fixed.content == content {
            break;
        }

        applied_fixes += fixed.applied_fixes;
        fix_details.extend(fixed.details);
        content = fixed.content;
        diagnostics = lint_for_path(file_path, &content, options).map_err(|err| err.to_string())?;
    }

    Ok(FixedContent {
        content,
        diagnostics,
        applied_fixes,
        fix_details,
    })
}

pub(super) fn is_applicable_fix(diagnostic: &crate::LintResult, include_unsafe: bool) -> bool {
    diagnostic
        .fix
        .as_ref()
        .is_some_and(|fix| include_unsafe || fix.safety == FixSafety::Safe)
}
