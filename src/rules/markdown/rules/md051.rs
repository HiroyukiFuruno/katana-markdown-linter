use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD051 / link-fragments — Link fragments.
pub struct LinkFragmentsRule;

impl MarkdownRule for LinkFragmentsRule {
    fn id(&self) -> &'static str {
        "MD051"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD051",
            title: "link-fragments",
            description: "Link fragments.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md051.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[
                crate::rule_prop!(Boolean, "ignore_case", "Ignore case of fragments", "false"),
                crate::rule_prop!(
                    String,
                    "ignored_pattern",
                    "Pattern for ignoring additional fragments",
                    ""
                ),
            ],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD051");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.contains("(#") {
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
