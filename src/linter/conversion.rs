use crate::i18n::MessageCatalog;
use crate::rules::markdown::{DiagnosticFix, DiagnosticSeverity, MarkdownDiagnostic};
use crate::{Fix, FixSafety, LintResult, Range, Severity};

impl From<MarkdownDiagnostic> for LintResult {
    fn from(value: MarkdownDiagnostic) -> Self {
        let fix = value.fix_info.map(|fix| convert_fix(fix, &value.rule_id));
        let rule_name = value
            .official_meta
            .as_ref()
            .map(|meta| meta.title)
            .unwrap_or_default();
        Self {
            message_id: MessageCatalog::diagnostic_message_id(&value.rule_id, &value.message),
            message_params: MessageCatalog::diagnostic_message_params(
                &value.rule_id,
                rule_name,
                &value.message,
            ),
            rule_id: value.rule_id,
            rule_name: rule_name.to_owned(),
            message: value.message,
            severity: match value.severity {
                DiagnosticSeverity::Error => Severity::Error,
                DiagnosticSeverity::Warning => Severity::Warning,
                DiagnosticSeverity::Info => Severity::Info,
            },
            line: value.range.start_line,
            column: value.range.start_column,
            end_line: value.range.end_line,
            end_column: value.range.end_column,
            fix,
        }
    }
}

fn convert_fix(fix: DiagnosticFix, rule_id: &str) -> Fix {
    Fix {
        range: Range {
            start_line: fix.start_line,
            start_column: fix.start_column,
            end_line: fix.end_line,
            end_column: fix.end_column,
        },
        replacement: fix.replacement,
        safety: if rule_id == "MD036" {
            FixSafety::Unsafe
        } else {
            FixSafety::Safe
        },
    }
}
