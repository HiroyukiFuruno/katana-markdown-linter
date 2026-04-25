use crate::rules::markdown::{
    DiagnosticFix, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, TableBlock, TableRow,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD055 / table-pipe-style — Table pipe style.
pub struct TablePipeStyleRule;

impl MarkdownRule for TablePipeStyleRule {
    fn id(&self) -> &'static str {
        "MD055"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        let mut meta = crate::rules::markdown::catalog::get_official_meta("MD055")?;
        meta.is_fixable = true;
        Some(meta)
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
        let meta = self.official_meta().expect("always Some for MD055");
        let configured = configured_pipe_style(config);
        let default_style = ctx
            .tables()
            .first()
            .and_then(|table| table.rows.first())
            .map(row_style);
        ctx.tables()
            .iter()
            .filter_map(|table| evaluate_table(ctx, table, &meta, configured.or(default_style)))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PipeStyle {
    LeadingAndTrailing,
    LeadingOnly,
    NoLeadingOrTrailing,
    TrailingOnly,
}

fn configured_pipe_style(config: Option<&RuleConfig>) -> Option<PipeStyle> {
    config
        .and_then(|config| config.properties.get("style"))
        .and_then(|style| match style.as_str() {
            "leading_and_trailing" => Some(PipeStyle::LeadingAndTrailing),
            "leading_only" => Some(PipeStyle::LeadingOnly),
            "no_leading_or_trailing" => Some(PipeStyle::NoLeadingOrTrailing),
            "trailing_only" => Some(PipeStyle::TrailingOnly),
            _ => None,
        })
}

fn evaluate_table(
    ctx: &DocumentContext<'_>,
    table: &TableBlock<'_>,
    meta: &OfficialRuleMeta,
    expected: Option<PipeStyle>,
) -> Option<MarkdownDiagnostic> {
    let expected = expected?;
    if table.rows.iter().all(|row| row_style(row) == expected) {
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
        fix_info: safe_fix(ctx, table, expected).map(|replacement| DiagnosticFix {
            start_line: range.start_line,
            start_column: range.start_column,
            end_line: range.end_line,
            end_column: range.end_column,
            replacement,
        }),
    })
}

fn safe_fix(
    ctx: &DocumentContext<'_>,
    table: &TableBlock<'_>,
    expected: PipeStyle,
) -> Option<String> {
    if !table.rows.iter().all(|row| row.safe_to_fix) {
        return None;
    }
    let mut replacement = table
        .rows
        .iter()
        .map(|row| format_row(row, expected))
        .collect::<Vec<_>>()
        .join("\n");
    if ctx.content()[table.range.start..table.range.end].ends_with('\n') {
        replacement.push('\n');
    }
    Some(replacement)
}

fn row_style(row: &TableRow<'_>) -> PipeStyle {
    match (row.leading_pipe, row.trailing_pipe) {
        (true, true) => PipeStyle::LeadingAndTrailing,
        (true, false) => PipeStyle::LeadingOnly,
        (false, true) => PipeStyle::TrailingOnly,
        (false, false) => PipeStyle::NoLeadingOrTrailing,
    }
}

fn format_row(row: &TableRow<'_>, style: PipeStyle) -> String {
    let inner = row
        .cells
        .iter()
        .map(|cell| cell.text)
        .collect::<Vec<_>>()
        .join(" | ");
    match style {
        PipeStyle::LeadingAndTrailing => format!("| {inner} |"),
        PipeStyle::LeadingOnly => format!("| {inner}"),
        PipeStyle::TrailingOnly => format!("{inner} |"),
        PipeStyle::NoLeadingOrTrailing => inner.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{fix_with_results, lint, LintOptions, RuleConfig};
    use std::collections::HashMap;

    fn md055_options(style: &str) -> LintOptions {
        let mut rules = HashMap::new();
        rules.insert(
            "MD055".to_string(),
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
    fn fixes_table_pipe_style_to_leading_and_trailing() {
        let content = "| A | B |\n|---|---\n  C | D\n";
        let results = lint(content, &md055_options("leading_and_trailing")).expect("lint runs");
        let md055 = results
            .iter()
            .find(|result| result.rule_id == "MD055")
            .expect("MD055 diagnostic exists");

        assert!(md055.fix.is_some());
        let fixed = fix_with_results(content, &results);
        assert_eq!(fixed.content, "| A | B |\n| --- | --- |\n| C | D |\n");
    }

    #[test]
    fn ignores_pipe_text_inside_code_fences() {
        let content = "```md\nA | B\n---|---\n```\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");

        assert!(results.iter().all(|result| result.rule_id != "MD055"));
    }

    #[test]
    fn ignores_html_links_and_mermaid_pipes_that_are_not_tables() {
        let content = concat!(
            "<p align=\"center\">\n",
            "  <a href=\"sample_diagrams.md\">English</a> | 日本語\n",
            "</p>\n\n",
            "```mermaid\n",
            "graph TD\n",
            "    B -->|Yes| C[処理A]\n",
            "    B -->|No| D[処理B]\n",
            "```\n"
        );
        let results = lint(content, &LintOptions::default()).expect("lint runs");

        assert!(results.iter().all(|result| result.rule_id != "MD055"));
    }
}
