use super::build;
use super::compare;
use super::report;
use super::{RuleFixtureDriftType, RuleFixtureEntry, RuleFixtureMatrix, RuleFixtureMatrixSummary};
use crate::upstream::{UpstreamCatalog, UpstreamDocumentService};
use std::fs;

#[test]
fn builds_fixture_matrix_with_required_schema_fields() {
    let catalog = UpstreamCatalog {
        source: "test".to_string(),
        rules: vec![UpstreamDocumentService::parse_rule_document(
            r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

Parameters:

- `enabled`: Enable example (`boolean`, default `true`)

Fixable: Some violations can be fixed by tooling.

```markdown
bad
```
"#,
        )
        .expect("document should parse")],
    };

    let matrix = build::build_fixture_matrix(&catalog);
    let json = serde_json::to_value(&matrix).expect("matrix should serialize");
    let rule = &matrix.rules[0];

    assert_eq!(matrix.summary.total_rules, 1);
    assert_eq!(matrix.summary.rules_with_examples, 1);
    assert_eq!(matrix.summary.rules_with_parameters, 1);
    assert_eq!(matrix.summary.rules_with_fix_metadata, 1);
    assert_eq!(rule.rule_id, "MD999");
    assert_eq!(rule.aliases, vec!["example-rule"]);
    assert_eq!(rule.tags, vec!["test"]);
    assert_eq!(rule.parameters[0].key, "enabled");
    assert_eq!(rule.fixable, Some(true));
    assert_eq!(rule.check_fail.len(), 1);
    assert_eq!(
        rule.config_valid[0].source,
        r#"{ "MD999": { "enabled": true } }"#
    );
    assert_eq!(
        rule.config_invalid[0].expected.as_deref(),
        Some("invalid type")
    );

    for key in [
        "rule_id",
        "aliases",
        "tags",
        "parameters",
        "fixable",
        "check_pass",
        "check_fail",
        "fix",
        "config_valid",
        "config_invalid",
        "edge",
        "manual_required",
    ] {
        assert!(
            json["rules"][0].get(key).is_some(),
            "missing schema key {key}"
        );
    }
}

#[test]
fn fixture_matrix_reports_missing_stale_and_manual_required_items() {
    let catalog = UpstreamCatalog {
        source: "test".to_string(),
        rules: vec![UpstreamDocumentService::parse_rule_document(
            r#"# `MD001` - Heading increment

Tags: `headings`

Aliases: `heading-increment`
"#,
        )
        .expect("document should parse")],
    };
    let matrix = RuleFixtureMatrix {
        upstream_source: "test".to_string(),
        summary: RuleFixtureMatrixSummary::default(),
        rules: vec![RuleFixtureEntry {
            rule_id: "MD999".to_string(),
            aliases: Vec::new(),
            tags: Vec::new(),
            parameters: Vec::new(),
            fixable: None,
            check_pass: Vec::new(),
            check_fail: Vec::new(),
            fix: Vec::new(),
            config_valid: Vec::new(),
            config_invalid: Vec::new(),
            edge: Vec::new(),
            manual_required: vec!["review needed".to_string()],
        }],
    };

    let drift = compare::compare_fixture_matrix(&catalog, &matrix);

    assert_eq!(drift.summary.missing_fixtures, 1);
    assert_eq!(drift.summary.stale_fixtures, 1);
    assert_eq!(drift.summary.manual_required, 1);
    assert!(drift
        .items
        .iter()
        .any(|item| item.drift_type == RuleFixtureDriftType::MissingFixture));
    assert!(drift
        .items
        .iter()
        .any(|item| item.drift_type == RuleFixtureDriftType::StaleFixture));
}

#[test]
fn renders_and_writes_fixture_matrix_reports() {
    let catalog = UpstreamCatalog {
        source: "test".to_string(),
        rules: vec![UpstreamDocumentService::parse_rule_document(
            r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

```markdown
bad
```
"#,
        )
        .expect("document should parse")],
    };
    let matrix = build::build_fixture_matrix(&catalog);
    let markdown = report::render_fixture_matrix_summary(&matrix);
    let dir = std::env::temp_dir().join(format!(
        "katana-markdown-linter-fixture-matrix-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);

    report::write_fixture_matrix_files(&matrix, &dir).expect("matrix files should be written");

    assert!(markdown.contains("Rule Fixture Matrix"));
    assert!(markdown.contains("MD999"));
    assert!(dir.join("rule-fixture-matrix.json").exists());
    assert!(dir.join("rule-fixture-matrix.md").exists());
    let _ = fs::remove_dir_all(dir);
}
