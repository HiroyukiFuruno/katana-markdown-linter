use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD058 / table-spacing — Table spacing consistency.
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
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with('|') && trimmed.ends_with('|') {
                let middle = trimmed.trim_matches('|');
                if middle.contains("  ") {
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
        }
        diagnostics
    }
}
