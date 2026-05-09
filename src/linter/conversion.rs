use crate::{rules, Fix, FixSafety, LintResult, Range, Severity};

impl From<rules::markdown::MarkdownDiagnostic> for LintResult {
    fn from(value: rules::markdown::MarkdownDiagnostic) -> Self {
        let fix = value.fix_info.map(|fix_info| Fix {
            range: Range {
                start_line: fix_info.start_line,
                start_column: fix_info.start_column,
                end_line: fix_info.end_line,
                end_column: fix_info.end_column,
            },
            replacement: fix_info.replacement,
            safety: FixSafetyClassifier::for_rule(&value.rule_id),
        });
        Self {
            message_id: crate::i18n::MessageCatalog::diagnostic_message_id(
                &value.rule_id,
                &value.message,
            ),
            message_params: crate::i18n::MessageCatalog::diagnostic_message_params(
                &value.rule_id,
                value
                    .official_meta
                    .as_ref()
                    .map(|meta| meta.title)
                    .unwrap_or_default(),
                &value.message,
            ),
            rule_id: value.rule_id,
            rule_name: value
                .official_meta
                .as_ref()
                .map(|meta| meta.title.to_string())
                .unwrap_or_default(),
            message: value.message,
            severity: match value.severity {
                rules::markdown::DiagnosticSeverity::Error => Severity::Error,
                rules::markdown::DiagnosticSeverity::Warning => Severity::Warning,
                rules::markdown::DiagnosticSeverity::Info => Severity::Info,
            },
            line: value.range.start_line,
            column: value.range.start_column,
            end_line: value.range.end_line,
            end_column: value.range.end_column,
            fix,
        }
    }
}

struct FixSafetyClassifier;

impl FixSafetyClassifier {
    fn for_rule(rule_id: &str) -> FixSafety {
        if matches!(rule_id, "MD036") {
            FixSafety::Unsafe
        } else {
            FixSafety::Safe
        }
    }
}
