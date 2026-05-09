use katana_markdown_linter::{LintOptions, MarkdownLinter, RuleCatalogService, RuleConfig};
use std::collections::HashMap;

fn md036_options() -> LintOptions {
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
        "MD036".to_string(),
        RuleConfig {
            enabled: true,
            properties: HashMap::new(),
        },
    );
    options
}

#[test]
fn ignores_emphasized_labels_that_end_with_default_punctuation() {
    let content = "**Goals:**\n\n- item\n\n**重要：**\n\n本文\n";
    let diagnostics = MarkdownLinter::lint(content, &md036_options()).expect("lint should run");

    assert!(diagnostics.is_empty(), "MD036 diagnostics: {diagnostics:?}");
}

#[test]
fn configured_punctuation_controls_emphasis_heading_detection() {
    let mut options = md036_options();
    options
        .rules
        .get_mut("MD036")
        .expect("MD036 should be enabled")
        .properties
        .insert("punctuation".to_string(), String::new());

    let diagnostics =
        MarkdownLinter::lint("**Goals:**\n\n- item\n", &options).expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "MD036");
}
