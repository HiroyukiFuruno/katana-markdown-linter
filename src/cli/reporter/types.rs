use crate::i18n::Locale;
use crate::FixSafety;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct LocalizedRuleMeta {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) english_description: String,
    pub(crate) docs_url: String,
    pub(crate) fixable: bool,
    pub(crate) locale: &'static str,
}

impl LocalizedRuleMeta {
    pub(in crate::cli) fn from_rule(rule: &crate::RuleMeta, locale: Locale) -> Self {
        Self {
            id: rule.id.clone(),
            name: rule.name.clone(),
            description: rule.localized_description(locale.code()),
            english_description: rule.description.clone(),
            docs_url: rule.docs_url.clone(),
            fixable: rule.fixable,
            locale: locale.code(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CliReport {
    pub(crate) command: &'static str,
    pub(crate) summary: CliSummary,
    pub(crate) files: Vec<FileReport>,
    pub(crate) errors: Vec<super::error::CliError>,
}

impl CliReport {
    pub(in crate::cli) fn recompute_summary(&mut self) {
        let mut summary = CliSummary {
            total_files: self.files.len(),
            ..CliSummary::default()
        };

        for file in &self.files {
            if !file.diagnostics.is_empty() {
                summary.files_with_issues += 1;
            }
            summary.total_issues += file.diagnostics.len();
            summary.fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.fix.is_some())
                .count();
            summary.safe_fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic
                        .fix
                        .as_ref()
                        .is_some_and(|fix| fix.safety == FixSafety::Safe)
                })
                .count();
            summary.unsafe_fixable_issues += file
                .diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic
                        .fix
                        .as_ref()
                        .is_some_and(|fix| fix.safety == FixSafety::Unsafe)
                })
                .count();
            summary.applied_fixes += file.applied_fixes;
        }

        if summary.unsafe_fixable_issues > 0 {
            summary
                .unsafe_fix_status
                .get_or_insert_with(|| "unsafe_mode_not_enabled".to_string());
        }
        self.summary = summary;
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct CliSummary {
    pub(crate) total_files: usize,
    pub(crate) files_with_issues: usize,
    pub(crate) total_issues: usize,
    pub(crate) fixable_issues: usize,
    pub(crate) safe_fixable_issues: usize,
    pub(crate) unsafe_fixable_issues: usize,
    pub(crate) applied_fixes: usize,
    pub(crate) unsafe_fix_status: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct FileReport {
    pub(crate) path: String,
    pub(crate) diagnostics: Vec<crate::LintResult>,
    pub(crate) applied_fixes: usize,
    pub(crate) fix_details: Vec<crate::FixDetail>,
    pub(crate) changed: bool,
}
