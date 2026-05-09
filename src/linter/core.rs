use super::error::Error;
use super::severity::SeverityMapBuilder;
use crate::{fix::FixApplicator, rules, FixResult, LintOptions, LintResult};
use std::path::Path;

const MAX_FIX_PASSES: usize = 8;

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
        let mut content = content.to_string();
        let mut applied_fixes = 0;
        let mut all_details = Vec::new();
        let severity_map = SeverityMapBuilder::fix(options);

        for _ in 0..MAX_FIX_PASSES {
            let diagnostics = rules::markdown::MarkdownLinterOps::evaluate_all(
                Path::new("<memory>"),
                &content,
                true,
                &severity_map,
                &options.rules,
            );
            let results = diagnostics
                .into_iter()
                .map(Into::into)
                .collect::<Vec<LintResult>>();
            if !results.iter().any(|result| result.fix.is_some()) {
                break;
            }

            let fixed = Self::fix_with_results(&content, &results);
            if fixed.applied_fixes == 0 || fixed.content == content {
                break;
            }

            applied_fixes += fixed.applied_fixes;
            all_details.extend(fixed.details);
            content = fixed.content;
        }

        Ok(FixResult {
            content,
            applied_fixes,
            details: all_details,
        })
    }

    pub fn fix_with_results(content: &str, results: &[LintResult]) -> FixResult {
        FixApplicator::apply(results, content, false)
    }

    pub fn fix_with_results_including_unsafe(content: &str, results: &[LintResult]) -> FixResult {
        FixApplicator::apply(results, content, true)
    }
}
