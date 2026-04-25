use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD014 / dollar-signs-before-commands — Dollar signs used before commands without spaces.
pub struct DollarSignsBeforeCommandsRule;

impl MarkdownRule for DollarSignsBeforeCommandsRule {
    fn id(&self) -> &'static str {
        "MD014"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD014")
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
        let meta = self.official_meta().expect("always Some for MD014");
        let mut diagnostics = Vec::new();
        for line_index in dollar_prompt_only_code_lines(ctx) {
            let line = &ctx.lines()[line_index];
            let trimmed = line.text.trim_start();
            let indent = line.text.len() - trimmed.len();
            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: line.number,
                    start_column: indent + 1,
                    end_line: line.number,
                    end_column: indent + 3,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                    start_line: line.number,
                    start_column: indent + 1,
                    end_line: line.number,
                    end_column: indent + 3,
                    replacement: String::new(),
                }),
            });
        }
        diagnostics
    }
}

fn dollar_prompt_only_code_lines(ctx: &DocumentContext<'_>) -> Vec<usize> {
    let mut lines = Vec::new();
    for block in ctx.code_blocks() {
        let mut block_lines = Vec::new();
        let mut has_output = false;
        for line_index in block.start_line..=block.end_line {
            let line = &ctx.lines()[line_index];
            let trimmed = line.text.trim_start();
            if trimmed.is_empty() || RuleHelpers::is_fence(trimmed) {
                continue;
            }
            if is_dollar_prompt(trimmed) {
                block_lines.push(line_index);
            } else {
                has_output = true;
                break;
            }
        }
        if !has_output && !block_lines.is_empty() {
            lines.extend(block_lines);
        }
    }
    lines
}

fn is_dollar_prompt(trimmed: &str) -> bool {
    trimmed.starts_with("$ ") && trimmed.len() > 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_dollar_sign_before_code_block_commands() {
        let rule = DollarSignsBeforeCommandsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```\n$ ls\n$ cat foo\n```\n");

        assert_eq!(diagnostics.len(), 2);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("dollar prompt should be fixable");
        assert_eq!(fix.replacement, "");
        assert_eq!(fix.start_column, 1);
        assert_eq!(fix.end_column, 3);
    }

    #[test]
    fn ignores_latex_math_delimiters_and_inline_examples() {
        let rule = DollarSignsBeforeCommandsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "$$ \\sum {k=1}^{n} k = \\frac{n(n+1)}{2} $$\n\n`$echo hi`\n",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_code_blocks_with_command_output() {
        let rule = DollarSignsBeforeCommandsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```\n$ ls\nfoo\n$ cat bar\n```\n");

        assert!(diagnostics.is_empty());
    }
}
