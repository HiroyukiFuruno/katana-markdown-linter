use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD050 / strong-style — Strong style.
pub struct StrongStyleRule;

impl MarkdownRule for StrongStyleRule {
    fn id(&self) -> &'static str {
        "MD050"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD050",
            title: "strong-style",
            description: "Strong style.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md050.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[crate::rule_prop_enum!(
                "style",
                "Strong style",
                "consistent",
                &["consistent", "asterisk", "underscore"]
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD050");
        if content.contains("**") && content.contains("__") {
            return vec![MarkdownDiagnostic {
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
            }];
        }
        Vec::new()
    }
}
