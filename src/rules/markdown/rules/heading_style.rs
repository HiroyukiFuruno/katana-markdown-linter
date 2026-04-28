use crate::rules::markdown::document::LineInfo;
use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticFix, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta,
};
use std::path::Path;

/// MD003 / heading-style — Enforce consistent heading style (atx).
pub struct HeadingStyleRule;

impl MarkdownRule for HeadingStyleRule {
    fn id(&self) -> &'static str {
        "MD003"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        let mut meta = crate::rules::markdown::catalog::get_official_meta("MD003")?;
        meta.is_fixable = true;
        Some(meta)
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD003");
        let mut diagnostics = Vec::new();
        let ctx = DocumentContext::new(file_path, content);
        let ctx_lines = ctx.lines();
        for i in 0..ctx_lines.len() {
            if ctx.is_code_line(i) || is_front_matter_line(&ctx, i) {
                continue;
            }
            /* WHY: Setext heading markers only count when attached to paragraph text.
             * Diagnostic points to the heading text line (i-1), not the underline (i),
             * matching markdownlint reference implementation behavior. */
            if is_setext_heading_marker(ctx_lines, i) {
                let heading_line = ctx_lines[i - 1].text;
                RuleHelpers::push_diag_with_fix(
                    &mut diagnostics,
                    file_path,
                    i - 1,
                    heading_line,
                    &meta,
                    DiagnosticSeverity::Warning,
                    setext_heading_fix(&ctx, i - 1, i),
                );
            }
        }
        diagnostics
    }
}

fn setext_heading_fix(
    ctx: &DocumentContext<'_>,
    heading_index: usize,
    underline_index: usize,
) -> Option<DiagnosticFix> {
    let heading = ctx.lines().get(heading_index)?;
    let underline = ctx.lines().get(underline_index)?;
    let level = setext_heading_level(underline.text.trim())?;
    let range = crate::rules::markdown::document::SourceRange {
        start: heading.content_range.start,
        end: underline.full_range.end,
    };
    let diagnostic_range = ctx.diagnostic_range(range);
    Some(DiagnosticFix {
        start_line: diagnostic_range.start_line,
        start_column: diagnostic_range.start_column,
        end_line: diagnostic_range.end_line,
        end_column: diagnostic_range.end_column,
        replacement: atx_heading_replacement(ctx, heading, level, range),
    })
}

fn setext_heading_level(trimmed: &str) -> Option<usize> {
    if trimmed.chars().all(|c| c == '=') {
        Some(1)
    } else if trimmed.chars().all(|c| c == '-') {
        Some(2)
    } else {
        None
    }
}

fn atx_heading_replacement(
    ctx: &DocumentContext<'_>,
    heading: &LineInfo<'_>,
    level: usize,
    range: crate::rules::markdown::document::SourceRange,
) -> String {
    let indent_len = heading.text.len() - heading.text.trim_start().len();
    let indent = &heading.text[..indent_len];
    let mut replacement = format!("{indent}{} {}", "#".repeat(level), heading.text.trim());
    let replaced = &ctx.content()[range.start..range.end];
    if replaced.ends_with("\r\n") {
        replacement.push_str("\r\n");
    } else if replaced.ends_with('\n') {
        replacement.push('\n');
    }
    replacement
}

fn is_front_matter_line(ctx: &DocumentContext<'_>, line_index: usize) -> bool {
    let Some(front_matter) = ctx.front_matter() else {
        return false;
    };
    ctx.lines().get(line_index).is_some_and(|line| {
        front_matter.start <= line.content_range.start && line.full_range.end <= front_matter.end
    })
}

fn is_setext_heading_marker(lines: &[LineInfo<'_>], idx: usize) -> bool {
    if idx == 0 || !is_setext_underline(lines[idx].text.trim()) {
        return false;
    }
    let previous = lines[idx - 1].text.trim();
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
        // Diagnostic must point to the heading text line (line 1), not the underline (line 2),
        // matching the markdownlint reference implementation.
        let diagnostics = HeadingStyleRule.evaluate(Path::new("test.md"), "Heading\n---\n");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "MD003");
        assert_eq!(diagnostics[0].range.start_line, 1);
    }

    #[test]
    fn fixes_setext_h1_to_atx_h1() {
        let diagnostics = HeadingStyleRule.evaluate(Path::new("test.md"), "Heading\n=======\n");

        let fix = diagnostics[0].fix_info.as_ref().expect("fix should exist");
        assert_eq!(fix.start_line, 1);
        assert_eq!(fix.start_column, 1);
        assert_eq!(fix.end_line, 3);
        assert_eq!(fix.end_column, 1);
        assert_eq!(fix.replacement, "# Heading\n");
    }

    #[test]
    fn fixes_setext_h2_to_atx_h2() {
        let diagnostics = HeadingStyleRule.evaluate(Path::new("test.md"), "Heading\n-------\n");

        let fix = diagnostics[0].fix_info.as_ref().expect("fix should exist");
        assert_eq!(fix.replacement, "## Heading\n");
    }

    #[test]
    fn fixes_setext_heading_preserving_crlf() {
        let diagnostics = HeadingStyleRule.evaluate(Path::new("test.md"), "Heading\r\n=======\r\n");

        let fix = diagnostics[0].fix_info.as_ref().expect("fix should exist");
        assert_eq!(fix.replacement, "# Heading\r\n");
    }

    #[test]
    fn setext_heading_level_rejects_mixed_markers() {
        assert_eq!(setext_heading_level("=-="), None);
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
