use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD058 / blanks-around-tables — Tables should be surrounded by blank lines.
pub struct TableSpacingRule;

impl MarkdownRule for TableSpacingRule {
    fn id(&self) -> &'static str {
        "MD058"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD058")
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
        let meta = self.official_meta().expect("always Some for MD058");
        let mut diagnostics = Vec::new();

        for table in ctx.tables() {
            if table.start_line > 0 && !ctx.lines()[table.start_line - 1].text.trim().is_empty() {
                diagnostics.push(table_blank_fix(
                    ctx,
                    table.start_line,
                    &meta,
                    TableBlankFix::Before,
                ));
            }
            if table.end_line + 1 < ctx.lines().len()
                && !ctx.lines()[table.end_line + 1].text.trim().is_empty()
            {
                diagnostics.push(table_blank_fix(
                    ctx,
                    table.end_line,
                    &meta,
                    TableBlankFix::After,
                ));
            }
        }

        diagnostics
    }
}

enum TableBlankFix {
    Before,
    After,
}

fn table_blank_fix(
    ctx: &DocumentContext<'_>,
    line_idx: usize,
    meta: &OfficialRuleMeta,
    kind: TableBlankFix,
) -> MarkdownDiagnostic {
    let line = &ctx.lines()[line_idx];
    let (start_column, replacement) = match kind {
        TableBlankFix::Before => (1, "\n".to_string()),
        TableBlankFix::After => (line.text.len() + 1, "\n".to_string()),
    };
    MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: DiagnosticRange {
            start_line: line_idx + 1,
            start_column,
            end_line: line_idx + 1,
            end_column: start_column,
        },
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
            start_line: line_idx + 1,
            start_column,
            end_line: line_idx + 1,
            end_column: start_column,
            replacement,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_missing_blank_lines_around_table() {
        let rule = TableSpacingRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "Text\n| A | B |\n|---|---|\n| 1 | 2 |\n> Quote",
        );

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.fix_info.is_some()));
    }

    #[test]
    fn accepts_table_at_document_boundaries() {
        let rule = TableSpacingRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "| A |\n|---|\n| 1 |");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_pipe_text_outside_tables_and_code_blocks() {
        let rule = TableSpacingRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "<p align=\"center\">\n  <a href=\"sample.md\">English</a> | 日本語\n</p>\n\n```mermaid\ngraph TD\nB -->|Yes| C\nB -->|No| D\n```\n\nText | not a table\n",
        );

        assert!(diagnostics.is_empty());
    }
}
