use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD056 / table-column-count — Table column count consistency.
pub struct TableColumnCountRule;

impl MarkdownRule for TableColumnCountRule {
    fn id(&self) -> &'static str {
        "MD056"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD056",
            title: "table-column-count",
            description: "Table column count consistency.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md056.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD056");
        let mut diagnostics = Vec::new();
        let mut expected_columns: Option<usize> = None;

        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if !trimmed.contains('|') {
                expected_columns = None;
                continue;
            }
            let columns = trimmed
                .split('|')
                .filter(|segment| !segment.trim().is_empty())
                .count();
            if columns == 0 {
                continue;
            }
            if let Some(expected) = expected_columns {
                if columns != expected {
                    RuleHelpers::push_diag(
                        &mut diagnostics,
                        file_path,
                        i,
                        line,
                        &meta,
                        DiagnosticSeverity::Warning,
                    );
                }
            } else {
                expected_columns = Some(columns);
            }
        }

        diagnostics
    }
}
