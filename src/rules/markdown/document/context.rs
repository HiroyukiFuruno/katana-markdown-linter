use super::fences::extract_code_blocks;
use super::headings::extract_headings;
use super::lines::{build_code_line_flags, extract_front_matter, split_lines};
use super::links::extract_links;
use super::tables::extract_tables;
use super::types::{DocumentContext, SourceRange};
use crate::rules::markdown::inline::{
    extract_inline_code_spans, extract_inline_html_elements, extract_inline_links,
    extract_reference_definitions, InlineCodeSpan, InlineHtmlAttribute, InlineHtmlElement,
    InlineLink, ReferenceDefinition,
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
        let code_blocks = extract_code_blocks(&lines);
        let code_line_flags = build_code_line_flags(lines.len(), &code_blocks);
        Self {
            content,
            file_path,
            lines,
            line_offsets,
            front_matter,
            code_blocks,
            code_line_flags,
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
        &self.code_blocks
    }

    pub fn headings(&self) -> &[super::Heading<'a>] {
        self.headings
            .get_or_init(|| extract_headings(&self.lines, &self.code_blocks))
            .as_slice()
    }

    pub fn inline_code_spans(&self) -> &[InlineCodeSpan] {
        self.inline_code_spans
            .get_or_init(|| extract_inline_code_spans(&self.lines, &self.code_line_flags))
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
                    &self.code_line_flags,
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
                extract_inline_links(&self.lines, &self.code_line_flags, self.inline_code_spans())
            })
            .as_slice()
    }

    pub fn reference_definitions(&self) -> &[ReferenceDefinition<'a>] {
        self.reference_definitions
            .get_or_init(|| extract_reference_definitions(&self.lines, &self.code_line_flags))
            .as_slice()
    }

    pub fn is_inside_inline_code(&self, range: SourceRange) -> bool {
        let spans = self.inline_code_spans();
        let candidate_index = spans
            .partition_point(|span| span.full_range.start <= range.start)
            .saturating_sub(1);
        spans.get(candidate_index).is_some_and(|span| {
            span.full_range.start <= range.start && range.end <= span.full_range.end
        })
    }

    pub fn links(&self) -> &[super::Link<'a>] {
        self.links
            .get_or_init(|| extract_links(self.inline_links()))
            .as_slice()
    }

    pub fn tables(&self) -> &[super::TableBlock<'a>] {
        self.tables
            .get_or_init(|| extract_tables(&self.lines, &self.code_blocks))
            .as_slice()
    }
}
