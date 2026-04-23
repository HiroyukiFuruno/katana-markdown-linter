use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD059 / link-text — Prohibited link text.
pub struct ProhibitedLinkTextRule;

impl MarkdownRule for ProhibitedLinkTextRule {
    fn id(&self) -> &'static str {
        "MD059"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD059",
            title: "link-text",
            description: "Prohibited link text.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md059.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[],
        })
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
