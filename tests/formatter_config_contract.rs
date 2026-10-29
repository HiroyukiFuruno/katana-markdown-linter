use katana_markdown_linter::{ConfigErrorKind, MarkdownLintConfig};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn enabled_rule_object_is_valid_and_matches_public_schema() {
    let config = MarkdownLintConfig {
        raw: json!({
            "default": false,
            "MD007": { "enabled": true, "indent": 4 }
        }),
    };

    let validation_errors = config.validate_against_schema();
    assert!(
        validation_errors.is_empty(),
        "enabled object should validate: {validation_errors:?}"
    );

    let schema = MarkdownLintConfig::schema();
    let enabled = schema["properties"]["MD007"]["anyOf"][1]["properties"]["enabled"]
        .as_object()
        .expect("schema should expose the enabled property");
    assert_eq!(enabled["type"], "boolean");
    assert_eq!(enabled["description"], "Enable or disable MD007.");
    assert!(!enabled.contains_key("default"));

    let options = config.to_lint_options();
    assert!(options.rules["MD007"].enabled);
    assert_eq!(options.rules["MD007"].properties["indent"], "4");
    assert!(!options.rules["MD001"].enabled);
}

#[test]
fn invalid_enabled_type_is_rejected_and_default_false_stays_disabled() {
    let invalid = MarkdownLintConfig {
        raw: json!({ "default": false, "MD007": { "enabled": "yes" } }),
    };
    let errors = invalid.validate_against_schema();
    assert!(errors.iter().any(|error| {
        error.rule_id.as_deref() == Some("MD007")
            && error.property.as_deref() == Some("enabled")
            && matches!(
                error.kind,
                ConfigErrorKind::InvalidType {
                    actual: "string",
                    ..
                }
            )
    }));

    let config = MarkdownLintConfig {
        raw: json!({ "default": false, "MD007": { "indent": 4 } }),
    };
    assert!(config.validate_against_schema().is_empty());
    let options = config.to_lint_options();
    assert!(!options.rules["MD007"].enabled);

    let dir = TestDir::new();
    let file = dir.path().join("invalid-enabled.md");
    let config_path = dir.path().join("invalid.json");
    fs::write(&file, "- parent\n").unwrap();
    fs::write(
        &config_path,
        r#"{"default":false,"MD007":{"enabled":"yes"}}"#,
    )
    .unwrap();
    let result = run_kml(&[
        "fmt",
        "--output",
        "json",
        "--config",
        &path(&config_path),
        &path(&file),
    ]);
    assert_eq!(result.status.code(), Some(2));
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["errors"][0]["kind"], "config");
}

#[test]
fn cli_fmt_uses_resolved_layout_config_for_file_and_stdin() {
    let dir = TestDir::new();
    let file = dir.path().join("four-space-list.md");
    let config = dir.path().join(".markdownlint.json");
    let input = "- parent\n    - child\n";
    fs::write(&file, input).expect("input should be written");
    fs::write(
        &config,
        r#"{"default":false,"MD007":{"enabled":true,"indent":4}}"#,
    )
    .expect("config should be written");

    let file_fmt = run_kml(&[
        "fmt",
        "--output",
        "json",
        "--config",
        &path(&config),
        &path(&file),
    ]);
    assert_eq!(file_fmt.status.code(), Some(0), "{}", stderr(&file_fmt));
    assert_eq!(fs::read_to_string(&file).unwrap(), input);

    let check = run_kml(&[
        "check",
        "--output",
        "json",
        "--config",
        &path(&config),
        &path(&file),
    ]);
    assert_eq!(check.status.code(), Some(0), "{}", stderr(&check));

    let stdin_fmt = run_kml_with_stdin(
        &[
            "fmt",
            "--stdin",
            "--output",
            "json",
            "--config",
            &path(&config),
        ],
        input,
    );
    assert_eq!(stdin_fmt.status.code(), Some(0), "{}", stderr(&stdin_fmt));
    assert_eq!(String::from_utf8(stdin_fmt.stdout).unwrap(), input);
}

