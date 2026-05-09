use super::*;
use std::path::Path;

#[test]
fn context_handles_empty_no_newline_crlf_and_unicode() {
    let empty = DocumentContext::new(Path::new("empty.md"), "");
    assert!(empty.lines().is_empty());
    assert_eq!(empty.offset_to_position(0), (1, 1));

    let no_newline = DocumentContext::new(Path::new("doc.md"), "# Title");
    assert_eq!(no_newline.lines()[0].text, "# Title");
    assert_eq!(no_newline.offset_to_position(7), (1, 8));

    let crlf = DocumentContext::new(Path::new("doc.md"), "# A\r\ntext\r\n");
    assert_eq!(crlf.lines()[0].text, "# A");
    assert_eq!(crlf.lines()[0].full_range.end, 5);
    assert_eq!(crlf.offset_to_position(5), (2, 1));

    let unicode = DocumentContext::new(Path::new("doc.md"), "é\n");
    assert_eq!(unicode.offset_to_position(2), (1, 3));
    assert_eq!(unicode.offset_to_position(3), (2, 1));
    let range = unicode.diagnostic_range(SourceRange { start: 0, end: 2 });
    assert_eq!(range.start_line, 1);
    assert_eq!(range.start_column, 1);
    assert_eq!(range.end_line, 1);
    assert_eq!(range.end_column, 3);
}

#[test]
fn context_extracts_structures_with_source_ranges() {
    let content = "---\ntitle: x\n---\n# Title\n\n[text](#title)\n\n| A | B |\n|---|---|\n| C | D |\n\n```md\n# ignored\n| x | y |\n```\n";
    let ctx = DocumentContext::new(Path::new("doc.md"), content);

    assert_eq!(ctx.front_matter(), Some(SourceRange { start: 0, end: 17 }));
    assert_eq!(ctx.headings().len(), 1);
    assert_eq!(ctx.headings()[0].text, "Title");
    assert_eq!(ctx.links().len(), 1);
    assert_eq!(ctx.links()[0].destination, "#title");
    assert_eq!(ctx.inline_links().len(), 1);
    assert_eq!(ctx.inline_links()[0].destination, Some("#title"));
    assert_eq!(ctx.tables().len(), 1);
    assert_eq!(ctx.tables()[0].rows.len(), 3);
    assert_eq!(ctx.code_blocks().len(), 1);
    assert!(!ctx.is_code_line(0));
    assert!(ctx.is_code_line(12));
    assert!(!ctx.is_code_line(99));
    assert_eq!(ctx.markdown_ast().blocks.len(), 3);
}

#[test]
fn context_extracts_source_preserving_inline_tokens() {
    let content = concat!(
        "See [nested [text]](https://example.com/path?q=1 \"title\") ",
        "and ![alt][image-ref] plus <https://example.org>.\n",
        "``[ignored](https://example.invalid)`` and `[ignored][]`\n",
        "`[unclosed](https://example.invalid)\n",
        "[image-ref]: <https://example.org/image.png> \"Image\"\n",
    );
    let ctx = DocumentContext::new(Path::new("doc.md"), content);

    assert_eq!(ctx.inline_code_spans().len(), 3);
    assert!(!ctx.inline_code_spans()[2].closed);
    let links = ctx.inline_links();
    assert_eq!(links.len(), 3);
    assert_eq!(links[0].text, Some("nested [text]"));
    assert_eq!(links[0].destination, Some("https://example.com/path?q=1"));
    assert_eq!(links[1].text, Some("alt"));
    assert_eq!(links[1].effective_label(), Some("image-ref"));
    assert_eq!(links[2].destination, Some("https://example.org"));
    assert_eq!(ctx.reference_definitions().len(), 1);
    assert_eq!(ctx.reference_definitions()[0].label, "image-ref");
    assert_eq!(
        ctx.reference_definitions()[0].destination,
        "https://example.org/image.png"
    );
}

