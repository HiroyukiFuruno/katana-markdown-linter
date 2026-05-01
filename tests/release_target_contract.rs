use std::process::{Command, Output};

struct ReleaseTargetCommand {
    target_version: &'static str,
    latest_version: &'static str,
    override_enabled: bool,
}

impl ReleaseTargetCommand {
    fn new(target_version: &'static str, latest_version: &'static str) -> Self {
        Self {
            target_version,
            latest_version,
            override_enabled: false,
        }
    }

    fn with_override(mut self) -> Self {
        self.override_enabled = true;
        self
    }

    fn run(&self) -> Output {
        let mut command = Command::new("python3");
        command
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .arg("scripts/release/verify-release-target.py")
            .arg("--target-version")
            .arg(self.target_version)
            .arg("--latest-version")
            .arg(self.latest_version);
        if self.override_enabled {
            command.env("KML_RELEASE_ALLOW_VERSION_LINE_OVERRIDE", "1");
        }
        command
            .output()
            .expect("release target verifier should run")
    }
}

#[test]
fn patch_release_after_latest_patch_is_accepted() {
    let output = ReleaseTargetCommand::new("v0.17.7", "v0.17.6").run();

    assert!(
        output.status.success(),
        "expected consecutive patch release to pass, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn new_minor_release_must_start_at_zero() {
    let output = ReleaseTargetCommand::new("v0.18.7", "v0.17.6").run();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected skipped minor release to fail"
    );
    assert!(
        stderr.contains("a new minor line must start at v0.18.0"),
        "expected actionable minor-line failure, stderr: {stderr}"
    );
}

#[test]
fn new_minor_zero_is_accepted() {
    let output = ReleaseTargetCommand::new("v0.18.0", "v0.17.6").run();

    assert!(
        output.status.success(),
        "expected first minor release to pass, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn patch_skip_requires_override() {
    let output = ReleaseTargetCommand::new("v0.18.7", "v0.18.0").run();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected skipped patch release to fail"
    );
    assert!(
        stderr.contains("patch releases must be consecutive"),
        "expected patch sequence failure, stderr: {stderr}"
    );

    let override_output = ReleaseTargetCommand::new("v0.18.7", "v0.18.0")
        .with_override()
        .run();

    assert!(
        override_output.status.success(),
        "expected explicit override to pass, stderr: {}",
        String::from_utf8_lossy(&override_output.stderr)
    );
}
