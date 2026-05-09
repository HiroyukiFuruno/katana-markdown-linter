use super::format::{format_simple_row_for_match, line_text, pipe_positions};
use super::options::{ConfiguredStyle, TableStyle, TableStyleOptions};
use crate::rules::markdown::{DocumentContext, TableBlock};

const HEADER_AND_DELIMITER_ROWS: usize = 2;

pub(super) fn table_matches<'a>(
    ctx: &DocumentContext<'a>,
    options: &TableStyleOptions,
    table: &TableBlock<'a>,
) -> bool {
    let style_ok = match options.style {
        ConfiguredStyle::Any => {
            matches_style(ctx, table, TableStyle::Aligned)
                || matches_style(ctx, table, TableStyle::Compact)
                || matches_style(ctx, table, TableStyle::Tight)
        }
        ConfiguredStyle::Style(style) => matches_style(ctx, table, style),
    };
    style_ok && (!options.aligned_delimiter || delimiter_aligned_with_header(ctx, table))
}

fn matches_style<'a>(ctx: &DocumentContext<'a>, table: &TableBlock<'a>, style: TableStyle) -> bool {
    match style {
        TableStyle::Aligned => rows_have_same_pipe_positions(ctx, table),
        TableStyle::Compact => table
            .rows
            .iter()
            .all(|row| line_text(ctx, row).trim() == format_simple_row_for_match(row, " ")),
        TableStyle::Tight => table
            .rows
            .iter()
            .all(|row| line_text(ctx, row).trim() == format_simple_row_for_match(row, "")),
    }
}

fn rows_have_same_pipe_positions<'a>(ctx: &DocumentContext<'a>, table: &TableBlock<'a>) -> bool {
    let Some(first) = table.rows.first() else {
        return true;
    };
    let expected = pipe_positions(line_text(ctx, first));
    table
        .rows
        .iter()
        .all(|row| pipe_positions(line_text(ctx, row)) == expected)
}

fn delimiter_aligned_with_header<'a>(ctx: &DocumentContext<'a>, table: &TableBlock<'a>) -> bool {
    if table.rows.len() < HEADER_AND_DELIMITER_ROWS {
        return true;
    }
    pipe_positions(line_text(ctx, &table.rows[0])) == pipe_positions(line_text(ctx, &table.rows[1]))
}
