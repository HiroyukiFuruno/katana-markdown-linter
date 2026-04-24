use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD005 / list-indent — Inconsistent indentation for list items.
pub struct ListIndentRule;

impl MarkdownRule for ListIndentRule {
    fn id(&self) -> &'static str {
        "MD005"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD005")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD005");
        let mut diagnostics = Vec::new();
        let mut in_code_block = false;
        let mut expected_indent: Option<usize> = None;

        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                expected_indent = None;
                continue;
            }
            if in_code_block {
                continue;
            }
            if RuleHelpers::is_list_item(trimmed) {
                let leading = line.len() - trimmed.len();
                if let Some(expected) = expected_indent {
                    if leading != expected {
                        let fix = crate::rules::markdown::types::DiagnosticFix {
                            start_line: i + 1,
                            start_column: 1,
                            end_line: i + 1,
                            end_column: leading + 1,
                            replacement: " ".repeat(expected),
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
                } else {
                    expected_indent = Some(leading);
                }
            }
        }

        diagnostics
    }
}
