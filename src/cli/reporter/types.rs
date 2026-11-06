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

        accumulate_files(&mut summary, &self.files);

        if summary.unsafe_fixable_issues > 0 {
            summary
                .unsafe_fix_status
                .get_or_insert_with(|| "unsafe_mode_not_enabled".to_string());
        }
        self.summary = summary;
    }
}

fn accumulate_files(summary: &mut CliSummary, files: &[FileReport]) {
    for file in files {
        summary.files_with_issues += usize::from(!file.diagnostics.is_empty());
        summary.total_issues += file.diagnostics.len();
        summary.applied_fixes += file.applied_fixes;
        accumulate_fixes(summary, &file.diagnostics);
    }
}

fn accumulate_fixes(summary: &mut CliSummary, diagnostics: &[crate::LintResult]) {
    for diagnostic in diagnostics {
        if let Some(fix) = &diagnostic.fix {
            summary.fixable_issues += 1;
            match fix.safety {
                FixSafety::Safe => summary.safe_fixable_issues += 1,
                FixSafety::Unsafe => summary.unsafe_fixable_issues += 1,
            }
        }
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
