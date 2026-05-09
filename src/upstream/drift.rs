mod compare;
mod metadata;
mod report;
mod types;

use std::collections::BTreeSet;
use std::path::Path;

use super::document::UpstreamCatalog;

pub use types::{DriftItem, DriftReport, DriftSummary, DriftType};

pub struct DriftService;

impl DriftService {
    pub fn compare_with_local(catalog: &UpstreamCatalog) -> DriftReport {
        compare::compare_with_local(catalog)
    }

    pub fn render_markdown_summary(report: &DriftReport) -> String {
        report::render_markdown_summary(report)
    }

    pub fn write_report_files(report: &DriftReport, output_dir: &Path) -> Result<(), String> {
        report::write_report_files(report, output_dir)
    }

    pub fn assert_no_unknown_drift(report: &DriftReport, allowed: &BTreeSet<(String, DriftType)>) {
        report::assert_no_unknown_drift(report, allowed);
    }

    pub fn known_current_drift_allowlist() -> BTreeSet<(String, DriftType)> {
        report::known_current_drift_allowlist()
    }
}

#[cfg(test)]
mod tests;
