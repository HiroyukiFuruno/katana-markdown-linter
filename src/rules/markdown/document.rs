use crate::rules::markdown::inline::{
    extract_inline_code_spans, extract_inline_links, extract_reference_definitions, InlineCodeSpan,
    InlineLink, ReferenceDefinition,
};
use crate::rules::markdown::DiagnosticRange;
use std::path::Path;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceKind {
    Backtick,
    Tilde,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineInfo<'a> {
    pub number: usize,
    pub text: &'a str,
    pub content_range: SourceRange,
    pub full_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockRange {
    pub start_line: usize,
    pub end_line: usize,
    pub range: SourceRange,
    pub fence: FenceKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading<'a> {
    pub line: usize,
    pub level: usize,
    pub text: &'a str,
    pub marker_range: SourceRange,
    pub text_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link<'a> {
    pub line: usize,
    pub text: &'a str,
    pub destination: &'a str,
    pub text_range: SourceRange,
    pub destination_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCell<'a> {
    pub text: &'a str,
    pub range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow<'a> {
    pub line: usize,
    pub cells: Vec<TableCell<'a>>,
    pub leading_pipe: bool,
    pub trailing_pipe: bool,
    pub delimiter: bool,
    pub safe_to_fix: bool,
    pub range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableBlock<'a> {
    pub start_line: usize,
    pub end_line: usize,
    pub rows: Vec<TableRow<'a>>,
    pub range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum AstBlock<'a> {
    Heading(Heading<'a>),
    CodeBlock(BlockRange),
    Table(TableBlock<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct MarkdownAst<'a> {
    pub(crate) blocks: Vec<AstBlock<'a>>,
}

pub struct DocumentContext<'a> {
    content: &'a str,
    file_path: &'a Path,
    lines: Vec<LineInfo<'a>>,
    line_offsets: Vec<usize>,
    front_matter: Option<SourceRange>,
    code_blocks: Vec<BlockRange>,
    code_line_flags: Vec<bool>,
    headings: OnceLock<Vec<Heading<'a>>>,
    inline_code_spans: OnceLock<Vec<InlineCodeSpan>>,
    inline_links: OnceLock<Vec<InlineLink<'a>>>,
    reference_definitions: OnceLock<Vec<ReferenceDefinition<'a>>>,
    links: OnceLock<Vec<Link<'a>>>,
    tables: OnceLock<Vec<TableBlock<'a>>>,
    #[allow(dead_code)]
    ast: OnceLock<MarkdownAst<'a>>,
}

impl<'a> DocumentContext<'a> {
    pub fn new(file_path: &'a Path, content: &'a str) -> Self {
        let lines = split_lines(content);
        let line_offsets = if lines.is_empty() {
            vec![0]
        } else {
            lines.iter().map(|line| line.content_range.start).collect()
        };
        let front_matter = extract_front_matter(&lines);
        let code_blocks = extract_code_blocks(&lines);
        let code_line_flags = build_code_line_flags(lines.len(), &code_blocks);
        Self {
            content,
            file_path,
            lines,
            line_offsets,
            front_matter,
            code_blocks,
            code_line_flags,
            headings: OnceLock::new(),
            inline_code_spans: OnceLock::new(),
            inline_links: OnceLock::new(),
            reference_definitions: OnceLock::new(),
            links: OnceLock::new(),
            tables: OnceLock::new(),
            ast: OnceLock::new(),
        }
    }

    pub fn content(&self) -> &'a str {
        self.content
    }

    pub fn file_path(&self) -> &'a Path {
        self.file_path
    }

    pub fn lines(&self) -> &[LineInfo<'a>] {
        &self.lines
    }

    pub fn line_offsets(&self) -> &[usize] {
        &self.line_offsets
    }

    pub fn front_matter(&self) -> Option<SourceRange> {
        self.front_matter
    }

    pub fn code_blocks(&self) -> &[BlockRange] {
        &self.code_blocks
    }

    pub fn headings(&self) -> &[Heading<'a>] {
        self.headings
            .get_or_init(|| extract_headings(&self.lines, &self.code_blocks))
            .as_slice()
    }

    pub fn inline_code_spans(&self) -> &[InlineCodeSpan] {
        self.inline_code_spans
            .get_or_init(|| extract_inline_code_spans(&self.lines, &self.code_blocks))
            .as_slice()
    }

    pub fn inline_links(&self) -> &[InlineLink<'a>] {
        self.inline_links
            .get_or_init(|| {
                extract_inline_links(&self.lines, &self.code_blocks, self.inline_code_spans())
            })
            .as_slice()
    }

    pub fn reference_definitions(&self) -> &[ReferenceDefinition<'a>] {
        self.reference_definitions
            .get_or_init(|| extract_reference_definitions(&self.lines, &self.code_blocks))
            .as_slice()
    }

    pub fn is_inside_inline_code(&self, range: SourceRange) -> bool {
        self.inline_code_spans()
            .iter()
            .any(|span| span.full_range.start <= range.start && range.end <= span.full_range.end)
    }

    pub fn links(&self) -> &[Link<'a>] {
        self.links
            .get_or_init(|| extract_links(self.inline_links()))
            .as_slice()
    }

    pub fn tables(&self) -> &[TableBlock<'a>] {
        self.tables
            .get_or_init(|| extract_tables(&self.lines, &self.code_blocks))
            .as_slice()
    }

    #[allow(dead_code)]
    pub(crate) fn markdown_ast(&self) -> &MarkdownAst<'a> {
        self.ast.get_or_init(|| {
            let mut blocks = Vec::new();
            blocks.extend(self.headings().iter().cloned().map(AstBlock::Heading));
            blocks.extend(self.code_blocks.iter().cloned().map(AstBlock::CodeBlock));
            blocks.extend(self.tables().iter().cloned().map(AstBlock::Table));
            blocks.sort_by_key(|block| match block {
                AstBlock::Heading(heading) => heading.marker_range.start,
                AstBlock::CodeBlock(block) => block.range.start,
                AstBlock::Table(table) => table.range.start,
            });
            MarkdownAst { blocks }
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

fn build_code_line_flags(line_count: usize, code_blocks: &[BlockRange]) -> Vec<bool> {
    let mut flags = vec![false; line_count];
    for block in code_blocks {
        if block.start_line >= line_count {
            continue;
        }
        let end_line = block.end_line.min(line_count - 1);
        for flag in &mut flags[block.start_line..=end_line] {
            *flag = true;
        }
    }
    flags
}

fn split_lines(content: &str) -> Vec<LineInfo<'_>> {
    let mut lines = Vec::new();
    let mut start = 0;
    for (idx, ch) in content.char_indices() {
        if ch != '\n' {
            continue;
        }
        let text_end = if idx > start && content.as_bytes()[idx - 1] == b'\r' {
            idx - 1
        } else {
            idx
        };
        lines.push(LineInfo {
            number: lines.len() + 1,
            text: &content[start..text_end],
            content_range: SourceRange {
                start,
                end: text_end,
            },
            full_range: SourceRange {
                start,
                end: idx + 1,
            },
        });
        start = idx + 1;
    }
    if start < content.len() {
        lines.push(LineInfo {
            number: lines.len() + 1,
            text: &content[start..],
            content_range: SourceRange {
                start,
                end: content.len(),
            },
            full_range: SourceRange {
                start,
                end: content.len(),
            },
        });
    }
    lines
}

fn extract_front_matter(lines: &[LineInfo<'_>]) -> Option<SourceRange> {
    if lines.first()?.text.trim() != "---" {
        return None;
    }
    lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.text.trim() == "---")
        .map(|(_, line)| SourceRange {
            start: 0,
            end: line.full_range.end,
        })
}

fn extract_code_blocks(lines: &[LineInfo<'_>]) -> Vec<BlockRange> {
    let mut blocks = Vec::new();
    let mut open: Option<(usize, FenceKind, usize)> = None;
    for (idx, line) in lines.iter().enumerate() {
        let Some((kind, length)) = fence_marker(line.text.trim_start()) else {
            continue;
        };
        if let Some((start, start_kind, start_length)) = open {
            if start_kind == kind && length >= start_length {
                blocks.push(BlockRange {
                    start_line: start,
                    end_line: idx,
                    range: SourceRange {
                        start: lines[start].content_range.start,
                        end: line.full_range.end,
                    },
                    fence: kind,
                });
                open = None;
            }
        } else {
            open = Some((idx, kind, length));
        }
    }
    if let Some((start, kind, _)) = open {
        if let Some(last) = lines.last() {
            blocks.push(BlockRange {
                start_line: start,
                end_line: lines.len() - 1,
                range: SourceRange {
                    start: lines[start].content_range.start,
                    end: last.full_range.end,
                },
                fence: kind,
            });
        }
    }
    blocks
}

fn fence_marker(trimmed: &str) -> Option<(FenceKind, usize)> {
    let mut chars = trimmed.chars();
    let marker = chars.next()?;
    if marker != '`' && marker != '~' {
        return None;
    }
    let length = trimmed.chars().take_while(|ch| *ch == marker).count();
    if length < 3 {
        return None;
    }
    if trimmed.starts_with("```") {
        Some((FenceKind::Backtick, length))
    } else if trimmed.starts_with("~~~") {
        Some((FenceKind::Tilde, length))
    } else {
        None
    }
}

fn extract_headings<'a>(lines: &[LineInfo<'a>], code_blocks: &[BlockRange]) -> Vec<Heading<'a>> {
    lines
        .iter()
        .enumerate()
        .filter(|(idx, _)| !line_in_blocks(*idx, code_blocks))
        .filter_map(|(idx, line)| parse_heading(idx, line))
        .collect()
}

