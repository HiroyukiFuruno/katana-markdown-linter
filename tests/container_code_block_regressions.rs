use katana_markdown_linter::{fix, implemented_rules, lint, LintOptions, RuleConfig};
use std::collections::HashMap;

fn only_rule(rule_id: &str) -> LintOptions {
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
            properties: HashMap::new(),
        },
    );
    options
}

#[test]
fn md027_ignores_code_lines_inside_blockquote_fences() {
    let content = "\
> ```rust
> fn main() {
>     println!(\"Hello\");
> }
> ```
";
    let options = only_rule("MD027");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty(), "diagnostics: {diagnostics:?}");
    assert_eq!(fixed.content, content);
}

#[test]
fn md040_recognizes_fences_opened_after_list_markers() {
    let content = "\
- ```rs
  let x = 3.14;
  ```
- Code blocks can be in lists too
";
    let options = only_rule("MD040");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty(), "diagnostics: {diagnostics:?}");
    assert_eq!(fixed.content, content);
}

#[test]
fn md031_treats_blockquote_marker_lines_as_blank_lines() {
    let content = "\
> Quote with a fenced block
>
> ```rust
> let quoted_code = true;
> ```
";
    let options = only_rule("MD031");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty(), "diagnostics: {diagnostics:?}");
    assert_eq!(fixed.content, content);
}

#[test]
fn md046_keeps_definition_list_continuation_paragraphs() {
    let content = "\
:   Definition 2

    ```rs
    let x = 3
    ```

    Third paragraph of definition 2.
";
    let options = only_rule("MD046");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty(), "diagnostics: {diagnostics:?}");
    assert_eq!(fixed.content, content);
}

#[test]
fn md046_ignores_nested_list_continuation_prose() {
    let content = "\
```bash
git status
```

1. Pre-flight check:
   - **Standard Case**:
     The integration branch (e.g., `<change-name>`) MUST have already been successfully merged into `master`.
     - Run `gh pr list --state merged --head <change-name>` to confirm.
";
    let options = only_rule("MD046");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty(), "diagnostics: {diagnostics:?}");
    assert_eq!(fixed.content, content);
}

#[test]
fn md046_ignores_ordered_list_continuation_prose() {
    let content = "\
```bash
git status
```

1. 承認後、`gh pr merge --merge --delete-branch` で PR をマージする。
   これにより CD ワークフローが発火し、配布物が公開される。

2. `branch-hygiene` スキルを使用する。
   タスクブランチが残っている場合は、未コミット差分と未統合状態を確認した上で削除する。
";
    let options = only_rule("MD046");

    let diagnostics = lint(content, &options).expect("lint should run");
    let fixed = fix(content, &options).expect("fix should run");

    assert!(diagnostics.is_empty(), "diagnostics: {diagnostics:?}");
    assert_eq!(fixed.content, content);
}
