mod scan;

use scan::bare_url_ranges;
#[cfg(test)]
mod tests;

use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD034 / no-bare-urls — No bare URLs.
pub struct NoBareUrlsRule;

impl MarkdownRule for NoBareUrlsRule {
    fn id(&self) -> &'static str {
        "MD034"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD034")
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
        let meta = self.official_meta().expect("always Some for MD034");
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            for (start, end) in bare_url_ranges(line.text) {
                let absolute_start = line.content_range.start + start;
                if is_ignored_url(ctx, i, line.text, start, absolute_start) {
                    continue;
                }
                diagnostics.push(url_diagnostic(
                    ctx,
                    line.number,
                    start,
                    end,
                    &line.text[start..end],
                    &meta,
                ));
            }
        }
        diagnostics
    }
}

fn url_diagnostic(
    ctx: &DocumentContext<'_>,
    line: usize,
    start: usize,
    end: usize,
    url: &str,
    meta: &OfficialRuleMeta,
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
        fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
            start_line: line,
            start_column: start + 1,
            end_line: line,
            end_column: end + 1,
            replacement: format!("<{url}>"),
        }),
    }
}

fn is_ignored_url(
    ctx: &DocumentContext<'_>,
    line_index: usize,
    line: &str,
    start: usize,
    absolute_start: usize,
) -> bool {
    if start > 0 && line.as_bytes()[start - 1] == b'<' {
        return true;
    }
    if line[..start].ends_with("](") || line[..start].ends_with(")[") {
        return true;
    }
    let range = crate::rules::markdown::SourceRange {
        start: absolute_start,
        end: absolute_start + 1,
    };
    if ctx.is_inside_inline_code(range) || ctx.overlaps_raw_html(range) {
        return true;
    }
    in_link_or_definition(ctx, line_index, absolute_start)
        || ctx.html_attribute_at(absolute_start).is_some()
}

fn in_link_or_definition(
    ctx: &DocumentContext<'_>,
    line_index: usize,
    absolute_start: usize,
) -> bool {
    let lo = ctx
        .inline_links()
        .partition_point(|link| link.line < line_index);
    if ctx.inline_links()[lo..]
        .iter()
        .take_while(|link| link.line == line_index)
        .any(|link| link.full_range.start <= absolute_start && absolute_start < link.full_range.end)
    {
        return true;
    }
    let lo = ctx
        .reference_definitions()
        .partition_point(|def| def.line < line_index);
    if ctx.reference_definitions()[lo..]
        .iter()
        .take_while(|def| def.line == line_index)
        .any(|def| def.full_range.start <= absolute_start && absolute_start < def.full_range.end)
    {
        return true;
    }

    false
}