fn parse_heading<'a>(line_index: usize, line: &LineInfo<'a>) -> Option<Heading<'a>> {
    let indent = line.text.len() - line.text.trim_start_matches(' ').len();
    if indent > 3 {
        return None;
    }
    let trimmed = &line.text[indent..];
    let level = trimmed.bytes().take_while(|byte| *byte == b'#').count();
    if !(1..=6).contains(&level) {
        return None;
    }
    if trimmed
        .as_bytes()
        .get(level)
        .is_some_and(|byte| *byte != b' ')
    {
        return None;
    }
    let marker_start = line.content_range.start + indent;
    let text_start = marker_start + level + usize::from(trimmed.len() > level);
    let mut text_end = line.content_range.end;
    let body = &line.text[text_start - line.content_range.start..];
    if let Some(closing_start) = body.rfind(" #") {
        if body[closing_start + 1..].bytes().all(|byte| byte == b'#') {
            text_end = text_start + closing_start;
        }
    }
    Some(Heading {
        line: line_index,
        level,
        text: &line.text
            [text_start - line.content_range.start..text_end - line.content_range.start],
        marker_range: SourceRange {
            start: marker_start,
            end: marker_start + level,
        },
        text_range: SourceRange {
            start: text_start,
            end: text_end,
        },
    })
}

