use super::metadata::{local_rule_map, normalize_default, normalize_summary, property_type_name};
use super::types::{DriftItem, DriftReport, DriftSummary, DriftType};
use crate::upstream::document::{UpstreamCatalog, UpstreamRuleDocument};
use std::collections::BTreeMap;

pub(super) fn compare_with_local(catalog: &UpstreamCatalog) -> DriftReport {
    let local_rules = local_rule_map();
    let upstream_rules = catalog
        .rules
        .iter()
        .map(|rule| (rule.id.as_str(), rule))
        .collect::<BTreeMap<_, _>>();
    let mut items = Vec::new();

    for upstream in &catalog.rules {
        let Some(local) = local_rules.get(upstream.id.as_str()) else {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::MissingLocalRule,
                message: "upstream rule is not present in local catalog".to_string(),
            });
            continue;
        };

        compare_rule_properties(upstream, local, &mut items);
        if normalize_summary(&upstream.summary) != normalize_summary(local.description) {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::SummaryMismatch,
                message: format!(
                    "summary differs: upstream {:?}, local {:?}",
                    upstream.summary, local.description
                ),
            });
        }
    }

    for local_id in local_rules.keys() {
        if !upstream_rules.contains_key(local_id) {
            items.push(DriftItem {
                rule_id: (*local_id).to_string(),
                drift_type: DriftType::RemovedUpstreamRule,
                message: "local rule is not present in upstream docs".to_string(),
            });
        }
    }

    DriftReport {
        upstream_source: catalog.source.clone(),
        summary: summarize_items(upstream_rules.len(), local_rules.len(), &items),
        items,
    }
}

fn summarize_items(upstream_rules: usize, local_rules: usize, items: &[DriftItem]) -> DriftSummary {
    let mut summary = DriftSummary {
        upstream_rules,
        local_rules,
        ..DriftSummary::default()
    };
    for item in items {
        match item.drift_type {
            DriftType::MissingLocalRule => summary.missing += 1,
            DriftType::RemovedUpstreamRule => summary.removed += 1,
            DriftType::DeprecatedUpstreamRule => summary.deprecated += 1,
            DriftType::UnknownNeedsReview => summary.unknown_needs_review += 1,
            _ => summary.mismatches += 1,
        }
    }
    summary
}

fn compare_rule_properties(
    upstream: &UpstreamRuleDocument,
    local: &crate::rules::markdown::OfficialRuleMeta,
    items: &mut Vec<DriftItem>,
) {
    let upstream_props = upstream
        .properties
        .iter()
        .map(|property| (property.key.as_str(), property))
        .collect::<BTreeMap<_, _>>();
    let local_props = local
        .properties
        .iter()
        .map(|property| (property.key, property))
        .collect::<BTreeMap<_, _>>();

    for (key, upstream_property) in &upstream_props {
        let Some(local_property) = local_props.get(key) else {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::PropertyMissingLocal,
                message: format!("property `{key}` is present upstream but missing locally"),
            });
            continue;
        };

        let local_type = property_type_name(local_property.prop_type);
        if upstream_property.value_type != local_type {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::PropertyTypeMismatch,
                message: format!(
                    "property `{key}` type differs: upstream {}, local {}",
                    upstream_property.value_type, local_type
                ),
            });
        }

        let Some(default_value) = &upstream_property.default_value else {
            continue;
        };
        if normalize_default(default_value) == normalize_default(local_property.default_value) {
            continue;
        }
        items.push(DriftItem {
            rule_id: upstream.id.clone(),
            drift_type: DriftType::PropertyDefaultMismatch,
            message: format!(
                "property `{key}` default differs: upstream {}, local {}",
                default_value, local_property.default_value
            ),
        });
    }

    for key in local_props.keys() {
        if !upstream_props.contains_key(key) {
            items.push(DriftItem {
                rule_id: upstream.id.clone(),
                drift_type: DriftType::PropertyMissingUpstream,
                message: format!("property `{key}` is present locally but missing upstream"),
            });
        }
    }
}
