use crate::rules::markdown::inline::{
    InlineCodeSpan, InlineHtmlElement, InlineLink, ReferenceDefinition,
};
use std::path::Path;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceKind {
    Backtick,
    Tilde,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineInfo<'a> {
    pub number: usize,
    pub text: &'a str,
    pub content_range: SourceRange,
    pub full_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockRange {
    pub start_line: usize,
    pub end_line: usize,
    pub range: SourceRange,
    pub fence: FenceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::rules::markdown) struct FenceLineMarker {
    pub(super) kind: FenceKind,
    pub(super) length: usize,
    pub(in crate::rules::markdown) info_start: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading<'a> {
    pub line: usize,
    pub level: usize,
    pub text: &'a str,
    pub marker_range: SourceRange,
    pub text_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link<'a> {
    pub line: usize,
    pub text: &'a str,
    pub destination: &'a str,
    pub text_range: SourceRange,
    pub destination_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCell<'a> {
    pub text: &'a str,
    pub range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow<'a> {
    pub line: usize,
    pub cells: Vec<TableCell<'a>>,
    pub leading_pipe: bool,
    pub trailing_pipe: bool,
    pub delimiter: bool,
    pub safe_to_fix: bool,
    pub range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableBlock<'a> {
    pub start_line: usize,
    pub end_line: usize,
    pub rows: Vec<TableRow<'a>>,
    pub range: SourceRange,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum AstBlock<'a> {
    Heading(Heading<'a>),
    CodeBlock(BlockRange),
    Table(TableBlock<'a>),
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MarkdownAst<'a> {
    pub(super) blocks: Vec<AstBlock<'a>>,
}

pub struct DocumentContext<'a> {
    pub(super) content: &'a str,
    pub(super) file_path: &'a Path,
    pub(super) lines: Vec<LineInfo<'a>>,
    pub(super) line_offsets: Vec<usize>,
    pub(super) front_matter: Option<SourceRange>,
    pub(super) code_blocks: Vec<BlockRange>,
    pub(super) code_line_flags: Vec<bool>,
    pub(super) headings: OnceLock<Vec<Heading<'a>>>,
    pub(super) inline_code_spans: OnceLock<Vec<InlineCodeSpan>>,
    pub(super) inline_html_elements: OnceLock<Vec<InlineHtmlElement<'a>>>,
    pub(super) inline_links: OnceLock<Vec<InlineLink<'a>>>,
    pub(super) reference_definitions: OnceLock<Vec<ReferenceDefinition<'a>>>,
    pub(super) links: OnceLock<Vec<Link<'a>>>,
    pub(super) tables: OnceLock<Vec<TableBlock<'a>>>,
    #[cfg(test)]
    pub(super) ast: OnceLock<MarkdownAst<'a>>,
}
