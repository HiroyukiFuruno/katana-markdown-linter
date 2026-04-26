use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn parser_heavy_check_fix_and_fmt_workflow_converges() {
    let dir = TestDir::new("parser-heavy-convergence");
    let file = dir.path().join("parser-heavy.md");
    let config = dir.path().join(".markdownlint.json");
    fs::write(
        &file,
        "#Title\r\n\n[ link ](https://example.com)\n` code ` and ``<span>``\n* text *\n_italic_ and *italic*\n__bold__ and **bold**\n\n\n",
    )
    .expect("fixture should be written");
    fs::write(
        &config,
        r#"{
  "default": false,
  "MD018": true,
  "MD037": true,
  "MD038": true,
  "MD039": true,
  "MD049": true,
  "MD050": true
}
"#,
    )
    .expect("config should be written");

    let original = read_file(&file);
    let check = run_kml([
        "check",
        "--output",
        "json",
        "--config",
        file_text(&config).as_str(),
        file_text(&file).as_str(),
    ]);
    assert!(!check.status.success());
    assert_eq!(read_file(&file), original);
    assert_eq!(stdout_json(&check)["command"], "check");

    let check_fix = run_kml([
        "check",
        "--fix",
        "--output",
        "json",
        "--config",
        file_text(&config).as_str(),
        file_text(&file).as_str(),
    ]);
    assert!(check_fix.status.success());
    assert_eq!(stdout_json(&check_fix)["files"][0]["changed"], true);
    let after_check_fix = read_file(&file);
    assert_ne!(after_check_fix, original);

    let fix = run_kml([
        "fix",
        "--output",
        "json",
        "--config",
        file_text(&config).as_str(),
        file_text(&file).as_str(),
    ]);
    assert!(fix.status.success());
    assert_eq!(stdout_json(&fix)["files"][0]["changed"], false);
    assert_eq!(read_file(&file), after_check_fix);

    let fmt = run_kml(["fmt", "--output", "json", file_text(&file).as_str()]);
    assert!(fmt.status.success());
    assert_eq!(stdout_json(&fmt)["files"][0]["changed"], true);
    let after_fmt = read_file(&file);

    let final_check = run_kml([
        "check",
        "--output",
        "json",
        "--config",
        file_text(&config).as_str(),
        file_text(&file).as_str(),
    ]);
    assert!(final_check.status.success());
    assert_eq!(stdout_json(&final_check)["summary"]["total_issues"], 0);

    let fmt_again = run_kml(["fmt", "--output", "json", file_text(&file).as_str()]);
    assert!(fmt_again.status.success());
    assert_eq!(stdout_json(&fmt_again)["files"][0]["changed"], false);
    assert_eq!(read_file(&file), after_fmt);
}

fn run_kml<const N: usize>(args: [&str; N]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kml"));
    command.args(args);
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().expect("kml should start");
    child
        .stdin
        .as_mut()
        .expect("stdin should be piped")
        .write_all(&[])
        .expect("stdin should be written");
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

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("fixture should be readable")
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
