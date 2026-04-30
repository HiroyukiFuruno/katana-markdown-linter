use katana_markdown_linter::rules::markdown::MarkdownLinterOps;
use katana_markdown_linter::{
    fix, fix_with_results_including_unsafe, lint, ConfigErrorKind, LintOptions, LintResult,
    MarkdownLintConfig, Range, RuleConfig,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::Path;

fn matrix() -> Value {
    serde_json::from_str(include_str!("fixtures/rule-fixture-matrix.json"))
        .expect("fixture matrix should deserialize")
}

fn rules(matrix: &Value) -> &[Value] {
    matrix["rules"]
        .as_array()
        .expect("rules should be an array")
}

fn rule_id(rule: &Value) -> &str {
    rule["rule_id"]
        .as_str()
        .expect("rule fixture should have rule_id")
}

fn cases<'a>(rule: &'a Value, field: &str) -> impl Iterator<Item = &'a Value> {
    rule[field]
        .as_array()
        .expect("fixture field should be an array")
        .iter()
}

fn optional_cases<'a>(rule: &'a Value, field: &str) -> impl Iterator<Item = &'a Value> {
    rule.get(field)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

fn case_name(case: &Value) -> &str {
    case["name"]
        .as_str()
        .expect("fixture case should have a name")
}

fn case_source(case: &Value) -> &str {
    case["source"]
        .as_str()
        .expect("fixture case should have source")
}

fn options_for_case(case: &Value) -> LintOptions {
    let mut options = LintOptions::default();
    let Some(config) = case.get("config").and_then(Value::as_object) else {
        return options;
    };

    for (rule_id, value) in config {
        match value {
            Value::Bool(enabled) => {
                options.rules.insert(
                    rule_id.clone(),
                    RuleConfig {
                        enabled: *enabled,
                        properties: Default::default(),
                    },
                );
            }
            Value::Object(properties) => {
                let enabled = properties
                    .get("enabled")
                    .and_then(Value::as_bool)
                    .unwrap_or(true);
                let properties = properties
                    .iter()
                    .filter(|(key, _)| key.as_str() != "enabled")
                    .map(|(key, value)| {
                        let value = value
                            .as_str()
                            .map(ToOwned::to_owned)
                            .unwrap_or_else(|| value.to_string());
                        (key.clone(), value)
                    })
                    .collect();
                options.rules.insert(
                    rule_id.clone(),
                    RuleConfig {
                        enabled,
                        properties,
                    },
                );
            }
            _ => {}
        }
    }

    options
}

#[test]
fn fixture_matrix_can_be_loaded_by_harness() {
    let matrix = matrix();
    let active = MarkdownLinterOps::get_official_rules()
        .into_iter()
        .filter_map(|rule| rule.official_meta().map(|meta| meta.code.to_string()))
        .collect::<HashSet<_>>();

    assert_eq!(matrix["summary"]["manual_required"].as_u64(), Some(13));
    assert!(rules(&matrix)
        .iter()
        .all(|rule| active.contains(rule_id(rule))));
}

#[test]
fn check_pass_and_fail_fixtures_execute() {
    let matrix = matrix();

    for rule in rules(&matrix) {
        for case in cases(rule, "check_pass") {
            let options = options_for_case(case);
            let diagnostics = lint(case_source(case), &options).expect("lint should run");
            assert!(
                diagnostics
                    .iter()
                    .all(|diagnostic| diagnostic.rule_id != rule_id(rule)),
                "{} / {} unexpectedly failed",
                rule_id(rule),
                case_name(case)
            );
        }
        for case in cases(rule, "check_fail") {
            let options = options_for_case(case);
            let diagnostics = lint(case_source(case), &options).expect("lint should run");
            assert!(
                diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.rule_id == rule_id(rule)),
                "{} / {} did not report expected violation",
                rule_id(rule),
                case_name(case)
            );
        }
    }
}

#[test]
fn fix_fixtures_compare_before_and_after() {
    let matrix = matrix();

    for rule in rules(&matrix) {
        for case in cases(rule, "fix") {
            let options = options_for_case(case);
            let fixed = fix(case_source(case), &options).expect("fix should run");
            assert_eq!(
                fixed.content,
                case["expected"].as_str().unwrap_or_default(),
                "{} / {} fix output differed",
                rule_id(rule),
                case_name(case)
            );
            let fixed_again = fix(&fixed.content, &options).expect("second fix should run");
            assert_eq!(
                fixed_again.content,
                fixed.content,
                "{} / {} fix output was not idempotent",
                rule_id(rule),
                case_name(case)
            );
        }
    }
}

