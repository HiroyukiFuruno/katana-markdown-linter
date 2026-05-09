use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DriftReport {
    pub upstream_source: String,
    pub summary: DriftSummary,
    pub items: Vec<DriftItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct DriftSummary {
    pub upstream_rules: usize,
    pub local_rules: usize,
    pub missing: usize,
    pub removed: usize,
    pub deprecated: usize,
    pub mismatches: usize,
    pub unknown_needs_review: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DriftItem {
    pub rule_id: String,
    pub drift_type: DriftType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftType {
    MissingLocalRule,
    RemovedUpstreamRule,
    DeprecatedUpstreamRule,
    PropertyMissingLocal,
    PropertyMissingUpstream,
    PropertyTypeMismatch,
    PropertyDefaultMismatch,
    SummaryMismatch,
    UnknownNeedsReview,
}
