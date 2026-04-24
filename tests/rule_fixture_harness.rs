use katana_markdown_linter::rules::markdown::MarkdownLinterOps;
use katana_markdown_linter::{fix, lint, LintOptions, MarkdownLintConfig};
use serde_json::Value;
use std::collections::{BTreeSet, HashSet};

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

#[test]
fn fixture_matrix_can_be_loaded_by_harness() {
    let matrix = matrix();
    let active = MarkdownLinterOps::get_official_rules()
        .into_iter()
        .filter_map(|rule| rule.official_meta().map(|meta| meta.code.to_string()))
        .collect::<HashSet<_>>();

    assert_eq!(matrix["summary"]["manual_required"].as_u64(), Some(0));
    assert!(rules(&matrix)
        .iter()
        .all(|rule| active.contains(rule_id(rule))));
}

#[test]
fn check_pass_and_fail_fixtures_execute() {
    let matrix = matrix();
    let options = LintOptions::default();

    for rule in rules(&matrix) {
        for case in cases(rule, "check_pass") {
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
    let options = LintOptions::default();

    for rule in rules(&matrix) {
        for case in cases(rule, "fix") {
            let fixed = fix(case_source(case), &options).expect("fix should run");
            assert_eq!(
                fixed.content,
                case["expected"].as_str().unwrap_or_default(),
                "{} / {} fix output differed",
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
fn fixable_rule_set_is_explicit_in_matrix() {
    let matrix = matrix();
    let actual = rules(&matrix)
        .iter()
        .filter(|rule| rule["fixable"].as_bool() == Some(true))
        .map(rule_id)
        .collect::<BTreeSet<_>>();
    let expected = [
        "MD004", "MD005", "MD007", "MD009", "MD010", "MD012", "MD018", "MD019", "MD020", "MD021",
        "MD022", "MD023", "MD027", "MD029", "MD030", "MD032", "MD037", "MD038", "MD047", "MD060",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}
