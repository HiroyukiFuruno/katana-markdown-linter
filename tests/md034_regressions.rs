use katana_markdown_linter::rules::markdown::{MarkdownRule, NoBareUrlsRule};
use std::path::Path;

#[test]
fn fixes_bare_url_with_angle_brackets() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "See http://example.com now.");
    let fix = diagnostics[0]
        .fix_info
        .as_ref()
        .expect("url should be fixable");
    assert_eq!(fix.replacement, "<http://example.com>");
}

#[test]
fn reports_each_bare_url_on_the_same_line() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "See https://example.com and https://example.org.",
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("first URL should be fixable")
            .replacement,
        "<https://example.com>"
    );
    assert_eq!(
        diagnostics[1]
            .fix_info
            .as_ref()
            .expect("second URL should be fixable")
            .replacement,
        "<https://example.org>"
    );
}

#[test]
fn excludes_closing_parenthesis_from_bare_url_fix() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "See (https://example.com).");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].range.start_column, 6);
    assert_eq!(diagnostics[0].range.end_column, 25);
    assert_eq!(
        diagnostics[0]
            .fix_info
            .as_ref()
            .expect("URL should be fixable")
            .replacement,
        "<https://example.com>"
    );
}

#[test]
fn ignores_url_inside_inline_link_destination() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "[Example](https://example.com)");

    assert!(diagnostics.is_empty());
}

#[test]
fn ignores_url_inside_reversed_link_destination() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "(Example)[https://example.com]");

    assert!(diagnostics.is_empty());
}

#[test]
fn ignores_url_inside_html_attribute_values() {
    let rule = NoBareUrlsRule;
    let content = concat!(
        "<p align=\"center\">\n",
        "  <a href=\"#\"><img src=\"https://img.shields.io/badge/license-MIT-blue.svg\" ",
        "alt=\"License: MIT\"></a>\n",
        "  <img src=\"data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22%3E\" ",
        "alt=\"inline\">\n",
        "</p>\n",
    );
    let diagnostics = rule.evaluate(Path::new("doc.md"), content);

    assert!(diagnostics.is_empty());
}

#[test]
fn ignores_urls_inside_code_spans_and_fenced_code() {
    let rule = NoBareUrlsRule;
    let content = concat!(
        "`https://example.com`\n",
        "```\n",
        "https://example.org\n",
        "```\n",
    );
    let diagnostics = rule.evaluate(Path::new("doc.md"), content);

    assert!(diagnostics.is_empty());
}

#[test]
fn detects_bare_url_after_html_attribute_value() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(
        Path::new("doc.md"),
        "<img src=\"https://example.com/image.png\"> See https://example.com.",
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].range.start_column, 47);
}

#[test]
fn ignores_url_inside_link_reference_definition() {
    let rule = NoBareUrlsRule;
    let diagnostics = rule.evaluate(Path::new("doc.md"), "[normal]: https://github.com\n");

    assert!(diagnostics.is_empty());
}

#[test]
fn ignores_urls_inside_nested_links_images_titles_and_code_spans() {
    let rule = NoBareUrlsRule;
    let content = concat!(
        "[nested [text]](https://example.com/path?q=1 \"title\")\n",
        "![alt][image-ref]\n",
        "[image-ref]: <https://example.org/image.png> \"Image\"\n",
        "``https://example.invalid``\n",
    );
    let diagnostics = rule.evaluate(Path::new("doc.md"), content);

    assert!(diagnostics.is_empty());
}
