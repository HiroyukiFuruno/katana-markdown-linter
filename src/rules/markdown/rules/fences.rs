use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD031 / blanks-around-fences - Fenced code blocks should be surrounded by blank lines.
pub struct BlanksAroundFencesRule;

impl MarkdownRule for BlanksAroundFencesRule {
    fn id(&self) -> &'static str {
        "MD031"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD031")
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
        let meta = self.official_meta().expect("always Some for MD031");
        let mut diagnostics = Vec::new();

        for block in ctx.code_blocks() {
            if block.start_line > 0 && !ctx.lines()[block.start_line - 1].text.trim().is_empty() {
                diagnostics.push(fence_blank_fix(
                    ctx,
                    block.start_line,
                    &meta,
                    FenceBlankFix::Before,
                ));
            }
            if block.end_line + 1 < ctx.lines().len()
                && !ctx.lines()[block.end_line + 1].text.trim().is_empty()
            {
                diagnostics.push(fence_blank_fix(
                    ctx,
                    block.end_line,
                    &meta,
                    FenceBlankFix::After,
                ));
            }
        }

        diagnostics
    }
}

enum FenceBlankFix {
    Before,
    After,
}

fn fence_blank_fix(
    ctx: &DocumentContext<'_>,
    line_idx: usize,
    meta: &OfficialRuleMeta,
    kind: FenceBlankFix,
) -> MarkdownDiagnostic {
    let line = &ctx.lines()[line_idx];
    let (start_column, replacement) = match kind {
        FenceBlankFix::Before => (1, "\n".to_string()),
        FenceBlankFix::After => (line.text.len() + 1, "\n".to_string()),
    };
    MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: DiagnosticRange {
            start_line: line_idx + 1,
            start_column,
            end_line: line_idx + 1,
            end_column: start_column,
        },
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
            start_line: line_idx + 1,
            start_column,
            end_line: line_idx + 1,
            end_column: start_column,
            replacement,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_fences_without_blank_lines_around_them() {
        let rule = BlanksAroundFencesRule;
        let content = "Paragraph\n```rust\ncode\n```\nNext";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.fix_info.is_some()));
    }

    #[test]
    fn ignores_fence_like_text_inside_fenced_code_blocks() {
        let rule = BlanksAroundFencesRule;
        let content = "\n````markdown\n```rust\ncode\n```\n````\n";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }
}
