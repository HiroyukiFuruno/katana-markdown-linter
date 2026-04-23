use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD019 / no-multiple-space-atx — Multiple spaces after hash on ATX heading.
pub struct NoMultipleSpaceAtxRule;

impl MarkdownRule for NoMultipleSpaceAtxRule {
    fn id(&self) -> &'static str {
        "MD019"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD019",
            title: "no-multiple-space-atx",
            description: "Multiple spaces after hash on ATX heading.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md019.md",
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD019");
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
            let hash_count = trimmed.chars().take_while(|c| *c == '#').count();
            let has_spaces = trimmed
                .get(hash_count..)
                .is_some_and(|rest| rest.starts_with("  "));
            if hash_count > 0 && has_spaces {
                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: line.len() - trimmed.len() + hash_count + 1,
                    end_line: i + 1,
                    end_column: line.len() - trimmed.len() + hash_count + 3,
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
