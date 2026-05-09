use super::types::{DocumentContext, SourceRange};
use crate::rules::markdown::DiagnosticRange;

impl<'a> DocumentContext<'a> {
    #[cfg(test)]
    pub(super) fn markdown_ast(&self) -> &super::types::MarkdownAst<'a> {
        self.ast.get_or_init(|| {
            let mut blocks = Vec::new();
            blocks.extend(
                self.headings()
                    .iter()
                    .cloned()
                    .map(super::types::AstBlock::Heading),
            );
            blocks.extend(
                self.code_blocks
                    .iter()
                    .cloned()
                    .map(super::types::AstBlock::CodeBlock),
            );
            blocks.extend(
                self.tables()
                    .iter()
                    .cloned()
                    .map(super::types::AstBlock::Table),
            );
            blocks.sort_by_key(|block| match block {
                super::types::AstBlock::Heading(heading) => heading.marker_range.start,
                super::types::AstBlock::CodeBlock(block) => block.range.start,
                super::types::AstBlock::Table(table) => table.range.start,
            });
            super::types::MarkdownAst { blocks }
        })
    }

    pub fn offset_to_position(&self, offset: usize) -> (usize, usize) {
        if self.lines.is_empty() {
            return (1, 1);
        }
        let offset = offset.min(self.content.len());
        if offset == self.content.len() && self.content.ends_with('\n') {
            return (self.lines.len() + 1, 1);
        }
        let index = self
            .line_offsets
            .partition_point(|line_start| *line_start <= offset)
            .saturating_sub(1);
        let line = &self.lines[index.min(self.lines.len() - 1)];
        (
            line.number,
            offset.saturating_sub(line.content_range.start) + 1,
        )
    }

    pub fn diagnostic_range(&self, range: SourceRange) -> DiagnosticRange {
        let (start_line, start_column) = self.offset_to_position(range.start);
        let (end_line, end_column) = self.offset_to_position(range.end);
        DiagnosticRange {
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }

    pub fn line_range(&self, line_index: usize) -> Option<SourceRange> {
        self.lines.get(line_index).map(|line| line.content_range)
    }

    pub fn is_code_line(&self, line_index: usize) -> bool {
        self.code_line_flags
            .get(line_index)
            .copied()
            .unwrap_or(false)
    }
}
