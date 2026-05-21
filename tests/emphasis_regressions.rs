use katana_markdown_linter::{LintOptions, MarkdownLinter, RuleCatalogService, RuleConfig};
use std::collections::HashMap;

fn only_rule(rule_id: &str) -> LintOptions {
    let mut options = LintOptions::default();
    for rule in RuleCatalogService::implemented_rules() {
        options.rules.insert(
            rule.id.to_string(),
            RuleConfig {
                enabled: false,
                properties: HashMap::new(),
            },
        );
    }
    options.rules.insert(
        rule_id.to_string(),
        RuleConfig {
            enabled: true,
            properties: HashMap::new(),
        },
    );
    options
}

#[test]
fn md049_ignores_markers_that_start_and_end_in_separate_inline_code_spans() {
    let content = "Use `_ui.rs` for rendering and exclude it from coverage (`COVERAGE_IGNORE`).\n";
    let options = only_rule("MD049");

    let diagnostics = MarkdownLinter::lint(content, &options).expect("lint should run");
    let fixed = MarkdownLinter::fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}

#[test]
fn md050_ignores_markers_that_start_and_end_in_separate_inline_code_spans() {
    let content =
        "Use `__ui.rs` for rendering and exclude it from coverage (`COVERAGE__IGNORE`).\n";
    let options = only_rule("MD050");

    let diagnostics = MarkdownLinter::lint(content, &options).expect("lint should run");
    let fixed = MarkdownLinter::fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}

#[test]
fn md037_does_not_merge_separate_strong_spans_on_one_line() {
    let content = "**Note:** Neovim support is provided as a **docs-only sample**.\n";
    let options = only_rule("MD037");

    let diagnostics = MarkdownLinter::lint(content, &options).expect("lint should run");
    let fixed = MarkdownLinter::fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}

#[test]
fn md037_trims_spaced_emphasis_after_supported_punctuation() {
    let options = only_rule("MD037");

    for punctuation in ['.', '!', '?', ',', ';', ':'] {
        let content = format!("Hello{punctuation}* spaced *.\n");
        let expected = format!("Hello{punctuation}*spaced*.\n");
        let diagnostics = MarkdownLinter::lint(&content, &options).expect("lint should run");
        let fixed = MarkdownLinter::fix(&content, &options).expect("fix should run");

        assert_eq!(diagnostics.len(), 1, "punctuation {punctuation}");
        assert_eq!(fixed.content, expected, "punctuation {punctuation}");
    }
}

#[test]
fn md037_trims_spaced_emphasis_next_to_separate_strong_span() {
    let content = "**Note:** Neovim support is provided as * docs-only sample *.\n";
    let options = only_rule("MD037");

    let diagnostics = MarkdownLinter::lint(content, &options).expect("lint should run");
    let fixed = MarkdownLinter::fix(content, &options).expect("fix should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        fixed.content,
        "**Note:** Neovim support is provided as *docs-only sample*.\n"
    );
}

#[test]
fn md037_trims_spaced_emphasis_after_matching_single_emphasis_span() {
    let content = "*Note:* Neovim support is provided as * docs-only sample *.\n";
    let options = only_rule("MD037");

    let diagnostics = MarkdownLinter::lint(content, &options).expect("lint should run");
    let fixed = MarkdownLinter::fix(content, &options).expect("fix should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        fixed.content,
        "*Note:* Neovim support is provided as *docs-only sample*.\n"
    );
}
