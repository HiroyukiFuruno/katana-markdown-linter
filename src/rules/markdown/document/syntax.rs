use super::types::{LineInfo, ParsedEmphasisSpan, SourceRange};
use crate::rules::markdown::inline::{
    extract_inline_code_spans, inline_code_spans_on_line, InlineCodeSpan,
};
use candidates::ParseCandidates;
use parsed::ParsedSyntax;
use pulldown_cmark::Options;
use semantic::SemanticIndex;
use std::sync::OnceLock;

mod candidates;
mod parsed;
mod semantic;
pub(crate) use semantic::MarkdownSemanticSignature;

pub(super) const COMMONMARK_OPTIONS: Options = Options::ENABLE_TABLES
    .union(Options::ENABLE_STRIKETHROUGH)
    .union(Options::ENABLE_TASKLISTS)
    .union(Options::ENABLE_FOOTNOTES)
    .union(Options::ENABLE_GFM)
    .union(Options::ENABLE_DEFINITION_LIST);

pub(super) struct SyntaxIndex {
    candidates: ParseCandidates,
    local_code_spans: OnceLock<Vec<InlineCodeSpan>>,
    parsed: OnceLock<(ParsedSyntax, SemanticIndex)>,
    semantic: OnceLock<SemanticIndex>,
}

impl SyntaxIndex {
    pub(super) fn new(content: &str) -> Self {
        Self {
            candidates: ParseCandidates::scan(content),
            local_code_spans: OnceLock::new(),
            parsed: OnceLock::new(),
            semantic: OnceLock::new(),
        }
    }

    pub(super) fn blocks(&self, content: &str, lines: &[LineInfo<'_>]) -> Option<&ParsedSyntax> {
        self.candidates
            .blocks
            .then(|| &self.parsed(content, lines).0)
    }

    pub(super) fn inline_code_spans(
        &self,
        content: &str,
        lines: &[LineInfo<'_>],
        code_line_flags: &[bool],
    ) -> Vec<InlineCodeSpan> {
        if !content.contains('`') {
            return Vec::new();
        }
        if self.needs_inline_parser(content, lines) {
            extract_inline_code_spans(
                lines,
                code_line_flags,
                self.parsed(content, lines).0.code_spans.clone(),
            )
        } else {
            self.local_code_spans(content, lines).to_vec()
        }
    }

    pub(super) fn html_spans(&self, content: &str, lines: &[LineInfo<'_>]) -> &[SourceRange] {
        if self.candidates.html {
            &self.parsed(content, lines).0.html_spans
        } else {
            &[]
        }
    }

    pub(super) fn parsed_emphasis_spans(
        &self,
        content: &str,
        lines: &[LineInfo<'_>],
    ) -> &[ParsedEmphasisSpan] {
        self.semantic_index(content, lines).emphasis_spans()
    }

    pub(super) fn replacement_preserves_inline_semantics(
        &self,
        content: &str,
        lines: &[LineInfo<'_>],
        range: SourceRange,
        replacement: &str,
        semantic_scope: usize,
        block_scope: Option<usize>,
    ) -> bool {
        self.semantic_index(content, lines)
            .replacement_preserves_inline_semantics(
                content,
                range,
                replacement,
                semantic_scope,
                block_scope,
            )
    }

    fn semantic_index(&self, content: &str, lines: &[LineInfo<'_>]) -> &SemanticIndex {
        if self.needs_inline_parser(content, lines) {
            &self.parsed(content, lines).1
        } else {
            self.semantic
                .get_or_init(|| semantic::parse_semantic_index(content))
        }
    }

    fn parsed(&self, content: &str, lines: &[LineInfo<'_>]) -> &(ParsedSyntax, SemanticIndex) {
        self.parsed
            .get_or_init(|| ParsedSyntax::parse(content, lines))
    }

    fn needs_inline_parser(&self, content: &str, lines: &[LineInfo<'_>]) -> bool {
        self.candidates.blocks
            || self.candidates.html
            || self
                .local_code_spans(content, lines)
                .iter()
                .any(|span| !span.closed)
    }

    fn local_code_spans(&self, content: &str, lines: &[LineInfo<'_>]) -> &[InlineCodeSpan] {
        self.local_code_spans.get_or_init(|| {
            if !content.contains('`') {
                return Vec::new();
            }
            lines
                .iter()
                .enumerate()
                .flat_map(|(line_index, line)| inline_code_spans_on_line(line_index, line))
                .collect()
        })
    }
}
