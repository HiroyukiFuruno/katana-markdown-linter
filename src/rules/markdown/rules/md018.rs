use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
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
        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let line = line.text;
            let trimmed = line.trim_start();
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
