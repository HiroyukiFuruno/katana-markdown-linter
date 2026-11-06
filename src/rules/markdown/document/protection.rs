use super::types::{DocumentContext, SourceRange};

impl DocumentContext<'_> {
    pub(crate) fn literal_content_at_eof(&self) -> bool {
        self.syntax()
            .blocks(self.content, &self.lines)
            .is_some_and(|syntax| {
                syntax.fenced_code_content_at_eof
                    || syntax
                        .html_blocks
                        .last()
                        .is_some_and(|block| block.end == self.content.len())
            })
    }

    pub(in crate::rules::markdown) fn is_literal_block_line(&self, line: usize) -> bool {
        self.is_code_line(line)
            || self
                .syntax()
                .blocks(self.content, &self.lines)
                .is_some_and(|syntax| {
                    self.lines
                        .get(line)
                        .is_some_and(|line| overlaps_ranges(&syntax.html_blocks, line.full_range))
                })
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

    pub(in crate::rules::markdown) fn overlaps_inline_code(&self, range: SourceRange) -> bool {
        let spans = self.inline_code_spans();
        let next = spans.partition_point(|span| span.full_range.end <= range.start);
        range.start < range.end
            && spans
                .get(next)
                .is_some_and(|span| span.full_range.start < range.end)
    }

    pub(in crate::rules::markdown) fn overlaps_raw_html(&self, range: SourceRange) -> bool {
        overlaps_ranges(self.syntax().html_spans(self.content, &self.lines), range)
    }

    pub(in crate::rules::markdown) fn has_raw_html_boundary(&self, range: SourceRange) -> bool {
        range.start < range.end
            && (self.is_raw_html_at(range.start) || self.is_raw_html_at(range.end - 1))
    }

    pub(super) fn is_raw_html_at(&self, offset: usize) -> bool {
        self.overlaps_raw_html(SourceRange {
            start: offset,
            end: offset + 1,
        })
    }
}

fn overlaps_ranges(spans: &[SourceRange], range: SourceRange) -> bool {
    let next = spans.partition_point(|span| span.end <= range.start);
    range.start < range.end && spans.get(next).is_some_and(|span| span.start < range.end)
}
