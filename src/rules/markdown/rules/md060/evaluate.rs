use super::format::safe_fix;
use super::matching::table_matches;
use super::options::TableStyleOptions;
use crate::rules::markdown::{
    DiagnosticFix, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, OfficialRuleMeta,
    TableBlock,
};

pub(super) fn evaluate_table<'a>(
    ctx: &DocumentContext<'a>,
    meta: &OfficialRuleMeta,
    options: &TableStyleOptions,
    table: &TableBlock<'a>,
) -> Option<MarkdownDiagnostic> {
    if table_matches(ctx, options, table) {
        return None;
    }
    let range = ctx.diagnostic_range(table.range);
    Some(MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: range.clone(),
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: safe_fix(ctx, options, table).map(|replacement| DiagnosticFix {
            start_line: range.start_line,
            start_column: range.start_column,
            end_line: range.end_line,
            end_column: range.end_column,
            replacement,
        }),
    })
}
