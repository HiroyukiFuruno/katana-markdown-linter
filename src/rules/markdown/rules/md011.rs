use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use regex::Regex;
use std::path::Path;

/// MD011 / no-reversed-links — Reversed link syntax.
pub struct NoReversedLinksRule;

impl MarkdownRule for NoReversedLinksRule {
    fn id(&self) -> &'static str {
        "MD011"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD011")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD011");
        let pattern = Regex::new(r"\]\([^)]+?\[").expect("valid regex");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if pattern.is_match(line) {
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
