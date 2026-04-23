use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD052 / reference-links-images — Reference links and images.
pub struct ReferenceLinksImagesRule;

impl MarkdownRule for ReferenceLinksImagesRule {
    fn id(&self) -> &'static str {
        "MD052"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD052",
            title: "reference-links-images",
            description: "Reference links and images.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md052.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[
                crate::rule_prop!(
                    StringArray,
                    "ignored_labels",
                    "Ignored link labels",
                    "[\"x\"]"
                ),
                crate::rule_prop!(
                    Boolean,
                    "shortcut_syntax",
                    "Include shortcut syntax",
                    "false"
                ),
            ],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD052");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.contains("][]") || line.contains("![][") {
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