#[test]
fn cli_fmt_discovers_config_and_honors_disabled_layout_and_non_layout_exclusion() {
    let dir = TestDir::new();
    let file = dir.path().join("two-space-list.md");
    let config = dir.path().join(".markdownlint.json");
    let input = "- parent\n  - child\n";
    fs::write(&file, input).expect("input should be written");
    fs::write(
        &config,
        r#"{"default":true,"MD007":{"enabled":false,"indent":4},"MD018":{"enabled":true}}"#,
    )
    .expect("config should be written");

    let discovered_fmt = run_kml(&["fmt", "--output", "json", &path(&file)]);
    assert_eq!(
        discovered_fmt.status.code(),
        Some(0),
        "{}",
        stderr(&discovered_fmt)
    );
    assert_eq!(fs::read_to_string(&file).unwrap(), input);

    let non_layout = dir.path().join("non-layout.md");
    fs::write(&non_layout, "#Title\n").expect("non-layout input should be written");
    let non_layout_fmt = run_kml(&["fmt", "--output", "json", &path(&non_layout)]);
    assert_eq!(
        non_layout_fmt.status.code(),
        Some(0),
        "{}",
        stderr(&non_layout_fmt)
    );
    assert_eq!(fs::read_to_string(&non_layout).unwrap(), "#Title\n");
}

#[test]
fn cli_fmt_file_batch_reports_fix_engine_errors_and_preserves_failed_file() {
    let dir = TestDir::new();
    let failed = dir.path().join("conflicting.md");
    let successful = dir.path().join("successful.md");
    let config = dir.path().join("conflicting.json");
    let original = "# Title\nParagraph\n";
    fs::write(&failed, original).expect("failed fixture should be written");
    fs::write(&successful, "# Ready\n").expect("successful fixture should be written");
    fs::write(
        &config,
        r#"{"default":true,"MD012":{"maximum":0},"MD022":true}"#,
    )
    .expect("contradictory config should be written");

    let result = run_kml(&[
        "fmt",
        "--output",
        "json",
        "--config",
        &path(&config),
        &path(&failed),
        &path(&successful),
    ]);
    assert_eq!(result.status.code(), Some(2));
    let report: serde_json::Value =
        serde_json::from_slice(&result.stdout).expect("fmt should report a structured JSON error");
    assert_eq!(report["errors"][0]["kind"], "rule");
    assert_eq!(report["errors"][0]["path"], path(&failed));
    assert_eq!(report["files"].as_array().unwrap().len(), 1);
    assert_eq!(report["files"][0]["path"], path(&successful));
    assert_eq!(fs::read_to_string(&failed).unwrap(), original);
    assert_eq!(fs::read_to_string(&successful).unwrap(), "# Ready\n");
}

#[test]
fn cli_fmt_stdin_reports_fix_engine_errors_as_json() {
    let dir = TestDir::new();
    let config = dir.path().join("conflicting.json");
    fs::write(
        &config,
        r#"{"default":true,"MD012":{"maximum":0},"MD022":true}"#,
    )
    .expect("contradictory config should be written");

    let result = run_kml_with_stdin(
        &[
            "fmt",
            "--stdin",
            "--output",
            "json",
            "--config",
            &path(&config),
        ],
        "# Title\nParagraph\n",
    );
    assert_eq!(result.status.code(), Some(2));
    let report: serde_json::Value =
        serde_json::from_slice(&result.stdout).expect("fmt should report a structured JSON error");
    assert_eq!(report["errors"][0]["kind"], "rule");
    assert_eq!(report["errors"][0]["path"], "<stdin>");
    assert!(report["files"].as_array().unwrap().is_empty());
}

fn run_kml(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(args)
        .output()
        .expect("kml should start")
}

fn run_kml_with_stdin(args: &[&str], input: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("kml should start");
    use std::io::Write;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
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
            .expect("system time should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "kml-formatter-config-{}-{nonce}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory should be created");
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
