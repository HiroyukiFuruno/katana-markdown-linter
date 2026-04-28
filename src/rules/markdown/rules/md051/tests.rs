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

#[test]
fn no_false_positive_for_emoji_mixed_heading_link() {
    let rule = LinkFragmentsRule;
    let content = "# Hello 🎉 World\n\n[Link](#hello-world)";
    assert!(
        rule.evaluate(Path::new("doc.md"), content).is_empty(),
        "emoji-mixed heading should produce fragment 'hello-world'"
    );
}

#[test]
fn false_positive_when_emoji_wrongly_included_in_fragment() {
    let rule = LinkFragmentsRule;
    let content = "# Hello 🎉 World\n\n[Bad link](#hello--world)";
    assert_eq!(
        rule.evaluate(Path::new("doc.md"), content).len(),
        1,
        "double dash fragment does not match 'hello-world'"
    );
}

#[test]
fn no_false_positive_for_cjk_heading_link() {
    let rule = LinkFragmentsRule;
    let content = "# 中文标题\n\n[Link](#中文标题)";
    assert!(
        rule.evaluate(Path::new("doc.md"), content).is_empty(),
        "CJK heading should produce fragment '中文标题'"
    );
}

#[test]
fn emoji_only_heading_produces_no_usable_fragment() {
    let rule = LinkFragmentsRule;
    let content = "# 🎉\n\n[Link](#🎉)";
    assert_eq!(
        rule.evaluate(Path::new("doc.md"), content).len(),
        1,
        "emoji-only heading produces empty slug; fragment '#🎉' is unknown"
    );
}
