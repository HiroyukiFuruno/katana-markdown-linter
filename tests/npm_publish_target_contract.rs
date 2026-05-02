#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_TEST_DIR_ID: AtomicU64 = AtomicU64::new(0);

struct NpmPublishTargetCommand {
    mock_npm: MockNpm,
}

impl NpmPublishTargetCommand {
    fn new(stdout: &str, stderr: &str, status: i32) -> Self {
        Self {
            mock_npm: MockNpm::new(stdout, stderr, status),
        }
    }

    fn run(&self) -> Output {
        let original_path = std::env::var("PATH").unwrap_or_default();
        let path = format!("{}:{original_path}", self.mock_npm.bin_dir.display());
        Command::new("node")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .arg("scripts/release/verify-npm-publish-target.js")
            .arg("wrappers/npm")
            .env("PATH", path)
            .output()
            .expect("npm publish target verifier should run")
    }
}

struct MockNpm {
    root: PathBuf,
    bin_dir: PathBuf,
}

impl MockNpm {
    fn new(stdout: &str, stderr: &str, status: i32) -> Self {
        let root = TestDir::new("npm-publish-target").into_path();
        let bin_dir = root.join("bin");
        fs::create_dir_all(&bin_dir).expect("mock npm bin directory should be created");
        let script = bin_dir.join("npm");
        fs::write(
            &script,
            format!(
                "#!/usr/bin/env python3\nimport sys\nsys.stdout.write({stdout:?})\nsys.stderr.write({stderr:?})\nsys.exit({status})\n"
            ),
        )
        .expect("mock npm script should be written");
        let mut permissions = fs::metadata(&script)
            .expect("mock npm metadata should be readable")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&script, permissions).expect("mock npm should be executable");
        Self { root, bin_dir }
    }
}

impl Drop for MockNpm {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn npm_publish_target_accepts_missing_version() {
    let output =
        NpmPublishTargetCommand::new("", "npm ERR! code E404\nnpm ERR! 404 Not Found\n", 1).run();

    assert!(
        output.status.success(),
        "expected missing npm version to be publishable\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn npm_publish_target_rejects_existing_version() {
    let output = NpmPublishTargetCommand::new(env!("CARGO_PKG_VERSION"), "", 0).run();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected existing npm version to be rejected"
    );
    assert!(
        stderr.contains("is already published on npm"),
        "expected published-version guidance, stderr: {stderr}"
    );
}

#[test]
fn npm_publish_target_rejects_unpublish_window() {
    let output = NpmPublishTargetCommand::new(
        "",
        "npm ERR! 404 Unpublished on 2026-05-01T03:48:41.592Z\n",
        1,
    )
    .run();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected npm unpublish state to be rejected"
    );
    assert!(
        stderr.contains("blocked by npm unpublish state"),
        "expected npm unpublish guidance, stderr: {stderr}"
    );
}

#[test]
fn npm_publish_target_rejects_unknown_registry_failure() {
    let output = NpmPublishTargetCommand::new("", "npm ERR! network timeout\n", 1).run();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected unknown npm failure to be rejected"
    );
    assert!(
        stderr.contains("Could not verify npm publish target"),
        "expected registry verification guidance, stderr: {stderr}"
    );
}

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let sequence = NEXT_TEST_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "katana-markdown-linter-{name}-{}-{nanos}-{sequence}",
            std::process::id(),
        ));
        fs::create_dir_all(&path).expect("test dir should be created");
        Self { path }
    }

    fn into_path(self) -> PathBuf {
        let path = self.path.clone();
        std::mem::forget(self);
        path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
