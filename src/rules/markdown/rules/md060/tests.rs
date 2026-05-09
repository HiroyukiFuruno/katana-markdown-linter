use crate::{LintOptions, MarkdownLinter, RuleConfig};
use std::collections::HashMap;

fn md060_options(style: &str) -> LintOptions {
    let mut rules = HashMap::new();
    rules.insert(
        "MD060".to_string(),
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
fn accepts_official_table_styles() {
    for (style, content) in [
        ("aligned", "| A | B |\n|---|---|\n| C | D |\n"),
        ("compact", "| A | B |\n| --- | --- |\n| C | D |\n"),
        ("tight", "|A|B|\n|---|---|\n|C|D|\n"),
    ] {
        let results = MarkdownLinter::lint(content, &md060_options(style)).expect("lint runs");
        assert!(
            results.iter().all(|result| result.rule_id != "MD060"),
            "{style} should pass"
        );
    }
}

#[test]
fn fixes_table_style_from_context_range() {
    let content = "|A|B|\n|---|---|\n|C|D|\n";
    let options = md060_options("compact");
    let results = MarkdownLinter::lint(content, &options).expect("lint runs");
    let md060 = results
        .iter()
        .find(|result| result.rule_id == "MD060")
        .expect("MD060 diagnostic exists");

    assert!(md060.fix.is_some());
    let fixed = MarkdownLinter::fix_with_results(content, &results);
    assert_eq!(fixed.content, "| A | B |\n| --- | --- |\n| C | D |\n");
}
