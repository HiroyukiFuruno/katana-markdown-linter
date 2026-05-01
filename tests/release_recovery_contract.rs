use std::process::{Command, Output};

struct RecoveryPlanCommand {
    bad_version: &'static str,
    latest_version: &'static str,
    replacement_version: Option<&'static str>,
}

impl RecoveryPlanCommand {
    fn new(bad_version: &'static str, latest_version: &'static str) -> Self {
        Self {
            bad_version,
            latest_version,
            replacement_version: None,
        }
    }

    fn with_replacement(mut self, replacement_version: &'static str) -> Self {
        self.replacement_version = Some(replacement_version);
        self
    }

    fn run(&self) -> Output {
        let mut command = Command::new("python3");
        command
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .arg("scripts/release/recover-accidental-release.py")
            .arg("--bad-version")
            .arg(self.bad_version)
            .arg("--latest-version")
            .arg(self.latest_version)
            .arg("--output")
            .arg("json");
        if let Some(replacement_version) = self.replacement_version {
            command
                .arg("--replacement-version")
                .arg(replacement_version);
        }
        command.output().expect("recovery planner should run")
    }
}

#[test]
fn accidental_release_plan_defaults_to_latest_prior_version() {
    let output = RecoveryPlanCommand::new("v0.18.7", "v0.17.6").run();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        output.status.success(),
        "expected recovery plan to succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("\"bad_version\": \"v0.18.7\""));
    assert!(stdout.contains("\"replacement_version\": \"v0.17.6\""));
    assert!(stdout.contains("\"cargo\""));
    assert!(stdout.contains("\"yank\""));
    assert!(stdout.contains("\"deprecate\""));
    assert!(stdout.contains("\"dist-tag\""));
    assert!(stdout.contains("\"add\""));
    assert!(stdout.contains("pypi.org/manage/project/katana-markdown-linter/releases/"));
    assert!(stdout.contains("Formula/kml@0.18.7.rb"));
}

#[test]
fn recovery_plan_rejects_non_suspicious_release_target() {
    let output = RecoveryPlanCommand::new("v0.17.7", "v0.17.6").run();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected non-suspicious release target to be rejected"
    );
    assert!(
        stderr.contains("is not flagged as a suspicious release target"),
        "expected refusal message, stderr: {stderr}"
    );
}

#[test]
fn recovery_plan_accepts_explicit_lower_replacement() {
    let output = RecoveryPlanCommand::new("v0.18.7", "v0.17.6")
        .with_replacement("v0.18.0")
        .run();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        output.status.success(),
        "expected lower replacement to pass, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("\"replacement_version\": \"v0.18.0\""));
}
