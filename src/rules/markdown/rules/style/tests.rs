use crate::{LintOptions, MarkdownLinter, RuleConfig};
use std::collections::HashMap;

fn md035_options(style: &str) -> LintOptions {
    let mut rules = HashMap::new();
    rules.insert(
        "MD035".to_string(),
        RuleConfig {
            enabled: true,
            properties: HashMap::from([("style".to_string(), style.to_string())]),
        },
    );
    LintOptions {
        rules,
        ..LintOptions::default()
    }
}

#[test]
fn fixes_consistent_horizontal_rule_style() {
    let content = "---\n\n  * * *\n";
    let results = MarkdownLinter::lint(content, &LintOptions::default()).expect("lint runs");
    let md035 = results
        .iter()
        .find(|result| result.rule_id == "MD035")
        .expect("MD035 diagnostic exists");

    assert!(md035.fix.is_some());
    let fixed = MarkdownLinter::fix_with_results(content, &results);
    assert_eq!(fixed.content, "---\n\n  ---\n");
}

#[test]
fn fixes_configured_horizontal_rule_style() {
    let content = "---\n";
    let results = MarkdownLinter::lint(content, &md035_options("***")).expect("lint runs");
    let fixed = MarkdownLinter::fix_with_results(content, &results);

    assert_eq!(fixed.content, "***\n");
}

#[test]
fn front_matter_delimiter_does_not_seed_horizontal_rule_style() {
    let content = "---\ntitle: Doc\n---\n\n***\n\n***\n";
    let results = MarkdownLinter::lint(content, &LintOptions::default()).expect("lint runs");

    assert!(results.iter().all(|result| result.rule_id != "MD035"));
}

#[test]
fn md036_exposes_unsafe_heading_fix_without_default_application() {
    let content = "**Important**\n\nText\n";
    let results = MarkdownLinter::lint(content, &LintOptions::default()).expect("lint runs");
    let md036 = results
        .iter()
        .find(|result| result.rule_id == "MD036")
        .expect("MD036 diagnostic exists");

    assert_eq!(
        md036.fix.as_ref().map(|fix| fix.safety),
        Some(crate::FixSafety::Unsafe)
    );
    let safe_fixed = MarkdownLinter::fix_with_results(content, &results);
    assert_eq!(safe_fixed.content, content);
    let unsafe_fixed = crate::MarkdownLinter::fix_with_results_including_unsafe(content, &results);
    assert_eq!(unsafe_fixed.content, "# Important\n\nText\n");
}
