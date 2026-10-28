use super::error::Error;
use super::severity::SeverityMapBuilder;
use crate::{
    fix::{FixApplicator, FixEngine, DEFAULT_MAX_FIX_PASSES},
    rules, FixResult, LintOptions, LintResult,
};
use std::path::Path;

pub struct MarkdownLinter;

impl MarkdownLinter {
    pub fn lint(content: &str, options: &LintOptions) -> Result<Vec<LintResult>, Error> {
        Self::lint_for_path(Path::new("<memory>"), content, options)
    }

    pub(crate) fn lint_for_path(
        file_path: &Path,
        content: &str,
        options: &LintOptions,
    ) -> Result<Vec<LintResult>, Error> {
        let severity_map = SeverityMapBuilder::lint(options);
        let diagnostics = rules::markdown::MarkdownLinterOps::evaluate_all(
            file_path,
            content,
            true,
            &severity_map,
            &options.rules,
        );
        Ok(diagnostics.into_iter().map(Into::into).collect())
    }

    pub fn fix(content: &str, options: &LintOptions) -> Result<FixResult, Error> {
        let severity_map = SeverityMapBuilder::fix(options);
        let path = Path::new("<memory>");
        let evaluate = |current: &str| {
            Ok(rules::markdown::MarkdownLinterOps::evaluate_all(
                path,
                current,
                true,
                &severity_map,
                &options.rules,
            )
            .into_iter()
            .map(Into::into)
            .collect::<Vec<LintResult>>())
        };
        let results = evaluate(content)?;
        let engine = FixEngine::run(content, results, DEFAULT_MAX_FIX_PASSES, false, |current| {
            evaluate(current)
        })?;
        Ok(FixResult {
            content: engine.content,
            applied_fixes: engine.applied_fixes,
            details: engine.details,
        })
    }

    pub fn fix_with_results(content: &str, results: &[LintResult]) -> FixResult {
        FixApplicator::apply(results, content, false)
    }

    pub fn fix_with_results_including_unsafe(content: &str, results: &[LintResult]) -> FixResult {
        FixApplicator::apply(results, content, true)
    }
}
