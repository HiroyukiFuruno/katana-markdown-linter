use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
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
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        _config: Option<&crate::types::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD005");
        let mut diagnostics = Vec::new();
        let mut previous_list_indent: Option<usize> = None;

        let file_path = ctx.file_path();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_literal_block_line(i) {
                previous_list_indent = None;
                continue;
            }
            previous_list_indent = check_list_line(
                &mut diagnostics,
                file_path,
                i,
                line.text,
                &meta,
                previous_list_indent,
            );
        }

        diagnostics
    }
}

fn check_list_line(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    file_path: &Path,
    i: usize,
    line: &str,
    meta: &OfficialRuleMeta,
    previous: Option<usize>,
) -> Option<usize> {
    let trimmed = line.trim_start();
    if !RuleHelpers::is_list_item(trimmed) {
        return None;
    }
    let leading = line.len() - trimmed.len();
    Some(
        push_indent_diagnostic(diagnostics, file_path, i, line, meta, previous, leading)
            .unwrap_or(leading),
    )
}

fn push_indent_diagnostic(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    file_path: &Path,
    line_index: usize,
    line: &str,
    meta: &OfficialRuleMeta,
    previous_list_indent: Option<usize>,
    leading: usize,
) -> Option<usize> {
    let expected = safe_expected_indent(previous_list_indent?, leading)?;
    let fix = crate::rules::markdown::types::DiagnosticFix {
        start_line: line_index + 1,
        start_column: 1,
        end_line: line_index + 1,
        end_column: leading + 1,
        replacement: " ".repeat(expected),
    };
    RuleHelpers::push_diag_with_fix(
        diagnostics,
        file_path,
        line_index,
        line,
        meta,
        DiagnosticSeverity::Warning,
        Some(fix),
    );
    Some(expected)
}

fn safe_expected_indent(previous: usize, current: usize) -> Option<usize> {
    if previous == 0 || current <= previous {
        return None;
    }
    if current - previous == 1 {
        Some(previous)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_one_space_drift_for_adjacent_nested_siblings() {
        let rule = ListIndentRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "- one\n  - nested\n   - inconsistent\n",
        );

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("safe sibling indent should be fixable");
        assert_eq!(fix.start_line, 3);
        assert_eq!(fix.replacement, "  ");
    }

    #[test]
    fn leaves_ambiguous_top_level_to_nested_transition_unfixed() {
        let rule = ListIndentRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "- one\n - maybe nested\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn leaves_non_adjacent_list_items_unfixed() {
        let rule = ListIndentRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "  - nested\n  continuation\n   - not adjacent\n",
        );

        assert!(diagnostics.is_empty());
    }
}
