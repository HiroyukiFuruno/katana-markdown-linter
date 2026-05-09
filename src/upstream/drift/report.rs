use super::types::{DriftReport, DriftType};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub(super) fn render_markdown_summary(report: &DriftReport) -> String {
    let mut out = String::new();
    out.push_str("# Upstream Drift Report\n\n");
    out.push_str(&format!(
        "- upstream rules: {}\n",
        report.summary.upstream_rules
    ));
    out.push_str(&format!("- local rules: {}\n", report.summary.local_rules));
    out.push_str(&format!("- missing: {}\n", report.summary.missing));
    out.push_str(&format!("- removed: {}\n", report.summary.removed));
    out.push_str(&format!("- deprecated: {}\n", report.summary.deprecated));
    out.push_str(&format!("- mismatches: {}\n", report.summary.mismatches));
    out.push_str(&format!(
        "- unknown_needs_review: {}\n\n",
        report.summary.unknown_needs_review
    ));

    if report.items.is_empty() {
        out.push_str("No drift detected.\n");
        return out;
    }

    out.push_str("| Rule | Type | Message |\n");
    out.push_str("| --- | --- | --- |\n");
    for item in &report.items {
        out.push_str(&format!(
            "| {} | {:?} | {} |\n",
            item.rule_id,
            item.drift_type,
            item.message.replace('|', "\\|")
        ));
    }
    out
}

pub(super) fn write_report_files(report: &DriftReport, output_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|err| format!("{}: {err}", output_dir.display()))?;
    let json = serde_json::to_string_pretty(report).map_err(|err| err.to_string())?;
    fs::write(output_dir.join("upstream-drift.json"), json)
        .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    fs::write(
        output_dir.join("upstream-drift.md"),
        render_markdown_summary(report),
    )
    .map_err(|err| format!("{}: {err}", output_dir.display()))?;
    Ok(())
}

pub(super) fn assert_no_unknown_drift(
    report: &DriftReport,
    allowed: &BTreeSet<(String, DriftType)>,
) {
    let unknown = report
        .items
        .iter()
        .filter(|item| !allowed.contains(&(item.rule_id.clone(), item.drift_type)))
        .collect::<Vec<_>>();
    assert!(
        unknown.is_empty(),
        "unknown upstream drift detected:\n{}",
        render_markdown_summary(report)
    );
}

pub(super) fn known_current_drift_allowlist() -> BTreeSet<(String, DriftType)> {
    [
        ("MD003", DriftType::SummaryMismatch),
        ("MD005", DriftType::SummaryMismatch),
        ("MD007", DriftType::SummaryMismatch),
        ("MD013", DriftType::SummaryMismatch),
        ("MD014", DriftType::SummaryMismatch),
        ("MD018", DriftType::SummaryMismatch),
        ("MD019", DriftType::SummaryMismatch),
        ("MD020", DriftType::SummaryMismatch),
        ("MD021", DriftType::SummaryMismatch),
        ("MD022", DriftType::PropertyTypeMismatch),
        ("MD034", DriftType::SummaryMismatch),
        ("MD043", DriftType::SummaryMismatch),
        ("MD044", DriftType::SummaryMismatch),
        ("MD051", DriftType::SummaryMismatch),
        ("MD052", DriftType::SummaryMismatch),
        ("MD053", DriftType::SummaryMismatch),
        ("MD054", DriftType::SummaryMismatch),
        ("MD056", DriftType::SummaryMismatch),
        ("MD058", DriftType::SummaryMismatch),
        ("MD059", DriftType::SummaryMismatch),
    ]
    .into_iter()
    .map(|(rule_id, drift_type)| (rule_id.to_string(), drift_type))
    .collect()
}
