use katana_markdown_linter::{LintOptions, LintResult, MarkdownLinter};
use serde::Deserialize;
use std::collections::BTreeSet;

#[derive(Debug, Deserialize)]
struct GoldenBaseline {
    schema_version: u64,
    upstream_source: String,
    cases: Vec<GoldenCase>,
}

#[derive(Debug, Deserialize)]
struct GoldenCase {
    rule_id: String,
    fixture: String,
    source: String,
    expected: Vec<NormalizedDiagnostic>,
    expected_fixed_output: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct NormalizedDiagnostic {
    rule_id: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    fixable: bool,
}

#[derive(Debug, Deserialize)]
struct KnownDeltaFile {
    schema_version: u64,
    deltas: Vec<KnownDelta>,
}

#[derive(Debug, Deserialize)]
struct KnownDelta {
    rule_id: String,
    fixture: String,
    reason: String,
    resolution: String,
}

fn baseline() -> GoldenBaseline {
    serde_json::from_str(include_str!("fixtures/upstream-golden-baseline.json"))
        .expect("golden baseline should parse")
}

fn known_deltas() -> KnownDeltaFile {
    serde_json::from_str(include_str!("fixtures/upstream-golden-known-deltas.json"))
        .expect("known delta file should parse")
}

#[test]
fn deterministic_golden_diagnostics_match_locked_upstream_baseline() {
    let baseline = baseline();
    assert_eq!(baseline.schema_version, 1);
    assert!(!baseline.upstream_source.trim().is_empty());
    let options = LintOptions::default();
    let deltas = known_deltas();

    for case in baseline.cases {
        let actual = normalize_for_rule(
            &case.rule_id,
            MarkdownLinter::lint(&case.source, &options).expect("lint should run"),
        );
        let failure = compare_case(&case, &actual, &deltas);
        assert!(failure.is_none(), "{}", failure.unwrap_or_default());
    }
}

#[test]
fn deterministic_golden_fix_outputs_match_locked_upstream_baseline() {
    let options = LintOptions::default();
    for case in baseline().cases {
        let Some(expected) = case.expected_fixed_output else {
            continue;
        };
        let actual = MarkdownLinter::fix(&case.source, &options).expect("fix should run");
        assert_eq!(
            actual.content, expected,
            "{} / {} fixed output differed",
            case.rule_id, case.fixture
        );
    }
}

#[test]
fn known_delta_entries_require_reason_and_resolution() {
    let deltas = known_deltas();
    assert_eq!(deltas.schema_version, 1);
    for delta in deltas.deltas {
        assert!(
            !delta.rule_id.trim().is_empty(),
            "delta rule_id is required"
        );
        assert!(
            !delta.fixture.trim().is_empty(),
            "delta fixture is required"
        );
        assert!(!delta.reason.trim().is_empty(), "delta reason is required");
        assert!(
            !delta.resolution.trim().is_empty(),
            "delta resolution is required"
        );
    }
}

#[test]
fn unknown_delta_failure_reports_rule_fixture_expected_and_actual() {
    let case = GoldenCase {
        rule_id: "MD999".to_string(),
        fixture: "synthetic".to_string(),
        source: String::new(),
        expected: vec![NormalizedDiagnostic {
            rule_id: "MD999".to_string(),
            line: 1,
            column: 1,
            end_line: 1,
            end_column: 1,
            fixable: false,
        }],
        expected_fixed_output: None,
    };
    let deltas = KnownDeltaFile {
        schema_version: 1,
        deltas: Vec::new(),
    };
    let message = compare_case(&case, &[], &deltas).expect("delta should be unknown");
    assert!(message.contains("MD999"));
    assert!(message.contains("synthetic"));
    assert!(message.contains("expected="));
    assert!(message.contains("actual="));
}

#[test]
fn golden_cases_reference_fixture_matrix_rules() {
    let matrix: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/rule-fixture-matrix.json"))
            .expect("fixture matrix should parse");
    let matrix_ids = matrix["rules"]
        .as_array()
        .expect("rules should be an array")
        .iter()
        .filter_map(|rule| rule["rule_id"].as_str())
        .collect::<BTreeSet<_>>();

    for case in baseline().cases {
        assert!(
            matrix_ids.contains(case.rule_id.as_str()),
            "{} / {} is not represented in the fixture matrix",
            case.rule_id,
            case.fixture
        );
    }
}

fn normalize_for_rule(rule_id: &str, diagnostics: Vec<LintResult>) -> Vec<NormalizedDiagnostic> {
    diagnostics
        .into_iter()
        .filter(|diagnostic| diagnostic.rule_id == rule_id)
        .map(|diagnostic| NormalizedDiagnostic {
            rule_id: diagnostic.rule_id,
            line: diagnostic.line,
            column: diagnostic.column,
            end_line: diagnostic.end_line,
            end_column: diagnostic.end_column,
            fixable: diagnostic.fix.is_some(),
        })
        .collect()
}

fn compare_case(
    case: &GoldenCase,
    actual: &[NormalizedDiagnostic],
    deltas: &KnownDeltaFile,
) -> Option<String> {
    if case.expected == actual {
        return None;
    }
    if deltas
        .deltas
        .iter()
        .any(|delta| delta.rule_id == case.rule_id && delta.fixture == case.fixture)
    {
        return None;
    }
    Some(format!(
        "{} / {} golden delta: expected={:?} actual={:?}",
        case.rule_id, case.fixture, case.expected, actual
    ))
}
