use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD020 / no-space-in-blockquote — No space inside blockquote marker.
pub struct NoSpaceInBlockquoteRule;

impl MarkdownRule for NoSpaceInBlockquoteRule {
    fn id(&self) -> &'static str {
        "MD020"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD020",
            title: "no-space-in-blockquote",
            description: "No space inside blockquote marker.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md020.md",
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD020");
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
            if trimmed.starts_with('>') && trimmed.get(1..2) == Some(" ") {
                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: line.len() - trimmed.len() + 2,
                    end_line: i + 1,
                    end_column: line.len() - trimmed.len() + 3,
                    replacement: String::new(),
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