#[test]
fn unsafe_fix_fixtures_compare_before_and_after() {
    let matrix = matrix();

    for rule in rules(&matrix) {
        for case in optional_cases(rule, "unsafe_fix") {
            let options = options_for_case(case);
            let diagnostics = lint(case_source(case), &options).expect("lint should run");
            assert!(
                diagnostics.iter().any(|diagnostic| {
                    diagnostic.rule_id == rule_id(rule)
                        && diagnostic.fix.as_ref().is_some_and(|fix| {
                            fix.safety == katana_markdown_linter::FixSafety::Unsafe
                        })
                }),
                "{} / {} did not expose expected unsafe fix",
                rule_id(rule),
                case_name(case)
            );
            let fixed = fix_with_results_including_unsafe(case_source(case), &diagnostics);
            assert_eq!(
                fixed.content,
                case["expected"].as_str().unwrap_or_default(),
                "{} / {} unsafe fix output differed",
                rule_id(rule),
                case_name(case)
            );
        }
    }
}

#[test]
fn config_valid_and_invalid_fixtures_execute() {
    let matrix = matrix();
    let configurable_rules = MarkdownLinterOps::get_user_configurable_rules();

    for rule in rules(&matrix) {
        for case in cases(rule, "config_valid") {
            let config = MarkdownLintConfig {
                raw: serde_json::from_str(case_source(case)).expect("valid fixture should parse"),
            };
            let errors = config.validate(&configurable_rules);
            assert!(
                errors.is_empty(),
                "{} / {} expected valid config, got {:?}",
                rule_id(rule),
                case_name(case),
                errors
            );
        }
        for case in cases(rule, "config_invalid") {
            let config = MarkdownLintConfig {
                raw: serde_json::from_str(case_source(case)).expect("invalid fixture should parse"),
            };
            let errors = config.validate(&configurable_rules);
            assert!(
                !errors.is_empty(),
                "{} / {} expected invalid config",
                rule_id(rule),
                case_name(case)
            );
        }
    }
}

#[test]
fn edge_case_fixtures_execute() {
    let matrix = matrix();

    for rule in rules(&matrix) {
        for case in cases(rule, "edge") {
            let options = options_for_case(case);
            let diagnostics = lint(case_source(case), &options).expect("lint should run");
            match case["expected"].as_str() {
                Some(expected) => assert!(
                    diagnostics
                        .iter()
                        .any(|diagnostic| diagnostic.rule_id == expected),
                    "{} / {} did not report expected edge violation",
                    rule_id(rule),
                    case_name(case)
                ),
                None => assert!(
                    diagnostics
                        .iter()
                        .all(|diagnostic| diagnostic.rule_id != rule_id(rule)),
                    "{} / {} unexpectedly reported edge violation",
                    rule_id(rule),
                    case_name(case)
                ),
            }
        }
    }
}

#[test]
fn config_alias_and_lifecycle_behavior_is_fixed() {
    let rules = MarkdownLinterOps::get_user_configurable_rules();
    let alias_config = MarkdownLintConfig {
        raw: serde_json::json!({ "heading-increment": true }),
    };
    let alias_errors = alias_config.validate(&rules);
    assert!(alias_errors.is_empty());

    let unknown_config = MarkdownLintConfig {
        raw: serde_json::json!({ "not-a-real-rule": true }),
    };
    let unknown_errors = unknown_config.validate(&rules);
    assert!(unknown_errors
        .iter()
        .any(|error| matches!(error.kind, ConfigErrorKind::UnknownRule)));

    let catalog = katana_markdown_linter::rule_catalog();
    assert!(catalog.deprecated.is_empty());
    assert!(catalog.removed.is_empty());
}

#[test]
fn config_property_error_shapes_are_fixed() {
    let rules = MarkdownLinterOps::get_user_configurable_rules();
    let cases = [
        (
            serde_json::json!({ "MD060": { "unknown": true } }),
            "unknown property",
        ),
        (
            serde_json::json!({ "MD060": { "aligned_delimiter": null } }),
            "wrong type",
        ),
        (
            serde_json::json!({ "MD060": { "style": "invalid" } }),
            "invalid enum",
        ),
    ];

    for (raw, label) in cases {
        let errors = MarkdownLintConfig { raw }.validate(&rules);
        assert!(!errors.is_empty(), "{label} should be invalid");
    }
}