fn extract_links<'a>(inline_links: &[InlineLink<'a>]) -> Vec<Link<'a>> {
    inline_links
        .iter()
        .filter(|link| link.kind.is_inline() && !link.kind.is_image())
        .filter_map(|link| {
            Some(Link {
                line: link.line,
                text: link.text?,
                destination: link.destination?,
                text_range: link.text_range?,
                destination_range: link.destination_range?,
            })
        })
        .collect()
}

fn extract_tables<'a>(lines: &[LineInfo<'a>], code_blocks: &[BlockRange]) -> Vec<TableBlock<'a>> {
    let mut tables = Vec::new();
    let mut idx = 0;
    while idx + 1 < lines.len() {
        if line_in_blocks(idx, code_blocks) {
            idx += 1;
            continue;
        }
        let Some(header) = parse_table_row(idx, &lines[idx]) else {
            idx += 1;
            continue;
        };
        let Some(delimiter) = parse_table_row(idx + 1, &lines[idx + 1]) else {
            idx += 1;
            continue;
        };
        if !delimiter.delimiter || header.cells.len() != delimiter.cells.len() {
            idx += 1;
            continue;
        }
        let mut rows = vec![header, delimiter];
        let mut end = idx + 1;
        while end + 1 < lines.len() && !line_in_blocks(end + 1, code_blocks) {
            let Some(next) = parse_table_row(end + 1, &lines[end + 1]) else {
                break;
            };
            rows.push(next);
            end += 1;
        }
        tables.push(TableBlock {
            start_line: idx,
            end_line: end,
            range: SourceRange {
                start: lines[idx].content_range.start,
                end: lines[end].full_range.end,
            },
            rows,
        });
        idx = end + 1;
    }
    tables
}

