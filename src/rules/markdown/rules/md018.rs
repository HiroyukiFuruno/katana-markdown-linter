use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD018 / no-missing-space-atx — No space after hash on ATX heading.
pub struct NoMissingSpaceAtxRule;

impl MarkdownRule for NoMissingSpaceAtxRule {
    fn id(&self) -> &'static str {
        "MD018"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD018")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD018");
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
            if trimmed.starts_with('#') && !RuleHelpers::is_atx_heading(trimmed) {
                let hash_count = trimmed.chars().take_while(|c| *c == '#').count();
                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: line.len() - trimmed.len() + hash_count + 1,
                    end_line: i + 1,
                    end_column: line.len() - trimmed.len() + hash_count + 1,
                    replacement: " ".to_string(),
                };
                RuleHelpers::push_diag_with_fix(
                    &mut diagnostics,
                    file_path,
                    i,
                    line,
                    &meta,
                    DiagnosticSeverity::Error,
                    Some(fix),
                );
            }
        }
        diagnostics
    }
}
