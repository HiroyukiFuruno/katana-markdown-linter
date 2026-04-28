use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    SourceRange,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD045 / no-alt-text — Images should have alternate text (alt text).
pub struct NoAltTextRule;

impl MarkdownRule for NoAltTextRule {
    fn id(&self) -> &'static str {
        "MD045"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD045")
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
        let meta = self.official_meta().expect("always Some for MD045");
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            if has_empty_alt_image_outside_inline_code(ctx, line.content_range.start, line.text) {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    ctx.file_path(),
                    i,
                    line.text,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}

fn has_empty_alt_image_outside_inline_code(
    ctx: &DocumentContext<'_>,
    line_start: usize,
    line: &str,
) -> bool {
    line.match_indices("![]").any(|(index, token)| {
        let start = line_start + index;
        !ctx.is_inside_inline_code(SourceRange {
            start,
            end: start + token.len(),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_missing_alt_text_inside_fenced_code_blocks() {
        let rule = NoAltTextRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```md\n![](image.png)\n```\n");

        assert!(diagnostics.is_empty());
    }
}
