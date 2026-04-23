use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD013 / line-length — Line length.
pub struct LineLengthRule;

impl MarkdownRule for LineLengthRule {
    fn id(&self) -> &'static str {
        "MD013"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD013",
            title: "line-length",
            description: "Line length.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md013.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[crate::rule_prop!(
                Number,
                "line_length",
                "Number of characters",
                "80"
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD013");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.len() > 80 {
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
