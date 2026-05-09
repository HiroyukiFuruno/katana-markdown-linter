use super::test_dir;
use crate::cli::args::{Cli, Command, OutputFormat};
use crate::cli::workflow::CliWorkflow;
use std::fs;

#[test]
fn fmt_applies_layout_formatting_and_is_idempotent() {
    let dir = test_dir("fmt-layout");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    fs::write(&file, "# Title\r\nText\n\n\n-  item").expect("test file should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Fmt,
        inputs: vec![file.display().to_string()],
        ..Cli::default()
    })
    .expect("fmt should run");

    assert_eq!(exit, 0);
    assert_eq!(
        fs::read_to_string(&file).expect("fixed file should be readable"),
        "# Title\n\nText\n\n- item\n"
    );

    let second_exit = CliWorkflow::run(Cli {
        command: Command::Fmt,
        inputs: vec![file.display().to_string()],
        ..Cli::default()
    })
    .expect("fmt should run again");

    assert_eq!(second_exit, 0);
    assert_eq!(
        fs::read_to_string(&file).expect("fixed file should be readable"),
        "# Title\n\nText\n\n- item\n"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn fmt_does_not_apply_non_layout_safe_fixes_or_return_one_for_lint_issues() {
    let dir = test_dir("fmt-non-layout");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    fs::write(&file, "#Title\n").expect("test file should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Fmt,
        inputs: vec![file.display().to_string()],
        ..Cli::default()
    })
    .expect("fmt should run");

    assert_eq!(exit, 0);
    assert_eq!(
        fs::read_to_string(&file).expect("fixed file should be readable"),
        "#Title\n"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn fmt_stdin_contract_formats_markdown_only() {
    let formatted = crate::cli::workflow::fmt::format_stdin_content("# Title\r\nText\n\n\n")
        .expect("stdin format should run");

    assert_eq!(formatted, "# Title\n\nText\n");
}

#[test]
fn explicit_config_can_disable_default_rules() {
    let dir = test_dir("config-disable");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("bad.md");
    let config = dir.join(".markdownlint.jsonc");
    fs::write(&file, "#Title\n").expect("test file should be written");
    fs::write(
        &config,
        "{\n  // Disable all rules\n  \"default\": false,\n}\n",
    )
    .expect("config should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Check,
        config: Some(config),
        format: OutputFormat::Json,
        inputs: vec![file.display().to_string()],
        ..Cli::default()
    })
    .expect("check should run");

    assert_eq!(exit, 0);
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn missing_explicit_config_is_config_error() {
    let dir = test_dir("missing-config");
    fs::create_dir_all(&dir).expect("test dir should be created");
    let file = dir.join("good.md");
    fs::write(&file, "# Title\n").expect("test file should be written");

    let exit = CliWorkflow::run(Cli {
        command: Command::Check,
        config: Some(dir.join("missing.json")),
        format: OutputFormat::Json,
        inputs: vec![file.display().to_string()],
        ..Cli::default()
    })
    .expect("check should run");

    assert_eq!(exit, 2);
    let _ = fs::remove_dir_all(dir);
}
