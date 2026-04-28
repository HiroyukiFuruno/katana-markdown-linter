use crate::rules::markdown::{
    DiagnosticFix, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, TableBlock, TableRow,
};
use crate::types::RuleConfig;
use std::path::Path;
use unicode_width::UnicodeWidthStr;

/// MD060 / table-column-style - Table column style.
pub struct TableColumnStyleRule;

impl MarkdownRule for TableColumnStyleRule {
    fn id(&self) -> &'static str {
        "MD060"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD060")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD060");
        let options = TableStyleOptions::from_config(config);
        ctx.tables()
            .iter()
            .filter_map(|table| evaluate_table(ctx, &meta, &options, table))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TableStyle {
    Aligned,
    Compact,
    Tight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConfiguredStyle {
    Any,
    Style(TableStyle),
}

struct TableStyleOptions {
    style: ConfiguredStyle,
    aligned_delimiter: bool,
}

impl TableStyleOptions {
    fn from_config(config: Option<&RuleConfig>) -> Self {
        let style = config
            .and_then(|config| config.properties.get("style"))
            .map(|style| match style.as_str() {
                "aligned" => ConfiguredStyle::Style(TableStyle::Aligned),
                "compact" => ConfiguredStyle::Style(TableStyle::Compact),
                "tight" => ConfiguredStyle::Style(TableStyle::Tight),
                _ => ConfiguredStyle::Any,
            })
            .unwrap_or(ConfiguredStyle::Any);
        let aligned_delimiter = config
            .and_then(|config| config.properties.get("aligned_delimiter"))
            .is_some_and(|value| value == "true");
        Self {
            style,
            aligned_delimiter,
        }
    }
}

fn evaluate_table<'a>(
    ctx: &DocumentContext<'a>,
    meta: &OfficialRuleMeta,
    options: &TableStyleOptions,
    table: &TableBlock<'a>,
) -> Option<MarkdownDiagnostic> {
    if table_matches(ctx, options, table) {
        return None;
    }
    let range = ctx.diagnostic_range(table.range);
    Some(MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: range.clone(),
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: safe_fix(ctx, options, table).map(|replacement| DiagnosticFix {
            start_line: range.start_line,
            start_column: range.start_column,
            end_line: range.end_line,
            end_column: range.end_column,
            replacement,
        }),
    })
}

fn table_matches<'a>(
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

fn safe_fix<'a>(
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
                normalize_delimiter(cell.text, 3)
            } else {
                cell.text.to_string()
            }
        })
        .collect::<Vec<_>>();
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

fn format_simple_row_for_match(row: &TableRow<'_>, padding: &str) -> String {
    let cells = row
        .cells
        .iter()
        .map(|cell| cell.text.to_string())
        .collect::<Vec<_>>();
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
    for (idx, cell) in row.cells.iter().enumerate() {
        let width = widths[idx];
        let text = if row.delimiter {
            normalize_delimiter(cell.text, width.max(3))
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
    let mut widths = vec![3; column_count];
    for row in &table.rows {
        for (idx, cell) in row.cells.iter().enumerate() {
            widths[idx] = widths[idx].max(display_width(cell.text));
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
        "-".repeat(dash_count.max(3)),
        if right { ":" } else { "" }
    )
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
    if table.rows.len() < 2 {
        return true;
    }
    pipe_positions(line_text(ctx, &table.rows[0])) == pipe_positions(line_text(ctx, &table.rows[1]))
}

fn pipe_positions(line: &str) -> Vec<usize> {
    line.char_indices()
        .filter_map(|(idx, char)| (char == '|').then_some(display_width(&line[..idx])))
        .collect()
}

fn line_text<'a>(ctx: &DocumentContext<'a>, row: &TableRow<'a>) -> &'a str {
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

#[cfg(test)]
mod tests {
    use crate::{fix_with_results, lint, LintOptions, RuleConfig};
    use std::collections::HashMap;

    fn md060_options(style: &str) -> LintOptions {
        let mut rules = HashMap::new();
        rules.insert(
            "MD060".to_string(),
            RuleConfig {
                enabled: true,
                properties: HashMap::from([("style".to_string(), style.to_string())]),
            },
        );
        LintOptions {
            rules,
            ..LintOptions::default()
        }
    }

    #[test]
    fn accepts_official_table_styles() {
        for (style, content) in [
            ("aligned", "| A | B |\n|---|---|\n| C | D |\n"),
            ("compact", "| A | B |\n| --- | --- |\n| C | D |\n"),
            ("tight", "|A|B|\n|---|---|\n|C|D|\n"),
        ] {
            let results = lint(content, &md060_options(style)).expect("lint runs");
            assert!(
                results.iter().all(|result| result.rule_id != "MD060"),
                "{style} should pass"
            );
        }
    }

    #[test]
    fn fixes_table_style_from_context_range() {
        let content = "|A|B|\n|---|---|\n|C|D|\n";
        let options = md060_options("compact");
        let results = lint(content, &options).expect("lint runs");
        let md060 = results
            .iter()
            .find(|result| result.rule_id == "MD060")
            .expect("MD060 diagnostic exists");

        assert!(md060.fix.is_some());
        let fixed = fix_with_results(content, &results);
        assert_eq!(fixed.content, "| A | B |\n| --- | --- |\n| C | D |\n");
    }
}
