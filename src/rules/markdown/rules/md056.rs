use crate::rules::markdown::{
    DiagnosticFix, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, TableRow,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD056 / table-column-count — Table column count consistency.
pub struct TableColumnCountRule;

impl MarkdownRule for TableColumnCountRule {
    fn id(&self) -> &'static str {
        "MD056"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD056")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        _config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD056");
        let mut diagnostics = Vec::new();

        for table in ctx.tables() {
            let Some(header) = table.rows.first() else {
                continue;
            };
            let expected_columns = header.cells.len();
            for row in &table.rows {
                if row.cells.len() == expected_columns {
                    continue;
                }
                let range = ctx.diagnostic_range(row.range);
                let fix_info =
                    safe_pad_fix(row, expected_columns).map(|replacement| DiagnosticFix {
                        start_line: range.start_line,
                        start_column: range.start_column,
                        end_line: range.end_line,
                        end_column: range.end_column,
                        replacement,
                    });
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range,
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info,
                });
            }
        }

        diagnostics
    }
}

fn safe_pad_fix(row: &TableRow<'_>, expected_columns: usize) -> Option<String> {
    if !row.safe_to_fix {
        return None;
    }
    if row.cells.len() >= expected_columns {
        return None;
    }
    let missing = expected_columns - row.cells.len();
    let mut texts: Vec<&str> = row.cells.iter().map(|c| c.text).collect();
    texts.extend(vec![""; missing]);
    let inner = texts.join(" | ");
    Some(match (row.leading_pipe, row.trailing_pipe) {
        (true, true) => format!("| {inner} |"),
        (true, false) => format!("| {inner}"),
        (false, true) => format!("{inner} |"),
        (false, false) => inner,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_mismatched_columns_inside_a_table() {
        let rule = TableColumnCountRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "| A | B |\n|---|---|\n| C |\n");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].range.start_line, 3);
    }

    #[test]
    fn ignores_pipe_text_outside_tables_and_code_blocks() {
        let rule = TableColumnCountRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "<p align=\"center\">\n  <a href=\"sample.md\">English</a> | 日本語\n</p>\n\n```mermaid\ngraph TD\nB -->|Yes| C\nB -->|No| D\n```\n\nText | not a table\n",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn fix_pads_short_row_with_empty_cells() {
        let rule = TableColumnCountRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "| a | b |\n|---|---|\n| 1 |\n");
        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("short row should be fixable");
        assert_eq!(fix.replacement, "| 1 |  |");
    }

    #[test]
    fn fix_skips_overflow_row_to_avoid_data_loss() {
        let rule = TableColumnCountRule;
        let diagnostics =
            rule.evaluate(Path::new("doc.md"), "| a | b |\n|---|---|\n| 1 | 2 | 3 |\n");
        assert_eq!(diagnostics.len(), 1);
        assert!(
            diagnostics[0].fix_info.is_none(),
            "overflow row should not be auto-fixed"
        );
    }

    #[test]
    fn fix_preserves_pipe_style_no_leading_or_trailing() {
        let rule = TableColumnCountRule;
        // 先頭・末尾パイプなし: 3列ヘッダーに対して2列行
        let diagnostics = rule.evaluate(Path::new("doc.md"), "a | b | c\n---|---|---\n1 | 2\n");
        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("short row should be fixable");
        assert_eq!(fix.replacement, "1 | 2 | ");
    }

    #[test]
    fn fix_skips_unsafe_row_with_escaped_pipe() {
        let rule = TableColumnCountRule;
        // escaped pipe を含む行は safe_to_fix=false — 3列ヘッダーに対して2列行（\| で分割されるセル）
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "| a | b | c |\n|---|---|---|\n| 1 \\| 2 |\n",
        );
        assert_eq!(diagnostics.len(), 1);
        assert!(
            diagnostics[0].fix_info.is_none(),
            "row with escaped pipe should not be auto-fixed"
        );
    }
}
