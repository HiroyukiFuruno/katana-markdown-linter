use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
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
        let meta = self.official_meta().expect("always Some for MD038");
        let mut diagnostics = Vec::new();

        let ctx = DocumentContext::new(file_path, content);
        for span in ctx.inline_code_spans() {
            if !span.closed {
                continue;
            }
            let line = &ctx.lines()[span.line];
            let inner_start = span.content_range.start - line.content_range.start;
            let inner_end = span.content_range.end - line.content_range.start;
            let inner_text = &line.text[inner_start..inner_end];
            if !(inner_text.starts_with(' ') || inner_text.ends_with(' ')) {
                continue;
            }
            let marker = "`".repeat(span.marker_len);
            let replacement = format!("{marker}{}{marker}", inner_text.trim());
            let range = ctx.diagnostic_range(span.full_range);
            let fix = crate::rules::markdown::types::DiagnosticFix {
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                replacement,
            };
            diagnostics.push(MarkdownDiagnostic {
                file: file_path.to_path_buf(),
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
}
