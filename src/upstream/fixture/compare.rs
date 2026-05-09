use super::types::{
    RuleFixtureDriftItem, RuleFixtureDriftReport, RuleFixtureDriftSummary, RuleFixtureDriftType,
    RuleFixtureMatrix,
};
use crate::upstream::document::UpstreamCatalog;
use std::collections::BTreeMap;

pub(super) fn compare_fixture_matrix(
    catalog: &UpstreamCatalog,
    matrix: &RuleFixtureMatrix,
) -> RuleFixtureDriftReport {
    let upstream_rules = catalog
        .rules
        .iter()
        .map(|rule| (rule.id.as_str(), rule))
        .collect::<BTreeMap<_, _>>();
    let matrix_rules = matrix
        .rules
        .iter()
        .map(|rule| (rule.rule_id.as_str(), rule))
        .collect::<BTreeMap<_, _>>();
    let mut items = Vec::new();

    for rule_id in upstream_rules.keys() {
        if !matrix_rules.contains_key(rule_id) {
            items.push(RuleFixtureDriftItem {
                rule_id: (*rule_id).to_string(),
                drift_type: RuleFixtureDriftType::MissingFixture,
                message: "upstream rule has no fixture matrix entry".to_string(),
            });
        }
    }

    for entry in &matrix.rules {
        if !upstream_rules.contains_key(entry.rule_id.as_str()) {
            items.push(RuleFixtureDriftItem {
                rule_id: entry.rule_id.clone(),
                drift_type: RuleFixtureDriftType::StaleFixture,
                message: "fixture matrix entry is not present in upstream docs".to_string(),
            });
        }
        if !entry.manual_required.is_empty() {
            items.push(RuleFixtureDriftItem {
                rule_id: entry.rule_id.clone(),
                drift_type: RuleFixtureDriftType::ManualRequired,
                message: entry.manual_required.join("; "),
            });
        }
    }

    RuleFixtureDriftReport {
        upstream_source: catalog.source.clone(),
        summary: summarize_items(upstream_rules.len(), matrix_rules.len(), &items),
        items,
    }
}

fn summarize_items(
    upstream_rules: usize,
    matrix_rules: usize,
    items: &[RuleFixtureDriftItem],
) -> RuleFixtureDriftSummary {
    let mut summary = RuleFixtureDriftSummary {
        upstream_rules,
        matrix_rules,
        ..RuleFixtureDriftSummary::default()
    };
    for item in items {
        match item.drift_type {
            RuleFixtureDriftType::MissingFixture => summary.missing_fixtures += 1,
            RuleFixtureDriftType::StaleFixture => summary.stale_fixtures += 1,
            RuleFixtureDriftType::ManualRequired => summary.manual_required += 1,
        }
    }
    summary
}
