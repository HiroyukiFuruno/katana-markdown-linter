use super::{SemanticIndex, SemanticScope};
use crate::rules::markdown::document::SourceRange;
use crate::rules::markdown::inline::InlineEmphasisKind;
use pulldown_cmark::{Event, Tag, TagEnd};
use std::ops::Range;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScopeKind {
    Inline,
    Block,
}

struct OpenScope {
    kind: ScopeKind,
    end_tag: TagEnd,
    range: SourceRange,
    event_start: usize,
    scope_index: usize,
}

#[derive(Default)]
pub(in crate::rules::markdown::document::syntax) struct SemanticIndexBuilder {
    emphasis_spans: Vec<crate::rules::markdown::document::types::ParsedEmphasisSpan>,
    inline_scopes: Vec<SemanticScope>,
    block_scopes: Vec<SemanticScope>,
    open_scopes: Vec<OpenScope>,
    open_inline_scopes: Vec<usize>,
    open_block_scopes: Vec<usize>,
    event_count: usize,
    has_reference_definitions: bool,
    has_footnote_syntax: bool,
}

impl SemanticIndexBuilder {
    pub(in crate::rules::markdown::document::syntax) fn new(
        has_reference_definitions: bool,
        has_footnote_syntax: bool,
    ) -> Self {
        Self {
            has_reference_definitions,
            has_footnote_syntax,
            ..Self::default()
        }
    }

    pub(in crate::rules::markdown::document::syntax) fn push(
        &mut self,
        event: &Event<'_>,
        range: Range<usize>,
    ) {
        let event_index = self.event_count;
        self.event_count += 1;
        match event {
            Event::Start(tag) => self.start_scope(tag, event_index, &range),
            Event::End(tag_end) => self.end_scope(*tag_end, self.event_count, &range),
            _ => {}
        }
    }

    pub(in crate::rules::markdown::document::syntax) fn finish(
        self,
        content: &str,
    ) -> SemanticIndex {
        let mut index = SemanticIndex {
            emphasis_spans: self.emphasis_spans,
            inline_scopes: self.inline_scopes,
            block_scopes: self.block_scopes,
            events: std::sync::OnceLock::new(),
            has_reference_definitions: self.has_reference_definitions,
            has_footnote_syntax: self.has_footnote_syntax,
        };
        for scope in &mut index.block_scopes {
            scope.has_link_brackets = content
                .get(scope.range.start..scope.range.end)
                .is_none_or(|block| block.contains(['[', ']']));
        }
        index
    }

    fn start_scope(&mut self, tag: &Tag<'_>, event_start: usize, range: &Range<usize>) {
        if let Some(kind) = scope_kind(tag) {
            let scope_index = match kind {
                ScopeKind::Inline => self.inline_scopes.len(),
                ScopeKind::Block => self.block_scopes.len(),
            };
            let scope = SemanticScope {
                range: SourceRange {
                    start: range.start,
                    end: range.end,
                },
                event_range: event_start..0,
                has_link_brackets: false,
            };
            match kind {
                ScopeKind::Inline => self.inline_scopes.push(scope),
                ScopeKind::Block => self.block_scopes.push(scope),
            }
            self.open_scopes.push(OpenScope {
                kind,
                end_tag: tag.to_end(),
                range: SourceRange {
                    start: range.start,
                    end: range.end,
                },
                event_start,
                scope_index,
            });
            match kind {
                ScopeKind::Inline => self.open_inline_scopes.push(scope_index),
                ScopeKind::Block => self.open_block_scopes.push(scope_index),
            }
        }

        let kind = match tag {
            Tag::Emphasis => Some(InlineEmphasisKind::Emphasis),
            Tag::Strong => Some(InlineEmphasisKind::Strong),
            _ => None,
        };
        if let Some(kind) = kind {
            let semantic_scope = *self
                .open_inline_scopes
                .first()
                .expect("emphasis has an inline scope");
            self.emphasis_spans.push(
                crate::rules::markdown::document::types::ParsedEmphasisSpan {
                    kind,
                    range: SourceRange {
                        start: range.start,
                        end: range.end,
                    },
                    semantic_scope,
                    block_scope: self.open_block_scopes.last().copied(),
                },
            );
        }
    }

    fn end_scope(&mut self, end_tag: TagEnd, event_end: usize, range: &Range<usize>) {
        let Some(position) = self
            .open_scopes
            .iter()
            .rposition(|open| open.end_tag == end_tag)
        else {
            return;
        };
        let open = self.open_scopes.remove(position);
        let scope = SemanticScope {
            range: SourceRange {
                start: open.range.start,
                end: range.end,
            },
            event_range: open.event_start..event_end,
            has_link_brackets: false,
        };
        match open.kind {
            ScopeKind::Inline => {
                self.inline_scopes[open.scope_index] = scope;
                remove_open_scope(&mut self.open_inline_scopes, open.scope_index);
            }
            ScopeKind::Block => {
                self.block_scopes[open.scope_index] = scope;
                remove_open_scope(&mut self.open_block_scopes, open.scope_index);
            }
        }
    }
}

fn remove_open_scope(scopes: &mut Vec<usize>, scope_index: usize) {
    if let Some(position) = scopes.iter().rposition(|open| *open == scope_index) {
        scopes.remove(position);
    }
}

fn scope_kind(tag: &Tag<'_>) -> Option<ScopeKind> {
    match tag {
        Tag::Emphasis
        | Tag::Strong
        | Tag::Strikethrough
        | Tag::Superscript
        | Tag::Subscript
        | Tag::Link { .. }
        | Tag::Image { .. } => Some(ScopeKind::Inline),
        Tag::Paragraph
        | Tag::Heading { .. }
        | Tag::BlockQuote(_)
        | Tag::CodeBlock(_)
        | Tag::HtmlBlock
        | Tag::List(_)
        | Tag::Item
        | Tag::FootnoteDefinition(_)
        | Tag::DefinitionList
        | Tag::DefinitionListTitle
        | Tag::DefinitionListDefinition
        | Tag::Table(_)
        | Tag::TableHead
        | Tag::TableRow
        | Tag::TableCell
        | Tag::MetadataBlock(_) => Some(ScopeKind::Block),
    }
}
