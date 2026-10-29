use super::delimiters::target_delimiters_can_wrap;
use crate::rules::markdown::document::{DocumentContext, LineInfo, SourceRange};

const MINIMUM_EMPHASIS_MARKERS: usize = 2;
const MINIMUM_STRONG_MARKERS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::rules::markdown) enum InlineEmphasisKind {
    Emphasis,
    Strong,
}

pub(in crate::rules::markdown) struct InlineEmphasisSpan<'a> {
    pub(in crate::rules::markdown) marker: char,
    pub(in crate::rules::markdown) marker_len: usize,
    pub(in crate::rules::markdown) start: usize,
    pub(in crate::rules::markdown) end: usize,
    pub(in crate::rules::markdown) inner: &'a str,
    pub(in crate::rules::markdown) semantic_scope: usize,
    pub(in crate::rules::markdown) block_scope: Option<usize>,
}

pub(in crate::rules::markdown) fn extract_emphasis_spans<'a>(
    ctx: &DocumentContext<'a>,
    line: &LineInfo<'a>,
    kind: InlineEmphasisKind,
) -> Vec<InlineEmphasisSpan<'a>> {
    let minimum_markers = match kind {
        InlineEmphasisKind::Emphasis => MINIMUM_EMPHASIS_MARKERS,
        InlineEmphasisKind::Strong => MINIMUM_STRONG_MARKERS,
    };
    if line
        .text
        .bytes()
        .filter(|byte| matches!(byte, b'*' | b'_'))
        .take(minimum_markers)
        .count()
        < minimum_markers
    {
        return Vec::new();
    }

    parsed_spans_for_line(ctx, line, kind)
}

pub(in crate::rules::markdown) fn safe_replacement(
    ctx: &DocumentContext<'_>,
    line: &LineInfo<'_>,
    span: &InlineEmphasisSpan<'_>,
    marker: char,
) -> Option<String> {
    if !target_delimiters_can_wrap(line.text, span.start, span.end, span.marker_len, marker) {
        return None;
    }
    let delimiter = std::iter::repeat_n(marker, span.marker_len).collect::<String>();
    let replacement = format!("{delimiter}{}{delimiter}", span.inner);
    let range = SourceRange {
        start: line.content_range.start + span.start,
        end: line.content_range.start + span.end,
    };
    if !ctx.replacement_preserves_inline_semantics(
        range,
        &replacement,
        span.semantic_scope,
        span.block_scope,
    ) {
        return None;
    }
    Some(replacement)
}

fn parsed_spans_for_line<'a>(
    ctx: &DocumentContext<'a>,
    line: &LineInfo<'a>,
    kind: InlineEmphasisKind,
) -> Vec<InlineEmphasisSpan<'a>> {
    ctx.parsed_emphasis_spans(line.content_range)
        .iter()
        .filter(|span| {
            span.kind == kind
                && span.range.start >= line.content_range.start
                && span.range.end <= line.content_range.end
        })
        .filter_map(|span| {
            let start = span.range.start - line.content_range.start;
            let end = span.range.end - line.content_range.start;
            let marker_len = match span.kind {
                InlineEmphasisKind::Emphasis => 1,
                InlineEmphasisKind::Strong => 2,
            };
            if end.saturating_sub(start) < marker_len * 2 {
                return None;
            }
            let marker = *line.text.as_bytes().get(start)?;
            if !matches!(marker, b'*' | b'_') {
                return None;
            }
            if !target_delimiters_can_wrap(line.text, start, end, marker_len, marker as char) {
                return None;
            }
            Some(InlineEmphasisSpan {
                marker: marker as char,
                marker_len,
                start,
                end,
                inner: &line.text[start + marker_len..end - marker_len],
                semantic_scope: span.semantic_scope,
                block_scope: span.block_scope,
            })
        })
        .collect()
}

pub(in crate::rules::markdown) fn touches_inline_code(
    ctx: &DocumentContext<'_>,
    line_start: usize,
    span: &InlineEmphasisSpan<'_>,
) -> bool {
    ctx.is_inside_inline_code(SourceRange {
        start: line_start + span.start,
        end: line_start + span.start + 1,
    }) || ctx.is_inside_inline_code(SourceRange {
        start: line_start + span.end.saturating_sub(1),
        end: line_start + span.end,
    }) || ctx.is_inside_inline_code(SourceRange {
        start: line_start + span.start,
        end: line_start + span.end,
    })
}
