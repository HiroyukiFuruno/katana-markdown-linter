use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD043 / required-headings — Required headings.
pub struct RequiredHeadingsRule;

impl MarkdownRule for RequiredHeadingsRule {
    fn id(&self) -> &'static str {
        "MD043"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD043")
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
        let meta = self.official_meta().expect("always Some for MD043");
        let required = required_headings(config);
        if required.is_empty() || heading_structure_matches(ctx, &required, match_case(config)) {
            return Vec::new();
        }

        vec![MarkdownDiagnostic {
            file: ctx.file_path().to_path_buf(),
            severity: DiagnosticSeverity::Warning,
            range: first_problem_range(ctx),
            message: meta.description.to_string(),
            rule_id: meta.code.to_string(),
            official_meta: Some(meta),
            fix_info: None,
        }]
    }
}

fn required_headings(config: Option<&RuleConfig>) -> Vec<String> {
    config
        .and_then(|config| config.properties.get("headings"))
        .and_then(|raw| serde_json::from_str::<Vec<String>>(raw).ok())
        .unwrap_or_default()
}

fn match_case(config: Option<&RuleConfig>) -> bool {
    config
        .and_then(|config| config.properties.get("match_case"))
        .and_then(|value| value.parse::<bool>().ok())
        .unwrap_or(false)
}

fn heading_structure_matches(
    ctx: &DocumentContext<'_>,
    required: &[String],
    match_case: bool,
) -> bool {
    let actual = ctx
        .headings()
        .iter()
        .map(|heading| format!("{} {}", "#".repeat(heading.level), heading.text))
        .collect::<Vec<_>>();
    let actual = normalize_headings(&actual, match_case);
    let required = normalize_headings(required, match_case);
    heading_pattern_matches(&actual, &required)
}

fn normalize_headings(values: &[String], match_case: bool) -> Vec<String> {
    values
        .iter()
        .map(|value| {
            if match_case {
                value.clone()
            } else {
                value.to_lowercase()
            }
        })
        .collect()
}

fn heading_pattern_matches(actual: &[String], required: &[String]) -> bool {
    if required.is_empty() {
        return actual.is_empty();
    }
    match required[0].as_str() {
        "*" => {
            heading_pattern_matches(actual, &required[1..])
                || (!actual.is_empty() && heading_pattern_matches(&actual[1..], required))
        }
        "+" => {
            !actual.is_empty()
                && (heading_pattern_matches(&actual[1..], &required[1..])
                    || heading_pattern_matches(&actual[1..], required))
        }
        "?" => !actual.is_empty() && heading_pattern_matches(&actual[1..], &required[1..]),
        expected => {
            actual.first().is_some_and(|actual| actual == expected)
                && heading_pattern_matches(&actual[1..], &required[1..])
        }
    }
}

fn first_problem_range(ctx: &DocumentContext<'_>) -> DiagnosticRange {
    if let Some(heading) = ctx.headings().first() {
        ctx.diagnostic_range(heading.marker_range)
    } else {
        let line = ctx.lines().last();
        DiagnosticRange {
            start_line: line.map_or(1, |line| line.number),
            start_column: 1,
            end_line: line.map_or(1, |line| line.number),
            end_column: line.map_or(1, |line| line.text.len().max(1)),
        }
    }
}
