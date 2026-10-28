/// Maps one-based line and byte columns to UTF-8 byte offsets.
pub(super) struct LineOffsetIndex<'a> {
    content: &'a str,
    line_starts: Vec<usize>,
}

impl<'a> LineOffsetIndex<'a> {
    pub(super) fn new(content: &'a str) -> Self {
        let mut line_starts = Vec::with_capacity(
            content
                .as_bytes()
                .iter()
                .filter(|byte| **byte == b'\n')
                .count()
                + 1,
        );
        line_starts.push(0);
        for (index, byte) in content.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(index + 1);
            }
        }
        Self {
            content,
            line_starts,
        }
    }

    pub(super) fn offset_for_position(&self, line: usize, column: usize) -> Option<usize> {
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
}

fn previous_char_boundary(content: &str, mut offset: usize) -> usize {
    while offset > 0 && !content.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}
