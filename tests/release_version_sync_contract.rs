use std::path::PathBuf;
use std::process::Command;

#[test]
fn release_metadata_versions_match_cargo_version() {
    let output = Command::new("python3")
        .arg("scripts/release/verify-version-sync.py")
        .current_dir(workspace_root())
        .output()
        .expect("version sync verifier should execute");

    assert!(
        output.status.success(),
        "version sync verifier failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
