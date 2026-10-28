use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::path::Path;

/// MD038 / no-space-in-code — Spaces inside code span elements
pub struct NoSpaceInCodeRule;

impl MarkdownRule for NoSpaceInCodeRule {
    fn id(&self) -> &'static str {
        "MD038"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD038")
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
        let meta = self.official_meta().expect("always Some for MD038");
        let mut diagnostics = Vec::new();

        for span in ctx.inline_code_spans() {
            if !span.closed {
                continue;
            }
            let inner_text = &ctx.content()[span.content_range.start..span.content_range.end];
            if !(inner_text.starts_with(' ') || inner_text.ends_with(' ')) {
                continue;
            }
            if trim_requires_padding_space(inner_text) || inner_text.trim().is_empty() {
                continue;
            }
            let marker = "`".repeat(span.marker_len);
            let replacement = format!("{marker}{}{marker}", inner_text.trim_matches(' '));
            let range = ctx.diagnostic_range(span.full_range);
            let fix = crate::rules::markdown::types::DiagnosticFix {
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                replacement,
            };
            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range,
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: Some(fix),
            });
        }

        diagnostics
    }
}

fn trim_requires_padding_space(inner_text: &str) -> bool {
    let trimmed = inner_text.trim();
    trimmed.starts_with('`') || trimmed.ends_with('`')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_spaces_inside_long_code_span() {
        let rule = NoSpaceInCodeRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "`` spaced ``");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("code span should be fixable");
        assert_eq!(fix.replacement, "``spaced``");
    }

    #[test]
    fn ignores_unclosed_code_span() {
        let rule = NoSpaceInCodeRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "` spaced\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_padding_spaces_needed_to_show_backtick_literals() {
        let rule = NoSpaceInCodeRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "` ``` ` and ` ```mermaid `");

        assert!(diagnostics.is_empty());
    }
}
