use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn public_confidence_fixture_records_convergent_check_fix_and_fmt_evidence() {
    let dir = TestDir::new("public-confidence");
    let report = dir.path().join("public-confidence-report.json");
    let script = workspace_root().join("scripts/ci/public-confidence.py");
    let corpus = workspace_root().join("tests/fixtures/public-confidence/corpus");
    let config = workspace_root().join("tests/fixtures/public-confidence/.markdownlint.json");

    let output = Command::new("python3")
        .arg(script)
        .arg("--report")
        .arg(&report)
        .arg("--corpus")
        .arg(&corpus)
        .arg("--config")
        .arg(&config)
        .arg("--")
        .arg(env!("CARGO_BIN_EXE_kml"))
        .output()
        .expect("public confidence runner should start");

    assert!(
        output.status.success(),
        "runner failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let evidence: Value = serde_json::from_str(
        &fs::read_to_string(report).expect("public confidence report should exist"),
    )
    .expect("public confidence report should be JSON");

    assert_eq!(evidence["schema_version"], 1);
    assert_eq!(evidence["source"]["mode"], "curated");
    assert_eq!(evidence["check"]["source_unchanged"], true);
    assert_eq!(evidence["fix"]["converged"], true);
    assert_eq!(evidence["fmt"]["converged"], true);
    assert_eq!(
        evidence["release_blocking_issues"]
            .as_array()
            .expect("release blockers should be an array")
            .len(),
        0
    );
    let check = &evidence["check"];
    assert!(
        check
            .get("unclassified_count")
            .and_then(Value::as_u64)
            .expect("unclassified_count should be numeric")
            == 0,
        "classification must not contain unclassified findings"
    );
    let summary = check
        .get("classification_summary")
        .expect("check should include classification_summary");
    for key in [
        "true-positive",
        "false-positive",
        "false-negative",
        "unsafe-fix-risk",
        "fmt-policy-gap",
        "unclassified",
    ] {
        assert!(
            summary.get(key).is_some(),
            "missing classification bucket: {key}"
        );
    }
    assert_required_syntax(&evidence);
}

fn assert_required_syntax(evidence: &Value) {
    let syntax = &evidence["inventory"]["syntax"];
    for field in [
        "links",
        "images",
        "inline_html",
        "fenced_code",
        "tables",
        "reference_definitions",
        "mixed_japanese_english",
    ] {
        assert!(
            syntax[field]
                .as_u64()
                .expect("syntax count should be numeric")
                > 0,
            "missing required syntax evidence for {field}"
        );
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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
