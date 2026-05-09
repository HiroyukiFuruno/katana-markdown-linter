use super::BenchResult;
use katana_markdown_linter::cli::{Cli, CliWorkflow, Command, OutputFormat};
use katana_markdown_linter::MarkdownLintConfig;
use std::fs;
use std::path::{Path, PathBuf};

pub(super) const CLI_WORKSPACE_FILES: usize = 80;

pub(super) fn prepare_cli_workspace(
    name: &str,
    config: &str,
    content: &str,
) -> BenchResult<PathBuf> {
    let dir =
        std::env::temp_dir().join(format!("kml-perf-workspace-{name}-{}", std::process::id()));
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    fs::create_dir_all(&dir)?;
    fs::write(dir.join(".markdownlint.json"), config)?;
    for index in 0..CLI_WORKSPACE_FILES {
        fs::write(dir.join(format!("doc-{index:03}.md")), content)?;
    }
    Ok(dir)
}

pub(super) fn run_cli_mutating_workflow(
    name: &str,
    command: Command,
    config: &str,
    content: &str,
) -> BenchResult<usize> {
    let workspace = prepare_cli_workspace(name, config, content)?;
    let exit = CliWorkflow::run(Cli {
        command,
        format: OutputFormat::Text,
        inputs: vec![workspace.display().to_string()],
        quiet: true,
        ..Cli::default()
    })
    .map_err(std::io::Error::other)?;
    fs::remove_dir_all(&workspace)?;
    if exit != 0 {
        return Err(
            std::io::Error::other(format!("kml mutating workflow exited with {exit}")).into(),
        );
    }
    Ok(CLI_WORKSPACE_FILES)
}

pub(super) fn prepare_config_fixture() -> BenchResult<PathBuf> {
    let dir = std::env::temp_dir().join(format!("kml-perf-config-{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    let path = dir.join(".markdownlint.jsonc");
    fs::write(
        &path,
        r#"{
  "default": true,
  "MD013": false,
  "MD024": true,
  "MD031": true,
  "MD048": true
}
"#,
    )?;
    Ok(path)
}

pub(super) fn run_cli_check(workspace: &Path) -> BenchResult<usize> {
    let exit = CliWorkflow::run(Cli {
        command: Command::Check,
        format: OutputFormat::Text,
        inputs: vec![workspace.display().to_string()],
        quiet: true,
        ..Cli::default()
    })
    .map_err(std::io::Error::other)?;
    if exit != 0 {
        return Err(std::io::Error::other(format!("kml check exited with {exit}")).into());
    }
    Ok(CLI_WORKSPACE_FILES)
}

pub(super) fn validate_config(config_path: &Path) -> BenchResult<usize> {
    let config = MarkdownLintConfig::load(config_path)?;
    let rules =
        katana_markdown_linter::rules::markdown::MarkdownLinterOps::user_configurable_rules();
    let errors = config.validate_cached_rules();
    if !errors.is_empty() {
        return Err(std::io::Error::other(format!(
            "config validation failed with {} errors",
            errors.len()
        ))
        .into());
    }
    Ok(rules.len())
}
