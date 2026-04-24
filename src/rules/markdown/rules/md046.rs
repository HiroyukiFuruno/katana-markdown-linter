use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD046 / code-block-style — Code block style.
pub struct CodeBlockStyleRule;

impl MarkdownRule for CodeBlockStyleRule {
    fn id(&self) -> &'static str {
        "MD046"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD046")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD046");
        let has_fenced = content
            .lines()
            .any(|line| RuleHelpers::is_fence(line.trim_start()));
        let has_indented = content
            .lines()
            .any(|line| line.starts_with("    ") && !line.trim().is_empty());
        if has_fenced && has_indented {
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
