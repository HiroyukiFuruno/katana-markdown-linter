use super::types::{BlockRange, FenceKind, LineInfo, ParsedEmphasisSpan, SourceRange};
use crate::rules::markdown::inline::{inline_code_spans_on_line, InlineCodeSpan};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use semantic::collector::SemanticIndexBuilder;
use semantic::SemanticIndex;

mod semantic;
pub(crate) use semantic::MarkdownSemanticSignature;

pub(super) const COMMONMARK_OPTIONS: Options = Options::ENABLE_TABLES
    .union(Options::ENABLE_STRIKETHROUGH)
    .union(Options::ENABLE_TASKLISTS)
    .union(Options::ENABLE_FOOTNOTES)
    .union(Options::ENABLE_GFM)
    .union(Options::ENABLE_DEFINITION_LIST);

pub(super) struct SyntaxIndex {
    pub(super) code_blocks: Vec<BlockRange>,
    pub(super) code_spans: Vec<InlineCodeSpan>,
    semantic: std::sync::OnceLock<SemanticIndex>,
}

impl SyntaxIndex {
    pub(super) fn new(content: &str, lines: &[LineInfo<'_>]) -> Self {
        let mut index = Self {
            code_blocks: Vec::new(),
            code_spans: Vec::new(),
            semantic: std::sync::OnceLock::new(),
        };
        if !content.contains(['`', '~']) {
            return index;
        }
        let line_spans: Vec<_> = lines
            .iter()
            .enumerate()
            .flat_map(|(line_index, line)| inline_code_spans_on_line(line_index, line))
            .collect();
        let needs_block_parser = line_spans.iter().any(|span| !span.closed)
            || lines
                .iter()
                .any(|line| super::fences::fence_line_marker(line.text).is_some());
        if !needs_block_parser {
            index.code_spans = line_spans;
            return index;
        }
        let parser = Parser::new_ext(content, COMMONMARK_OPTIONS).into_offset_iter();
        let has_reference_definitions = parser.reference_definitions().iter().next().is_some();
        let mut semantic_builder =
            SemanticIndexBuilder::new(has_reference_definitions, content.contains("[^"));
        for (event, range) in parser {
            semantic_builder.push(&event, range.clone());
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                    index.code_blocks.push(BlockRange {
                        start_line: line_at(lines, range.start),
                        end_line: line_at(lines, range.end.saturating_sub(1)),
                        fence: if content.as_bytes()[range.start] == b'~' {
                            FenceKind::Tilde
                        } else {
                            FenceKind::Backtick
                        },
                        range: SourceRange {
                            start: lines[line_at(lines, range.start)].content_range.start,
                            end: lines[line_at(lines, range.end.saturating_sub(1))]
                                .full_range
                                .end,
                        },
                    });
                }
                Event::Code(_) => {
                    let marker_len = content.as_bytes()[range.start..range.end]
                        .iter()
                        .take_while(|byte| **byte == b'`')
                        .count();
                    index.code_spans.push(InlineCodeSpan {
                        line: line_at(lines, range.start),
                        marker_len,
                        content_range: SourceRange {
                            start: range.start + marker_len,
                            end: range.end - marker_len,
                        },
                        full_range: SourceRange {
                            start: range.start,
                            end: range.end,
                        },
                        closed: true,
                    });
                }
                _ => {}
            }
        }
        assert!(
            index.semantic.set(semantic_builder.finish(content)).is_ok(),
            "semantic ranges are initialized once"
        );
        index
    }

    pub(super) fn parsed_emphasis_spans(&self, content: &str) -> &[ParsedEmphasisSpan] {
        self.semantic
            .get_or_init(|| semantic::parse_semantic_index(content))
            .emphasis_spans()
    }

    pub(super) fn replacement_preserves_inline_semantics(
        &self,
        content: &str,
        range: SourceRange,
        replacement: &str,
        semantic_scope: usize,
        block_scope: Option<usize>,
    ) -> bool {
        self.semantic
            .get_or_init(|| semantic::parse_semantic_index(content))
            .replacement_preserves_inline_semantics(
                content,
                range,
                replacement,
                semantic_scope,
                block_scope,
            )
    }
}

fn line_at(lines: &[LineInfo<'_>], offset: usize) -> usize {
    lines
        .partition_point(|line| line.content_range.start <= offset)
        .saturating_sub(1)
}
