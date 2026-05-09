use crate::cli::reporter::{CliError, CliReport};
use crate::config::ConfigLoader;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(super) fn collect_config_validation_errors(
    files: &[PathBuf],
    explicit: Option<&Path>,
) -> Vec<CliError> {
    if let Some(path) = explicit {
        let config = match ConfigLoader::load_effective_config(path, Some(path)) {
            Ok(config) => config,
            Err(err) => return vec![CliError::config(path, err)],
        };
        return config
            .validate_against_schema()
            .into_iter()
            .map(|error| CliError::config_validation(path, error))
            .collect();
    }

    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for file in files {
        let effective = match ConfigLoader::load_effective_config_with_source(file, None) {
            Ok(effective) => effective,
            Err(err) => {
                errors.push(CliError::config(file, err));
                continue;
            }
        };
        let Some(source) = effective.source else {
            continue;
        };
        if !seen.insert(source.clone()) {
            continue;
        }
        errors.extend(
            effective
                .config
                .validate_against_schema()
                .into_iter()
                .map(|error| CliError::config_validation(&source, error)),
        );
    }
    errors
}

pub(super) fn has_blocking_errors(report: &CliReport, ignore_config_errors: bool) -> bool {
    report.errors.iter().any(|error| {
        !ignore_config_errors
            || error.kind != "config"
            || error.message_id.as_str() == "config.error"
    })
}
