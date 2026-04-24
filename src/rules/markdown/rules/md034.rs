use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD034 / no-bare-urls — No bare URLs.
pub struct NoBareUrlsRule;

impl MarkdownRule for NoBareUrlsRule {
    fn id(&self) -> &'static str {
        "MD034"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD034")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD034");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.contains("http://") || line.contains("https://") {
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
