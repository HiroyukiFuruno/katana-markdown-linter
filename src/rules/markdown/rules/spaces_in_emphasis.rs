use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    SourceRange,
};
use markers::{emphasis_markers, matching_end_marker, valid_start, EmphasisMarker};
use std::path::Path;

mod markers;

struct EmphasisEnvironment<'a, 'doc> {
    file_path: &'a Path,
    ctx: &'a DocumentContext<'doc>,
    meta: &'a OfficialRuleMeta,
}

struct EmphasisLine<'a> {
    line_index: usize,
    line: &'a str,
    line_start: usize,
    markers: &'a [EmphasisMarker],
    has_inline_code_marker: bool,
}

/// MD037 / no-space-in-emphasis — Spaces inside emphasis markers
pub struct SpacesInEmphasisRule;

impl MarkdownRule for SpacesInEmphasisRule {
    fn id(&self) -> &'static str {
        "MD037"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD037")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD037");
        let mut diagnostics = Vec::new();

        let ctx = DocumentContext::new(file_path, content);
        let env = EmphasisEnvironment {
            file_path,
            ctx: &ctx,
            meta: &meta,
        };
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let has_inline_code_marker = line.text.contains('`');
            let line_start = line.content_range.start;
            let line = line.text;
            let markers = emphasis_markers(line);
            let line_context = EmphasisLine {
                line_index: i,
                line,
                line_start,
                markers: &markers,
                has_inline_code_marker,
            };
            for marker_index in 0..markers.len() {
                push_space_diagnostic(&mut diagnostics, &env, &line_context, marker_index);
            }
        }

        diagnostics
    }
}

fn push_space_diagnostic(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    env: &EmphasisEnvironment<'_, '_>,
    line_context: &EmphasisLine<'_>,
    marker_index: usize,
) {
    let Some(fix) = space_fix(line_context, marker_index, env.ctx) else {
        return;
    };
    RuleHelpers::push_diag_with_fix(
        diagnostics,
        env.file_path,
        line_context.line_index,
        line_context.line,
        env.meta,
        DiagnosticSeverity::Warning,
        Some(fix),
    );
}

fn space_fix(
    line_context: &EmphasisLine<'_>,
    marker_index: usize,
    ctx: &DocumentContext<'_>,
) -> Option<crate::rules::markdown::types::DiagnosticFix> {
    let line = line_context.line;
    let marker = line_context.markers[marker_index];
    let after_marker = marker.start + marker.len;
    if !line[after_marker..].starts_with(' ')
        || !valid_start(line, line_context.markers, marker_index)
    {
        return None;
    }
    let end_marker = matching_end_marker(line_context.markers, marker_index)?;
    if !line[..end_marker.start].ends_with(' ') {
        return None;
    }
    let full_range = SourceRange {
        start: line_context.line_start + marker.start,
        end: line_context.line_start + end_marker.start + end_marker.len,
    };
    if line_context.has_inline_code_marker && ctx.is_inside_inline_code(full_range) {
        return None;
    }
    let inner_text = &line[after_marker..end_marker.start];
    if inner_text.contains('`') || !inner_text.chars().any(|char| !char.is_whitespace()) {
        return None;
    }
    let marker_str: String = std::iter::repeat_n(marker.kind, marker.len).collect();
    Some(crate::rules::markdown::types::DiagnosticFix {
        start_line: line_context.line_index + 1,
        start_column: marker.start + 1,
        end_line: line_context.line_index + 1,
        end_column: end_marker.start + end_marker.len + 1,
        replacement: format!("{}{}{}", marker_str, inner_text.trim(), marker_str),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_emphasis_markers_inside_long_and_unclosed_code_spans() {
        let rule = SpacesInEmphasisRule;
        let content = "``* spaced *``\n`_ spaced _\n";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }
}
