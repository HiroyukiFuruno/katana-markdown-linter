use katana_markdown_linter::{LintOptions, MarkdownLintConfig, MarkdownLinter};
use pulldown_cmark::{Event, Parser};
use serde_json::json;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const REFERENCE_SOURCE: &str = "[***a*** ***b***]\n\n[_**a**_ _**b**_]: /new\n";
const REFERENCE_EXPECTED: &str = "[_**a**_ ***b***]\n\n[_**a**_ _**b**_]: /new\n";

fn meaning(source: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for event in Parser::new(source) {
        match event {
            Event::Text(text) => match result.last_mut() {
                Some(previous) if previous.starts_with("TEXT:") => previous.push_str(&text),
                _ => result.push(format!("TEXT:{text}")),
            },
            other => result.push(format!("{other:?}")),
        }
    }
    result
}

fn options(raw: serde_json::Value) -> LintOptions {
    MarkdownLintConfig { raw }.to_lint_options()
}

fn md049() -> LintOptions {
    options(json!({"default": false, "MD049": {"enabled": true, "style": "underscore"}}))
}

#[test]
fn reference_label_semantics_survive_batch_fix_planning() {
    let options = md049();
    let fixed = MarkdownLinter::fix(REFERENCE_SOURCE, &options).expect("fix should converge");
    assert_eq!(fixed.content, REFERENCE_EXPECTED);
    assert_eq!(meaning(&fixed.content), meaning(REFERENCE_SOURCE));

    let results = MarkdownLinter::lint(REFERENCE_SOURCE, &options).expect("lint should run");
    let one_pass = MarkdownLinter::fix_with_results(REFERENCE_SOURCE, &results);
    assert_eq!(one_pass.content, REFERENCE_EXPECTED);
    assert_eq!(one_pass.applied_fixes, 1);
    assert_eq!(meaning(&one_pass.content), meaning(REFERENCE_SOURCE));

    let mut current = REFERENCE_SOURCE.to_owned();
    for _ in 0..3 {
        current = MarkdownLinter::fix(&current, &options)
            .expect("fix should converge over three passes")
            .content;
    }
    assert_eq!(current, REFERENCE_EXPECTED);
}

#[test]
fn mixed_rule_batch_preserves_reference_and_inline_code_semantics() {
    let source = "[*a* **b**]\n\n[_a_ __b__]: /new\n";
    let expected = "[_a_ **b**]\n\n[_a_ __b__]: /new\n";
    let opts = options(json!({
        "default": false,
        "MD049": {"enabled": true, "style": "underscore"},
        "MD050": {"enabled": true, "style": "underscore"},
        "MD038": true
    }));
    let fixed = MarkdownLinter::fix(source, &opts).expect("mixed fix should converge");
    assert_eq!(fixed.content, expected);
    assert_eq!(meaning(&fixed.content), meaning(source));
    let results = MarkdownLinter::lint(source, &opts).expect("mixed lint should run");
    let one_pass = MarkdownLinter::fix_with_results(source, &results);
    assert_eq!(one_pass.content, expected);
    assert_eq!(one_pass.applied_fixes, 1);
    assert_eq!(meaning(&one_pass.content), meaning(source));

    let mixed_code = "[*a* ` b `]\n\n[_a_ `b`]: /new\n";
    let observed = MarkdownLinter::fix(mixed_code, &opts).expect("mixed code fix should converge");
    assert_eq!(observed.content, "[*a* `b`]\n\n[_a_ `b`]: /new\n");
    assert_eq!(meaning(&observed.content), meaning(mixed_code));
}

#[test]
fn one_hundred_twenty_eight_independent_edits_apply_without_pass_exhaustion() {
    let source = (0..128)
        .map(|i| format!("***w{i}***"))
        .collect::<Vec<_>>()
        .join(" ")
        + "\n";
    let expected = (0..128)
        .map(|i| format!("_**w{i}**_"))
        .collect::<Vec<_>>()
        .join(" ")
        + "\n";
    let fixed = MarkdownLinter::fix(&source, &md049())
        .expect("all edits should converge within normal limit");
    assert_eq!(fixed.content, expected);
    assert_eq!(meaning(&fixed.content), meaning(&source));
    let results = MarkdownLinter::lint(&source, &md049()).expect("lint should run");
    let one_pass = MarkdownLinter::fix_with_results(&source, &results);
    assert_eq!(one_pass.content, expected);
    assert_eq!(one_pass.applied_fixes, 128);
}

#[test]
fn cli_stdin_keeps_reference_meaning_and_converges() {
    let dir = std::env::temp_dir().join(format!(
        "kml-batch-contract-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).expect("temporary directory should be created");
    let config = dir.join(".markdownlint.json");
    fs::write(
        &config,
        r#"{"default":false,"MD049":{"enabled":true,"style":"underscore"}}"#,
    )
    .expect("config should be written");
    let mut current = REFERENCE_SOURCE.to_owned();
    for _ in 0..3 {
        let mut child = Command::new(env!("CARGO_BIN_EXE_kml"))
            .args(["fix", "--stdin", "--config", config.to_str().unwrap()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("kml should start");
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(current.as_bytes())
            .expect("stdin should be written");
        let output = child.wait_with_output().expect("kml should finish");
        assert_eq!(
            output.status.code(),
            Some(1),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        current = String::from_utf8(output.stdout).expect("fixed output should be UTF-8");
    }
    assert_eq!(current, REFERENCE_EXPECTED);
    assert_eq!(meaning(&current), meaning(REFERENCE_SOURCE));
    fs::remove_dir_all(dir).expect("temporary directory should be removed");
}

#[test]
fn cli_file_keeps_reference_meaning_and_stays_byte_stable_for_three_runs() {
    let dir = std::env::temp_dir().join(format!(
        "kml-batch-file-contract-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).expect("temporary directory should be created");
    let config = dir.join(".markdownlint.json");
    let file = dir.join("case.md");
    fs::write(
        &config,
        r#"{"default":false,"MD049":{"enabled":true,"style":"underscore"}}"#,
    )
    .expect("config should be written");
    fs::write(&file, REFERENCE_SOURCE).expect("fixture should be written");

    for _ in 0..3 {
        let output = Command::new(env!("CARGO_BIN_EXE_kml"))
            .args([
                "fix",
                "--output",
                "json",
                "--config",
                config.to_str().unwrap(),
                file.to_str().unwrap(),
            ])
            .output()
            .expect("kml should start");
        assert_eq!(
            output.status.code(),
            Some(1),
            "stdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            fs::read_to_string(&file).expect("fixed file should be readable"),
            REFERENCE_EXPECTED
        );
    }
    assert_eq!(
        meaning(&fs::read_to_string(&file).unwrap()),
        meaning(REFERENCE_SOURCE)
    );
    fs::remove_dir_all(dir).expect("temporary directory should be removed");
}

#[test]
fn existing_syntax_fixes_remain_available_in_default_mode() {
    let fixed =
        MarkdownLinter::fix("#Title\n\n", &LintOptions::default()).expect("fix should converge");
    assert_eq!(fixed.content, "# Title\n");
    assert!(fixed
        .details
        .iter()
        .any(|detail| detail.rule_id == "MD018" && detail.applied));
}
