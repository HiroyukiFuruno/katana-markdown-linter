use super::{policy::FixStage, FixApplicator};
use crate::{Error, FixDetail, LintResult};
use std::collections::HashSet;

pub(crate) const DEFAULT_MAX_FIX_PASSES: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FixEngineResult {
    pub(crate) content: String,
    pub(crate) diagnostics: Vec<LintResult>,
    pub(crate) applied_fixes: usize,
    pub(crate) details: Vec<FixDetail>,
}

/// Shared convergence loop for library, CLI, and formatter fix entrypoints.
pub(crate) struct FixEngine;

impl FixEngine {
    pub(crate) fn run(
        original: &str,
        initial_diagnostics: Vec<LintResult>,
        max_passes: usize,
        include_unsafe: bool,
        mut check: impl FnMut(&str) -> Result<Vec<LintResult>, Error>,
    ) -> Result<FixEngineResult, Error> {
        let max_passes = max_passes.max(1);
        let mut content = original.to_string();
        let mut diagnostics = initial_diagnostics;
        let mut applied_fixes = 0;
        let mut details = Vec::new();
        let mut seen = HashSet::from([content.clone()]);

        for _ in 0..max_passes {
            let mut selected = None;
            for stage in [FixStage::Syntax, FixStage::Content, FixStage::Layout] {
                let candidates = diagnostics
                    .iter()
                    .filter(|diagnostic| {
                        diagnostic.fix.is_some() && stage.contains(&diagnostic.rule_id)
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                if candidates.is_empty() {
                    continue;
                }
                let fixed = FixApplicator::apply(&candidates, &content, include_unsafe);
                if fixed.content != content {
                    selected = Some(fixed);
                    break;
                }
            }

            let Some(fixed) = selected else {
                return Ok(FixEngineResult {
                    content,
                    diagnostics,
                    applied_fixes,
                    details,
                });
            };

            if !seen.insert(fixed.content.clone()) {
                return Err(Error::new("fix engine detected a repeated content cycle"));
            }
            applied_fixes += fixed.applied_fixes;
            details.extend(fixed.details);
            content = fixed.content;
            diagnostics = check(&content)?;
        }

        if FixStage::ALL.iter().any(|stage| {
            let candidates = diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic.fix.is_some() && stage.contains(&diagnostic.rule_id)
                })
                .cloned()
                .collect::<Vec<_>>();
            FixApplicator::has_effective(&candidates, &content, include_unsafe)
        }) {
            return Err(Error::new(format!(
                "fix engine did not converge after {max_passes} passes; applicable fixes remain"
            )));
        }

        Ok(FixEngineResult {
            content,
            diagnostics,
            applied_fixes,
            details,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Fix, FixSafety, Severity};
    use std::cell::Cell;

    fn diagnostic(rule_id: &str, content: &str, replacement: &str) -> LintResult {
        let index = content.find(replacement).unwrap_or(0);
        let line = content[..index]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count()
            + 1;
        let column = content[..index]
            .rsplit('\n')
            .next()
            .unwrap_or_default()
            .chars()
            .count()
            + 1;
        LintResult {
            rule_id: rule_id.to_string(),
            rule_name: String::new(),
            message: String::new(),
            message_id: "rule.generic".to_string(),
            message_params: Default::default(),
            severity: Severity::Warning,
            line,
            column,
            end_line: line,
            end_column: column + 1,
            fix: Some(Fix {
                range: crate::Range {
                    start_line: line,
                    start_column: column,
                    end_line: line,
                    end_column: column + 1,
                },
                replacement: replacement.to_string(),
                safety: FixSafety::Safe,
            }),
        }
    }

    #[test]
    fn reports_exact_text_cycle() {
        let initial = vec![diagnostic("MD018", "a", "b")];
        let mut checks = 0;
        let error = FixEngine::run("a", initial, 8, false, |content| {
            checks += 1;
            Ok(vec![diagnostic(
                "MD018",
                content,
                if content == "b" { "a" } else { "b" },
            )])
        })
        .unwrap_err();
        assert!(error.to_string().contains("cycle"));
        assert_eq!(checks, 1);
    }

    #[test]
    fn errors_when_editable_fixes_remain_at_budget() {
        let initial = vec![diagnostic("MD018", "a", "b")];
        let checks = Cell::new(0);
        let error = FixEngine::run("a", initial, 1, false, |content| {
            let call = checks.get();
            checks.set(call + 1);
            Ok(vec![diagnostic(
                "MD018",
                content,
                if call == 0 { "c" } else { "d" },
            )])
        })
        .unwrap_err();
        assert!(error.to_string().contains("did not converge"));
    }
}
