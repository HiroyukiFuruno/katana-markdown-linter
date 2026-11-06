use super::semantic::collector::SemanticIndexBuilder;
use super::semantic::SemanticIndex;
use super::COMMONMARK_OPTIONS;
use crate::rules::markdown::document::types::{BlockRange, FenceKind, LineInfo, SourceRange};
use crate::rules::markdown::inline::InlineCodeSpan;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

#[derive(Default)]
pub(in crate::rules::markdown::document) struct ParsedSyntax {
    pub(in crate::rules::markdown::document) code_blocks: Vec<BlockRange>,
    pub(super) code_spans: Vec<InlineCodeSpan>,
    pub(super) html_spans: Vec<SourceRange>,
    pub(in crate::rules::markdown::document) html_blocks: Vec<SourceRange>,
    pub(in crate::rules::markdown::document) fenced_code_content_at_eof: bool,
}

impl ParsedSyntax {
    pub(super) fn parse(content: &str, lines: &[LineInfo<'_>]) -> (Self, SemanticIndex) {
        let mut syntax = Self::default();
        let parser = Parser::new_ext(content, COMMONMARK_OPTIONS).into_offset_iter();
        let has_reference_definitions = parser.reference_definitions().iter().next().is_some();
        let mut semantic_builder =
            SemanticIndexBuilder::new(has_reference_definitions, content.contains("[^"));
        let mut in_fenced_code = false;
        for (event, range) in parser {
            semantic_builder.push(&event, range.clone());
            syntax.record_event(event, range, content, lines, &mut in_fenced_code);
        }
        (syntax, semantic_builder.finish(content))
    }

    fn record_event(
        &mut self,
        event: Event<'_>,
        range: std::ops::Range<usize>,
        content: &str,
        lines: &[LineInfo<'_>],
        in_fenced_code: &mut bool,
    ) {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                *in_fenced_code = true;
                self.record_fenced_block(&range, content, lines);
            }
            Event::End(TagEnd::CodeBlock) => *in_fenced_code = false,
            Event::Text(_) if *in_fenced_code && range.end == content.len() => {
                self.fenced_code_content_at_eof = true;
            }
            Event::InlineHtml(_) | Event::Html(_) => {
                self.html_spans.push(SourceRange {
                    start: range.start,
                    end: range.end,
                });
            }
            Event::Start(Tag::HtmlBlock) => self.html_blocks.push(SourceRange {
                start: range.start,
                end: range.end,
            }),
            Event::Code(_) => self.record_code_span(&range, content, lines),
            _ => {}
        }
    }

    fn record_fenced_block(
        &mut self,
        range: &std::ops::Range<usize>,
        content: &str,
        lines: &[LineInfo<'_>],
    ) {
        let start_line = line_at(lines, range.start);
        let end_line = line_at(lines, range.end.saturating_sub(1));
        self.code_blocks.push(BlockRange {
            start_line,
            end_line,
            fence: if content.as_bytes()[range.start] == b'~' {
                FenceKind::Tilde
            } else {
                FenceKind::Backtick
            },
            range: SourceRange {
                start: lines[start_line].content_range.start,
                end: lines[end_line].full_range.end,
            },
        });
    }

    fn record_code_span(
        &mut self,
        range: &std::ops::Range<usize>,
        content: &str,
        lines: &[LineInfo<'_>],
    ) {
        let marker_len = content.as_bytes()[range.start..range.end]
            .iter()
            .take_while(|byte| **byte == b'`')
            .count();
        self.code_spans.push(InlineCodeSpan {
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
}

fn line_at(lines: &[LineInfo<'_>], offset: usize) -> usize {
    lines
        .partition_point(|line| line.content_range.start <= offset)
        .saturating_sub(1)
}
