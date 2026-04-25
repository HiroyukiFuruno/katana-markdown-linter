use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD003 / heading-style — Enforce consistent heading style (atx).
pub struct HeadingStyleRule;

impl MarkdownRule for HeadingStyleRule {
    fn id(&self) -> &'static str {
        "MD003"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD003")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD003");
        let mut diagnostics = Vec::new();
        let ctx = DocumentContext::new(file_path, content);
        let lines = ctx.lines().iter().map(|line| line.text).collect::<Vec<_>>();
        for (i, line) in lines.iter().enumerate() {
            if ctx.is_code_line(i) || is_front_matter_line(&ctx, i) {
                continue;
            }
            /* WHY: Setext heading markers only count when attached to paragraph text. */
            if is_setext_heading_marker(&lines, i) {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    file_path,
                    i,
                    line,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}

fn is_front_matter_line(ctx: &DocumentContext<'_>, line_index: usize) -> bool {
    let Some(front_matter) = ctx.front_matter() else {
        return false;
    };
    ctx.lines().get(line_index).is_some_and(|line| {
        front_matter.start <= line.content_range.start && line.full_range.end <= front_matter.end
    })
}

fn is_setext_heading_marker(lines: &[&str], idx: usize) -> bool {
    if idx == 0 || !is_setext_underline(lines[idx].trim()) {
        return false;
    }
    let previous = lines[idx - 1].trim();
    !previous.is_empty()
        && !RuleHelpers::is_fence(previous)
        && !RuleHelpers::is_atx_heading(previous)
        && !is_setext_underline(previous)
}

fn is_setext_underline(trimmed: &str) -> bool {
    if trimmed.len() < 2 {
        return false;
    }
    trimmed.chars().all(|c| c == '=') || trimmed.chars().all(|c| c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_setext_heading_marker() {
        let diagnostics = HeadingStyleRule.evaluate(Path::new("test.md"), "Heading\n---\n");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "MD003");
        assert_eq!(diagnostics[0].range.start_line, 2);
    }

    #[test]
    fn ignores_standalone_horizontal_rule_after_blank_line() {
        let diagnostics =
            HeadingStyleRule.evaluate(Path::new("test.md"), "# Heading\n\n---\n\nText\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_front_matter_delimiters() {
        let diagnostics =
            HeadingStyleRule.evaluate(Path::new("test.md"), "---\ntitle: Doc\n---\n\n# Doc\n");

        assert!(diagnostics.is_empty());
    }
}
