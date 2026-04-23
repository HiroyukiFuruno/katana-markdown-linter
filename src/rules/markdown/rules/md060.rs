use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD060 / table-column-style — Table column style.
pub struct TableColumnStyleRule;

impl MarkdownRule for TableColumnStyleRule {
    fn id(&self) -> &'static str {
        "MD060"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD060",
            title: "table-column-style",
            description: "Table column style.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md060.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[
                crate::rule_prop_enum!(
                    "style",
                    "Table column style",
                    "any",
                    &["any", "aligned", "compact", "tight"]
                ),
                crate::rule_prop!(
                    Boolean,
                    "aligned_delimiter",
                    "Aligned delimiter columns",
                    "false"
                ),
            ],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD060");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.contains('|') && trimmed.contains(" | ") {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    file_path,
                    i,
                    line,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}
