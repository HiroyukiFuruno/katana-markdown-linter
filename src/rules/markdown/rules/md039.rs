use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD039 / no-space-in-links — Spaces inside link text.
pub struct NoSpacesInLinksRule;

impl MarkdownRule for NoSpacesInLinksRule {
    fn id(&self) -> &'static str {
        "MD039"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD039")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD039");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.contains("[ ") || line.contains(" ]") {
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
