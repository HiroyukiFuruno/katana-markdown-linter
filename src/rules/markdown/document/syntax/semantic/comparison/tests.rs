use super::ScopePreference;
use crate::rules::markdown::document::syntax::semantic::parse_semantic_index;
use crate::rules::markdown::inline::InlineEmphasisKind;

#[test]
fn nested_candidates_with_global_definitions_use_constant_inline_comparisons() {
    let paragraph = std::iter::repeat_n("***word***", 128)
        .collect::<Vec<_>>()
        .join(" ");
    let source = format!("{paragraph}\n\n[unused]: /destination\n\n[^note]: footnote\n");
    let index = parse_semantic_index(&source);
    let strong_spans = index
        .emphasis_spans
        .iter()
        .filter(|span| span.kind == InlineEmphasisKind::Strong)
        .collect::<Vec<_>>();
    assert_eq!(strong_spans.len(), 128);

    for span in strong_spans {
        let scope = &index.inline_scopes[span.semantic_scope];
        assert_eq!(scope.range.end - scope.range.start, 10);
        assert!(!scope.has_link_brackets);
        assert_eq!(
            index.scope_preference(span.range, span.semantic_scope, span.block_scope),
            ScopePreference::Inline(span.semantic_scope)
        );
        assert_eq!(
            index.compare_inline_scope(&source, span.range, "__word__", scope),
            Some(true)
        );
    }

    let linked_source = "***word*** [label]\n\n[label]: /destination\n\n[^note]: footnote\n";
    let linked_index = parse_semantic_index(linked_source);
    let linked_span = linked_index
        .emphasis_spans
        .iter()
        .find(|span| span.kind == InlineEmphasisKind::Strong)
        .expect("the linked source contains a strong span");
    assert_eq!(
        linked_index.scope_preference(
            linked_span.range,
            linked_span.semantic_scope,
            linked_span.block_scope,
        ),
        ScopePreference::FullDocument
    );
}
