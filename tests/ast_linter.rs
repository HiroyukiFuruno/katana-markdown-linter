#[test]
fn ast_linter_parses_rule_doc_fixture() {
    let document = katana_markdown_linter::upstream::parse_rule_document(
        r#"# `MD001` - Heading levels should only increment by one level at a time

Tags: `headings`

Aliases: `heading-increment`

Parameters:

- `front_matter_title`: RegExp for matching title in front matter (`string`,
  default `^\s*title\s*[:=]`)
"#,
    )
    .expect("fixture should parse");

    assert_eq!(document.id, "MD001");
    assert_eq!(document.name, "heading-increment");
    assert_eq!(document.properties[0].key, "front_matter_title");
}
