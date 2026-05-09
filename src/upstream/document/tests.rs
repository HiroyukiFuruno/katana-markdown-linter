use super::parse;

#[test]
fn parses_rule_document_parameters() {
    let document = parse::parse_rule_document(
        r#"# `MD060` - Table column style

Tags: `table`

Aliases: `table-column-style`

Parameters:

- `aligned_delimiter`: Aligned delimiter columns (`boolean`, default `false`)
- `style`: Table column style (`string`, default `any`, values `aligned` /
  `any` / `compact` / `tight`)
"#,
    )
    .expect("document should parse");

    assert_eq!(document.id, "MD060");
    assert_eq!(document.name, "table-column-style");
    assert_eq!(document.tags, vec!["table"]);
    assert_eq!(document.properties.len(), 2);
    assert_eq!(
        document.properties[1].values,
        vec!["aligned", "any", "compact", "tight"]
    );
}

#[test]
fn parses_rule_document_fenced_examples() {
    let document = parse::parse_rule_document(
        r#"# `MD999` - Example rule

Tags: `test`

Aliases: `example-rule`

This is not a fixture.

```markdown
# Heading

text
```

```json
{ "MD999": false }
```
"#,
    )
    .expect("document should parse");

    assert_eq!(document.examples.len(), 2);
    assert_eq!(document.examples[0].language.as_deref(), Some("markdown"));
    assert_eq!(document.examples[0].content, "# Heading\n\ntext");
    assert_eq!(document.examples[1].language.as_deref(), Some("json"));
}
