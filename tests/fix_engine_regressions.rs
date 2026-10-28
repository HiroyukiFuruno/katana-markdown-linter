use katana_markdown_linter::{LintOptions, MarkdownLinter};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const INPUT: &str = "#Title\n\n\nParagraph\n";
const EXPECTED: &str = "# Title\n\nParagraph\n";

#[test]
fn public_fix_applies_syntax_before_layout_and_is_idempotent() {
    let options = LintOptions::default();
    let first = MarkdownLinter::fix(INPUT, &options).expect("fix should converge");
    assert_eq!(first.content, EXPECTED);
    let applied = first
        .details
        .iter()
        .filter(|detail| detail.applied)
        .map(|detail| detail.rule_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(applied, ["MD018", "MD012"]);

    let second = MarkdownLinter::fix(&first.content, &options).expect("second fix should converge");
    assert_eq!(second.content, first.content);
    assert_eq!(second.applied_fixes, 0);
}

#[test]
fn cli_check_fix_writes_ordered_converged_output_and_second_run_is_unchanged() {
    let dir = TestDir::new();
    let file = dir.path().join("fix-order.md");
    fs::write(&file, INPUT).expect("fixture should be written");

    let first = run_kml(&["check", "--fix", "--output", "json", path(&file).as_str()]);
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let report: Value = serde_json::from_slice(&first.stdout).expect("CLI should emit JSON");
    assert_eq!(report["files"][0]["changed"], true);
    assert_eq!(fs::read_to_string(&file).unwrap(), EXPECTED);

    let second = run_kml(&["check", "--fix", "--output", "json", path(&file).as_str()]);
    assert!(
        second.status.success(),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    let report: Value = serde_json::from_slice(&second.stdout).expect("CLI should emit JSON");
    assert_eq!(report["files"][0]["changed"], false);
    assert_eq!(fs::read_to_string(&file).unwrap(), EXPECTED);
}

#[test]
fn conflicting_layout_policy_returns_json_error_and_preserves_failed_input() {
    let dir = TestDir::new();
    let file = dir.path().join("cycle.md");
    let successful_file = dir.path().join("successful.md");
    let config = dir.path().join(".markdownlint.json");
    let input = "#Title\nParagraph\n";
    fs::write(&file, input).expect("fixture should be written");
    fs::write(&successful_file, "# Ready\n").expect("successful fixture should be written");
    fs::write(
        &config,
        r#"{"default":true,"MD012":{"maximum":0},"MD022":true}"#,
    )
    .expect("conflicting policy should be written");
    let file_path = path(&file);
    let successful_path = path(&successful_file);
    let result = run_kml(&[
        "check",
        "--fix",
        "--output",
        "json",
        "--config",
        path(&config).as_str(),
        file_path.as_str(),
        successful_path.as_str(),
    ]);
    assert_eq!(result.status.code(), Some(2));
    let report: Value = serde_json::from_slice(&result.stdout).expect("CLI should emit JSON");
    assert_eq!(report["command"], "check");
    assert_eq!(report["errors"][0]["kind"], "rule");
    assert_eq!(report["errors"][0]["path"], file_path);
    assert!(report["errors"][0]["message"]
        .as_str()
        .unwrap()
        .contains("cycle"));
    assert_eq!(report["files"].as_array().unwrap().len(), 1);
    assert_eq!(report["files"][0]["path"], successful_path);
    assert_eq!(fs::read_to_string(file).unwrap(), input);
    assert_eq!(fs::read_to_string(successful_file).unwrap(), "# Ready\n");
}

#[test]
fn stdin_conflicting_layout_policy_returns_json_error_without_fixed_content() {
    let dir = TestDir::new();
    let config = dir.path().join(".markdownlint.json");
    fs::write(
        &config,
        r#"{"default":true,"MD012":{"maximum":0},"MD022":true}"#,
    )
    .expect("conflicting policy should be written");
    let config_path = path(&config);
    let result = run_kml_with_stdin(
        &[
            "check",
            "--fix",
            "--stdin",
            "--output",
            "json",
            "--config",
            config_path.as_str(),
        ],
        "#Title\nParagraph\n",
    );

    assert_eq!(result.status.code(), Some(2));
    let report: Value = serde_json::from_slice(&result.stdout).expect("CLI should emit JSON");
    assert_eq!(report["errors"][0]["kind"], "rule");
    assert_eq!(report["errors"][0]["path"], "<stdin>");
    assert!(report["errors"][0]["message"]
        .as_str()
        .unwrap()
        .contains("cycle"));
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
    child
        .stdin
        .as_mut()
        .expect("stdin should be piped")
        .write_all(input.as_bytes())
        .expect("input should be written");
    child.wait_with_output().expect("kml should finish")
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
            .expect("system clock should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "kml-fix-engine-{}-{nonce}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("temporary fixture directory should be created");
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
