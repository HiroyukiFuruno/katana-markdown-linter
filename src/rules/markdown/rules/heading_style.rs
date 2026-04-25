use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
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
        let lines: Vec<&str> = content.lines().collect();
        let front_matter = detect_front_matter(&lines);
        let mut in_code_block = false;
        for (i, line) in lines.iter().enumerate() {
            if front_matter.is_some_and(|(start, end)| (start..=end).contains(&i)) {
                continue;
            }
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
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

fn detect_front_matter(lines: &[&str]) -> Option<(usize, usize)> {
    if lines.first().map(|line| line.trim()) != Some("---") {
        return None;
    }
    lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| matches!(line.trim(), "---" | "..."))
        .map(|(end, _)| (0, end))
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
