use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/* WHY: Section: Content-level markdownlint rule implementations (Extended)
======================================================= */

/// MD042 / no-empty-links — No empty links.
pub struct NoEmptyLinksRule;

impl MarkdownRule for NoEmptyLinksRule {
    fn id(&self) -> &'static str {
        "MD042"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD042")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD042");
        let mut diagnostics = Vec::new();
        let mut in_code_block = false;
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }
            if line.contains("]()") || line.contains("](#)") {
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
