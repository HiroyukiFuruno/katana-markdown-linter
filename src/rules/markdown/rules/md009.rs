use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD009 / no-trailing-spaces — Trailing spaces.
pub struct NoTrailingSpacesRule;

impl MarkdownRule for NoTrailingSpacesRule {
    fn id(&self) -> &'static str {
        "MD009"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD009")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD009");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let trimmed_len = line.trim_end_matches(' ').len();
            if trimmed_len == line.len() {
                continue;
            }

            diagnostics.push(MarkdownDiagnostic {
                file: file_path.to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: i + 1,
                    start_column: trimmed_len + 1,
                    end_line: i + 1,
                    end_column: line.len() + 1,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: trimmed_len + 1,
                    end_line: i + 1,
                    end_column: line.len() + 1,
                    replacement: String::new(),
                }),
            });
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_trailing_spaces() {
        let rule = NoTrailingSpacesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "text   \nnext");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("trailing spaces should be fixable");
        assert_eq!(fix.start_column, 5);
        assert_eq!(fix.end_column, 8);
        assert_eq!(fix.replacement, "");
    }
}
