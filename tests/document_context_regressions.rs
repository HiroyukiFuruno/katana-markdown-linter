use katana_markdown_linter::rules::markdown::{
    DocumentContext, LinkFragmentsRule, MarkdownRule, NoBareUrlsRule, SourceRange,
};
use std::path::Path;

#[test]
fn inline_code_lookup_uses_the_containing_span_only() {
    let content = "`one` text ``two`` text\n";
    let ctx = DocumentContext::new(Path::new("doc.md"), content);

    let one_start = content.find("one").expect("one span should exist");
    let text_start = content.find("text").expect("plain text should exist");
    let two_start = content.find("two").expect("two span should exist");

    assert!(ctx.is_inside_inline_code(SourceRange {
        start: one_start,
        end: one_start + "one".len(),
    }));
    assert!(!ctx.is_inside_inline_code(SourceRange {
        start: text_start,
        end: text_start + "text".len(),
    }));
    assert!(ctx.is_inside_inline_code(SourceRange {
        start: two_start,
        end: two_start + "two".len(),
    }));
}

#[test]
fn inline_html_context_extracts_nested_quoted_attributes() {
    let content = concat!(
        "<span data-id=\"outer\"><a id=\"target\" href=\"https://example.com?q=>\">x</a></span>\n",
        "`<a id=\"code\"></a>`\n",
    );
    let ctx = DocumentContext::new(Path::new("doc.md"), content);
    let opening_elements = ctx
        .inline_html_elements()
        .iter()
        .filter(|element| !element.closing)
        .collect::<Vec<_>>();

    assert_eq!(opening_elements.len(), 2);
    assert_eq!(opening_elements[0].name, "span");
    assert_eq!(
        opening_elements[0].attribute_value("data-id"),
        Some("outer")
    );
    assert_eq!(opening_elements[1].name, "a");
    assert_eq!(opening_elements[1].attribute_value("id"), Some("target"));
    assert_eq!(
        opening_elements[1].attribute_value("href"),
        Some("https://example.com?q=>")
    );

    let code_attribute = ctx
        .inline_html_elements()
        .iter()
        .flat_map(|element| element.attributes.iter())
        .any(|attribute| attribute.value == Some("code"));
    assert!(!code_attribute);
}

#[test]
fn html_attribute_context_is_shared_by_url_and_fragment_rules() {
    let content = concat!(
        "<span><a id=\"target\" href=\"https://example.com?q=>\">x</a></span>\n",
        "[Target](#target)\n",
    );
    let md034 = NoBareUrlsRule;
    let md051 = LinkFragmentsRule;

    assert!(md034.evaluate(Path::new("doc.md"), content).is_empty());
    assert!(md051.evaluate(Path::new("doc.md"), content).is_empty());
}

#[test]
fn inline_html_context_does_not_parse_markdown_autolinks() {
    let content = "<https://example.com>\n<user@example.com>\n";
    let ctx = DocumentContext::new(Path::new("doc.md"), content);

    assert!(ctx.inline_html_elements().is_empty());
}