#[test]
fn edge_cases_cover_empty_no_newline_long_code_fence_and_html() {
    let options = LintOptions::default();
    let empty = lint("", &options).expect("lint should run");
    assert!(empty.iter().all(|diagnostic| diagnostic.rule_id != "MD043"));

    let missing_newline = lint("text", &options).expect("lint should run");
    assert!(missing_newline
        .iter()
        .any(|diagnostic| diagnostic.rule_id == "MD047"));

    let long_line = format!("{} b c\n", "a".repeat(78));
    let long_line_diagnostics = lint(&long_line, &options).expect("lint should run");
    assert!(long_line_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.rule_id == "MD013"));

    let fenced_heading =
        lint("# Title\n\n```\n### skipped\n```\n", &options).expect("lint should run");
    assert!(fenced_heading
        .iter()
        .all(|diagnostic| diagnostic.rule_id != "MD001"));

    let html = lint("<span>text</span>\n", &options).expect("lint should run");
    assert!(html.iter().any(|diagnostic| diagnostic.rule_id == "MD033"));
}

#[test]
fn edge_cases_cover_list_heading_and_table_boundaries() {
    let options = LintOptions::default();
    let list = lint("- one\n  - nested\n   - off\n", &options).expect("lint should run");
    assert!(list.iter().any(|diagnostic| diagnostic.rule_id == "MD005"));
    assert!(list.iter().any(|diagnostic| diagnostic.rule_id == "MD007"));

    let heading = lint("# H1\n\n### H3\n", &options).expect("lint should run");
    assert!(heading
        .iter()
        .any(|diagnostic| diagnostic.rule_id == "MD001"));

    let table = lint("| a | b |\n|---|---|\n| 1 | 2 | 3 |\n", &options).expect("lint should run");
    assert!(table.iter().any(|diagnostic| diagnostic.rule_id == "MD056"));
}

#[test]
fn front_matter_and_gfm_extension_behavior_is_explicit() {
    let options = LintOptions::default();
    let front_matter = "---\ntitle: Doc\n---\n\n# Doc\n";
    let diagnostics = lint(front_matter, &options).expect("lint should run");
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.rule_id == "MD041"),
        "front matter is currently parsed as markdown content, not skipped for first-line heading"
    );

    let gfm_table = "|a| b |\n|---|---|\n| c | d |\n";
    let table_diagnostics = lint(gfm_table, &options).expect("lint should run");
    assert!(table_diagnostics
        .iter()
        .any(|diagnostic| diagnostic.rule_id == "MD060"));
}

#[test]
fn cli_reference_edge_cases_are_represented_without_copying_implementations() {
    let matrix = matrix();
    let matrix_rule_ids = rules(&matrix).iter().map(rule_id).collect::<BTreeSet<_>>();

    for rule_id in ["MD013", "MD033", "MD047", "MD056", "MD060"] {
        assert!(
            matrix_rule_ids.contains(rule_id),
            "{rule_id} is kept as a CLI-parity edge case inspired by rumdl/mado behavior"
        );
    }
}

