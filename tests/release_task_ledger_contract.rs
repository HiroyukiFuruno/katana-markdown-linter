use std::fs;
use std::process::{Command, Output};

fn verify(name: &str, tasks: &str, score: usize) -> Output {
    let path = std::env::temp_dir().join(format!("kml-ledger-{}-{name}.md", std::process::id()));
    let content =
        format!("{tasks}\n| 項目 | 最大 | 現在 |\n| --- | --- | --- |\n| 合計 | 100 | {score} |\n");
    fs::write(&path, content).expect("task ledger should be written");
    let result = Command::new("python3")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/scripts/release/verify-task-ledger.py"
        ))
        .args(["--version", "v0.20.0", "--change"])
        .arg(&path)
        .output()
        .expect("real ledger verifier should run");
    fs::remove_file(path).expect("temporary ledger should be removed");
    result
}

#[test]
fn completed_feedback_uses_its_own_done_marker() {
    let result = verify(
        "feedback",
        "- [x] completed work\n- [/] addressed feedback",
        100,
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn pending_tasks_still_block_the_release() {
    let result = verify(
        "pending",
        "- [ ] pending work\n- [/] addressed feedback",
        100,
    );
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("pending work"));
}

#[test]
fn incomplete_quality_score_still_blocks() {
    let result = verify("score", "- [x] completed work", 90);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("score is 90/100"));
}
