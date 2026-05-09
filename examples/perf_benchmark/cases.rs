#[path = "cases/api.rs"]
mod api;
#[path = "cases/cli.rs"]
mod cli;
#[path = "cases/context.rs"]
mod context;

use super::args::Args;
use super::documents::BenchDocuments;
use super::measure::measure;
use super::report::Case;
use super::workspace;
use super::BenchResult;
use katana_markdown_linter::{LintOptions, RuleCatalogService};

const CONFIG_CASE_SCALE: usize = 50;
const CATALOG_CASE_SCALE: usize = 50;

pub(super) fn collect_cases(args: &Args) -> BenchResult<Vec<Case>> {
    let options = LintOptions::default();
    let documents = BenchDocuments::generate();
    let check_workspace = workspace::prepare_cli_workspace(
        "check",
        "{ \"default\": true }\n",
        "# Document\n\nParagraph text.\n\n",
    )?;
    let config_path = workspace::prepare_config_fixture()?;
    let mut cases = Vec::new();

    api::extend_api_cases(&mut cases, args, &documents, &options)?;
    context::extend_context_cases(&mut cases, args, &documents)?;
    cli::extend_cli_cases(&mut cases, args, &check_workspace)?;
    cases.push(measure(
        "config_validate_representative",
        &args.scaled(CONFIG_CASE_SCALE),
        1,
        "config",
        || workspace::validate_config(&config_path),
    )?);
    cases.push(measure(
        "api_rule_catalog",
        &args.scaled(CATALOG_CASE_SCALE),
        1,
        "catalog",
        || Ok(RuleCatalogService::available_rules().len()),
    )?);

    Ok(cases)
}
