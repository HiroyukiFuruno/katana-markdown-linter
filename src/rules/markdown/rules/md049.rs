use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD049 / emphasis-style — Emphasis style.
pub struct EmphasisStyleRule;

impl MarkdownRule for EmphasisStyleRule {
    fn id(&self) -> &'static str {
        "MD049"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD049")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD049");
        if content.contains('*') && content.contains('_') {
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
