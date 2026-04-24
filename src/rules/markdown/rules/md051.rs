use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD051 / link-fragments — Link fragments.
pub struct LinkFragmentsRule;

impl MarkdownRule for LinkFragmentsRule {
    fn id(&self) -> &'static str {
        "MD051"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD051")
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
