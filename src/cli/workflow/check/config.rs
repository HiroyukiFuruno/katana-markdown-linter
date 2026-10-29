use crate::cli::reporter::{CliError, CliReport};
use crate::config::ConfigSession;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(super) fn collect_config_validation_errors(
    files: &[PathBuf],
    session: &mut ConfigSession,
) -> Vec<CliError> {
    if let Some(path) = session.explicit_path().map(Path::to_path_buf) {
        let config = match session.load_for_path(&path) {
            Ok(config) => config,
            Err(err) => return vec![CliError::config(&path, err)],
        };
        return config
            .validation_errors
            .iter()
            .cloned()
            .map(|error| CliError::config_validation(&path, error))
            .collect();
    }

    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for file in files {
        let effective = match session.load_for_path(file) {
            Ok(effective) => effective,
            Err(err) => {
                errors.push(CliError::config(file, err));
                continue;
            }
        };
        let Some(source) = effective.source.as_deref() else {
            continue;
        };
        if !seen.insert(source.to_path_buf()) {
            continue;
        }
        errors.extend(
            effective
                .validation_errors
                .iter()
                .cloned()
                .map(|error| CliError::config_validation(source, error)),
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
