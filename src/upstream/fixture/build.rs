use super::compare::compare_fixture_matrix;
use super::entry::rule_fixture_entry;
use super::types::{RuleFixtureMatrix, RuleFixtureMatrixSummary};
use crate::upstream::document::UpstreamCatalog;

pub(super) fn build_fixture_matrix(catalog: &UpstreamCatalog) -> RuleFixtureMatrix {
    let mut rules = catalog
        .rules
        .iter()
        .map(rule_fixture_entry)
        .collect::<Vec<_>>();
    rules.sort_by(|a, b| a.rule_id.cmp(&b.rule_id));

    let mut summary = RuleFixtureMatrixSummary {
        total_rules: rules.len(),
        ..RuleFixtureMatrixSummary::default()
    };
    for entry in &rules {
        if !entry.check_fail.is_empty() || !entry.check_pass.is_empty() {
            summary.rules_with_examples += 1;
        }
        if entry.fixable.is_some() {
            summary.rules_with_fix_metadata += 1;
        }
        if !entry.parameters.is_empty() {
            summary.rules_with_parameters += 1;
        }
        if !entry.manual_required.is_empty() {
            summary.manual_required += 1;
        }
    }

    let mut matrix = RuleFixtureMatrix {
        upstream_source: catalog.source.clone(),
        summary,
        rules,
    };
    let drift = compare_fixture_matrix(catalog, &matrix);
    matrix.summary.missing_fixtures = drift.summary.missing_fixtures;
    matrix.summary.stale_fixtures = drift.summary.stale_fixtures;
    matrix
}