fn parse_table_row<'a>(line_index: usize, line: &LineInfo<'a>) -> Option<TableRow<'a>> {
    let trimmed = line.text.trim();
    if !trimmed.contains('|') || trimmed.is_empty() {
        return None;
    }
    let leading_pipe = trimmed.starts_with('|');
    let trailing_pipe = trimmed.ends_with('|');
    let safe_to_fix = !trimmed.contains("\\|") && !trimmed.contains('`');
    let mut inner = trimmed;
    let leading_trim = line.text.find(trimmed).unwrap_or(0);
    let mut inner_start = line.content_range.start + leading_trim;
    if leading_pipe {
        inner = &inner[1..];
        inner_start += 1;
    }
    if trailing_pipe {
        inner = &inner[..inner.len().saturating_sub(1)];
    }
    let mut cells = Vec::new();
    let mut cell_start = inner_start;
    for raw in inner.split('|') {
        let left_trim = raw.len() - raw.trim_start().len();
        let text = raw.trim();
        let start = cell_start + left_trim;
        cells.push(TableCell {
            text,
            range: SourceRange {
                start,
                end: start + text.len(),
            },
        });
        cell_start += raw.len() + 1;
    }
    if cells.is_empty() {
        return None;
    }
    let delimiter = cells.iter().all(|cell| is_delimiter_cell(cell.text));
    Some(TableRow {
        line: line_index,
        cells,
        leading_pipe,
        trailing_pipe,
        delimiter,
        safe_to_fix,
        range: line.content_range,
    })
}

fn is_delimiter_cell(cell: &str) -> bool {
    let trimmed = cell.trim();
    let core = trimmed.trim_matches(':');
    core.len() >= 3
        && core.bytes().all(|byte| byte == b'-')
        && trimmed.bytes().all(|byte| byte == b'-' || byte == b':')
}

