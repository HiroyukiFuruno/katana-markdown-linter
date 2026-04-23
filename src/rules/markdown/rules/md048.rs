use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD048 / code-fence-style — Code fence style.
pub struct CodeFenceStyleRule;

impl MarkdownRule for CodeFenceStyleRule {
    fn id(&self) -> &'static str {
        "MD048"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD048",
            title: "code-fence-style",
            description: "Code fence style.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md048.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[crate::rule_prop_enum!(
                "style",
                "Code fence style",
                "consistent",
                &["consistent", "backtick", "tilde"]
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD048");
        let has_backtick = content
            .lines()
            .any(|line| line.trim_start().starts_with("```"));
        let has_tilde = content
            .lines()
            .any(|line| line.trim_start().starts_with("~~~"));
        if has_backtick && has_tilde {
            vec![MarkdownDiagnostic {
                file: file_path.to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: crate::rules::markdown::DiagnosticRange {
                    start_line: 1,
                    start_column: 1,
                    end_line: 1,
                    end_column: content.lines().next().unwrap_or("").len().max(1),
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta),
                fix_info: None,
            }]
        } else {
            Vec::new()
        }
    }
}
