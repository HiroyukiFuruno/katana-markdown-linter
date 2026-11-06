use super::headings::extract_headings;
use super::lines::{build_code_line_flags, extract_front_matter, split_lines};
use super::links::extract_links;
use super::syntax::SyntaxIndex;
use super::tables::extract_tables;
use super::types::{DocumentContext, ParsedEmphasisSpan, SourceRange};
use crate::rules::markdown::inline::{
    extract_inline_html_elements, extract_inline_links, extract_reference_definitions,
    InlineCodeSpan, InlineHtmlAttribute, InlineHtmlElement, InlineLink, ReferenceDefinition,
};
use std::path::Path;
use std::sync::OnceLock;

impl<'a> DocumentContext<'a> {
    pub fn new(file_path: &'a Path, content: &'a str) -> Self {
        let lines = split_lines(content);
        let line_offsets = if lines.is_empty() {
            vec![0]
        } else {
            lines.iter().map(|line| line.content_range.start).collect()
        };
        let front_matter = extract_front_matter(&lines);
        Self {
            content,
            file_path,
            lines,
            line_offsets,
            front_matter,
            syntax: OnceLock::new(),
            code_line_flags: OnceLock::new(),
            headings: OnceLock::new(),
            inline_code_spans: OnceLock::new(),
            inline_html_elements: OnceLock::new(),
            inline_links: OnceLock::new(),
            reference_definitions: OnceLock::new(),
            links: OnceLock::new(),
            tables: OnceLock::new(),
            #[cfg(test)]
            ast: OnceLock::new(),
        }
    }

    pub fn content(&self) -> &'a str {
        self.content
    }

    pub fn file_path(&self) -> &'a Path {
        self.file_path
    }

    pub fn lines(&self) -> &[super::LineInfo<'a>] {
        &self.lines
    }

    pub fn line_offsets(&self) -> &[usize] {
        &self.line_offsets
    }

    pub fn front_matter(&self) -> Option<SourceRange> {
        self.front_matter
    }

    pub fn code_blocks(&self) -> &[super::BlockRange] {
        self.syntax()
            .blocks(self.content, &self.lines)
            .map_or(&[], |syntax| syntax.code_blocks.as_slice())
    }

    pub(in crate::rules::markdown) fn parsed_emphasis_spans(
        &self,
        range: SourceRange,
    ) -> &[ParsedEmphasisSpan] {
        let spans = self
            .syntax()
            .parsed_emphasis_spans(self.content, &self.lines);
        let start = spans.partition_point(|span| span.range.start < range.start);
        let end = spans.partition_point(|span| span.range.start < range.end);
        &spans[start..end]
    }

    pub(in crate::rules::markdown) fn replacement_preserves_inline_semantics(
        &self,
        range: SourceRange,
        replacement: &str,
        semantic_scope: usize,
        block_scope: Option<usize>,
    ) -> bool {
        self.syntax().replacement_preserves_inline_semantics(
            self.content,
            &self.lines,
            range,
            replacement,
            semantic_scope,
            block_scope,
        )
    }

    pub(super) fn syntax(&self) -> &SyntaxIndex {
        self.syntax.get_or_init(|| SyntaxIndex::new(self.content))
    }

    pub(super) fn code_line_flags(&self) -> &[bool] {
        self.code_line_flags
            .get_or_init(|| build_code_line_flags(self.lines.len(), self.code_blocks()))
    }

    pub fn headings(&self) -> &[super::Heading<'a>] {
        self.headings
            .get_or_init(|| extract_headings(self))
            .as_slice()
    }

    pub fn inline_code_spans(&self) -> &[InlineCodeSpan] {
        self.inline_code_spans
            .get_or_init(|| {
                let mut spans = self.syntax().inline_code_spans(
                    self.content,
                    &self.lines,
                    self.code_line_flags(),
                );
                spans.retain(|span| span.closed || !self.is_raw_html_at(span.full_range.start));
                spans
            })
            .as_slice()
    }

    pub fn inline_html_elements(&self) -> &[InlineHtmlElement<'a>] {
        self.inline_html_elements
            .get_or_init(|| {
                if !self.content.contains('<') {
                    return Vec::new();
                }
                extract_inline_html_elements(
                    &self.lines,
                    self.code_line_flags(),
                    self.inline_code_spans(),
                )
            })
            .as_slice()
    }

    pub fn html_attribute_at(&self, offset: usize) -> Option<&InlineHtmlAttribute<'a>> {
        self.inline_html_elements()
            .iter()
            .flat_map(|element| element.attributes.iter())
            .find(|attribute| {
                attribute
                    .value_range
                    .is_some_and(|range| range.start <= offset && offset < range.end)
            })
    }

    pub fn inline_links(&self) -> &[InlineLink<'a>] {
        self.inline_links
            .get_or_init(|| {
                extract_inline_links(
                    &self.lines,
                    self.code_line_flags(),
                    self.inline_code_spans(),
                )
                .into_iter()
                .filter(|link| !self.has_raw_html_boundary(link.full_range))
                .collect()
            })
            .as_slice()
    }

    pub fn reference_definitions(&self) -> &[ReferenceDefinition<'a>] {
        self.reference_definitions
            .get_or_init(|| extract_reference_definitions(&self.lines, self.code_line_flags()))
            .as_slice()
    }

    pub fn links(&self) -> &[super::Link<'a>] {
        self.links
            .get_or_init(|| extract_links(self.inline_links()))
            .as_slice()
    }

    pub fn tables(&self) -> &[super::TableBlock<'a>] {
        self.tables.get_or_init(|| extract_tables(self)).as_slice()
    }
}