fn line_in_blocks(line_index: usize, blocks: &[BlockRange]) -> bool {
    blocks
        .iter()
        .any(|block| (block.start_line..=block.end_line).contains(&line_index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_handles_empty_no_newline_crlf_and_unicode() {
        let empty = DocumentContext::new(Path::new("empty.md"), "");
        assert!(empty.lines().is_empty());
        assert_eq!(empty.offset_to_position(0), (1, 1));

        let no_newline = DocumentContext::new(Path::new("doc.md"), "# Title");
        assert_eq!(no_newline.lines()[0].text, "# Title");
        assert_eq!(no_newline.offset_to_position(7), (1, 8));

        let crlf = DocumentContext::new(Path::new("doc.md"), "# A\r\ntext\r\n");
        assert_eq!(crlf.lines()[0].text, "# A");
        assert_eq!(crlf.lines()[0].full_range.end, 5);
        assert_eq!(crlf.offset_to_position(5), (2, 1));

        let unicode = DocumentContext::new(Path::new("doc.md"), "é\n");
        assert_eq!(unicode.offset_to_position(2), (1, 3));
        assert_eq!(unicode.offset_to_position(3), (2, 1));
        let range = unicode.diagnostic_range(SourceRange { start: 0, end: 2 });
        assert_eq!(range.start_line, 1);
        assert_eq!(range.start_column, 1);
        assert_eq!(range.end_line, 1);
        assert_eq!(range.end_column, 3);
    }

    #[test]
    fn context_extracts_structures_with_source_ranges() {
        let content = "---\ntitle: x\n---\n# Title\n\n[text](#title)\n\n| A | B |\n|---|---|\n| C | D |\n\n```md\n# ignored\n| x | y |\n```\n";
        let ctx = DocumentContext::new(Path::new("doc.md"), content);

        assert_eq!(ctx.front_matter(), Some(SourceRange { start: 0, end: 17 }));
        assert_eq!(ctx.headings().len(), 1);
        assert_eq!(ctx.headings()[0].text, "Title");
        assert_eq!(ctx.links().len(), 1);
        assert_eq!(ctx.links()[0].destination, "#title");
        assert_eq!(ctx.inline_links().len(), 1);
        assert_eq!(ctx.inline_links()[0].destination, Some("#title"));
        assert_eq!(ctx.tables().len(), 1);
        assert_eq!(ctx.tables()[0].rows.len(), 3);
        assert_eq!(ctx.code_blocks().len(), 1);
        assert!(!ctx.is_code_line(0));
        assert!(ctx.is_code_line(12));
        assert!(!ctx.is_code_line(99));
        assert_eq!(ctx.markdown_ast().blocks.len(), 3);
    }

    #[test]
    fn context_extracts_source_preserving_inline_tokens() {
        let content = concat!(
            "See [nested [text]](https://example.com/path?q=1 \"title\") ",
            "and ![alt][image-ref] plus <https://example.org>.\n",
            "``[ignored](https://example.invalid)`` and `[ignored][]`\n",
            "`[unclosed](https://example.invalid)\n",
            "[image-ref]: <https://example.org/image.png> \"Image\"\n",
        );
        let ctx = DocumentContext::new(Path::new("doc.md"), content);

        assert_eq!(ctx.inline_code_spans().len(), 3);
        assert!(!ctx.inline_code_spans()[2].closed);
        let links = ctx.inline_links();
        assert_eq!(links.len(), 3);
        assert_eq!(links[0].text, Some("nested [text]"));
        assert_eq!(links[0].destination, Some("https://example.com/path?q=1"));
        assert_eq!(links[1].text, Some("alt"));
        assert_eq!(links[1].effective_label(), Some("image-ref"));
        assert_eq!(links[2].destination, Some("https://example.org"));
        assert_eq!(ctx.reference_definitions().len(), 1);
        assert_eq!(ctx.reference_definitions()[0].label, "image-ref");
        assert_eq!(
            ctx.reference_definitions()[0].destination,
            "https://example.org/image.png"
        );
    }

    #[test]
    fn context_inline_tokens_handle_empty_crlf_and_unicode() {
        let empty_ctx = DocumentContext::new(Path::new("empty.md"), "");
        assert!(empty_ctx.inline_code_spans().is_empty());
        assert!(empty_ctx.inline_links().is_empty());
        assert!(empty_ctx.reference_definitions().is_empty());

        let content = "`コード` [表示](#見出し)\r\n# 見出し\r\n";
        let ctx = DocumentContext::new(Path::new("doc.md"), content);

        assert_eq!(ctx.inline_code_spans().len(), 1);
        assert_eq!(ctx.inline_links().len(), 1);
        assert_eq!(ctx.inline_links()[0].text, Some("表示"));
        assert_eq!(ctx.inline_links()[0].destination, Some("#見出し"));
    }

    #[test]
    fn context_keeps_irregular_table_rows_for_column_rules() {
        let content = "| A | B |\n|---|---|\n| C |\n| D | E | F |\n";
        let ctx = DocumentContext::new(Path::new("doc.md"), content);

        assert_eq!(ctx.tables().len(), 1);
        assert_eq!(ctx.tables()[0].rows.len(), 4);
        assert_eq!(ctx.tables()[0].rows[2].cells.len(), 1);
        assert_eq!(ctx.tables()[0].rows[3].cells.len(), 3);
    }

    #[test]
    fn context_respects_nested_shorter_fences_inside_longer_fences() {
        let content = "````markdown\n```rust\ncode\n```\n````\n";
        let ctx = DocumentContext::new(Path::new("doc.md"), content);

        assert_eq!(ctx.code_blocks().len(), 1);
        assert_eq!(ctx.code_blocks()[0].start_line, 0);
        assert_eq!(ctx.code_blocks()[0].end_line, 4);
        assert!(ctx.is_code_line(1));
        assert!(ctx.is_code_line(3));
    }

    #[test]
    fn ast_is_lazy_and_not_required_for_structural_indexes() {
        let content = "# Title\n\n| A | B |\n|---|---|\n";
        let ctx = DocumentContext::new(Path::new("doc.md"), content);

        assert!(ctx.ast.get().is_none());
        assert!(ctx.headings.get().is_none());
        assert!(ctx.tables.get().is_none());
        assert_eq!(ctx.headings().len(), 1);
        assert_eq!(ctx.tables().len(), 1);
        assert!(ctx.headings.get().is_some());
        assert!(ctx.tables.get().is_some());
        assert!(ctx.ast.get().is_none());

        assert_eq!(ctx.markdown_ast().blocks.len(), 2);
        assert!(ctx.ast.get().is_some());
    }
}
