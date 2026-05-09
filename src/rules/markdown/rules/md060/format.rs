use super::options::{ConfiguredStyle, TableStyle, TableStyleOptions};
use crate::rules::markdown::{DocumentContext, TableBlock, TableRow};
use unicode_width::UnicodeWidthStr;

const MIN_DELIMITER_WIDTH: usize = 3;

pub(super) fn safe_fix<'a>(
    ctx: &DocumentContext<'a>,
    options: &TableStyleOptions,
    table: &TableBlock<'a>,
) -> Option<String> {
    if !safe_to_fix(table) {
        return None;
    }
    let style = match options.style {
        ConfiguredStyle::Style(style) => style,
        ConfiguredStyle::Any => closest_style(ctx, table),
    };
    let mut replacement = match style {
        TableStyle::Aligned => table
            .rows
            .iter()
            .map(|row| format_aligned_row(table, row))
            .collect::<Vec<_>>()
            .join("\n"),
        TableStyle::Compact => table
            .rows
            .iter()
            .map(|row| format_simple_row(row, " "))
            .collect::<Vec<_>>()
            .join("\n"),
        TableStyle::Tight => table
            .rows
            .iter()
            .map(|row| format_simple_row(row, ""))
            .collect::<Vec<_>>()
            .join("\n"),
    };
    if ctx.content()[table.range.start..table.range.end].ends_with('\n') {
        replacement.push('\n');
    }
    Some(replacement)
}

fn safe_to_fix(table: &TableBlock<'_>) -> bool {
    let Some(first) = table.rows.first() else {
        return false;
    };
    table.rows.iter().all(|row| {
        row.safe_to_fix
            && row.cells.len() == first.cells.len()
            && row.leading_pipe == first.leading_pipe
            && row.trailing_pipe == first.trailing_pipe
    })
}

fn closest_style<'a>(ctx: &DocumentContext<'a>, table: &TableBlock<'a>) -> TableStyle {
    let compact_distance = table
        .rows
        .iter()
        .filter(|row| line_text(ctx, row).trim() != format_simple_row(row, " "))
        .count();
    let tight_distance = table
        .rows
        .iter()
        .filter(|row| line_text(ctx, row).trim() != format_simple_row(row, ""))
        .count();
    if tight_distance < compact_distance {
        TableStyle::Tight
    } else {
        TableStyle::Compact
    }
}

fn format_simple_row(row: &TableRow<'_>, padding: &str) -> String {
    let cells = row
        .cells
        .iter()
        .map(|cell| {
            if row.delimiter {
                normalize_delimiter(cell.text, MIN_DELIMITER_WIDTH)
            } else {
                cell.text.to_string()
            }
        })
        .collect::<Vec<_>>();
    render_cells(row, &cells, padding)
}

pub(super) fn format_simple_row_for_match(row: &TableRow<'_>, padding: &str) -> String {
    let cells = row
        .cells
        .iter()
        .map(|cell| cell.text.to_string())
        .collect::<Vec<_>>();
    render_cells(row, &cells, padding)
}

fn render_cells(row: &TableRow<'_>, cells: &[String], padding: &str) -> String {
    let separator = format!("{padding}|{padding}");
    let mut output = cells.join(&separator);
    if row.leading_pipe {
        output = format!("|{padding}{output}");
    }
    if row.trailing_pipe {
        output.push_str(padding);
        output.push('|');
    }
    output
}

fn format_aligned_row(table: &TableBlock<'_>, row: &TableRow<'_>) -> String {
    let widths = column_widths(table);
    let mut cells = Vec::new();
    for (index, cell) in row.cells.iter().enumerate() {
        let width = widths[index];
        let text = if row.delimiter {
            normalize_delimiter(cell.text, width.max(MIN_DELIMITER_WIDTH))
        } else {
            cell.text.to_string()
        };
        cells.push(pad_to_width(&text, width));
    }
    let mut output = cells.join(" | ");
    if row.leading_pipe {
        output = format!("| {output}");
    }
    if row.trailing_pipe {
        output.push_str(" |");
    }
    output
}

fn column_widths(table: &TableBlock<'_>) -> Vec<usize> {
    let column_count = table.rows.first().map(|row| row.cells.len()).unwrap_or(0);
    let mut widths = vec![MIN_DELIMITER_WIDTH; column_count];
    for row in &table.rows {
        for (index, cell) in row.cells.iter().enumerate() {
            widths[index] = widths[index].max(display_width(cell.text));
        }
    }
    widths
}

fn normalize_delimiter(cell: &str, width: usize) -> String {
    let left = cell.starts_with(':');
    let right = cell.ends_with(':');
    let dash_count = width.saturating_sub(usize::from(left) + usize::from(right));
    format!(
        "{}{}{}",
        if left { ":" } else { "" },
        "-".repeat(dash_count.max(MIN_DELIMITER_WIDTH)),
        if right { ":" } else { "" }
    )
}

pub(super) fn pipe_positions(line: &str) -> Vec<usize> {
    line.char_indices()
        .filter_map(|(index, char)| (char == '|').then_some(display_width(&line[..index])))
        .collect()
}

pub(super) fn line_text<'a>(ctx: &DocumentContext<'a>, row: &TableRow<'a>) -> &'a str {
    ctx.lines()[row.line].text
}

fn display_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

fn pad_to_width(text: &str, width: usize) -> String {
    let mut output = text.to_string();
    output.push_str(&" ".repeat(width.saturating_sub(display_width(text))));
    output
}
