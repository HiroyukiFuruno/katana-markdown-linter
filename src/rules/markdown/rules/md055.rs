use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD055 / table-pipe-style — Table pipe style.
pub struct TablePipeStyleRule;

impl MarkdownRule for TablePipeStyleRule {
    fn id(&self) -> &'static str {
        "MD055"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD055",
            title: "table-pipe-style",
            description: "Table pipe style.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md055.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[crate::rule_prop_enum!(
                "style",
                "Table pipe style",
                "consistent",
                &[
                    "consistent",
                    "leading_only",
                    "trailing_only",
                    "leading_and_trailing",
                    "no_leading_or_trailing"
                ]
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD055");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.contains('|') && !(trimmed.starts_with('|') && trimmed.ends_with('|')) {
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
