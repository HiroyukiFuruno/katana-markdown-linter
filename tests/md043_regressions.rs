use katana_markdown_linter::{LintOptions, MarkdownLinter, RuleCatalogService, RuleConfig};
use std::collections::HashMap;

fn md043_options(properties: HashMap<String, String>) -> LintOptions {
    let mut options = LintOptions::default();
    for rule in RuleCatalogService::implemented_rules() {
        options.rules.insert(
            rule.id,
            RuleConfig {
                enabled: false,
                properties: HashMap::new(),
            },
        );
    }
    options.rules.insert(
        "MD043".to_string(),
        RuleConfig {
            enabled: true,
            properties,
        },
    );
    options
}

#[test]
fn default_required_headings_does_not_require_any_heading() {
    let diagnostics = MarkdownLinter::lint("No heading here.\n", &md043_options(HashMap::new()))
        .expect("lint should run");

    assert!(diagnostics.is_empty(), "MD043 diagnostics: {diagnostics:?}");
}

#[test]
fn explicit_empty_headings_rejects_document_heading() {
    let options = md043_options(HashMap::from([("headings".into(), "[]".into())]));
    let diagnostics = MarkdownLinter::lint("# Title\n", &options).expect("lint should run");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "MD043");
}

#[test]
fn explicit_empty_headings_accepts_document_without_heading() {
    let options = md043_options(HashMap::from([("headings".into(), "[]".into())]));
    assert!(MarkdownLinter::lint("Text\n", &options)
        .expect("lint should run")
        .is_empty());
}

#[test]
fn omitted_and_null_headings_allow_existing_headings() {
    for properties in [
        HashMap::new(),
        HashMap::from([("headings".into(), "null".into())]),
    ] {
        for text in ["# Title\n", "Text\n"] {
            let diagnostics = MarkdownLinter::lint(text, &md043_options(properties.clone()))
                .expect("lint should run");
            assert!(diagnostics.is_empty(), "MD043 diagnostics: {diagnostics:?}");
        }
    }
}

#[test]
fn configured_required_headings_report_mismatch() {
    let diagnostics = MarkdownLinter::lint(
        "# Other\n",
        &md043_options(HashMap::from([(
            "headings".to_string(),
            "[\"# Title\"]".to_string(),
        )])),
    )
    .expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "MD043");
}

#[test]
fn configured_required_headings_accept_matching_structure() {
    let diagnostics = MarkdownLinter::lint(
        "# Title\n\n## Details\n",
        &md043_options(HashMap::from([(
            "headings".to_string(),
            "[\"# Title\", \"## Details\"]".to_string(),
        )])),
    )
    .expect("lint should run");

    assert!(diagnostics.is_empty(), "MD043 diagnostics: {diagnostics:?}");
}
