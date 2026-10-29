use crate::rules::markdown::inline::reversed_links_on_line;
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD011 / no-reversed-links — Reversed link syntax.
pub struct NoReversedLinksRule;

impl MarkdownRule for NoReversedLinksRule {
    fn id(&self) -> &'static str {
        "MD011"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD011")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        _config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD011");
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            for reversed in reversed_links_on_line(line, ctx) {
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: line.number,
                        start_column: reversed.start + 1,
                        end_line: line.number,
                        end_column: reversed.end + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: line.number,
                        start_column: reversed.start + 1,
                        end_line: line.number,
                        end_column: reversed.end + 1,
                        replacement: format!("[{}]({})", reversed.text, reversed.destination),
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
    fn fixes_reversed_link_syntax() {
        let rule = NoReversedLinksRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "(Example)[https://www.example.com/] and text",
        );

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("reversed link should be fixable");
        assert_eq!(fix.replacement, "[Example](https://www.example.com/)");
    }

    #[test]
    fn ignores_markdown_extra_footnotes() {
        let rule = NoReversedLinksRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "For (example)[^1]");

        assert!(diagnostics.is_empty());
    }
}
