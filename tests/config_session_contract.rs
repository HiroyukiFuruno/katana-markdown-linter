use katana_markdown_linter::cli::{Cli, CliWorkflow, Command, OutputFormat};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn consecutive_cli_workflow_runs_observe_config_changes() {
    let dir = TestDir::new();
    let file = dir.path().join("input.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "#Title\n").unwrap();
    fs::write(&config, r#"{"default":false}"#).unwrap();

    let first = CliWorkflow::run(Cli {
        command: Command::Check,
        format: OutputFormat::Json,
        inputs: vec![path(&file)],
        ..Cli::default()
    })
    .unwrap();
    fs::write(&config, r#"{"default":false,"MD018":true}"#).unwrap();
    let second = CliWorkflow::run(Cli {
        command: Command::Check,
        format: OutputFormat::Json,
        inputs: vec![path(&file)],
        ..Cli::default()
    })
    .unwrap();

    assert_eq!(first, 0);
    assert_eq!(second, 1);
}

#[test]
fn consecutive_formatter_runs_observe_layout_property_changes() {
    let dir = TestDir::new();
    let file = dir.path().join("input.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "- outer\n    - inner\n").unwrap();
    for (indent, expected) in [(4, "- outer\n    - inner\n"), (2, "- outer\n  - inner\n")] {
        fs::write(
            &config,
            format!(r#"{{"default":false,"MD007":{{"enabled":true,"indent":{indent}}}}}"#),
        )
        .unwrap();
        let exit = CliWorkflow::run(Cli {
            command: Command::Fmt,
            format: OutputFormat::Json,
            inputs: vec![path(&file)],
            ..Cli::default()
        })
        .unwrap();
        assert_eq!(exit, 0);
        assert_eq!(fs::read_to_string(&file).unwrap(), expected);
    }
}

#[test]
fn nearest_child_config_wins_and_explicit_config_overrides_discovery() {
    let dir = TestDir::new();
    let child = dir.path().join("child");
    fs::create_dir_all(&child).unwrap();
    let parent_config = dir.path().join(".markdownlint.json");
    let child_config = child.join(".markdownlint.json");
    let explicit_config = dir.path().join("explicit.json");
    let parent_file = dir.path().join("parent.md");
    let child_file = child.join("child.md");
    fs::write(&parent_config, r#"{"default":false,"MD018":true}"#).unwrap();
    fs::write(&child_config, r#"{"default":false}"#).unwrap();
    fs::write(&explicit_config, r#"{"default":false,"MD018":true}"#).unwrap();
    fs::write(&parent_file, "#Title\n").unwrap();
    fs::write(&child_file, "#Title\n").unwrap();

    let nearest = CliWorkflow::run(Cli {
        command: Command::Fix,
        format: OutputFormat::Json,
        inputs: vec![path(&parent_file), path(&child_file)],
        ..Cli::default()
    })
    .unwrap();
    assert_eq!(nearest, 0);
    assert_eq!(fs::read_to_string(&parent_file).unwrap(), "# Title\n");
    assert_eq!(fs::read_to_string(&child_file).unwrap(), "#Title\n");

    let explicit = CliWorkflow::run(Cli {
        command: Command::Fix,
        config: Some(explicit_config),
        format: OutputFormat::Json,
        inputs: vec![path(&child_file)],
        ..Cli::default()
    })
    .unwrap();
    assert_eq!(explicit, 0);
    assert_eq!(fs::read_to_string(&child_file).unwrap(), "# Title\n");
}

#[test]
fn invalid_config_keeps_strict_and_ignore_error_behavior() {
    let dir = TestDir::new();
    let file = dir.path().join("input.md");
    let config = dir.path().join("invalid.json");
    let original = "#Title\n";
    fs::write(&file, original).unwrap();
    fs::write(&config, r#"{"default":false,"MD018":{"enabled":"yes"}}"#).unwrap();

    let strict = CliWorkflow::run(Cli {
        command: Command::Fix,
        config: Some(config.clone()),
        format: OutputFormat::Json,
        inputs: vec![path(&file)],
        ..Cli::default()
    })
    .unwrap();
    assert_eq!(strict, 2);
    assert_eq!(fs::read_to_string(&file).unwrap(), original);

    let ignored = CliWorkflow::run(Cli {
        command: Command::Fix,
        config: Some(config),
        format: OutputFormat::Json,
        inputs: vec![path(&file)],
        ignore_config_errors: true,
        ..Cli::default()
    })
    .unwrap();
    assert_eq!(ignored, 0);
    assert_eq!(fs::read_to_string(&file).unwrap(), original);
}

fn path(path: &Path) -> String {
    path.display().to_string()
}

struct TestDir(PathBuf);

impl TestDir {
    fn new() -> Self {
        static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "kml-config-session-contract-{}-{nonce}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
