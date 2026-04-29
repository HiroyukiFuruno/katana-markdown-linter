use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn document_answer_manifest_records_public_and_original_samples() {
    let root = workspace_root();
    let manifest_path = root.join("tests/fixtures/document-answer-fix/manifest.json");
    let manifest = read_json(&manifest_path);
    let samples = manifest["samples"]
        .as_array()
        .expect("manifest samples should be an array");
    let public_count = count_kind(samples, "public");
    let original_count = count_kind(samples, "original");

    assert!(
        public_count >= 200,
        "public corpus should contain at least 200 samples"
    );
    assert_eq!(
        original_count, 50,
        "original corpus should contain exactly 50 samples"
    );

    for sample in samples {
        assert_manifest_sample(&root, sample);
    }
}

#[test]
fn document_answer_runner_compares_fixed_output_with_answer_fixture() {
    let root = workspace_root();
    let dir = create_temp_dir("document-answer-fix-runner");
    let input = dir.join("sample.md");
    let answer = dir.join("sample_answer.md");
    let manifest = dir.join("manifest.json");
    let config = dir.join(".markdownlint.json");
    let report = dir.join("report.json");

    fs::write(&input, "#Title\n").expect("input fixture should be written");
    fs::write(&answer, "# Title\n").expect("answer fixture should be written");
    fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
        .expect("config should be written");
    fs::write(
        &manifest,
        format!(
            r#"{{
  "schema_version": 1,
  "license_allowlist": ["MIT"],
  "samples": [
    {{
      "id": "mini-public-md018",
      "kind": "public",
      "input_path": "{}",
      "answer_path": "{}",
      "source_repository": "HiroyukiFuruno/katana-markdown-linter",
      "source_commit": "d57b33878aae976677e361a0c1cae2e9f6463d4e",
      "source_path": "mini/sample.md",
      "license": "MIT",
      "retrieved_at": "2026-04-29",
      "selection_reason": "Minimal runner contract fixture.",
      "historical_patterns": ["MD018"],
      "answer_reviewed": true,
      "answer_review_note": "Expected heading spacing was reviewed manually."
    }}
  ]
}}
"#,
            input.display(),
            answer.display()
        ),
    )
    .expect("manifest should be written");

    let output = Command::new("python3")
        .arg(root.join("scripts/ci/document-answer-fix.py"))
        .arg("--manifest")
        .arg(&manifest)
        .arg("--config")
        .arg(&config)
        .arg("--report")
        .arg(&report)
        .arg("--minimum-public")
        .arg("1")
        .arg("--minimum-original")
        .arg("0")
        .arg("--")
        .arg(env!("CARGO_BIN_EXE_kml"))
        .output()
        .expect("document answer runner should start");

    assert!(
        output.status.success(),
        "runner failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence = read_json(&report);
    assert_eq!(evidence["summary"]["mismatches"], 0);
    assert_eq!(
        evidence["release_blocking_issues"]
            .as_array()
            .expect("release blockers should be an array")
            .len(),
        0
    );
    let _ = fs::remove_dir_all(dir);
}

fn assert_manifest_sample(root: &Path, sample: &Value) {
    let id = non_empty(sample, "id");
    let kind = non_empty(sample, "kind");
    let input = sample_path(root, sample, "input_path");
    let answer = sample_path(root, sample, "answer_path");
    let answer_name = answer
        .file_name()
        .and_then(|it| it.to_str())
        .expect("answer path should have a UTF-8 file name");

    assert!(input.is_file(), "{id}: input file is missing");
    assert!(answer.is_file(), "{id}: answer file is missing");
    assert!(
        answer_name.ends_with("_answer.md"),
        "{id}: answer fixture should use xxx_answer.md naming"
    );
    assert!(non_empty(sample, "source_repository").contains('/'));
    assert_eq!(non_empty(sample, "source_commit").len(), 40);
    assert!(non_empty(sample, "source_path").ends_with(".md"));
    assert_eq!(non_empty(sample, "license"), "MIT");
    assert!(!non_empty(sample, "retrieved_at").is_empty());
    assert!(!non_empty(sample, "selection_reason").is_empty());
    assert_eq!(
        sample["answer_reviewed"], true,
        "{id}: answer review missing"
    );
    assert!(!non_empty(sample, "answer_review_note").is_empty());

    if kind == "original" {
        let content = fs::read_to_string(input).expect("original input should be readable");
        assert!(
            content.chars().count() >= 200,
            "{id}: original sample should contain at least 200 characters"
        );
        let patterns = sample["historical_patterns"]
            .as_array()
            .expect("historical_patterns should be an array");
        assert!(
            patterns.len() >= 2,
            "{id}: original sample should combine at least two historical patterns"
        );
    }
}

fn count_kind(samples: &[Value], kind: &str) -> usize {
    samples
        .iter()
        .filter(|sample| sample["kind"].as_str() == Some(kind))
        .count()
}

fn non_empty<'a>(sample: &'a Value, key: &str) -> &'a str {
    let value = sample[key]
        .as_str()
        .unwrap_or_else(|| panic!("sample field `{key}` should be a string"));
    assert!(!value.trim().is_empty(), "sample field `{key}` is empty");
    value
}

fn sample_path(root: &Path, sample: &Value, key: &str) -> PathBuf {
    let path = PathBuf::from(non_empty(sample, key));
    if path.is_absolute() {
        path
    } else {
        root.join(path)
    }
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("JSON file should be readable"))
        .expect("JSON file should parse")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn create_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be available")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "katana-markdown-linter-{name}-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("test dir should be created");
    path
}
