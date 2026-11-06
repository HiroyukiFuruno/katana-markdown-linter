use crate::rules::markdown::document::LineInfo;
use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, OfficialRuleMeta,
};
use crate::types::RuleConfig;

pub(super) fn check_headings(
    ctx: &DocumentContext<'_>,
    config: Option<&RuleConfig>,
    meta: &OfficialRuleMeta,
) -> Vec<MarkdownDiagnostic> {
    let include_front_matter = config
        .and_then(|config| config.properties.get("include_front_matter"))
        .and_then(|value| value.parse().ok())
        .unwrap_or(false);
    let mut diagnostics = Vec::new();
    for heading in ctx.headings() {
        let i = heading.line;
        if !include_front_matter && is_front_matter_line(ctx, i) {
            continue;
        }
        let before = i > 0 && !is_blank(ctx, i - 1, include_front_matter);
        let after = i + 1 < ctx.lines().len() && !is_blank(ctx, i + 1, include_front_matter);
        if before || after {
            push_spacing_diagnostic(ctx, i, before, after, meta, &mut diagnostics);
        }
    }
    diagnostics
}

fn push_spacing_diagnostic(
    ctx: &DocumentContext<'_>,
    i: usize,
    before: bool,
    after: bool,
    meta: &OfficialRuleMeta,
    diagnostics: &mut Vec<MarkdownDiagnostic>,
) {
    let line = &ctx.lines()[i];
    let mut replacement = String::new();
    if before {
        replacement.push('\n');
    }
    replacement.push_str(line.text);
    if after {
        replacement.push('\n');
    }
    let fix = crate::rules::markdown::types::DiagnosticFix {
        start_line: i + 1,
        start_column: 1,
        end_line: i + 1,
        end_column: line.text.len().max(1) + if line.text.is_empty() { 0 } else { 1 },
        replacement,
    };
    RuleHelpers::push_diag_with_fix(
        diagnostics,
        ctx.file_path(),
        i,
        line.text,
        meta,
        DiagnosticSeverity::Warning,
        Some(fix),
    );
}

fn is_blank(ctx: &DocumentContext<'_>, i: usize, include_front_matter: bool) -> bool {
    if !include_front_matter && is_front_matter_line(ctx, i) {
        return true;
    }
    let line: &LineInfo<'_> = &ctx.lines()[i];
    let trimmed = line.text.trim();
    trimmed.is_empty() || (trimmed.starts_with("<!--") && trimmed.ends_with("-->"))
}

fn is_front_matter_line(ctx: &DocumentContext<'_>, line_index: usize) -> bool {
    let Some(front_matter) = ctx.front_matter() else {
        return false;
    };
    ctx.lines().get(line_index).is_some_and(|line| {
        front_matter.start <= line.content_range.start && line.full_range.end <= front_matter.end
    })
}
