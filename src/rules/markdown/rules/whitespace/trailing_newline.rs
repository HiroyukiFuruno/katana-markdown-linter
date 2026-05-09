use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleParityStatus,
};
use std::path::Path;

pub struct SingleTrailingNewlineRule;

impl MarkdownRule for SingleTrailingNewlineRule {
    fn id(&self) -> &'static str {
        "MD047"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD047",
            title: "single-trailing-newline",
            description: "Files should end with a single newline character.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md047.md",
            aliases: &["single-trailing-newline"],
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD047");
        if content.is_empty() || content.ends_with('\n') {
            return Vec::new();
        }
        let line_count = content.lines().count();
        let end_column = content.lines().last().map_or(1, |line| line.len() + 1);
        vec![MarkdownDiagnostic {
            file: file_path.to_path_buf(),
            severity: DiagnosticSeverity::Warning,
            range: DiagnosticRange {
                start_line: line_count,
                start_column: 1,
                end_line: line_count,
                end_column: 1,
            },
            message: meta.description.to_string(),
            rule_id: meta.code.to_string(),
            official_meta: Some(meta),
            fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                start_line: line_count,
                start_column: end_column,
                end_line: line_count,
                end_column,
                replacement: "\n".to_string(),
            }),
        }]
    }
}
