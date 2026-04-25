use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
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
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range,
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: None,
                });
            }
        }

        diagnostics
    }
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
}
