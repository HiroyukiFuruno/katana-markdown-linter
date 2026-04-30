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
fn bare_command_prints_usage_without_linting_workspace() {
    let dir = TestDir::new("bare-command-help");
    let output = run_kml_in([], None, dir.path());

    assert!(
        output.status.success(),
        "bare command should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_contains(&stdout, "Usage: kml <command> [options] [paths...]");
    assert_contains(&stdout, "Commands:");
    assert_contains(&stdout, "check");
}

#[test]
fn help_commands_print_usage_without_linting_workspace() {
    for argument in ["--help", "-h", "help"] {
        let dir = TestDir::new("help-command");
        let output = run_kml_in([argument], None, dir.path());

        assert!(
            output.status.success(),
            "{argument} should exit successfully\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_contains(&stdout, "Usage: kml <command> [options] [paths...]");
        assert_contains(&stdout, "Commands:");
        assert_contains(&stdout, "check");
        assert_contains(&stdout, "version");
    }
}

#[test]
fn command_help_flags_print_command_usage() {
    for command_name in [
        "check",
        "fix",
        "fmt",
        "rule",
        "config",
        "init-config",
        "lsp",
        "version",
    ] {
        for help_flag in ["--help", "-h"] {
            let dir = TestDir::new("command-help");
            let output = run_kml_in([command_name, help_flag], None, dir.path());

            assert!(
                output.status.success(),
                "{command_name} {help_flag} should exit successfully\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(String::from_utf8_lossy(&output.stderr), "");

            let stdout = String::from_utf8_lossy(&output.stdout);
            assert_contains(&stdout, &format!("Usage: kml {command_name}"));
        }
    }
}

#[test]
fn help_uses_explicit_or_environment_japanese_locale() {
    let explicit_dir = TestDir::new("explicit-japanese-help");
    let explicit = run_kml_in(["--locale", "ja", "help"], None, explicit_dir.path());

    assert!(
        explicit.status.success(),
        "explicit Japanese help should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&explicit.stdout),
        String::from_utf8_lossy(&explicit.stderr)
    );
    let stdout = String::from_utf8_lossy(&explicit.stdout);
    assert_contains(&stdout, "使い方: kml <command> [options] [paths...]");
    assert_contains(&stdout, "--locale <locale>, -l");
    assert_contains(&stdout, "診断・ヘルプの表示言語を指定します");

    let english_dir = TestDir::new("english-help-locale-option");
    let english = run_kml_in(["help"], None, english_dir.path());

    assert!(
        english.status.success(),
        "English help should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&english.stdout),
        String::from_utf8_lossy(&english.stderr)
    );
    let stdout = String::from_utf8_lossy(&english.stdout);
    assert_contains(&stdout, "--locale <locale>, -l");
    assert_contains(&stdout, "Use localized output and help text.");

    let env_dir = TestDir::new("environment-japanese-help");
    let environment = run_kml_in_with_env(
        ["check", "--help"],
        None,
        env_dir.path(),
        [("LC_ALL", "ja_JP.UTF-8"), ("LANG", "ja_JP.UTF-8")],
    );

    assert!(
        environment.status.success(),
        "environment Japanese help should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&environment.stdout),
        String::from_utf8_lossy(&environment.stderr)
    );
    let stdout = String::from_utf8_lossy(&environment.stdout);
    assert_contains(&stdout, "使い方: kml check [options] [paths...]");
    assert_contains(&stdout, "--locale <locale>, -l");
    assert_contains(&stdout, "診断・ヘルプの表示言語を指定します");
}

#[test]
fn check_accepts_official_alias_config_without_config_errors() {
    let dir = TestDir::new("official-alias-config");
    let file = dir.path().join("ok.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "# Title\n\nText\n").expect("fixture should be written");
    fs::write(
        &config,
        r#"{
  "default": false,
  "first-line-h1": { "allow_preamble": true },
  "first-line-heading": false,
  "no-duplicate-heading": false,
  "no-inline-html": false,
  "MD022": {
    "lines_above": [1, 1, 1, 1, 1, 1],
    "lines_below": 1
  }
}"#,
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

    assert!(
        output.status.success(),
        "official alias config should not produce config errors\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(stdout_json(&output)["summary"]["total_issues"], 0);
}

#[test]
fn invalid_config_stops_before_linting_and_guides_override() {
    let dir = TestDir::new("invalid-config-stops");
    let file = dir.path().join("bad.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "#Title\n").expect("fixture should be written");
    fs::write(
        &config,
        r#"{ "default": false, "MD018": true, "MD003": { "style": "invalid" } }"#,
    )
    .expect("config should be written");

    let output = run_kml(
        [
            "check",
            "--config",
            file_text(&config).as_str(),
            file_text(&file).as_str(),
        ],
        None,
    );

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_contains(&stderr, "config error:");
    assert_contains(&stderr, "--ignore-config-errors");
    assert!(
        !stderr.contains("MD018"),
        "lint diagnostics should not be emitted when config is invalid\nstderr:\n{stderr}"
    );
}

#[test]
fn ignore_config_errors_continues_with_invalid_entries_ignored() {
    let dir = TestDir::new("ignore-config-errors");
    let file = dir.path().join("bad.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&file, "#Title\n").expect("fixture should be written");
    fs::write(
        &config,
        r#"{ "default": false, "MD018": true, "MD003": { "style": "invalid" } }"#,
    )
    .expect("config should be written");

    let output = run_kml(
        [
            "check",
            "--ignore-config-errors",
            "--output",
            "json",
            "--config",
            file_text(&config).as_str(),
            file_text(&file).as_str(),
        ],
        None,
    );

    assert_eq!(output.status.code(), Some(1));
    let json = stdout_json(&output);
    assert_eq!(json["errors"][0]["kind"], "config");
    assert_eq!(json["files"][0]["diagnostics"][0]["rule_id"], "MD018");
}

#[test]
fn version_aliases_print_package_version() {
    for argument in ["version", "--version", "-V", "-v"] {
        let dir = TestDir::new("version-command");
        let output = run_kml_in([argument], None, dir.path());

        assert!(
            output.status.success(),
            "{argument} should exit successfully\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stderr), "");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout.trim(), env!("CARGO_PKG_VERSION"));
    }
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
    let current_dir = std::env::current_dir().expect("current dir should be available");
    run_kml_in(args, stdin, &current_dir)
}

fn run_kml_in<const N: usize>(
    args: [&str; N],
    stdin: Option<&str>,
    current_dir: &Path,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kml"));
    command.args(args);
    command.current_dir(current_dir);
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

fn run_kml_in_with_env<const N: usize, const M: usize>(
    args: [&str; N],
    stdin: Option<&str>,
    current_dir: &Path,
    envs: [(&str, &str); M],
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kml"));
    command.args(args);
    command.current_dir(current_dir);
    command.envs(envs);
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

fn assert_contains(text: &str, expected: &str) {
    assert!(
        text.contains(expected),
        "expected output to contain {expected:?}\noutput:\n{text}"
    );
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
