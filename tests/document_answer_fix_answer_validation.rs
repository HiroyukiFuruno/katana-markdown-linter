use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn document_answer_runner_reports_answer_validation_summary() {
    let case = DocumentAnswerCase::new("answer-validation-clean");
    case.write_sample("#Title\n", "# Title\n");
    case.write_config("{ \"default\": false, \"MD018\": true }\n");
    case.write_manifest("MD018");

    let output = case.run();

    assert!(
        output.status.success(),
        "runner failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report = case.report();
    assert_eq!(report["summary"]["mismatches"], 0);
    assert_eq!(report["summary"]["answer_diagnostics"], 0);
    assert_eq!(report["summary"]["answer_fix_changes"], 0);
}

#[test]
fn document_answer_runner_reports_stable_remaining_diagnostics() {
    let case = DocumentAnswerCase::new("answer-validation-diagnostic");
    let line = "This answer fixture keeps an overlong line that cannot be repaired by the fixer, so it must be rejected by answer validation.\n";
    case.write_sample(line, line);
    case.write_config(r#"{ "default": false, "MD013": true }"#);
    case.write_manifest("MD013");

    let output = case.run();

    assert!(
        output.status.success(),
        "runner should accept stable non-fixable diagnostics\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report = case.report();
    assert_eq!(report["summary"]["mismatches"], 0);
    assert!(
        report["summary"]["answer_diagnostics"]
            .as_u64()
            .unwrap_or(0)
            > 0,
        "answer diagnostics should be reported"
    );
    assert_eq!(report["summary"]["answer_fix_changes"], 0);
    assert_eq!(
        report["release_blocking_issues"]
            .as_array()
            .expect("release blockers should be an array")
            .len(),
        0
    );
}

#[test]
fn document_answer_runner_blocks_answers_changed_by_second_fix() {
    let case = DocumentAnswerCase::new("answer-validation-fix-change");
    case.write_sample("# Title\n", "#Title\n");
    case.write_config("{ \"default\": false, \"MD018\": true }\n");
    case.write_manifest("MD018");

    let output = case.run();

    assert!(
        !output.status.success(),
        "runner should reject answer fixtures that change when fixed again"
    );
    let report = case.report();
    assert!(
        report["summary"]["answer_fix_changes"]
            .as_u64()
            .unwrap_or(0)
            > 0,
        "answer fix changes should be reported"
    );
    assert!(
        report["release_blocking_issues"]
            .as_array()
            .expect("release blockers should be an array")
            .iter()
            .any(|issue| issue
                .as_str()
                .unwrap_or("")
                .contains("changes when fixed again")),
        "second-pass answer fix changes should block release"
    );
}

struct DocumentAnswerCase {
    root: PathBuf,
    dir: PathBuf,
}

impl DocumentAnswerCase {
    fn new(name: &str) -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dir = create_temp_dir(name);
        Self { root, dir }
    }

    fn write_sample(&self, input: &str, answer: &str) {
        fs::write(self.input(), input).expect("input fixture should be written");
        fs::write(self.answer(), answer).expect("answer fixture should be written");
    }

    fn write_config(&self, content: &str) {
        fs::write(self.config(), content).expect("config should be written");
    }

    fn write_manifest(&self, rule_id: &str) {
        write_json(
            &self.manifest(),
            &json!({
                "schema_version": 1,
                "license_allowlist": ["MIT"],
                "samples": [
                    {
                        "id": "mini-public-answer-validation",
                        "kind": "public",
                        "input_path": self.input().display().to_string(),
                        "answer_path": self.answer().display().to_string(),
                        "source_repository": "HiroyukiFuruno/katana-markdown-linter",
                        "source_commit": "d57b33878aae976677e361a0c1cae2e9f6463d4e",
                        "source_path": "mini/sample.md",
                        "license": "MIT",
                        "retrieved_at": "2026-04-29",
                        "selection_reason": "Minimal answer validation fixture.",
                        "historical_patterns": [rule_id],
                        "answer_reviewed": true,
                        "answer_review_note": "Expected Markdown output was reviewed independently."
                    }
                ]
            }),
        );
    }

    fn run(&self) -> std::process::Output {
        Command::new("python3")
            .arg(self.root.join("scripts/ci/document-answer-fix.py"))
            .arg("--manifest")
            .arg(self.manifest())
            .arg("--config")
            .arg(self.config())
            .arg("--report")
            .arg(self.report_path())
            .arg("--minimum-public")
            .arg("1")
            .arg("--minimum-original")
            .arg("0")
            .arg("--")
            .arg(env!("CARGO_BIN_EXE_kml"))
            .output()
            .expect("document answer runner should start")
    }

    fn report(&self) -> Value {
        read_json(&self.report_path())
    }

    fn input(&self) -> PathBuf {
        self.dir.join("sample.md")
    }

    fn answer(&self) -> PathBuf {
        self.dir.join("sample_answer.md")
    }

    fn config(&self) -> PathBuf {
        self.dir.join(".markdownlint.json")
    }

    fn manifest(&self) -> PathBuf {
        self.dir.join("manifest.json")
    }

    fn report_path(&self) -> PathBuf {
        self.dir.join("report.json")
    }
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("JSON file should be readable"))
        .expect("JSON file should parse")
}

fn write_json(path: &Path, value: &Value) {
    fs::write(
        path,
        serde_json::to_string_pretty(value).expect("JSON should serialize"),
    )
    .expect("JSON file should be written");
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
