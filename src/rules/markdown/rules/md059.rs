use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD059 / link-text — Prohibited link text.
pub struct ProhibitedLinkTextRule;

impl MarkdownRule for ProhibitedLinkTextRule {
    fn id(&self) -> &'static str {
        "MD059"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD059")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD059");
        let mut diagnostics = Vec::new();
        let prohibited = ["click here", "here", "link", "more"];
        for (i, line) in content.lines().enumerate() {
            if prohibited
                .iter()
                .any(|needle| line.to_lowercase().contains(needle))
            {
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
