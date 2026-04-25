use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD046 / code-block-style — Code block style.
pub struct CodeBlockStyleRule;

impl MarkdownRule for CodeBlockStyleRule {
    fn id(&self) -> &'static str {
        "MD046"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD046")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD046");
        let ctx = DocumentContext::new(file_path, content);
        let has_fenced = !ctx.code_blocks().is_empty();
        let has_indented = ctx
            .lines()
            .iter()
            .enumerate()
            .any(|(idx, line)| is_indented_code_line(&ctx, idx, line.text));
        if has_fenced && has_indented {
            vec![MarkdownDiagnostic {
                file: file_path.to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: crate::rules::markdown::DiagnosticRange {
                    start_line: 1,
                    start_column: 1,
                    end_line: 1,
                    end_column: content.lines().next().unwrap_or("").len().max(1),
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta),
                fix_info: None,
            }]
        } else {
            Vec::new()
        }
    }
}

fn is_indented_code_line(ctx: &DocumentContext<'_>, line_index: usize, line: &str) -> bool {
    !ctx.is_code_line(line_index) && line.starts_with("    ") && !line.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use crate::{lint, LintOptions};

    #[test]
    fn ignores_indented_lines_inside_fenced_diagrams() {
        let content = "# Title\n\n```mermaid\ngraph TD\n    A --> B\n```\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");

        assert!(results.iter().all(|result| result.rule_id != "MD046"));
    }
}
