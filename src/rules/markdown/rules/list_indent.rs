use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD007 / ul-indent — Unordered list indentation.
pub struct UnorderedListIndentRule;

impl MarkdownRule for UnorderedListIndentRule {
    fn id(&self) -> &'static str {
        "MD007"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD007")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD007");
        let mut diagnostics = Vec::new();
        let indent = 2;

        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let line = line.text;
            let trimmed = line.trim_start();

            if RuleHelpers::get_bullet_char(trimmed).is_some() {
                let leading = line.len() - trimmed.len();
                if leading % indent != 0 {
                    let fix = crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: 1,
                        end_line: i + 1,
                        end_column: leading.saturating_add(1),
                        replacement: " ".repeat(leading / indent * indent),
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
        }

        diagnostics
    }
}
