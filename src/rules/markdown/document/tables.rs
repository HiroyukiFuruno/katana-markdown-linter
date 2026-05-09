use super::blocks::line_in_blocks;
use super::types::{BlockRange, LineInfo, SourceRange, TableBlock, TableCell, TableRow};

const MIN_DELIMITER_CELL_WIDTH: usize = 3;

pub(super) fn extract_tables<'a>(
    lines: &[LineInfo<'a>],
    code_blocks: &[BlockRange],
) -> Vec<TableBlock<'a>> {
    let mut tables = Vec::new();
    let mut index = 0;
    while index + 1 < lines.len() {
        if line_in_blocks(index, code_blocks) {
            index += 1;
            continue;
        }
        let Some(header) = parse_table_row(index, &lines[index]) else {
            index += 1;
            continue;
        };
        let Some(delimiter) = parse_table_row(index + 1, &lines[index + 1]) else {
            index += 1;
            continue;
        };
        if !delimiter.delimiter || header.cells.len() != delimiter.cells.len() {
            index += 1;
            continue;
        }
        let (rows, end) = collect_table_rows(lines, code_blocks, index, header, delimiter);
        tables.push(TableBlock {
            start_line: index,
            end_line: end,
            range: SourceRange {
                start: lines[index].content_range.start,
                end: lines[end].full_range.end,
            },
            rows,
        });
        index = end + 1;
    }
    tables
}

fn collect_table_rows<'a>(
    lines: &[LineInfo<'a>],
    code_blocks: &[BlockRange],
    index: usize,
    header: TableRow<'a>,
    delimiter: TableRow<'a>,
) -> (Vec<TableRow<'a>>, usize) {
    let mut rows = vec![header, delimiter];
    let mut end = index + 1;
    while end + 1 < lines.len() && !line_in_blocks(end + 1, code_blocks) {
        let Some(next) = parse_table_row(end + 1, &lines[end + 1]) else {
            break;
        };
        rows.push(next);
        end += 1;
    }
    (rows, end)
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
    let cells = parse_table_cells(inner, inner_start);
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

fn parse_table_cells<'a>(inner: &'a str, inner_start: usize) -> Vec<TableCell<'a>> {
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
    cells
}

fn is_delimiter_cell(cell: &str) -> bool {
    let trimmed = cell.trim();
    let core = trimmed.trim_matches(':');
    core.len() >= MIN_DELIMITER_CELL_WIDTH
        && core.bytes().all(|byte| byte == b'-')
        && trimmed.bytes().all(|byte| byte == b'-' || byte == b':')
}
