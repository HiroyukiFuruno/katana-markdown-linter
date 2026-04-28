use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

/* WHY: Section: List-related markdownlint rule implementations (Extended)
======================================================= */

/// MD032 / blanks-around-lists — Lists should be surrounded by blank lines.
pub struct BlanksAroundListsRule;

impl MarkdownRule for BlanksAroundListsRule {
    fn id(&self) -> &'static str {
        "MD032"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD032")
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
        let meta = self.official_meta().expect("always Some for MD032");
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            let trimmed = line.text.trim_start();
            if ctx.is_code_line(i) || !RuleHelpers::is_list_item(trimmed) {
                continue;
            }
            let prev_is_problem = i > 0
                && !ctx.lines()[i - 1].text.trim().is_empty()
                && !RuleHelpers::is_list_item(ctx.lines()[i - 1].text.trim_start())
                && !previous_line_continues_list_item(ctx, i);
            if prev_is_problem {
                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: 1,
                    end_line: i + 1,
                    end_column: 1,
                    replacement: "\n".to_string(),
                };
                RuleHelpers::push_diag_with_fix(
                    &mut diagnostics,
                    ctx.file_path(),
                    i,
                    line.text,
                    &meta,
                    DiagnosticSeverity::Warning,
                    Some(fix),
                );
            }
        }
        diagnostics
    }
}

fn previous_line_continues_list_item(ctx: &DocumentContext<'_>, index: usize) -> bool {
    let mut previous_index = index.saturating_sub(1);
    loop {
        let previous = ctx.lines()[previous_index].text;
        let previous_trimmed = previous.trim_start();
        if previous_trimmed.trim().is_empty() {
            return false;
        }
        if RuleHelpers::is_list_item(previous_trimmed) {
            return true;
        }
        if previous_index == 0 {
            return false;
        }
        previous_index -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_list_markers_inside_fenced_code_blocks() {
        let rule = BlanksAroundListsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```md\ntext\n- code\n```\n");

        assert!(diagnostics.is_empty());
    }
}
