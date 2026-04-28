use katana_markdown_linter::{fix, implemented_rules, lint, LintOptions, RuleConfig};
use std::collections::HashMap;

fn only_rule(rule_id: &str) -> LintOptions {
    only_rule_with_properties(rule_id, HashMap::new())
}

fn only_rule_with_properties(rule_id: &str, properties: HashMap<String, String>) -> LintOptions {
    let mut options = LintOptions::default();
    for rule in implemented_rules() {
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
            properties,
        },
    );
    options
}

#[test]
fn md029_keeps_ordered_list_numbering_across_indented_code_blocks() {
    let content = "\
1. First step:

   ```sh
   cargo build --release
   ```

2. Second step:

   ```sh
   ./target/release/KatanA
   ```

3. Verify:
   - Sub-item A
   - Sub-item B
";
    let options = only_rule("MD029");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}

#[test]
fn md029_reports_broken_nested_numbering_at_the_same_level() {
    let content = "\
1. First item
2. Second item
   1. Nested 2-1
   3. Nested 2-2
3. Third item
";
    let diagnostics = lint(content, &only_rule("MD029")).expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, 4);
    assert_eq!(
        diagnostics[0]
            .fix
            .as_ref()
            .expect("fix should exist")
            .replacement,
        "2"
    );
}

#[test]
fn md029_default_accepts_all_one_ordered_list_style() {
    let content = "\
1. First
1. Second
1. Third
";
    let options = only_rule("MD029");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}

#[test]
fn md029_default_accepts_zero_based_ordered_list_style() {
    let content = "\
0. Prepare
1. Apply
2. Verify
";
    let options = only_rule("MD029");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}

#[test]
fn md029_keeps_ordered_numbering_after_lazy_continuation_lines() {
    let content = "\
1.
    1. First item text wraps
    onto the next source line
    2. Second item keeps its number
";
    let options = only_rule("MD029");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}

#[test]
fn md029_default_does_not_continue_numbering_across_section_boundaries() {
    let content = "\
1. Open
2. Confirm

---

## New section

1. First
1. Second
1. Third
";
    let options = only_rule("MD029");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}

#[test]
fn md029_configured_ordered_rewrites_all_one_ordered_list_style() {
    let content = "\
1. First
1. Second
1. Third
";
    let options = only_rule_with_properties(
        "MD029",
        HashMap::from([("style".to_string(), "ordered".to_string())]),
    );
    let diagnostics = lint(content, &options).expect("lint should run");

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(
        diagnostics[0]
            .fix
            .as_ref()
            .expect("fix should exist")
            .replacement,
        "2"
    );
}

#[test]
fn md007_accepts_unordered_sublist_aligned_to_ordered_item_content() {
    let content = "\
1. Verify:
   - Sub-item A
   - Sub-item B
";
    let options = only_rule("MD007");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty());
    assert_eq!(fixed.content, content);
}
