use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn standard_gate_checks_production_after_external_test_module() {
    let production = "#[cfg(test)]\nmod tests;\n".to_owned() + &"const _: () = ();\n".repeat(201);
    let report = failed_gate("external-test-module", &production);

    assert!(report.contains("src/lib.rs:1:1"), "{report}");
    assert!(
        report.contains("File exceeds 200-line limit (current: 201, excluding tests)"),
        "{report}"
    );
}

#[test]
fn standard_gate_checks_function_brace_boundaries() {
    let production = format!(
        "#[cfg(test)]\nmod tests;\nfn at_limit(\n    _arg: (),\n) {{\n{}}}\nfn over_limit() {{\n{}}}\n",
        "    let _ = ();\n".repeat(29),
        "    let _ = ();\n".repeat(30),
    );
    let report = failed_gate("function-boundary", &production);

    assert!(report.contains("src/lib.rs:36:4"), "{report}");
    assert!(
        report.contains("Function `over_limit` exceeds 30-line limit (current: 31)"),
        "{report}"
    );
    assert!(!report.contains("Function `at_limit`"), "{report}");
}

fn failed_gate(case: &str, source: &str) -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("read system clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("kml-ast-{}-{case}-{nonce}", std::process::id()));
    fs::create_dir(&root).expect("create isolated KAL fixture");
    let output = run_fixture(&root, source);
    fs::remove_dir_all(root).expect("remove isolated KAL fixture");
    let output = output.expect("run repository AST gate on fixture");
    assert!(!output.status.success(), "fixture must fail the AST gate");
    format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
    .replace('\\', "/")
}

fn run_fixture(root: &Path, source: &str) -> std::io::Result<Output> {
    let src = root.join("src");
    fs::create_dir(&src)?;
    fs::write(root.join("Cargo.toml"), "[workspace]\n")?;
    fs::copy(
        super::workspace_root().join("kal.json"),
        root.join("kal.json"),
    )?;
    fs::write(src.join("lib.rs"), source)?;
    fs::write(src.join("tests.rs"), "")?;

    // WHY: 子プロセスで同じ検査を起動し、並列テストの作業ディレクトリを変えない。
    Command::new(std::env::current_exe()?)
        .args(["--exact", "ast_linter_repository_standard_rules_are_clean"])
        .env_remove("KAL_CONFIG_PATH")
        .current_dir(root)
        .output()
}
