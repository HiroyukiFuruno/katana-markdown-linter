use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD039 / no-space-in-links — Spaces inside link text.
pub struct NoSpacesInLinksRule;

impl MarkdownRule for NoSpacesInLinksRule {
    fn id(&self) -> &'static str {
        "MD039"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD039",
            title: "no-space-in-links",
            description: "Spaces inside link text.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md039.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[],
        })
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
