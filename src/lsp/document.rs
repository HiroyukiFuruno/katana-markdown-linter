use super::range::SelectedLineRange;
use super::LspUri;
use crate::{FixSafety, FormatOptions, LintOptions, LintResult, MarkdownFormatter, MarkdownLinter};
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
        let diagnostics = MarkdownLinter::lint_for_path(&path, content, options)
            .map_err(|err| err.to_string())?
            .into_iter()
            .map(lsp_diagnostic)
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
            "range": full_document_range(content),
            "newText": formatted
        }]))
    }

    pub(crate) fn range_formatting_edits_with_options(
        content: &str,
        range: &Value,
        options: &FormatOptions,
        lint_options: &LintOptions,
    ) -> Result<Value, String> {
        let Some(selection) = SelectedLineRange::from_lsp_range(content, range) else {
            return Ok(json!([]));
        };
        let selected = &content[selection.start_offset..selection.end_offset];
        let formatted =
            MarkdownFormatter::format_markdown_with_lint_options(selected, options, lint_options)
                .map_err(|err| err.to_string())?
                .content;
        if formatted == selected {
            return Ok(json!([]));
        }
        Ok(json!([{
            "range": selection.lsp_range(),
            "newText": formatted
        }]))
    }

    pub(crate) fn code_actions(
        uri: &str,
        content: &str,
        options: &LintOptions,
    ) -> Result<Value, String> {
        let path = LspUri::path(uri);
        let diagnostics = MarkdownLinter::lint_for_path(&path, content, options)
            .map_err(|err| err.to_string())?;
        let actions = diagnostics
            .iter()
            .filter_map(|diagnostic| code_action(uri, diagnostic))
            .collect::<Vec<_>>();
        Ok(json!(actions))
    }
}

fn code_action(uri: &str, diagnostic: &LintResult) -> Option<Value> {
    let fix = diagnostic.fix.as_ref()?;
    if fix.safety != FixSafety::Safe {
        return None;
    }
    Some(json!({
        "title": format!("Apply {} fix", diagnostic.rule_id),
        "kind": "quickfix",
        "diagnostics": [lsp_diagnostic(diagnostic.clone())],
        "edit": {
            "changes": {
                uri: [{
                    "range": lsp_range(
                        fix.range.start_line,
                        fix.range.start_column,
                        fix.range.end_line,
                        fix.range.end_column
                    ),
                    "newText": fix.replacement
                }]
            }
        }
    }))
}

fn lsp_diagnostic(diagnostic: LintResult) -> Value {
    json!({
        "range": lsp_range(
            diagnostic.line,
            diagnostic.column,
            diagnostic.end_line,
            diagnostic.end_column
        ),
        "severity": severity_code(diagnostic.severity),
        "code": diagnostic.rule_id,
        "source": "kml",
        "message": diagnostic.message
    })
}

fn severity_code(severity: crate::Severity) -> u8 {
    match severity {
        crate::Severity::Error => LSP_SEVERITY_ERROR,
        crate::Severity::Warning => LSP_SEVERITY_WARNING,
        crate::Severity::Info => LSP_SEVERITY_INFO,
    }
}

fn lsp_range(start_line: usize, start_column: usize, end_line: usize, end_column: usize) -> Value {
    json!({
        "start": position(start_line, start_column),
        "end": position(end_line, end_column)
    })
}

fn position(line: usize, column: usize) -> Value {
    json!({
        "line": line.saturating_sub(1),
        "character": column.saturating_sub(1)
    })
}

fn full_document_range(content: &str) -> Value {
    let (line, character) = end_position(content);
    json!({
        "start": { "line": 0, "character": 0 },
        "end": { "line": line, "character": character }
    })
}

fn end_position(content: &str) -> (usize, usize) {
    let mut line = 0;
    let mut character = 0;
    for ch in content.chars() {
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
    }
    (line, character)
}
