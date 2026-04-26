use katana_markdown_linter::{implemented_rules, lint, LintOptions, RuleConfig};
use std::collections::HashMap;

const CONTEXT_SENSITIVE_RULES: &[&str] = &[
    "MD009", "MD011", "MD014", "MD029", "MD034", "MD039", "MD046", "MD049", "MD050", "MD051",
    "MD052", "MD053", "MD054", "MD055", "MD056", "MD058", "MD059", "MD037", "MD038",
];

fn only_rules(rule_ids: &[&str]) -> LintOptions {
    let mut options = LintOptions::default();
    options.rules.insert(
        "md-broken-link".to_string(),
        RuleConfig {
            enabled: false,
            properties: HashMap::new(),
        },
    );

    for rule in implemented_rules() {
        options.rules.insert(
            rule.id,
            RuleConfig {
                enabled: false,
                properties: HashMap::new(),
            },
        );
    }

    for rule_id in rule_ids {
        options.rules.insert(
            (*rule_id).to_string(),
            RuleConfig {
                enabled: true,
                properties: HashMap::new(),
            },
        );
    }

    options
}

fn only_rule(rule_id: &str) -> LintOptions {
    only_rules(&[rule_id])
}

#[test]
fn mixed_markdown_document_does_not_trigger_context_false_positives() {
    let mut options = only_rules(CONTEXT_SENSITIVE_RULES);
    options
        .rules
        .get_mut("MD054")
        .expect("MD054 is enabled")
        .properties
        .insert("collapsed".to_string(), "false".to_string());

    let content = concat!(
        r###"# KatanA rendering fixtures

This document mixes common authoring patterns that should be parsed by context.

<p align="center">
  <a href="sample_diagrams.md">English</a> | 日本語
  <a href="#"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="#"><img src="https://img.shields.io/badge/CI-passing-brightgreen.svg" alt="CI"></a>
</p>

See [Normal link](https://github.com) and [Email link](mailto:test@example.com).
See [nested [text]](https://example.com/path?q=1 "title") and ![badge][badge-image].
The [reference text][normal] and ![image alt][image-target] are full reference forms.

`https://github.com` and `$echo hi` are inline code examples.
`[ link ](https://github.com)` is also inline code.
``[link](https://github.com)`` is a longer inline code span.
`(Example)[https://github.com]` is inline code, not a reversed link.
`*one* and _two_` plus `**one** and __two__` are inline code examples.
`* spaced *` and ``__ spaced __`` stay inline code examples.

$$ \sum {k=1}^{n} k = \frac{n(n+1)}{2} $$

---

1. First item
2. Second item
   1. Nested 2-1
   2. Nested 2-2
3. Third item

| Name | Value |
| --- | --- |
| Mermaid | Enabled |

````markdown
```mermaid
graph TD
    A[Start] --> B{Decision}
    B -->|Yes| C[Path A]
    B -->|No| D[Path B]
```

$echo hi
https://example.com
[ link ](https://example.com)
(Example)[https://example.com]
[ref][]
[ref]: https://example.com
[ref]: https://example.org
*one* and _two_
**one** and __two__
| looks | like |
| --- | --- |
````

~~~mermaid
graph TD
    A[Start] --> B{Decision}
    B -->|Yes| C[Path A]
    B -->|No| D[Path B]
~~~

[normal]: https://github.com
[badge-image]: <https://img.shields.io/badge/example-passing-green.svg> "Badge"
[image-target]: <https://example.com/image.png> "Image"
"###,
        "```\n",
        "code keeps trailing spaces  \n",
        "```\n",
        "~~~\n",
        "tilde code keeps trailing spaces  \n",
        "~~~\n",
    );

    let diagnostics = lint(content, &options).expect("lint should run");
    let reported = diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "{}@{}:{}",
                diagnostic.rule_id, diagnostic.line, diagnostic.column
            )
        })
        .collect::<Vec<_>>();

    assert!(reported.is_empty(), "unexpected diagnostics: {reported:?}");
}

#[test]
fn context_sensitive_rules_still_report_real_markdown_violations() {
    let cases = [
        ("MD011", "(Example)[https://example.com]\n"),
        ("MD014", "```\n$ ls\n$ cat foo\n```\n"),
        ("MD009", "text   \n"),
        ("MD033", "<span>html</span>\n"),
        (
            "MD034",
            "Visit https://example.com and https://example.org now.\n",
        ),
        ("MD037", "* text *\n"),
        ("MD038", "` code `\n"),
        ("MD039", "[ text ](https://example.com)\n"),
        ("MD049", "*one* and _two_\n"),
        ("MD050", "**one** and __two__\n"),
        ("MD052", "[ref][]\n"),
        (
            "MD053",
            "[ref]: https://example.com\n[ref]: https://example.org\n",
        ),
        ("MD059", "[link](https://example.com)\n"),
    ];

    for (rule_id, content) in cases {
        let diagnostics = lint(content, &only_rule(rule_id)).expect("lint should run");
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule_id == rule_id),
            "{rule_id} should report a real violation"
        );
    }

    let mut md054_options = only_rule("MD054");
    md054_options
        .rules
        .get_mut("MD054")
        .expect("MD054 is enabled")
        .properties
        .insert("collapsed".to_string(), "false".to_string());
    let diagnostics = lint(
        "[inline](target)\n[ref][]\n[ref]: https://example.com\n",
        &md054_options,
    )
    .expect("lint should run");

    assert!(diagnostics
        .iter()
        .any(|diagnostic| diagnostic.rule_id == "MD054"));
}

#[test]
fn md033_ignores_inline_html_inside_code_spans() {
    let diagnostics =
        lint("``<span>html</span>``\n`<img>\n", &only_rule("MD033")).expect("lint should run");

    assert!(diagnostics.is_empty(), "MD033 diagnostics: {diagnostics:?}");
}

#[test]
fn configured_context_exclusions_are_honored() {
    let mut md010_options = only_rule("MD010");
    md010_options
        .rules
        .get_mut("MD010")
        .expect("MD010 is enabled")
        .properties
        .insert("code_blocks".to_string(), "false".to_string());
    let diagnostics = lint("```\na\tb\n```\n`a\tb`\n", &md010_options).expect("lint should run");
    assert!(diagnostics.is_empty(), "MD010 diagnostics: {diagnostics:?}");

    let mut md013_options = only_rule("MD013");
    let md013_config = md013_options
        .rules
        .get_mut("MD013")
        .expect("MD013 is enabled");
    md013_config
        .properties
        .insert("code_blocks".to_string(), "false".to_string());
    md013_config
        .properties
        .insert("headings".to_string(), "false".to_string());
    md013_config
        .properties
        .insert("tables".to_string(), "false".to_string());
    let long_token = "a".repeat(120);
    let content =
        format!("# {long_token}\n\n| H |\n| --- |\n| {long_token} |\n\n```\n{long_token}\n```\n");
    let diagnostics = lint(&content, &md013_options).expect("lint should run");
    assert!(diagnostics.is_empty(), "MD013 diagnostics: {diagnostics:?}");

    let mut md044_options = only_rule("MD044");
    let md044_config = md044_options
        .rules
        .get_mut("MD044")
        .expect("MD044 is enabled");
    md044_config
        .properties
        .insert("names".to_string(), "[\"GitHub\"]".to_string());
    md044_config
        .properties
        .insert("code_blocks".to_string(), "false".to_string());
    let diagnostics =
        lint("```\ngithub\n```\n`github`\n", &md044_options).expect("lint should run");
    assert!(diagnostics.is_empty(), "MD044 diagnostics: {diagnostics:?}");
}
