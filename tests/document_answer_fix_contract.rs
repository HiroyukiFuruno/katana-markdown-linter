use serde_json::{json, Value};
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
    assert!(
        original_count >= 70,
        "original corpus should retain 58 existing samples and add at least 12 independent samples"
    );

    for sample in samples {
        assert_manifest_sample(&root, sample);
    }
    let mutations = samples
        .iter()
        .filter(|sample| sample.get("mutation").and_then(Value::as_str).is_some())
        .collect::<Vec<_>>();
    assert_eq!(
        mutations.len(),
        4,
        "four intentional mutation fixtures are required"
    );
    for sample in mutations {
        let id = non_empty(sample, "id");
        let input = fs::read(sample_path(&root, sample, "input_path"))
            .expect("mutated input should be readable");
        let answer = fs::read(sample_path(&root, sample, "answer_path"))
            .expect("mutated answer should be readable");
        let input_text = String::from_utf8(input.clone()).expect("mutated input should be UTF-8");
        let answer_text =
            String::from_utf8(answer.clone()).expect("mutated answer should be UTF-8");
        assert_ne!(
            input, answer,
            "{id}: mutation must have an independent answer"
        );
        assert!(
            input_text.chars().count() >= 200,
            "{id}: mutated input should contain at least 200 characters"
        );
        let changed_lines = input_text
            .lines()
            .zip(answer_text.lines())
            .filter(|(before, after)| before != after)
            .count()
            + input_text
                .lines()
                .count()
                .abs_diff(answer_text.lines().count());
        assert!(
            (1..=3).contains(&changed_lines),
            "{id}: expected one to three independently authored line changes, saw {changed_lines}"
        );
        assert!(!non_empty(sample, "mutation").is_empty());
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
    write_json(
        &manifest,
        &json!({
            "schema_version": 1,
            "license_allowlist": ["MIT"],
            "samples": [
                {
                    "id": "mini-public-md018",
                    "kind": "public",
                    "input_path": input.display().to_string(),
                    "answer_path": answer.display().to_string(),
                    "source_repository": "HiroyukiFuruno/katana-markdown-linter",
                    "source_commit": "d57b33878aae976677e361a0c1cae2e9f6463d4e",
                    "source_path": "mini/sample.md",
                    "license": "MIT",
                    "retrieved_at": "2026-04-29",
                    "selection_reason": "Minimal runner contract fixture.",
                    "historical_patterns": ["MD018"],
                    "answer_reviewed": true,
                    "answer_review_note": "Expected heading spacing was reviewed manually."
                }
            ]
        }),
    );

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
    assert_eq!(evidence["summary"]["fix_pass_exit_codes"], json!([0, 0, 0]));
    assert_eq!(evidence["summary"]["safe_fixable_remaining"], 0);
    assert_eq!(evidence["summary"]["answer_safe_fixable_remaining"], 0);
    assert_eq!(
        evidence["release_blocking_issues"]
            .as_array()
            .expect("release blockers should be an array")
            .len(),
        0
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn duplicate_manifest_ids_fail_in_metadata_validation() {
    let root = workspace_root();
    let dir = create_temp_dir("document-answer-duplicate-id");
    let input_a = dir.join("a.md");
    let input_b = dir.join("b.md");
    let answer_a = dir.join("a_answer.md");
    let answer_b = dir.join("b_answer.md");
    let manifest = dir.join("manifest.json");
    let config = dir.join(".markdownlint.json");
    let report = dir.join("report.json");

    fs::write(&input_a, "# A\n").expect("first input should be written");
    fs::write(&input_b, "# B\n").expect("second input should be written");
    fs::write(&answer_a, "# A\n").expect("first answer should be written");
    fs::write(&answer_b, "# B\n").expect("second answer should be written");
    fs::write(&config, "{\"default\":false}\n").expect("config should be written");
    let mut first = mini_sample("duplicate-sample", &input_a, &answer_a);
    let mut second = mini_sample("duplicate-sample", &input_b, &answer_b);
    first["source_path"] = json!("mini/a.md");
    second["source_path"] = json!("mini/b.md");
    write_json(
        &manifest,
        &json!({
            "schema_version": 1,
            "license_allowlist": ["MIT"],
            "samples": [first, second]
        }),
    );

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
        !output.status.success(),
        "duplicate IDs must fail validation"
    );
    let evidence = read_json(&report);
    assert_eq!(evidence["summary"]["metadata_errors"], 1);
    assert!(evidence["summary"]["fix_pass_exit_codes"].is_null());
    assert!(evidence["samples"].is_null());
    assert!(
        evidence["release_blocking_issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue.as_str().unwrap().contains("duplicate sample id")),
        "metadata report should explain duplicate identity"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn casefold_colliding_ids_use_distinct_execution_paths_and_real_cli_samples() {
    let root = workspace_root();
    let dir = create_temp_dir("document-answer-casefold-id");
    let input_a = dir.join("a.md");
    let input_b = dir.join("b.md");
    let answer_a = dir.join("a_answer.md");
    let answer_b = dir.join("b_answer.md");
    let manifest = dir.join("manifest.json");
    let config = dir.join(".markdownlint.json");
    let report = dir.join("report.json");

    fs::write(&input_a, "#Title\n").expect("first input should be written");
    fs::write(&input_b, "#SecondTitle\n").expect("second input should be written");
    fs::write(&answer_a, "# Title\n").expect("first answer should be written");
    fs::write(&answer_b, "# SecondTitle\n").expect("second answer should be written");
    fs::write(&config, "{\"default\":false,\"MD018\":true}\n").expect("config should be written");
    let mut first = mini_sample("Case-ID", &input_a, &answer_a);
    let mut second = mini_sample("case-id", &input_b, &answer_b);
    first["source_path"] = json!("mini/Case-ID.md");
    second["source_path"] = json!("mini/case-id.md");
    write_json(
        &manifest,
        &json!({
            "schema_version": 1,
            "license_allowlist": ["MIT"],
            "samples": [first, second]
        }),
    );

    let allocator = Command::new("python3")
        .arg("-c")
        .arg("import pathlib,sys; sys.path.insert(0,sys.argv[1]); from document_answer_manifest import Manifest,execution_path; m=Manifest(pathlib.Path(sys.argv[2])); a,b=m.samples; root=pathlib.Path(sys.argv[3]); assert execution_path(a,root) != execution_path(b,root); assert execution_path(a,root).name == 'sample-000001.md'; assert execution_path(b,root).name == 'sample-000002.md'; assert execution_path(a,root,answer=True).name == 'sample-000001_answer.md'; assert execution_path(b,root,answer=True).name == 'sample-000002_answer.md'")
        .arg(root.join("scripts/ci"))
        .arg(&manifest)
        .arg(&dir)
        .output()
        .expect("execution path allocator probe should start");
    assert!(
        allocator.status.success(),
        "casefold IDs should map to separate stable physical names\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&allocator.stdout),
        String::from_utf8_lossy(&allocator.stderr)
    );

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
        "runner should process both casefold IDs independently\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence = read_json(&report);
    assert_eq!(evidence["summary"]["mismatches"], 0);
    assert_eq!(evidence["summary"]["fix_pass_exit_codes"], json!([0, 0, 0]));
    let samples = evidence["samples"]
        .as_array()
        .expect("sample reports should exist");
    assert_eq!(samples.len(), 2);
    assert_eq!(samples[0]["id"], "Case-ID");
    assert_eq!(samples[0]["matched"], true);
    assert_eq!(samples[1]["id"], "case-id");
    assert_eq!(samples[1]["matched"], true);
    assert_eq!(evidence["summary"]["answer_fix_changes"], 0);
    assert!(evidence["release_blocking_issues"]
        .as_array()
        .unwrap()
        .is_empty());
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn unsafe_ids_and_ambiguous_paths_fail_in_metadata_validation() {
    let root = workspace_root();
    let dir = create_temp_dir("document-answer-invalid-mapping");
    let input_a = dir.join("a.md");
    let input_b = dir.join("b.md");
    let answer_a = dir.join("a_answer.md");
    let answer_b = dir.join("b_answer.md");
    let config = dir.join(".markdownlint.json");
    fs::write(&input_a, "# A\n").expect("first input should be written");
    fs::write(&input_b, "# B\n").expect("second input should be written");
    fs::write(&answer_a, "# A\n").expect("first answer should be written");
    fs::write(&answer_b, "# B\n").expect("second answer should be written");
    fs::write(&config, "{\"default\":false}\n").expect("config should be written");

    let invalid_cases = [
        (
            "unsafe-id",
            vec![mini_sample("../escape", &input_a, &answer_a)],
            "safe filename component",
        ),
        (
            "duplicate-input",
            vec![
                mini_sample("sample-a", &input_a, &answer_a),
                mini_sample("sample-b", &input_a, &answer_b),
            ],
            "input path is also used",
        ),
        (
            "cross-role-path",
            vec![
                mini_sample("sample-a", &input_a, &input_b),
                mini_sample("sample-b", &input_b, &answer_b),
            ],
            "input path is also an answer path",
        ),
    ];

    for (case_id, samples, expected_error) in invalid_cases {
        let manifest = dir.join(format!("{case_id}.json"));
        let report = dir.join(format!("{case_id}-report.json"));
        write_json(
            &manifest,
            &json!({
                "schema_version": 1,
                "license_allowlist": ["MIT"],
                "samples": samples
            }),
        );
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
        assert!(!output.status.success(), "{case_id} must fail validation");
        let evidence = read_json(&report);
        assert!(evidence["summary"]["metadata_errors"].as_u64().unwrap() > 0);
        assert!(evidence["summary"]["fix_pass_exit_codes"].is_null());
        assert!(
            evidence["release_blocking_issues"]
                .as_array()
                .unwrap()
                .iter()
                .any(|issue| issue.as_str().unwrap().contains(expected_error)),
            "{case_id} report should explain invalid identity mapping"
        );
    }
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn document_answer_fix_pass_evaluator_catches_non_idempotent_bytes() {
    let root = workspace_root();
    let output = Command::new("python3")
        .arg("-c")
        .arg(
            "import sys; sys.path.insert(0, sys.argv[1]); from document_answer_fix_runner import evaluate_fix_passes; expected=b'answer\\n'; issues=evaluate_fix_passes(expected, [expected, b'answer \\n', expected]); assert any('fix pass 2 changed output' in issue for issue in issues), issues",
        )
        .arg(root.join("scripts/ci"))
        .output()
        .expect("fix pass evaluator should start");

    assert!(
        output.status.success(),
        "evaluator should reject a binary change between fix passes\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn original_064_nested_style_answer_preserves_commonmark_events() {
    let root = workspace_root().join("tests/fixtures/document-answer-fix/original");
    let input = fs::read_to_string(root.join("original-064.md"))
        .expect("original-064 input should be readable");
    let answer = fs::read_to_string(root.join("original-064_answer.md"))
        .expect("original-064 answer should be readable");

    assert_ne!(input, answer, "nested emphasis style should be normalized");
    assert_eq!(commonmark_events(&input), commonmark_events(&answer));
}

fn commonmark_events(source: &str) -> Vec<String> {
    pulldown_cmark::Parser::new(source)
        .map(|event| format!("{event:?}"))
        .collect()
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

fn write_json(path: &Path, value: &Value) {
    fs::write(
        path,
        serde_json::to_string_pretty(value).expect("JSON should serialize"),
    )
    .expect("JSON file should be written");
}

fn mini_sample(id: &str, input: &Path, answer: &Path) -> Value {
    json!({
        "id": id,
        "kind": "public",
        "input_path": input.display().to_string(),
        "answer_path": answer.display().to_string(),
        "source_repository": "HiroyukiFuruno/katana-markdown-linter",
        "source_commit": "d57b33878aae976677e361a0c1cae2e9f6463d4e",
        "source_path": format!("mini/{id}.md"),
        "license": "MIT",
        "retrieved_at": "2026-04-29",
        "selection_reason": "Minimal manifest identity contract fixture.",
        "historical_patterns": ["identity"],
        "answer_reviewed": true,
        "answer_review_note": "Minimal contract fixture."
    })
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
