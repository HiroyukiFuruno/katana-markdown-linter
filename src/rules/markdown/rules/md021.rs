use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD021 / no-multiple-space-blockquote — Multiple spaces after blockquote marker.
pub struct NoMultipleSpaceInBlockquoteRule;

impl MarkdownRule for NoMultipleSpaceInBlockquoteRule {
    fn id(&self) -> &'static str {
        "MD021"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD021")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD021");
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
            if trimmed.starts_with('>') && trimmed.get(1..3) == Some("  ") {
                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: line.len() - trimmed.len() + 2,
                    end_line: i + 1,
                    end_column: line.len() - trimmed.len() + 3,
                    replacement: " ".to_string(),
                };
                RuleHelpers::push_diag_with_fix(
                    &mut diagnostics,
                    file_path,
                    i,
                    line,
                    &meta,
                    DiagnosticSeverity::Warning,
                    Some(fix),
                );
            }
        }
        diagnostics
    }
}
