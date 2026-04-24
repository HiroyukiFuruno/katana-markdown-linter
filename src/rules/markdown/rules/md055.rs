use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD055 / table-pipe-style — Table pipe style.
pub struct TablePipeStyleRule;

impl MarkdownRule for TablePipeStyleRule {
    fn id(&self) -> &'static str {
        "MD055"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD055")
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
