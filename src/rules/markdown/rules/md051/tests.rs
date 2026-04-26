use super::*;
use std::collections::HashMap;

#[test]
fn fixes_case_mismatched_heading_fragment() {
    let rule = LinkFragmentsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "# Heading Name\n\n[Link](#Heading-Name)",
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("case mismatch should be fixable")
            .replacement,
        "#heading-name"
    );
}

#[test]
fn accepts_configured_and_special_fragments() {
    let rule = LinkFragmentsRule;
    let config = RuleConfig {
        enabled: true,
        properties: HashMap::from([("ignored_pattern".to_string(), "^figure-".to_string())]),
    };
    let content = concat!(
        "# Repeat\n",
        "# Repeat\n",
        "# Heading {#custom-name}\n",
        "<a id=\"bookmark\" name=\"legacy\"></a>\n\n",
        "[Second](#repeat-1)\n",
        "[Line](#L19C5-L21C11)\n",
        "[Top](#top)\n",
        "[Custom](#custom-name)\n",
        "[Bookmark](#bookmark)\n",
        "[Legacy](#legacy)\n",
        "[Figure](#figure-1)\n",
    );

    assert!(rule
        .evaluate_configured(Path::new("doc.md"), content, Some(&config))
        .is_empty());
}

#[test]
fn ignore_case_accepts_existing_fragment_but_missing_remains_unfixable() {
    let rule = LinkFragmentsRule;
    let config = RuleConfig {
        enabled: true,
        properties: HashMap::from([("ignore_case".to_string(), "true".to_string())]),
    };

    assert!(rule
        .evaluate_configured(
            Path::new("doc.md"),
            "# Heading Name\n\n[Link](#Heading-Name)",
            Some(&config),
        )
        .is_empty());

    let diagnostics =
        rule.evaluate_configured(Path::new("doc.md"), "[Missing](#missing)", Some(&config));
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].fix_info.is_none());
}