#[test]
fn fixable_rule_set_is_explicit_in_matrix() {
    let matrix = matrix();
    let actual = rules(&matrix)
        .iter()
        .filter(|rule| rule["fixable"].as_bool() == Some(true))
        .map(|rule| rule_id(rule).to_string())
        .collect::<BTreeSet<_>>();
    let expected = katana_markdown_linter::available_rules()
        .into_iter()
        .filter(|rule| rule.fixable)
        .map(|rule| rule.id)
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

#[test]
fn matrix_markdown_summary_matches_json_counts() {
    let matrix = matrix();
    let markdown = include_str!("fixtures/rule-fixture-matrix.md");
    let expected_manual = matrix["summary"]["manual_required"]
        .as_u64()
        .expect("summary should contain manual_required");
    assert!(
        markdown.contains(&format!("- manual required: {expected_manual}")),
        "markdown summary manual_required should match JSON"
    );
}

#[test]
fn fixture_matrix_parameters_match_upstream_docs() {
    let matrix = matrix();
    let upstream =
        katana_markdown_linter::upstream::load_catalog_from_dir(Path::new("upstream_docs"))
            .expect("upstream docs should load");
    let upstream_params = upstream
        .rules
        .iter()
        .map(|rule| {
            let params = rule
                .properties
                .iter()
                .map(|property| {
                    (
                        property.key.clone(),
                        (
                            property.value_type.clone(),
                            property.default_value.clone().unwrap_or_default(),
                            property.values.clone(),
                        ),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            (rule.id.as_str(), params)
        })
        .collect::<BTreeMap<_, _>>();

    for rule in rules(&matrix) {
        let actual = rule["parameters"]
            .as_array()
            .expect("parameters should be an array")
            .iter()
            .map(|param| {
                (
                    param["key"].as_str().unwrap_or_default().to_string(),
                    (
                        param["value_type"].as_str().unwrap_or_default().to_string(),
                        param["default_value"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                        param["values"]
                            .as_array()
                            .expect("values should be an array")
                            .iter()
                            .filter_map(|value| value.as_str().map(str::to_string))
                            .collect::<Vec<_>>(),
                    ),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let expected = upstream_params
            .get(rule_id(rule))
            .unwrap_or_else(|| panic!("{} missing upstream parameters", rule_id(rule)));
        assert_eq!(
            &actual,
            expected,
            "{} parameters drifted from upstream_docs",
            rule_id(rule)
        );
    }
}

#[test]
fn fixable_rules_without_fix_fixtures_explain_why() {
    let matrix = matrix();
    let unsupported = rules(&matrix)
        .iter()
        .filter(|rule| rule["fixable"].as_bool() == Some(true))
        .filter(|rule| cases(rule, "fix").next().is_none())
        .collect::<Vec<_>>();

    for rule in unsupported {
        let manual_required = rule["manual_required"]
            .as_array()
            .expect("manual_required should be an array");
        assert!(
            manual_required.iter().any(|reason| {
                reason.as_str().is_some_and(|value| {
                    value.starts_with("fix unsupported:")
                        || value.starts_with("fix requires:")
                        || value.starts_with("fix deferred:")
                })
            }),
            "{} should explain why fix is unsupported or gated",
            rule_id(rule)
        );
    }
}

#[test]
fn fixable_rules_without_fix_fixtures_have_explicit_metadata() {
    let matrix = matrix();
    let unsupported = rules(&matrix)
        .iter()
        .filter(|rule| rule["fixable"].as_bool() == Some(true))
        .filter(|rule| cases(rule, "fix").next().is_none())
        .map(rule_id)
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::<&str>::new();

    assert_eq!(unsupported, expected);
    for rule in rules(&matrix)
        .iter()
        .filter(|rule| unsupported.contains(rule_id(rule)))
    {
        let manual_required = rule["manual_required"]
            .as_array()
            .expect("manual_required should be an array");
        assert!(
            manual_required.iter().any(|reason| reason
                .as_str()
                .is_some_and(|value| value.starts_with("fix unsupported:"))),
            "{} should explain why fix is unsupported",
            rule_id(rule)
        );
    }
}

#[test]
fn overlapping_fix_ranges_are_detectable_before_application() {
    let first = LintResult {
        rule_id: "TEST001".to_string(),
        rule_name: "first".to_string(),
        message: "first".to_string(),
        message_id: "rule.generic".to_string(),
        message_params: katana_markdown_linter::i18n::diagnostic_message_params(
            "TEST001", "first", "first",
        ),
        severity: Default::default(),
        line: 1,
        column: 1,
        end_line: 1,
        end_column: 3,
        fix: Some(katana_markdown_linter::Fix {
            range: Range {
                start_line: 1,
                start_column: 1,
                end_line: 1,
                end_column: 3,
            },
            replacement: "A".to_string(),
            safety: katana_markdown_linter::FixSafety::Safe,
        }),
    };
    let mut second = first.clone();
    second.rule_id = "TEST002".to_string();
    second.fix.as_mut().expect("fix exists").range.start_column = 2;

    assert!(has_overlapping_fix_ranges(&[first, second]));
}

fn has_overlapping_fix_ranges(results: &[LintResult]) -> bool {
    let fixes = results
        .iter()
        .filter_map(|result| result.fix.as_ref().map(|fix| (&result.rule_id, &fix.range)))
        .collect::<Vec<_>>();

    for (idx, (left_rule, left)) in fixes.iter().enumerate() {
        for (right_rule, right) in fixes.iter().skip(idx + 1) {
            if left_rule == right_rule || left.start_line != right.start_line {
                continue;
            }
            if left.start_column < right.end_column && right.start_column < left.end_column {
                return true;
            }
        }
    }
    false
}
