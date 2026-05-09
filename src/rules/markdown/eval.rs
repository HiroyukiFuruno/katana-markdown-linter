mod entries;
mod entry_modules;
mod official;
mod stubs;
mod types;

use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::collections::HashMap;
use std::sync::OnceLock;

pub use types::{MarkdownLinterOps, RuleEntry};

impl MarkdownLinterOps {
    pub fn evaluate_all(
        file_path: &std::path::Path,
        content: &str,
        enabled: bool,
        severity_map: &std::collections::HashMap<&str, Option<DiagnosticSeverity>>,
        rule_configs: &std::collections::HashMap<String, crate::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let mut diagnostics = Vec::new();

        if !enabled {
            return diagnostics;
        }

        let ctx = DocumentContext::new(file_path, content);
        for rule in Self::official_rules() {
            let rule_id = rule.id();
            let sev_opt = severity_map
                .get(rule_id)
                .copied()
                .unwrap_or(Some(DiagnosticSeverity::Warning));
            if let Some(severity) = sev_opt {
                let mut diags = rule.evaluate_context(&ctx, rule_configs.get(rule_id));
                for diagnostic in &mut diags {
                    diagnostic.severity = severity;
                }
                diagnostics.extend(diags);
            }
        }

        diagnostics
    }

    pub fn official_rules() -> &'static [RuleEntry] {
        entries::official_rules()
    }

    pub fn get_official_rules() -> Vec<Box<dyn MarkdownRule>> {
        official::build_official_rules()
    }

    pub fn user_configurable_rules() -> &'static [RuleEntry] {
        entries::user_configurable_rules()
    }

    pub fn user_configurable_rule_meta_map() -> &'static HashMap<&'static str, OfficialRuleMeta> {
        static RULE_META_MAP: OnceLock<HashMap<&'static str, OfficialRuleMeta>> = OnceLock::new();
        RULE_META_MAP.get_or_init(|| {
            Self::user_configurable_rules()
                .iter()
                .filter_map(|rule| rule.official_meta().map(|meta| (meta.code, meta)))
                .collect()
        })
    }

    pub fn get_user_configurable_rules() -> Vec<Box<dyn MarkdownRule>> {
        stubs::build_user_configurable_rules()
    }
}
