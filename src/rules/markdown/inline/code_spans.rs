use super::scan::line_in_blocks;
use super::types::InlineCodeSpan;
use crate::rules::markdown::document::{BlockRange, LineInfo, SourceRange};

pub(crate) fn extract_inline_code_spans<'a>(
    lines: &[LineInfo<'a>],
    code_blocks: &[BlockRange],
) -> Vec<InlineCodeSpan> {
    let mut spans = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if line_in_blocks(idx, code_blocks) {
            continue;
        }
        spans.extend(inline_code_spans_on_line(idx, line));
    }
    spans
}

fn inline_code_spans_on_line(line_index: usize, line: &LineInfo<'_>) -> Vec<InlineCodeSpan> {
    let mut spans = Vec::new();
    let bytes = line.text.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] != b'`' {
            cursor += 1;
            continue;
        }

        let marker_len = bytes[cursor..]
            .iter()
            .take_while(|byte| **byte == b'`')
            .count();
        let content_start = cursor + marker_len;
        let marker = "`".repeat(marker_len);
        let Some(close_relative) = line.text[content_start..].find(&marker) else {
            spans.push(InlineCodeSpan {
                line: line_index,
                marker_len,
                content_range: SourceRange {
                    start: line.content_range.start + content_start,
                    end: line.content_range.end,
                },
                full_range: SourceRange {
                    start: line.content_range.start + cursor,
                    end: line.content_range.end,
                },
                closed: false,
            });
            break;
        };

        let close = content_start + close_relative;
        spans.push(InlineCodeSpan {
            line: line_index,
            marker_len,
            content_range: SourceRange {
                start: line.content_range.start + content_start,
                end: line.content_range.start + close,
            },
            full_range: SourceRange {
                start: line.content_range.start + cursor,
                end: line.content_range.start + close + marker_len,
            },
            closed: true,
        });
        cursor = close + marker_len;
    }
    spans
}
