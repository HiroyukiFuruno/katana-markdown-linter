use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn cli_check_resolves_local_links_relative_to_the_checked_file() {
    let dir = TestDir::new("cli-path-context");
    let docs = dir.path().join("docs");
    fs::create_dir_all(&docs).expect("docs directory should be created");
    fs::write(docs.join("target.md"), "# Target\n").expect("target should be written");
    let config = docs.join(".markdownlint.json");
    fs::write(&config, "{ \"default\": false }\n").expect("config should be written");
    let source = docs.join("source.md");
    fs::write(
        &source,
        concat!(
            "[Target](target.md)\n",
            "[Email](mailto:test@example.com)\n",
            "[Missing](missing.md)\n",
        ),
    )
    .expect("source should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(["check", "--output", "json", "--config"])
        .arg(&config)
        .arg(&source)
        .output()
        .expect("kml should run");

    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).expect("stdout should be JSON");
    let diagnostics = report["files"][0]["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array");
    let messages = diagnostics
        .iter()
        .map(|diagnostic| diagnostic["message"].as_str().unwrap_or_default())
        .collect::<Vec<_>>();

    assert_eq!(messages, ["Broken local link: missing.md"]);
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
