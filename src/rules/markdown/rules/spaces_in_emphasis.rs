use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    SourceRange,
};
use std::path::Path;

const MAX_EMPHASIS_MARKER_LEN: usize = 2;

#[derive(Clone, Copy)]
struct EmphasisMarker {
    start: usize,
    len: usize,
    kind: char,
}

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

fn emphasis_markers(line: &str) -> Vec<EmphasisMarker> {
    let mut markers = Vec::new();
    let mut chars = line.char_indices().peekable();
    while let Some((start, kind)) = chars.next() {
        if !is_emphasis_marker(kind) {
            continue;
        }
        let len = marker_run_len(kind, &mut chars);
        if len <= MAX_EMPHASIS_MARKER_LEN {
            markers.push(EmphasisMarker { start, len, kind });
        }
    }
    markers
}

fn marker_run_len(kind: char, chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>) -> usize {
    let mut len = 1;
    while let Some(&(_, next_kind)) = chars.peek() {
        if next_kind != kind {
            break;
        }
        len += 1;
        chars.next();
    }
    len
}

fn is_emphasis_marker(kind: char) -> bool {
    kind == '*' || kind == '_'
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
    if !line[after_marker..].starts_with(' ') || !valid_start(line, marker.start) {
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

fn valid_start(line: &str, marker_start: usize) -> bool {
    let Some(previous) = line[..marker_start].chars().next_back() else {
        return true;
    };
    previous.is_whitespace() || "([{\"'".contains(previous)
}

fn matching_end_marker(markers: &[EmphasisMarker], marker_index: usize) -> Option<EmphasisMarker> {
    let marker = markers[marker_index];
    markers
        .iter()
        .skip(marker_index + 1)
        .find(|candidate| candidate.kind == marker.kind && candidate.len == marker.len)
        .copied()
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
