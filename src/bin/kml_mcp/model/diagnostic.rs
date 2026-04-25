use katana_markdown_linter::{FixSafety, LintResult, Locale, Severity};
use rmcp::schemars;
use serde::Serialize;

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct Diagnostic {
    pub(crate) rule_id: String,
    pub(crate) rule_name: String,
    pub(crate) message: String,
    pub(crate) message_id: String,
    pub(crate) message_params: std::collections::BTreeMap<String, String>,
    pub(crate) severity: String,
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) end_line: usize,
    pub(crate) end_column: usize,
    pub(crate) fixable: bool,
    pub(crate) fix: Option<TextFix>,
}

impl Diagnostic {
    pub(crate) fn from_result(result: LintResult, locale: Locale) -> Self {
        let fixable = result.fix.is_some();
        let message = katana_markdown_linter::i18n::render_message(
            locale,
            result.message_id.as_str(),
            &result.message_params,
            result.message.as_str(),
        );
        Self {
            rule_id: result.rule_id,
            rule_name: result.rule_name,
            message,
            message_id: result.message_id,
            message_params: result.message_params,
            severity: severity_text(result.severity).to_string(),
            line: result.line,
            column: result.column,
            end_line: result.end_line,
            end_column: result.end_column,
            fixable,
            fix: result.fix.map(|fix| TextFix {
                start_line: fix.range.start_line,
                start_column: fix.range.start_column,
                end_line: fix.range.end_line,
                end_column: fix.range.end_column,
                replacement: fix.replacement,
                safety: fix_safety_text(fix.safety).to_string(),
            }),
        }
    }
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub(crate) struct TextFix {
    pub(crate) start_line: usize,
    pub(crate) start_column: usize,
    pub(crate) end_line: usize,
    pub(crate) end_column: usize,
    pub(crate) replacement: String,
    pub(crate) safety: String,
}

fn severity_text(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

fn fix_safety_text(safety: FixSafety) -> &'static str {
    match safety {
        FixSafety::Safe => "safe",
        FixSafety::Unsafe => "unsafe",
    }
}
