use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn check_json_does_not_mutate_file() {
    let dir = TestDir::new("check-json-no-write");
    let file = dir.path().join("bad.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "#Title\n").expect("fixture should be written");
    fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
        .expect("config should be written");

    let output = run_kml(
        [
            "check",
            "--output",
            "json",
            "--config",
            file_text(&config).as_str(),
            file_text(&file).as_str(),
        ],
        None,
    );

    assert!(!output.status.success());
    assert_eq!(
        fs::read_to_string(&file).expect("fixture should be readable"),
        "#Title\n"
    );
    let json = stdout_json(&output);
    assert_eq!(json["command"], "check");
    assert_eq!(json["files"][0]["changed"], false);
    assert_eq!(json["files"][0]["applied_fixes"], 0);
    assert_eq!(json["summary"]["total_issues"], 1);
}

#[test]
fn check_fix_json_keeps_check_command_identity() {
    let dir = TestDir::new("check-fix-json-command");
    let file = dir.path().join("bad.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "#Title\n").expect("fixture should be written");
    fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
        .expect("config should be written");

    let output = run_kml(
        [
            "check",
            "--fix",
            "--output",
            "json",
            "--config",
            file_text(&config).as_str(),
            file_text(&file).as_str(),
        ],
        None,
    );

    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(&file).expect("fixture should be readable"),
        "# Title\n"
    );
    let json = stdout_json(&output);
    assert_eq!(json["command"], "check");
    assert_eq!(json["files"][0]["changed"], true);
    assert_eq!(json["files"][0]["applied_fixes"], 1);
    assert_eq!(json["files"][0]["fix_details"][0]["rule_id"], "MD018");
    assert_eq!(json["files"][0]["fix_details"][0]["applied"], true);
    assert_eq!(json["summary"]["total_issues"], 0);
}

#[test]
fn fix_json_reports_remaining_unsafe_diagnostics_without_applying_them() {
    let dir = TestDir::new("fix-json-safe-only");
    let file = dir.path().join("bad.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "**Important**\n\nText\n").expect("fixture should be written");
    fs::write(&config, "{ \"default\": false, \"MD036\": true }\n")
        .expect("config should be written");

    let output = run_kml(
        [
            "fix",
            "--output",
            "json",
            "--config",
            file_text(&config).as_str(),
            file_text(&file).as_str(),
        ],
        None,
    );

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        fs::read_to_string(&file).expect("fixture should be readable"),
        "**Important**\n\nText\n"
    );
    let json = stdout_json(&output);
    assert_eq!(json["command"], "fix");
    assert_eq!(json["files"][0]["changed"], false);
    assert_eq!(
        json["files"][0]["diagnostics"][0]["fix"]["safety"],
        "unsafe"
    );
    assert_eq!(
        json["summary"]["unsafe_fix_status"],
        "unsafe_mode_not_enabled"
    );
}

#[test]
fn fmt_json_uses_formatter_contract_without_lint_fixing_heading_text() {
    let dir = TestDir::new("fmt-json-contract");
    let file = dir.path().join("bad.md");
    fs::write(&file, "#Title\r\nText\n\n\n").expect("fixture should be written");

    let output = run_kml(["fmt", "--output", "json", file_text(&file).as_str()], None);

    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(&file).expect("fixture should be readable"),
        "#Title\nText\n"
    );
    let json = stdout_json(&output);
    assert_eq!(json["command"], "fmt");
    assert_eq!(json["files"][0]["changed"], true);
    assert_eq!(json["files"][0]["diagnostics"].as_array().unwrap().len(), 0);
}

#[test]
fn config_schema_outputs_json_schema_for_editor_completion() {
    let output = run_kml(["config", "schema", "--output", "json"], None);

    assert!(output.status.success());
    let json = stdout_json(&output);
    assert_eq!(
        json["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        json["$id"],
        "https://schemas.katana.tools/kml/markdownlint.schema.json"
    );
    assert_eq!(json["type"], "object");
    assert_eq!(json["additionalProperties"], false);
    assert_eq!(json["properties"]["default"]["type"], "boolean");
    assert_eq!(json["properties"]["MD003"]["anyOf"][0]["type"], "boolean");
    let style_options = json["properties"]["MD003"]["anyOf"][1]["properties"]["style"]["enum"]
        .as_array()
        .expect("style enum should be an array");
    assert!(style_options.contains(&Value::String("atx".to_string())));
}

#[test]
fn config_validation_reports_schema_property_errors() {
    let dir = TestDir::new("config-schema-validation");
    let file = dir.path().join("bad.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "# Title\n").expect("fixture should be written");
    fs::write(
        &config,
        r#"{ "default": true, "MD003": { "style": "invalid" } }"#,
    )
    .expect("config should be written");

    let output = run_kml(
        [
            "check",
            "--output",
            "json",
            "--config",
            file_text(&config).as_str(),
            file_text(&file).as_str(),
        ],
        None,
    );

    assert_eq!(output.status.code(), Some(2));
    let json = stdout_json(&output);
    assert_eq!(json["errors"][0]["kind"], "config");
    assert_eq!(json["errors"][0]["message_id"], "config.invalid_enum_value");
    assert!(json["errors"][0]["message"]
        .as_str()
        .expect("message should be text")
        .contains("MD003.style"));
}

fn run_kml<const N: usize>(args: [&str; N], stdin: Option<&str>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kml"));
    command.args(args);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().expect("kml should start");
    if let Some(input) = stdin {
        child
            .stdin
            .as_mut()
            .expect("stdin should be piped")
            .write_all(input.as_bytes())
            .expect("stdin should be written");
    }
    child.wait_with_output().expect("kml should finish")
}

fn stdout_json(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout should be json: {error}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn file_text(path: &Path) -> String {
    path.display().to_string()
}

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "katana-markdown-linter-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test dir should be created");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
