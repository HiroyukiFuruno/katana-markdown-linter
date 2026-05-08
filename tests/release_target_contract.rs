use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

struct ReleaseTargetCommand {
    target_version: &'static str,
    latest_version: Option<&'static str>,
    github_releases_json: Option<PathBuf>,
    override_enabled: bool,
}

impl ReleaseTargetCommand {
    fn new(target_version: &'static str, latest_version: &'static str) -> Self {
        Self {
            target_version,
            latest_version: Some(latest_version),
            github_releases_json: None,
            override_enabled: false,
        }
    }

    fn with_github_releases_json(target_version: &'static str, path: PathBuf) -> Self {
        Self {
            target_version,
            latest_version: None,
            github_releases_json: Some(path),
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
            .arg(self.target_version);
        if let Some(latest_version) = self.latest_version {
            command.arg("--latest-version").arg(latest_version);
        }
        if let Some(path) = &self.github_releases_json {
            command.arg("--github-releases-json").arg(path);
        }
        if self.override_enabled {
            command.env("KML_RELEASE_ALLOW_VERSION_LINE_OVERRIDE", "1");
        }
        command
            .output()
            .expect("release target verifier should run")
    }
}

fn write_github_releases_fixture(name: &str, content: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "kml-release-target-{name}-{}.json",
        std::process::id()
    ));
    fs::write(&path, content).expect("release fixture should be writable");
    path
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
fn accidental_prerelease_is_ignored_when_resolving_latest_stable_release() {
    let fixture = write_github_releases_fixture(
        "accidental-prerelease",
        r#"[
            {"tag_name": "v0.18.7", "draft": false, "prerelease": true},
            {"tag_name": "v0.17.6", "draft": false, "prerelease": false}
        ]"#,
    );
    let output = ReleaseTargetCommand::with_github_releases_json("v0.17.7", fixture).run();

    assert!(
        output.status.success(),
        "expected accidental prerelease to be ignored, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn latest_stable_released_after_accidental_release_is_accepted() {
    let fixture = write_github_releases_fixture(
        "stable-plus-accidental",
        r#"[
            {"tag_name": "v0.18.1", "draft": false, "prerelease": false},
            {"tag_name": "v0.18.7", "draft": false, "prerelease": true}
        ]"#,
    );
    let output = ReleaseTargetCommand::with_github_releases_json("v0.18.2", fixture).run();

    assert!(
        output.status.success(),
        "expected latest stable release v0.18.1 and accidental pre-release v0.18.7 to pass, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn suffix_release_targets_are_rejected() {
    let output = ReleaseTargetCommand::new("v0.18.2-123456789", "v0.18.1").run();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected suffix target release to fail"
    );
    assert!(
        stderr.contains("expected a stable version like v1.2.3"),
        "expected stable version failure, stderr: {stderr}"
    );
}

#[test]
fn suffix_release_tags_are_ignored_when_resolving_latest_stable_release() {
    let fixture = write_github_releases_fixture(
        "suffix-tag",
        r#"[
            {"tag_name": "v0.18.1-123456789", "draft": false, "prerelease": false},
            {"tag_name": "v0.18.0", "draft": false, "prerelease": false}
        ]"#,
    );
    let output = ReleaseTargetCommand::with_github_releases_json("v0.18.1", fixture).run();

    assert!(
        output.status.success(),
        "expected suffix tag to be ignored, stderr: {}",
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
