pub mod document;
pub mod drift;
pub mod fixture;

pub use document::{
    load_catalog_from_dir, parse_rule_document, UpstreamCatalog, UpstreamRuleDocument,
    UpstreamRuleExample, UpstreamRuleProperty,
};
pub use drift::{
    assert_no_unknown_drift, compare_with_local, known_current_drift_allowlist,
    render_markdown_summary, write_report_files, DriftItem, DriftReport, DriftSummary, DriftType,
};
pub use fixture::{
    build_fixture_matrix, compare_fixture_matrix, render_fixture_matrix_summary,
    write_fixture_matrix_files, RuleFixtureCase, RuleFixtureDriftItem, RuleFixtureDriftReport,
    RuleFixtureDriftSummary, RuleFixtureDriftType, RuleFixtureEntry, RuleFixtureMatrix,
    RuleFixtureMatrixSummary,
};
