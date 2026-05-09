use super::*;

#[test]
fn fixes_strong_to_first_style() {
    let rule = StrongStyleRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "**Text** and __more__");

    assert_eq!(diagnostics.len(), 1);
    let fix = diagnostics[0]
        .fix_info
        .as_ref()
        .expect("strong style should be fixable");
    assert_eq!(fix.replacement, "**more**");
}

#[test]
fn ignores_intraword_underscores() {
    let rule = StrongStyleRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "like__this__one");

    assert!(diagnostics.is_empty());
}

#[test]
fn fixes_strong_to_configured_style() {
    let rule = StrongStyleRule;
    let config = RuleConfig {
        enabled: true,
        properties: [("style".to_string(), "underscore".to_string())]
            .into_iter()
            .collect(),
    };
    let diagnostics = rule.evaluate_configured(Path::new("doc.md"), "**Text**", Some(&config));

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0].fix_info.as_ref().unwrap().replacement,
        "__Text__"
    );
}

#[test]
fn ignores_strong_inside_fenced_code() {
    let rule = StrongStyleRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "```\n**one** and __two__\n```\n");

    assert!(diagnostics.is_empty());
}

#[test]
fn ignores_strong_inside_long_and_unclosed_code_spans() {
    let rule = StrongStyleRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "**one** ``__two__``\n`__three__\n");

    assert!(diagnostics.is_empty());
}
