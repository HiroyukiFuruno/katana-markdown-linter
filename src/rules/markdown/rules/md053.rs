use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD053 / link-definitions — Link definitions.
pub struct LinkDefinitionsRule;

impl MarkdownRule for LinkDefinitionsRule {
    fn id(&self) -> &'static str {
        "MD053"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD053")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD053");
        let mut diagnostics = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if let Some(label) = trimmed
                .strip_prefix('[')
                .and_then(|rest| rest.split_once("]:"))
            {
                if !seen.insert(label.0.to_lowercase()) {
                    RuleHelpers::push_diag(
                        &mut diagnostics,
                        file_path,
                        i,
                        line,
                        &meta,
                        DiagnosticSeverity::Warning,
                    );
                }
            }
        }
        diagnostics
    }
}
