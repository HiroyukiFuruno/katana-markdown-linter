use std::path::PathBuf;
use std::process::{Command, Output};

struct ReleaseVersionCommand {
    args: Vec<String>,
}

impl ReleaseVersionCommand {
    fn verify_version(input_version: String) -> Self {
        Self {
            args: vec![
                "scripts/release/verify-version.sh".to_string(),
                input_version,
            ],
        }
    }

    fn resolve_from_pull_request(head_ref: String) -> Self {
        Self {
            args: vec![
                "scripts/release/resolve-release-version.sh".to_string(),
                "pull_request".to_string(),
                String::new(),
                head_ref,
            ],
        }
    }

    fn run(&self) -> Output {
        let mut command = Command::new("bash");
        command.current_dir(workspace_root());
        command.env("MSYS2_ARG_CONV_EXCL", "*");
        command.env("MSYS_NO_PATHCONV", "1");
        for arg in &self.args {
            command.arg(arg);
        }
        command
            .output()
            .expect("release version command should execute")
    }
}

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

#[test]
fn release_version_rejects_suffix_input() {
    let output =
        ReleaseVersionCommand::verify_version(format!("v{}-123456789", env!("CARGO_PKG_VERSION")))
            .run();

    assert!(
        !output.status.success(),
        "expected suffix release version to fail"
    );
}

#[test]
fn release_pull_request_branch_must_not_include_suffix() {
    let output = ReleaseVersionCommand::resolve_from_pull_request(format!(
        "release/v{}-123456789",
        env!("CARGO_PKG_VERSION")
    ))
    .run();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected suffix release branch to fail"
    );
    assert!(
        stderr.contains("Release branch must be exactly release/vX.Y.Z"),
        "expected branch format error, stderr: {stderr}"
    );
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
