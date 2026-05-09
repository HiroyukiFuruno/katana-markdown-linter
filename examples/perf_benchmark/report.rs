use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct Report {
    pub(super) schema_version: u32,
    pub(super) generated_by: &'static str,
    pub(super) iterations: usize,
    pub(super) warmup_iterations: usize,
    pub(super) samples: usize,
    pub(super) cases: Vec<Case>,
}

#[derive(Debug, Serialize)]
pub(super) struct Case {
    pub(super) name: &'static str,
    pub(super) iterations: usize,
    pub(super) samples: usize,
    pub(super) work_units: usize,
    pub(super) work_unit_name: &'static str,
    pub(super) total_ms: f64,
    pub(super) mean_ms: f64,
    pub(super) median_ms: f64,
    pub(super) min_ms: f64,
    pub(super) max_ms: f64,
    pub(super) stddev_ms: f64,
    pub(super) sample_ms: Vec<f64>,
    pub(super) observed_items: usize,
}
