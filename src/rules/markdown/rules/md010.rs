use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD010 / no-hard-tabs — Hard tabs.
pub struct NoHardTabsRule;

impl MarkdownRule for NoHardTabsRule {
    fn id(&self) -> &'static str {
        "MD010"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD010")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD010");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            for (column, _) in line.match_indices('\t') {
                diagnostics.push(MarkdownDiagnostic {
                    file: file_path.to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: i + 1,
                        start_column: column + 1,
                        end_line: i + 1,
                        end_column: column + 2,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: column + 1,
                        end_line: i + 1,
                        end_column: column + 2,
                        replacement: " ".to_string(),
                    }),
                });
            }
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_hard_tabs_with_spaces() {
        let rule = NoHardTabsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "a\tb\tc");

        assert_eq!(diagnostics.len(), 2);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("hard tab should be fixable");
        assert_eq!(fix.start_column, 2);
        assert_eq!(fix.end_column, 3);
        assert_eq!(fix.replacement, " ");
    }
}
