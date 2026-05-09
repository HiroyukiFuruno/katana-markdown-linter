mod build;
mod compare;
mod entry;
mod report;
mod types;

use std::path::Path;

use super::document::UpstreamCatalog;

pub use types::{
    RuleFixtureCase, RuleFixtureDriftItem, RuleFixtureDriftReport, RuleFixtureDriftSummary,
    RuleFixtureDriftType, RuleFixtureEntry, RuleFixtureMatrix, RuleFixtureMatrixSummary,
};

pub struct RuleFixtureMatrixService;

impl RuleFixtureMatrixService {
    pub fn build_fixture_matrix(catalog: &UpstreamCatalog) -> RuleFixtureMatrix {
        build::build_fixture_matrix(catalog)
    }

    pub fn render_fixture_matrix_summary(matrix: &RuleFixtureMatrix) -> String {
        report::render_fixture_matrix_summary(matrix)
    }

    pub fn write_fixture_matrix_files(
        matrix: &RuleFixtureMatrix,
        output_dir: &Path,
    ) -> Result<(), String> {
        report::write_fixture_matrix_files(matrix, output_dir)
    }

    pub fn compare_fixture_matrix(
        catalog: &UpstreamCatalog,
        matrix: &RuleFixtureMatrix,
    ) -> RuleFixtureDriftReport {
        compare::compare_fixture_matrix(catalog, matrix)
    }
}

#[cfg(test)]
mod tests;
