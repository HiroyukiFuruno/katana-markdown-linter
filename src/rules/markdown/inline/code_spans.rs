use super::scan::is_escaped;
use super::types::InlineCodeSpan;
use crate::rules::markdown::document::{LineInfo, SourceRange};

pub(in crate::rules::markdown) fn extract_inline_code_spans<'a>(
    lines: &[LineInfo<'a>],
    code_line_flags: &[bool],
    mut spans: Vec<InlineCodeSpan>,
) -> Vec<InlineCodeSpan> {
    let mut unclosed = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if code_line_flags[idx] {
            continue;
        }
        for span in inline_code_spans_on_line(idx, line) {
            if span.closed {
                continue;
            }
            let next = spans.partition_point(|known| known.full_range.end <= span.full_range.start);
            if spans
                .get(next)
                .is_none_or(|known| known.full_range.start >= span.full_range.end)
            {
                unclosed.push(span);
            }
        }
    }
    spans.extend(unclosed);
    spans.sort_by_key(|span| span.full_range.start);
    spans
}

pub(in crate::rules::markdown) fn inline_code_spans_on_line(
    line_index: usize,
    line: &LineInfo<'_>,
) -> Vec<InlineCodeSpan> {
    let mut spans = Vec::new();
    let bytes = line.text.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] != b'`' {
            cursor += 1;
            continue;
        }
        if is_escaped(line.text, cursor) {
            /* WHY: エスケープされるのは直後の1文字だけで、残りのrunは構文になり得る。 */
            cursor += 1;
            continue;
        }

        let marker_len = bytes[cursor..]
            .iter()
            .take_while(|byte| **byte == b'`')
            .count();
        let content_start = cursor + marker_len;
        let Some(close_relative) = find_closing_marker(line.text, content_start, marker_len) else {
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

/// Find the first occurrence of exactly `marker_len` consecutive backticks starting at `start`,
/// returning the offset relative to `start`. Avoids String allocation from "`".repeat(n)`.
fn find_closing_marker(text: &str, start: usize, marker_len: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut cursor = start;
    while cursor < bytes.len() {
        if bytes[cursor] != b'`' {
            cursor += 1;
            continue;
        }
        let run = bytes[cursor..].iter().take_while(|&&b| b == b'`').count();
        if run == marker_len {
            return Some(cursor - start);
        }
        cursor += run;
    }
    None
}
