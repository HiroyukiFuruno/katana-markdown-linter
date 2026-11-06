use katana_markdown_linter::{LintOptions, MarkdownLinter, RuleCatalogService, RuleConfig};
use std::collections::HashMap;

fn only_rule(id: &str) -> LintOptions {
    let mut options = LintOptions::default();
    for rule in RuleCatalogService::implemented_rules() {
        options.rules.insert(
            rule.id.to_string(),
            RuleConfig {
                enabled: rule.id == id,
                properties: HashMap::new(),
            },
        );
    }
    options
}

#[test]
fn html_attributes_are_protected_and_outside_markdown_is_fixed() {
    for (id, input, expected) in [
        ("MD037", "<span title=\"* text *\">ok</span> * text *\n", "<span title=\"* text *\">ok</span> *text*\n"),
        ("MD011", "<span title=\"(Example)[https://example.com]\">ok</span> (Example)[https://example.com]\n", "<span title=\"(Example)[https://example.com]\">ok</span> [Example](https://example.com)\n"),
        ("MD039", "<span title=\"[ text ](u)\">ok</span> [ text ](u)\n", "<span title=\"[ text ](u)\">ok</span> [text](u)\n"),
    ] {
        let output = MarkdownLinter::fix(input, &only_rule(id)).unwrap();
        assert_eq!(output.content, expected, "{id}");
    }
}

#[test]
fn html_is_reported_by_md033_but_references_only_outside_tags_are_reported() {
    let input = "<span title=\"[missing][ref]\">ok</span> [missing][ref]\n";
    assert_eq!(
        MarkdownLinter::lint(input, &only_rule("MD052"))
            .unwrap()
            .len(),
        1
    );
    assert!(!MarkdownLinter::lint(input, &only_rule("MD033"))
        .unwrap()
        .is_empty());
}

#[test]
fn additional_scanners_ignore_attributes_and_keep_body_diagnostics() {
    for (id, input) in [
        (
            "MD034",
            "<span title=\"https://example.com\">ok</span> https://example.com\n",
        ),
        ("MD045", "<span title=\"![](x)\">ok</span> ![](x)\n"),
    ] {
        assert_eq!(
            MarkdownLinter::lint(input, &only_rule(id)).unwrap().len(),
            1,
            "{id}"
        );
    }
}

#[test]
fn proper_names_preserve_attributes_and_fix_body() {
    let mut options = only_rule("MD044");
    options
        .rules
        .get_mut("MD044")
        .unwrap()
        .properties
        .insert("names".into(), r#"["GitHub"]"#.into());
    let input = "<span title=\"github\">github</span>\n";
    let output = MarkdownLinter::fix(input, &options).unwrap();
    assert_eq!(output.content, "<span title=\"github\">GitHub</span>\n");
}

#[test]
fn html_blocks_are_protected_without_suppressing_later_paragraphs() {
    let input = "<div>\n* text *\n</div>\n\n* text *\n";
    let output = MarkdownLinter::fix(input, &only_rule("MD037")).unwrap();
    assert_eq!(output.content, "<div>\n* text *\n</div>\n\n*text*\n");

    let input = "<script>\n#literal\n</script>\n\n#Heading\n";
    let output = MarkdownLinter::fix(input, &only_rule("MD018")).unwrap();
    assert_eq!(
        output.content,
        "<script>\n#literal\n</script>\n\n# Heading\n"
    );
}

#[test]
fn a_markdown_link_can_contain_raw_html_in_its_label() {
    let input = "[ <b>text</b> ](u)\n";
    let output = MarkdownLinter::fix(input, &only_rule("MD039")).unwrap();
    assert_eq!(output.content, "[<b>text</b>](u)\n");
}

#[test]
fn backtick_in_an_html_attribute_does_not_mask_later_markdown() {
    let input = "<span title=\"`\">ok</span> [ text ](u)\n";
    let output = MarkdownLinter::fix(input, &only_rule("MD039")).unwrap();
    assert_eq!(output.content, "<span title=\"`\">ok</span> [text](u)\n");
}
