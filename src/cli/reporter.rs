mod error;
mod localized;
mod render;
mod text;
mod types;

pub(in crate::cli) use error::CliError;
pub(in crate::cli) use render::output_report;
pub(in crate::cli) use text::{plural, print_diff};
pub(crate) use types::{CliReport, CliSummary, FileReport, LocalizedRuleMeta};

#[cfg(test)]
mod tests;
