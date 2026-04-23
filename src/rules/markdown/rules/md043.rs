use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD043 / required-headings — Required headings.
pub struct RequiredHeadingsRule;

impl MarkdownRule for RequiredHeadingsRule {
    fn id(&self) -> &'static str {
        "MD043"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD043",
            title: "required-headings",
            description: "Required headings.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md043.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[
                crate::rule_prop!(StringArray, "headings", "List of headings", "[]"),
                crate::rule_prop!(Boolean, "match_case", "Match case of headings", "false"),
            ],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD043");
        let mut diagnostics = Vec::new();
        if !content
            .lines()
            .any(|line| RuleHelpers::is_atx_heading(line.trim_start()))
        {
            RuleHelpers::push_diag(
                &mut diagnostics,
                file_path,
                0,
                content.lines().next().unwrap_or(""),
                &meta,
                DiagnosticSeverity::Warning,
            );
        }
        diagnostics
    }
}
