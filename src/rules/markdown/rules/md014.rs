use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD014 / dollar-signs-before-commands — Dollar signs used before commands without spaces.
pub struct DollarSignsBeforeCommandsRule;

impl MarkdownRule for DollarSignsBeforeCommandsRule {
    fn id(&self) -> &'static str {
        "MD014"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD014",
            title: "dollar-signs-before-commands",
            description: "Dollar signs used before commands without spaces.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md014.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD014");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('$')
                && trimmed.chars().nth(1).is_some_and(|next| !next.is_whitespace())
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
