pub(super) mod collector;
mod comparison;
mod events;
mod selection;

use crate::rules::markdown::document::types::{ParsedEmphasisSpan, SourceRange};
use collector::SemanticIndexBuilder;
use events::SemanticEvent;
use pulldown_cmark::Parser;
use std::ops::Range;
use std::sync::OnceLock;

#[derive(Clone)]
pub(super) struct SemanticScope {
    pub(super) range: SourceRange,
    pub(super) event_range: Range<usize>,
    pub(super) has_link_brackets: bool,
}

pub(super) struct SemanticIndex {
    emphasis_spans: Vec<ParsedEmphasisSpan>,
    inline_scopes: Vec<SemanticScope>,
    block_scopes: Vec<SemanticScope>,
    events: OnceLock<Vec<SemanticEvent>>,
    has_reference_definitions: bool,
    has_footnote_syntax: bool,
}

pub(crate) struct MarkdownSemanticSignature(Vec<SemanticEvent>);

impl MarkdownSemanticSignature {
    pub(crate) fn new(source: &str) -> Self {
        Self(comparison::parse_signature(source))
    }

    pub(crate) fn matches(&self, source: &str) -> bool {
        self.0 == comparison::parse_signature(source)
    }
}

impl SemanticIndex {
    pub(super) fn emphasis_spans(&self) -> &[ParsedEmphasisSpan] {
        &self.emphasis_spans
    }
}

pub(super) fn parse_semantic_index(content: &str) -> SemanticIndex {
    let parser = Parser::new_ext(content, super::COMMONMARK_OPTIONS).into_offset_iter();
    let has_reference_definitions = parser.reference_definitions().iter().next().is_some();
    let mut builder = SemanticIndexBuilder::new(has_reference_definitions, content.contains("[^"));
    for (event, range) in parser {
        builder.push(&event, range);
    }
    builder.finish(content)
}