#[test]
fn context_extracts_source_preserving_inline_html_elements() {
    let content = concat!(
        "<span data-id=\"outer\"><a id=\"target\" href=\"https://example.com?q=>\">x</a></span>\n",
        "<img data-count=42 alt='Image' />\n",
        "<span =broken disabled></span>\n",
        "<span missing=\"unterminated></span>\n",
        "`<a id=\"code\"></a>`\n",
        "```\n<a id=\"fenced\"></a>\n```\n",
        "<https://example.com>\n",
        "<span",
    );
    let ctx = DocumentContext::new(Path::new("doc.md"), content);
    let elements = ctx.inline_html_elements();

    assert!(elements.iter().any(|element| {
        !element.closing
            && element.name == "span"
            && element.attribute_value("data-id") == Some("outer")
    }));
    assert!(elements.iter().any(|element| {
        !element.closing
            && element.name == "a"
            && element.attribute_value("id") == Some("target")
            && element.attribute_value("href") == Some("https://example.com?q=>")
    }));
    assert!(elements.iter().any(|element| {
        !element.closing
            && element.name == "img"
            && element.attribute_value("data-count").is_none()
            && element.attribute_value("alt") == Some("Image")
    }));
    assert!(elements.iter().any(|element| {
        !element.closing && element.name == "span" && element.attribute_value("disabled").is_none()
    }));
    assert!(elements
        .iter()
        .any(|element| element.closing && element.name == "a"));
    assert!(elements
        .iter()
        .all(|element| element.attribute_value("id") != Some("code")));
    assert!(elements
        .iter()
        .all(|element| element.attribute_value("id") != Some("fenced")));
    assert!(elements.iter().all(|element| element.name != "https:"));
}

#[test]
fn context_inline_tokens_handle_empty_crlf_and_unicode() {
    let empty_ctx = DocumentContext::new(Path::new("empty.md"), "");
    assert!(empty_ctx.inline_code_spans().is_empty());
    assert!(empty_ctx.inline_links().is_empty());
    assert!(empty_ctx.reference_definitions().is_empty());

    let content = "`コード` [表示](#見出し)\r\n# 見出し\r\n";
    let ctx = DocumentContext::new(Path::new("doc.md"), content);

    assert_eq!(ctx.inline_code_spans().len(), 1);
    assert_eq!(ctx.inline_links().len(), 1);
    assert_eq!(ctx.inline_links()[0].text, Some("表示"));
    assert_eq!(ctx.inline_links()[0].destination, Some("#見出し"));
}

#[test]
fn context_keeps_irregular_table_rows_for_column_rules() {
    let content = "| A | B |\n|---|---|\n| C |\n| D | E | F |\n";
    let ctx = DocumentContext::new(Path::new("doc.md"), content);

    assert_eq!(ctx.tables().len(), 1);
    assert_eq!(ctx.tables()[0].rows.len(), 4);
    assert_eq!(ctx.tables()[0].rows[2].cells.len(), 1);
    assert_eq!(ctx.tables()[0].rows[3].cells.len(), 3);
}

#[test]
fn context_respects_nested_shorter_fences_inside_longer_fences() {
    let content = "````markdown\n```rust\ncode\n```\n````\n";
    let ctx = DocumentContext::new(Path::new("doc.md"), content);

    assert_eq!(ctx.code_blocks().len(), 1);
    assert_eq!(ctx.code_blocks()[0].start_line, 0);
    assert_eq!(ctx.code_blocks()[0].end_line, 4);
    assert!(ctx.is_code_line(1));
    assert!(ctx.is_code_line(3));
}

#[test]
fn ast_is_lazy_and_not_required_for_structural_indexes() {
    let content = "# Title\n\n| A | B |\n|---|---|\n";
    let ctx = DocumentContext::new(Path::new("doc.md"), content);

    assert!(ctx.ast.get().is_none());
    assert!(ctx.headings.get().is_none());
    assert!(ctx.tables.get().is_none());
    assert_eq!(ctx.headings().len(), 1);
    assert_eq!(ctx.tables().len(), 1);
    assert!(ctx.headings.get().is_some());
    assert!(ctx.tables.get().is_some());
    assert!(ctx.ast.get().is_none());

    assert_eq!(ctx.markdown_ast().blocks.len(), 2);
    assert!(ctx.ast.get().is_some());
}
