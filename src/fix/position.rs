use crate::Range;
use std::ops;

/// Maps one-based line and byte columns to UTF-8 byte offsets.
pub(crate) struct LineOffsetIndex<'a> {
    content: &'a str,
    line_starts: Vec<usize>,
}

impl<'a> LineOffsetIndex<'a> {
    pub(crate) fn new(content: &'a str) -> Self {
        Self::with_line_endings(content, false)
    }

    pub(crate) fn for_lsp(content: &'a str) -> Self {
        Self::with_line_endings(content, true)
    }

    fn with_line_endings(content: &'a str, include_lone_cr: bool) -> Self {
        let bytes = content.as_bytes();
        let mut line_starts = Vec::new();
        line_starts.push(0);
        for (index, &byte) in bytes.iter().enumerate() {
            if byte == b'\n'
                || (include_lone_cr && byte == b'\r' && bytes.get(index + 1) != Some(&b'\n'))
            {
                line_starts.push(index + 1);
            }
        }
        Self {
            content,
            line_starts,
        }
    }

    pub(crate) fn offset_for_position(&self, line: usize, column: usize) -> Option<usize> {
        if line == 0 || column == 0 {
            return None;
        }

        let line_start = if let Some(line_start) = self.line_starts.get(line - 1) {
            *line_start
        } else if line == self.line_starts.len() + 1 && column == 1 {
            return Some(self.content.len());
        } else {
            return None;
        };

        let raw = line_start.saturating_add(column.saturating_sub(1));
        Some(previous_char_boundary(
            self.content,
            raw.min(self.content.len()),
        ))
    }

    pub(crate) fn offsets_for_range(&self, range: &Range) -> Option<ops::Range<usize>> {
        let start = self.offset_for_position(range.start_line, range.start_column)?;
        let end = self.offset_for_position(range.end_line, range.end_column)?;
        (start <= end).then_some(start..end)
    }

    pub(crate) fn line_start(&self, line: usize) -> Option<usize> {
        self.line_starts.get(line).copied()
    }

    pub(crate) fn line_end_including_newline(&self, line: usize) -> Option<usize> {
        self.line_start(line)?;
        Some(self.line_start(line + 1).unwrap_or(self.content.len()))
    }

    pub(crate) fn utf16_position(&self, offset: usize) -> (usize, usize) {
        let line = self.line_starts.partition_point(|&start| start <= offset) - 1;
        let start = self.line_starts[line];
        let end = self
            .line_end_including_newline(line)
            .unwrap_or(self.content.len());
        let line_text = self.content[start..end].trim_end_matches(['\r', '\n']);
        let end = previous_char_boundary(self.content, offset.min(start + line_text.len()));
        (line, self.content[start..end].encode_utf16().count())
    }
}

fn previous_char_boundary(content: &str, mut offset: usize) -> usize {
    while offset > 0 && !content.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}
