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
fn md032_accepts_wrapped_list_item_before_next_item() {
    let content = "- First item wraps\n  onto the next source line\n- Second item\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD032")).expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn md032_accepts_lazy_continuation_before_next_item() {
    let content = "- First item wraps\nwithout indentation\n- Second item\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD032")).expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn md032_accepts_empty_ordered_item_before_nested_item() {
    let content = "1.\n    1. Nested\n    2. Nested sibling\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD032")).expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn md022_treats_html_comment_line_as_blank() {
    let content = "### New Capabilities\n<!-- none -->\n\n### Modified Capabilities\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD022")).expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn md022_ignores_front_matter_by_default() {
    let content = "---\ntitle: Doc\n---\n# Doc\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD022")).expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn md022_can_include_front_matter_when_enabled() {
    let mut options = only_rule("MD022");
    options
        .rules
        .get_mut("MD022")
        .expect("MD022 should be present")
        .properties
        .insert("include_front_matter".to_string(), "true".to_string());
    let content = "---\ntitle: Doc\n---\n# Doc\n";
    let diagnostics = MarkdownLinter::lint(content, &options).expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, 4);
}

#[test]
fn md007_reports_unordered_item_indented_too_shallow_for_ordered_parent() {
    let content = "1. Verify:\n  - Sub-item\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD007")).expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, 2);
}

#[test]
fn md007_fix_keeps_unordered_child_inside_ordered_parent() {
    let content = "1. Verify:\n  - Sub-item\n";
    let fixed = MarkdownLinter::fix(content, &only_rule("MD007")).expect("fix should run");

    assert_eq!(fixed.content, "1. Verify:\n   - Sub-item\n");
}

#[test]
fn md007_does_not_pull_nested_unordered_child_out_of_ordered_parent() {
    let content = "- Parent\n    1. Ordered\n        - Task\n";
    let fixed = MarkdownLinter::fix(content, &only_rule("MD007")).expect("fix should run");

    assert_eq!(fixed.content, content);
}

#[test]
fn md007_fix_updates_descendants_after_unordered_parent_indent_changes() {
    let content = "- Parent\n    - Child\n        - Grandchild\n";
    let fixed = MarkdownLinter::fix(content, &only_rule("MD007")).expect("fix should run");

    assert_eq!(fixed.content, "- Parent\n  - Child\n    - Grandchild\n");
}

#[test]
fn md007_drops_stale_ordered_parent_before_top_level_unordered_list() {
    let content =
        "- A\n    1. B\n        - Task\n    2. C\n- D\n    - Child\n        - Grandchild\n";
    let fixed = MarkdownLinter::fix(content, &only_rule("MD007")).expect("fix should run");

    assert_eq!(
        fixed.content,
        "- A\n    1. B\n        - Task\n    2. C\n- D\n  - Child\n    - Grandchild\n"
    );
}

#[test]
fn md007_reports_unordered_sublist_indented_too_deep() {
    let content = "- Parent\n    - Child\n        - Grandchild\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD007")).expect("lint should run");

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].line, 2);
    assert_eq!(diagnostics[1].line, 3);
}

#[test]
fn md012_reports_extra_trailing_blank_line() {
    let diagnostics =
        MarkdownLinter::lint("# Title\n\n", &only_rule("MD012")).expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, 3);
}

#[test]
fn md045_ignores_inline_code_that_looks_like_empty_alt_text() {
    let diagnostics = MarkdownLinter::lint("Use `vec![]` for values.\n", &only_rule("MD045"))
        .expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn md036_ignores_emphasis_with_inline_markdown_tokens() {
    let content =
        "**[Risk] `egui_kittest` cannot expose `Response.rect` directly**\n\n- Mitigation\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD036")).expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn md033_ignores_rust_generic_type_in_prose() {
    let content = "Avoid Box<dyn Any> in typed Rust APIs.\n";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD033")).expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn md060_accepts_aligned_table_using_emoji_visual_width() {
    let content = "\
| Feature | Status | Notes |
| --------- | -------- | ------- |
| Markdown | ✅ | Full support |
| Mermaid | ✅ | Requires mmdc |
";
    let diagnostics = MarkdownLinter::lint(content, &only_rule("MD060")).expect("lint should run");

    assert!(diagnostics.is_empty());
}
