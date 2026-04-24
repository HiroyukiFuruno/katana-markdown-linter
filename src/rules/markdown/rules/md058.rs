use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
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
        let meta = self.official_meta().expect("always Some for MD058");
        let mut diagnostics = Vec::new();
        let lines = content.lines().collect::<Vec<_>>();
        let mut i = 0;

        while i + 1 < lines.len() {
            if !is_table_start(&lines, i) {
                i += 1;
                continue;
            }

            let start = i;
            let mut end = i + 1;
            while end + 1 < lines.len() && is_table_row(lines[end + 1]) {
                end += 1;
            }

            if start > 0 && !lines[start - 1].trim().is_empty() {
                diagnostics.push(table_blank_fix(
                    file_path,
                    start,
                    lines[start],
                    &meta,
                    TableBlankFix::Before,
                ));
            }
            if end + 1 < lines.len() && !lines[end + 1].trim().is_empty() {
                diagnostics.push(table_blank_fix(
                    file_path,
                    end,
                    lines[end],
                    &meta,
                    TableBlankFix::After,
                ));
            }

            i = end + 1;
        }

        diagnostics
    }
}

enum TableBlankFix {
    Before,
    After,
}

fn table_blank_fix(
    file_path: &Path,
    line_idx: usize,
    line: &str,
    meta: &OfficialRuleMeta,
    kind: TableBlankFix,
) -> MarkdownDiagnostic {
    let (start_column, replacement) = match kind {
        TableBlankFix::Before => (1, "\n".to_string()),
        TableBlankFix::After => (line.len() + 1, "\n".to_string()),
    };
    MarkdownDiagnostic {
        file: file_path.to_path_buf(),
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

fn is_table_start(lines: &[&str], index: usize) -> bool {
    is_table_row(lines[index]) && is_table_delimiter(lines[index + 1])
}

fn is_table_row(line: &str) -> bool {
    line.contains('|') && !line.trim().is_empty()
}

fn is_table_delimiter(line: &str) -> bool {
    let trimmed = line.trim().trim_matches('|');
    if trimmed.is_empty() {
        return false;
    }

    trimmed.split('|').all(|cell| {
        let cell = cell.trim();
        let hyphen_count = cell.chars().filter(|ch| *ch == '-').count();
        hyphen_count >= 3 && cell.chars().all(|ch| ch == '-' || ch == ':')
    })
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
}
