use crate::{rules, LintOptions, Severity};

pub(super) struct SeverityMapBuilder;

impl SeverityMapBuilder {
    pub(super) fn lint(
        options: &LintOptions,
    ) -> std::collections::HashMap<&str, Option<rules::markdown::DiagnosticSeverity>> {
        options
            .rules
            .iter()
            .map(|(rule_id, rule_config)| {
                let severity = rule_config
                    .enabled
                    .then_some(Self::diagnostic_severity(options.default_severity));
                (rule_id.as_str(), severity)
            })
            .collect()
    }

    pub(super) fn fix(
        options: &LintOptions,
    ) -> std::collections::HashMap<&'static str, Option<rules::markdown::DiagnosticSeverity>> {
        rules::markdown::MarkdownLinterOps::official_rules()
            .iter()
            .map(|rule| {
                let severity = if Self::safe_fix_rule(rule.id()) {
                    options
                        .rules
                        .get(rule.id())
                        .map(|rule_config| rule_config.enabled)
                        .unwrap_or(true)
                        .then_some(Self::diagnostic_severity(options.default_severity))
                } else {
                    None
                };
                (rule.id(), severity)
            })
            .collect()
    }

    fn diagnostic_severity(severity: Severity) -> rules::markdown::DiagnosticSeverity {
        match severity {
            Severity::Error => rules::markdown::DiagnosticSeverity::Error,
            Severity::Warning => rules::markdown::DiagnosticSeverity::Warning,
            Severity::Info => rules::markdown::DiagnosticSeverity::Info,
        }
    }

    fn safe_fix_rule(rule_id: &str) -> bool {
        matches!(
            rule_id,
            "MD003"
                | "MD004"
                | "MD005"
                | "MD007"
                | "MD009"
                | "MD010"
                | "MD011"
                | "MD012"
                | "MD014"
                | "MD018"
                | "MD019"
                | "MD020"
                | "MD021"
                | "MD022"
                | "MD023"
                | "MD025"
                | "MD026"
                | "MD027"
                | "MD029"
                | "MD030"
                | "MD031"
                | "MD032"
                | "MD034"
                | "MD035"
                | "MD037"
                | "MD038"
                | "MD039"
                | "MD040"
                | "MD044"
                | "MD046"
                | "MD047"
                | "MD048"
                | "MD049"
                | "MD050"
                | "MD051"
                | "MD052"
                | "MD053"
                | "MD054"
                | "MD055"
                | "MD056"
                | "MD058"
                | "MD060"
        )
    }
}
