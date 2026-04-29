use katana_markdown_linter::{fix, implemented_rules, lint, LintOptions, RuleConfig};
use std::collections::HashMap;

fn md052_options() -> LintOptions {
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
    options.rules.insert(
        "MD052".to_string(),
        RuleConfig {
            enabled: true,
            properties: HashMap::new(),
        },
    );
    options
}

#[test]
fn reports_missing_full_and_collapsed_reference_definitions() {
    let content = concat!("[text][missing]\n", "![alt][missing-image]\n", "[ref][]\n");
    let diagnostics = lint(content, &md052_options()).expect("lint should run");

    let locations = diagnostics
        .iter()
        .map(|diagnostic| (diagnostic.rule_id.as_str(), diagnostic.line))
        .collect::<Vec<_>>();

    assert_eq!(locations, vec![("MD052", 1), ("MD052", 2), ("MD052", 3)]);
}

#[test]
fn ignores_defined_reference_labels() {
    let content = concat!(
        "[text][target]\n",
        "![alt][image-target]\n",
        "[ref][]\n",
        "\n",
        "[target]: https://example.com\n",
        "[image-target]: https://example.com/image.png\n",
        "[ref]: https://example.com/ref\n",
    );
    let diagnostics = lint(content, &md052_options()).expect("lint should run");

    assert!(diagnostics.is_empty(), "MD052 diagnostics: {diagnostics:?}");
}

#[test]
fn reports_shortcut_syntax_only_when_enabled() {
    let mut options = md052_options();
    options
        .rules
        .get_mut("MD052")
        .expect("MD052 should be enabled")
        .properties
        .insert("shortcut_syntax".to_string(), "true".to_string());

    let diagnostics = lint("## [release] - released\n", &options).expect("lint should run");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "MD052");

    let diagnostics = lint("## [0.22.8] - released\n", &md052_options()).expect("lint should run");
    assert!(diagnostics.is_empty(), "MD052 diagnostics: {diagnostics:?}");
}

#[test]
fn ignores_changelog_version_headings_with_shortcut_syntax() {
    let mut options = md052_options();
    options
        .rules
        .get_mut("MD052")
        .expect("MD052 should be enabled")
        .properties
        .insert("shortcut_syntax".to_string(), "true".to_string());

    let content = concat!(
        "## [0.1.2] - 2026-03-20 01:54:57 (JST)\n",
        "### [v1.2.3-beta.1] - released\n",
    );
    let diagnostics = lint(content, &options).expect("lint should run");

    assert!(diagnostics.is_empty(), "MD052 diagnostics: {diagnostics:?}");
}

#[test]
fn does_not_double_report_full_reference_when_shortcut_syntax_is_enabled() {
    let mut options = md052_options();
    options
        .rules
        .get_mut("MD052")
        .expect("MD052 should be enabled")
        .properties
        .insert("shortcut_syntax".to_string(), "true".to_string());

    let diagnostics = lint("[text][missing]\n", &options).expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "MD052");
    assert_eq!(diagnostics[0].line, 1);
}

#[test]
fn ignores_reference_like_text_inside_indented_code_blocks() {
    let mut options = md052_options();
    options
        .rules
        .get_mut("MD052")
        .expect("MD052 should be enabled")
        .properties
        .insert("shortcut_syntax".to_string(), "true".to_string());

    let content = concat!(
        "Example:\n",
        "\n",
        "    [mcp_servers.kml]\n",
        "    args = [\"--workspace-root\", \"/workspace\"]\n",
        "    [ref][missing]\n",
        "    [ref][]\n",
        "\n",
        "[text][missing]\n",
    );

    let diagnostics = lint(content, &options).expect("lint should run");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "MD052");
    assert_eq!(diagnostics[0].line, 8);
}

#[test]
fn respects_ignored_labels_for_shortcut_syntax() {
    let mut options = md052_options();
    let config = options
        .rules
        .get_mut("MD052")
        .expect("MD052 should be enabled");
    config
        .properties
        .insert("shortcut_syntax".to_string(), "true".to_string());
    config.properties.insert(
        "ignored_labels".to_string(),
        "[\"x\", \"!tip\"]".to_string(),
    );

    let diagnostics = lint("> [!TIP]\n> text\n- [x] done\n", &options).expect("lint should run");

    assert!(diagnostics.is_empty(), "MD052 diagnostics: {diagnostics:?}");
}

#[test]
fn does_not_apply_unsafe_missing_reference_fix() {
    let result = fix("[ref][]\n", &md052_options()).expect("fix should run");

    assert_eq!(result.content, "[ref][]\n");
    assert_eq!(result.applied_fixes, 0);
}
