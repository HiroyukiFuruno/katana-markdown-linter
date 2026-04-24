use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD043 / required-headings — Required headings.
pub struct RequiredHeadingsRule;

impl MarkdownRule for RequiredHeadingsRule {
    fn id(&self) -> &'static str {
        "MD043"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD043")
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
