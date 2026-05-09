use super::super::args::Args;
use super::super::measure::measure;
use super::super::report::Case;
use super::super::workspace::{
    self, run_cli_check, run_cli_mutating_workflow, CLI_WORKSPACE_FILES,
};
use super::super::BenchResult;
use katana_markdown_linter::cli::Command;
use std::path::Path;

pub(super) fn extend_cli_cases(
    cases: &mut Vec<Case>,
    args: &Args,
    check_workspace: &Path,
) -> BenchResult<()> {
    cases.push(measure(
        "cli_check_many_small_files",
        args,
        CLI_WORKSPACE_FILES,
        "files",
        || run_cli_check(check_workspace),
    )?);
    cases.push(measure(
        "cli_fix_many_small_files",
        args,
        CLI_WORKSPACE_FILES,
        "files",
        || {
            run_cli_mutating_workflow(
                "fix",
                Command::Fix,
                "{ \"default\": false, \"MD018\": true }\n",
                "#Title\n",
            )
        },
    )?);
    cases.push(measure(
        "cli_fmt_many_small_files",
        args,
        CLI_WORKSPACE_FILES,
        "files",
        || {
            workspace::run_cli_mutating_workflow(
                "fmt",
                Command::Fmt,
                "{ \"default\": false }\n",
                "# Title\r\nText\n\n\n",
            )
        },
    )?);
    Ok(())
}
