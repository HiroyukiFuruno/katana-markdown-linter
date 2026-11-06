use super::range::{lsp_range, selected_line_offsets};
use super::LspUri;
use crate::{
    fix::LineOffsetIndex, FixSafety, FormatOptions, LintOptions, LintResult, MarkdownFormatter,
    MarkdownLinter, Range,
};
use serde_json::{json, Value};

const LSP_SEVERITY_ERROR: u8 = 1;
const LSP_SEVERITY_WARNING: u8 = 2;
const LSP_SEVERITY_INFO: u8 = 3;

pub(crate) struct LspDocumentOps;

impl LspDocumentOps {
    pub(crate) fn diagnostics(
        uri: &str,
        content: &str,
        options: &LintOptions,
    ) -> Result<Value, String> {
        let path = LspUri::path(uri);
        let positions = DiagnosticPositions::new(content);
        let diagnostics = MarkdownLinter::lint_for_path(&path, content, options)
            .map_err(|err| err.to_string())?
            .iter()
            .filter_map(|diagnostic| lsp_diagnostic(&positions, diagnostic))
            .collect::<Vec<_>>();
        Ok(json!({ "uri": uri, "diagnostics": diagnostics }))
    }

    pub(crate) fn formatting_edits_with_options(
        content: &str,
        options: &FormatOptions,
        lint_options: &LintOptions,
    ) -> Result<Value, String> {
        let formatted =
            MarkdownFormatter::format_markdown_with_lint_options(content, options, lint_options)
                .map_err(|err| err.to_string())?
                .content;

        if formatted == content {
            return Ok(json!([]));
        }
        Ok(json!([{
            "range": lsp_range(&LineOffsetIndex::for_lsp(content), 0..content.len()),
            "newText": formatted
        }]))
    }

    pub(crate) fn range_formatting_edits_with_options(
        content: &str,
        range: &Value,
        options: &FormatOptions,
        lint_options: &LintOptions,
    ) -> Result<Value, String> {
        let index = LineOffsetIndex::for_lsp(content);
        let Some(selection) = selected_line_offsets(&index, range) else {
            return Ok(json!([]));
        };
        let formatted = MarkdownFormatter::format_markdown_range(
            content,
            selection.clone(),
            options,
            lint_options,
        )
        .map_err(|err| err.to_string())?;
        if formatted == content[selection.clone()] {
            return Ok(json!([]));
        }
        Ok(json!([{
            "range": lsp_range(&index, selection),
            "newText": formatted
        }]))
    }

    pub(crate) fn code_actions(
        uri: &str,
        content: &str,
        options: &LintOptions,
    ) -> Result<Value, String> {
        let path = LspUri::path(uri);
        let positions = DiagnosticPositions::new(content);
        let diagnostics = MarkdownLinter::lint_for_path(&path, content, options)
            .map_err(|err| err.to_string())?;
        let actions = diagnostics
            .iter()
            .filter_map(|diagnostic| code_action(uri, &positions, diagnostic))
            .collect::<Vec<_>>();
        Ok(json!(actions))
    }
}

fn code_action(
    uri: &str,
    positions: &DiagnosticPositions<'_>,
    diagnostic: &LintResult,
) -> Option<Value> {
    let fix = diagnostic.fix.as_ref()?;
    if fix.safety != FixSafety::Safe {
        return None;
    }
    Some(json!({
        "title": format!("Apply {} fix", diagnostic.rule_id),
        "kind": "quickfix",
        "diagnostics": [lsp_diagnostic(positions, diagnostic)?],
        "edit": {
            "changes": {
                uri: [{
                    "range": positions.range(&fix.range)?,
                    "newText": fix.replacement
                }]
            }
        }
    }))
}

fn lsp_diagnostic(positions: &DiagnosticPositions<'_>, diagnostic: &LintResult) -> Option<Value> {
    let range = Range {
        start_line: diagnostic.line,
        start_column: diagnostic.column,
        end_line: diagnostic.end_line,
        end_column: diagnostic.end_column,
    };
    Some(json!({
        "range": positions.range(&range)?,
        "severity": severity_code(diagnostic.severity),
        "code": diagnostic.rule_id,
        "source": "kml",
        "message": diagnostic.message
    }))
}

fn severity_code(severity: crate::Severity) -> u8 {
    match severity {
        crate::Severity::Error => LSP_SEVERITY_ERROR,
        crate::Severity::Warning => LSP_SEVERITY_WARNING,
        crate::Severity::Info => LSP_SEVERITY_INFO,
    }
}

struct DiagnosticPositions<'a> {
    core: LineOffsetIndex<'a>,
    lone_cr: Option<LineOffsetIndex<'a>>,
}

impl<'a> DiagnosticPositions<'a> {
    fn new(content: &'a str) -> Self {
        let bytes = content.as_bytes();
        /* WHY: コアの行区切り契約を維持しつつ、LSP では単独 CR も行末として扱う。 */
        let has_lone_cr = bytes.last() == Some(&b'\r')
            || bytes
                .windows(2)
                .any(|pair| pair[0] == b'\r' && pair[1] != b'\n');
        Self {
            core: LineOffsetIndex::new(content),
            lone_cr: has_lone_cr.then(|| LineOffsetIndex::for_lsp(content)),
        }
    }

    fn range(&self, range: &Range) -> Option<Value> {
        let offsets = self.core.offsets_for_range(range)?;
        Some(lsp_range(
            self.lone_cr.as_ref().unwrap_or(&self.core),
            offsets,
        ))
    }
}
