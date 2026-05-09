use super::compare;
use super::report;
use super::{DriftItem, DriftReport, DriftSummary, DriftType};
use std::fs;
use std::path::Path;

#[test]
fn renders_markdown_summary() {
    let report = DriftReport {
        upstream_source: "test".to_string(),
        summary: DriftSummary {
            upstream_rules: 1,
            local_rules: 0,
            missing: 1,
            ..DriftSummary::default()
        },
        items: vec![DriftItem {
            rule_id: "MD999".to_string(),
            drift_type: DriftType::MissingLocalRule,
            message: "missing".to_string(),
        }],
    };

    let markdown = report::render_markdown_summary(&report);
    assert!(markdown.contains("MD999"));
    assert!(markdown.contains("missing: 1"));
}

#[test]
fn writes_json_and_markdown_reports() {
    let report = DriftReport {
        upstream_source: "test".to_string(),
        summary: DriftSummary::default(),
        items: Vec::new(),
    };
    let dir = std::env::temp_dir().join(format!(
        "katana-markdown-linter-drift-report-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);

    report::write_report_files(&report, &dir).expect("report files should be written");

    assert!(dir.join("upstream-drift.json").exists());
    assert!(dir.join("upstream-drift.md").exists());
    let _ = fs::remove_dir_all(dir);
}

#[test]
#[ignore = "requires KML_UPSTREAM_MARKDOWNLINT_DOC_DIR pointing at DavidAnson/markdownlint/doc"]
fn upstream_default_branch_drift_has_no_unknown_items() {
    let doc_dir = std::env::var("KML_UPSTREAM_MARKDOWNLINT_DOC_DIR")
        .expect("KML_UPSTREAM_MARKDOWNLINT_DOC_DIR must be set");
    let catalog =
        crate::upstream::UpstreamDocumentService::load_catalog_from_dir(Path::new(&doc_dir))
            .expect("catalog should load");
    let drift_report = compare::compare_with_local(&catalog);
    report::assert_no_unknown_drift(&drift_report, &report::known_current_drift_allowlist());
}
