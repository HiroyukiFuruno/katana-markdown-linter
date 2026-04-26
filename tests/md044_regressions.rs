use katana_markdown_linter::{implemented_rules, lint, LintOptions, RuleConfig};
use std::collections::HashMap;

fn md044_options(names: Option<&str>, code_blocks: Option<bool>) -> LintOptions {
    let mut options = LintOptions::default();
    for rule in implemented_rules() {
        options.rules.insert(
            rule.id,
            RuleConfig {
                enabled: false,
                properties: HashMap::new(),
            },
        );
    }

    let mut properties = HashMap::new();
    if let Some(names) = names {
        properties.insert("names".to_string(), names.to_string());
    }
    if let Some(code_blocks) = code_blocks {
        properties.insert("code_blocks".to_string(), code_blocks.to_string());
    }
    options.rules.insert(
        "MD044".to_string(),
        RuleConfig {
            enabled: true,
            properties,
        },
    );
    options
}

#[test]
fn fixes_known_proper_name_capitalization() {
    let diagnostics = lint(
        "markdownlint and github",
        &md044_options(Some("[\"Markdownlint\",\"GitHub\"]"), None),
    )
    .expect("lint should run");

    assert_eq!(diagnostics.len(), 2);
    let replacements = diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.fix.as_ref())
        .map(|fix| fix.replacement.as_str())
        .collect::<Vec<_>>();
    assert_eq!(replacements, ["Markdownlint", "GitHub"]);
}

#[test]
fn can_ignore_code_blocks_and_code_spans() {
    let diagnostics = lint(
        "```\ngithub\n```\n`github`\n",
        &md044_options(Some("[\"GitHub\"]"), Some(false)),
    )
    .expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn configured_code_blocks_true_keeps_code_span_behavior() {
    let diagnostics = lint(
        "`github`\n",
        &md044_options(Some("[\"GitHub\"]"), Some(true)),
    )
    .expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
}

#[test]
fn code_blocks_false_ignores_long_and_unclosed_code_spans() {
    let diagnostics = lint(
        "``github``\n`github\n",
        &md044_options(Some("[\"GitHub\"]"), Some(false)),
    )
    .expect("lint should run");

    assert!(diagnostics.is_empty());
}

#[test]
fn default_config_does_not_guess_proper_names() {
    let diagnostics =
        lint("markdownlint and github", &md044_options(None, None)).expect("lint should run");

    assert!(diagnostics.is_empty());
}
