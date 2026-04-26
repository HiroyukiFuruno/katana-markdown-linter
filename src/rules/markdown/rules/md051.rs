mod fragments;
#[cfg(test)]
mod tests;

use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use fragments::heading_fragments;
use std::collections::HashSet;
use std::path::Path;

/// MD051 / link-fragments — Link fragments.
pub struct LinkFragmentsRule;

impl MarkdownRule for LinkFragmentsRule {
    fn id(&self) -> &'static str {
        "MD051"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD051")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let ignore_case = config
            .and_then(|config| config.properties.get("ignore_case"))
            .map(|value| value == "true")
            .unwrap_or(false);
        let ignored_pattern = config
            .and_then(|config| config.properties.get("ignored_pattern"))
            .and_then(|pattern| regex::Regex::new(pattern).ok());
        self.evaluate_context_with_options(ctx, ignore_case, ignored_pattern.as_ref())
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, config)
    }
}

impl LinkFragmentsRule {
    fn evaluate_context_with_options(
        &self,
        ctx: &DocumentContext<'_>,
        ignore_case: bool,
        ignored_pattern: Option<&regex::Regex>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD051");
        let mut diagnostics = Vec::new();
        let headings = heading_fragments(ctx);
        for link in ctx.inline_links() {
            if !link.kind.is_inline() {
                continue;
            }
            let Some(destination) = link.destination else {
                continue;
            };
            let Some(fragment) = destination.strip_prefix('#') else {
                continue;
            };
            if fragment.is_empty()
                || is_allowed_special_fragment(fragment)
                || ignored_pattern.is_some_and(|pattern| pattern.is_match(fragment))
                || fragment_exists(&headings, fragment, ignore_case)
            {
                continue;
            }

            let replacement = if ignore_case {
                None
            } else {
                headings
                    .iter()
                    .find(|heading| heading.eq_ignore_ascii_case(fragment))
                    .map(|heading| format!("#{heading}"))
            };
            let range = ctx.diagnostic_range(
                link.destination_range
                    .expect("inline link destination should have a source range"),
            );
            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: range.start_line,
                    start_column: range.start_column,
                    end_line: range.end_line,
                    end_column: range.end_column,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: replacement.map(|replacement| {
                    crate::rules::markdown::types::DiagnosticFix {
                        start_line: range.start_line,
                        start_column: range.start_column,
                        end_line: range.end_line,
                        end_column: range.end_column,
                        replacement,
                    }
                }),
            });
        }
        diagnostics
    }
}

fn fragment_exists(fragments: &HashSet<String>, value: &str, ignore_case: bool) -> bool {
    fragments.contains(value)
        || (ignore_case
            && fragments
                .iter()
                .any(|fragment| fragment.eq_ignore_ascii_case(value)))
}

fn is_allowed_special_fragment(fragment: &str) -> bool {
    fragment == "top" || is_github_line_fragment(fragment)
}

fn is_github_line_fragment(fragment: &str) -> bool {
    let Some(rest) = fragment.strip_prefix('L') else {
        return false;
    };
    !rest.is_empty()
        && rest
            .chars()
            .all(|ch| ch.is_ascii_digit() || ch == 'C' || ch == '-' || ch == 'L')
}
