use crate::rules::markdown::inline::{
    extract_emphasis_spans, safe_replacement, touches_inline_code, InlineEmphasisKind,
};
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, OfficialRuleMeta,
};
use crate::types::RuleConfig;

pub(super) fn evaluate_emphasis_style(
    ctx: &DocumentContext<'_>,
    config: Option<&RuleConfig>,
    kind: InlineEmphasisKind,
    meta: OfficialRuleMeta,
) -> Vec<MarkdownDiagnostic> {
    let style = config
        .and_then(|config| config.properties.get("style"))
        .map(String::as_str);
    let mut expected = match style {
        Some("asterisk") => Some('*'),
        Some("underscore") => Some('_'),
        _ => None,
    };
    let mut diagnostics = Vec::new();
    for (i, line) in ctx.lines().iter().enumerate() {
        if !ctx.is_code_line(i) {
            check_emphasis_line(ctx, line, kind, &meta, &mut expected, &mut diagnostics);
        }
    }
    diagnostics
}

fn check_emphasis_line<'a>(
    ctx: &DocumentContext<'a>,
    line: &crate::rules::markdown::document::LineInfo<'a>,
    kind: InlineEmphasisKind,
    meta: &OfficialRuleMeta,
    expected: &mut Option<char>,
    diagnostics: &mut Vec<MarkdownDiagnostic>,
) {
    for span in extract_emphasis_spans(ctx, line, kind) {
        if line.text.contains('`') && touches_inline_code(ctx, line.content_range.start, &span) {
            continue;
        }
        let marker = *expected.get_or_insert(span.marker);
        if span.marker != marker {
            let replacement = safe_replacement(ctx, line, &span, marker);
            diagnostics.push(style_diagnostic(
                ctx,
                line.number,
                span.start,
                span.end,
                meta,
                replacement,
            ));
        }
    }
}

fn style_diagnostic(
    ctx: &DocumentContext<'_>,
    line: usize,
    start: usize,
    end: usize,
    meta: &OfficialRuleMeta,
    replacement: Option<String>,
) -> MarkdownDiagnostic {
    MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: DiagnosticRange {
            start_line: line,
            start_column: start + 1,
            end_line: line,
            end_column: end + 1,
        },
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: replacement.map(|replacement| crate::rules::markdown::types::DiagnosticFix {
            start_line: line,
            start_column: start + 1,
            end_line: line,
            end_column: end + 1,
            replacement,
        }),
    }
}
