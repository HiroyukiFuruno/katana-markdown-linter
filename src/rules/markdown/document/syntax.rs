use super::types::{BlockRange, FenceKind, LineInfo, SourceRange};
use crate::rules::markdown::inline::{inline_code_spans_on_line, InlineCodeSpan};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

pub(super) struct SyntaxIndex {
    pub(super) code_blocks: Vec<BlockRange>,
    pub(super) code_spans: Vec<InlineCodeSpan>,
}

impl SyntaxIndex {
    pub(super) fn new(content: &str, lines: &[LineInfo<'_>]) -> Self {
        let mut index = Self {
            code_blocks: Vec::new(),
            code_spans: Vec::new(),
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
        let options = Options::ENABLE_TABLES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_GFM
            | Options::ENABLE_DEFINITION_LIST;
        for (event, range) in Parser::new_ext(content, options).into_offset_iter() {
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
        index
    }
}

fn line_at(lines: &[LineInfo<'_>], offset: usize) -> usize {
    lines
        .partition_point(|line| line.content_range.start <= offset)
        .saturating_sub(1)
}
