use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Verifies that config discovery walks up from the checked file's directory,
/// not from the working directory. This ensures path context resolution is
/// relative to the source file.
#[test]
fn cli_check_discovers_config_relative_to_the_checked_file() {
    let dir = TestDir::new("cli-path-context");
    let docs = dir.path().join("docs");
    fs::create_dir_all(&docs).expect("docs directory should be created");

    // Config in docs/ enables only MD018 (no space after #)
    let config = docs.join(".markdownlint.json");
    fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
        .expect("config should be written");

    // Source file in docs/ triggers MD018
    let source = docs.join("source.md");
    fs::write(&source, "#Heading\n").expect("source should be written");

    // Run from the workspace root (not from docs/), so config must be discovered
    // by walking up from source.md's location, not from cwd.
    let output = Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(["check", "--output", "json"])
        .arg(&source)
        .output()
        .expect("kml should run");

    assert_eq!(
        output.status.code(),
        Some(1),
        "should report MD018 violation discovered via path-relative config"
    );

    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be JSON");
    let diagnostics = report["files"][0]["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array");
    let rule_ids: Vec<&str> = diagnostics
        .iter()
        .map(|d| d["rule_id"].as_str().unwrap_or_default())
        .collect();

    assert!(
        rule_ids.contains(&"MD018"),
        "MD018 should fire for #Heading when config enables it; got: {rule_ids:?}"
    );
}

/// Verifies that a file checked without any config (no .markdownlint.json in
/// any ancestor) uses default rules and does not crash.
#[test]
fn cli_check_runs_without_config_in_ancestor_directories() {
    let dir = TestDir::new("cli-no-config");
    // Use an isolated temp dir outside the project tree so no ancestor config
    // is discovered.
    let source = dir.path().join("clean.md");
    fs::write(&source, "# Title\n\nContent.\n").expect("source should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(["check", "--output", "json"])
        .arg(&source)
        .output()
        .expect("kml should run");

    // A clean file should produce exit 0.
    assert_eq!(
        output.status.code(),
        Some(0),
        "clean file should produce no violations"
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
