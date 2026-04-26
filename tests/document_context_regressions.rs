use katana_markdown_linter::rules::markdown::{DocumentContext, SourceRange};
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
