use katana_markdown_linter::rules::markdown::{LinkFragmentsRule, MarkdownRule};
use katana_markdown_linter::RuleConfig;
use std::collections::HashMap;
use std::path::Path;

#[test]
fn fixes_case_mismatched_heading_fragment() {
    let rule = LinkFragmentsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "# Heading Name\n\n[Link](#Heading-Name)",
    );

    assert_eq!(diagnostics.len(), 1);
    let fix = diagnostics[0]
        .fix_info
        .as_ref()
        .expect("case mismatch should be fixable");
    assert_eq!(fix.replacement, "#heading-name");
}

#[test]
fn reports_missing_heading_fragment_without_fix() {
    let rule = LinkFragmentsRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "[Link](#missing)");

    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].fix_info.is_none());
}

#[test]
fn accepts_configured_ignore_case_fragment() {
    let rule = LinkFragmentsRule;
    let config = RuleConfig {
        enabled: true,
        properties: HashMap::from([("ignore_case".to_string(), "true".to_string())]),
    };
    let diagnostics = rule.evaluate_configured(
        Path::new("doc.md"),
        "# Heading Name\n\n[Link](#Heading-Name)",
        Some(&config),
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn accepts_configured_ignored_pattern() {
    let rule = LinkFragmentsRule;
    let config = RuleConfig {
        enabled: true,
        properties: HashMap::from([("ignored_pattern".to_string(), "^figure-".to_string())]),
    };
    let diagnostics =
        rule.evaluate_configured(Path::new("doc.md"), "[Figure](#figure-1)", Some(&config));

    assert!(diagnostics.is_empty());
}

#[test]
fn ignore_case_missing_fragment_remains_unfixable() {
    let rule = LinkFragmentsRule;
    let config = RuleConfig {
        enabled: true,
        properties: HashMap::from([("ignore_case".to_string(), "true".to_string())]),
    };
    let diagnostics = rule.evaluate_configured(
        Path::new("doc.md"),
        "# Heading Name\n\n[Link](#missing)",
        Some(&config),
    );

    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].fix_info.is_none());
}

#[test]
fn accepts_custom_heading_anchor_and_html_fragments() {
    let rule = LinkFragmentsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        concat!(
            "# Heading Name {#custom-name}\n",
            "<a id=\"bookmark\"></a>\n",
            "<a name='legacy'></a>\n\n",
            "[Custom](#custom-name)\n",
            "[Bookmark](#bookmark)\n",
            "[Legacy](#legacy)",
        ),
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn ignores_code_spans_and_unclosed_fragments() {
    let rule = LinkFragmentsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "`[Code](#missing)` and [Unclosed](#missing",
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn accepts_duplicate_heading_suffixes_and_line_fragments() {
    let rule = LinkFragmentsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "# Repeat\n# Repeat\n\n[Second](#repeat-1)\n[Line](#L19C5-L21C11)\n[Top](#top)",
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn rejects_invalid_custom_heading_anchor() {
    let rule = LinkFragmentsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "# Heading Name {#Invalid Anchor}\n\n[Custom](#Invalid-Anchor)",
    );

    assert_eq!(diagnostics.len(), 1);
}

#[test]
fn html_attribute_parser_skips_prefixed_and_unquoted_attributes() {
    let rule = LinkFragmentsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "<span data-id=\"skip\" id=unquoted id=\"bookmark\"></span>\n\n[Bookmark](#bookmark)",
    );

    assert!(diagnostics.is_empty());
}
