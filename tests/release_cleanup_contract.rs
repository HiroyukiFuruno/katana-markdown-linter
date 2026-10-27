use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

struct CleanupFixture {
    root: PathBuf,
    checkout: PathBuf,
}

impl CleanupFixture {
    fn new(default_branch: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let fixture_id = NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "katana-markdown-linter-cleanup-{}-{nanos}-{fixture_id}",
            std::process::id(),
        ));
        fs::create_dir_all(&root).expect("fixture directory should be created");

        let origin = root.join("origin.git");
        let source = root.join("source");
        let checkout = root.join("checkout");
        let empty_hooks = root.join("empty-hooks");
        fs::create_dir_all(&source).expect("source repository should be created");
        fs::create_dir_all(&empty_hooks).expect("empty hooks directory should be created");

        run_git(
            &root,
            &["init", "--bare", origin.to_str().expect("origin path")],
        );
        run_git(&source, &["init"]);
        run_git(&source, &["config", "user.name", "Cleanup Contract Test"]);
        run_git(
            &source,
            &["config", "user.email", "cleanup-contract@example.invalid"],
        );
        run_git(&source, &["config", "commit.gpgsign", "false"]);
        run_git(
            &source,
            &[
                "config",
                "core.hooksPath",
                empty_hooks.to_str().expect("empty hooks path"),
            ],
        );
        fs::write(source.join("seed.txt"), "seed\n").expect("seed file should be written");
        run_git(&source, &["add", "seed.txt"]);
        run_git(&source, &["commit", "-m", "seed repository"]);
        run_git(&source, &["branch", "-M", default_branch]);
        run_git(
            &source,
            &[
                "remote",
                "add",
                "origin",
                origin.to_str().expect("origin path"),
            ],
        );
        run_git(&source, &["push", "-u", "origin", default_branch]);
        run_git(&source, &["branch", "release/cleanup-test"]);
        run_git(&source, &["push", "origin", "release/cleanup-test"]);
        let remote_head = format!("refs/heads/{default_branch}");
        run_git(&origin, &["symbolic-ref", "HEAD", remote_head.as_str()]);
        let hooks_option = format!("core.hooksPath={}", empty_hooks.display());
        run_git(
            &root,
            &[
                "-c",
                &hooks_option,
                "clone",
                "--branch",
                default_branch,
                origin.to_str().expect("origin path"),
                checkout.to_str().expect("checkout path"),
            ],
        );
        run_git(&checkout, &["config", "user.name", "Cleanup Contract Test"]);
        run_git(
            &checkout,
            &["config", "user.email", "cleanup-contract@example.invalid"],
        );
        run_git(&checkout, &["config", "commit.gpgsign", "false"]);
        run_git(
            &checkout,
            &[
                "config",
                "core.hooksPath",
                empty_hooks.to_str().expect("empty hooks path"),
            ],
        );
        run_git(
            &checkout,
            &["update-ref", "--no-deref", "-d", "refs/remotes/origin/HEAD"],
        );

        let origin_head = Command::new("git")
            .current_dir(&checkout)
            .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
            .output()
            .expect("git should inspect origin/HEAD");
        assert!(
            !origin_head.status.success(),
            "fixture must not have origin/HEAD"
        );

        Self { root, checkout }
    }

    fn run_cleanup(&self) -> Output {
        let mut command = Command::new(bash_executable());
        command
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .arg("scripts/release/release-cleanup.sh")
            .env("KATANA_CLEANUP_REPO_ROOT", &self.checkout)
            .env("CLEANUP_SKIP_RELEASE_GUARD", "1")
            .env("MSYS2_ARG_CONV_EXCL", "*")
            .env("MSYS_NO_PATHCONV", "1")
            .env_remove("CLEANUP_SKIP_REMOTE")
            .env_remove("CLEANUP_DRY_RUN");
        command.output().expect("release cleanup script should run")
    }

    fn set_origin_head(&self, branch: &str) {
        let target = format!("refs/remotes/origin/{branch}");
        run_git(
            &self.checkout,
            &["symbolic-ref", "refs/remotes/origin/HEAD", target.as_str()],
        );
    }

    fn set_remote_head(&self, branch: &str) {
        let target = format!("refs/heads/{branch}");
        run_git(
            &self.root.join("origin.git"),
            &["symbolic-ref", "HEAD", target.as_str()],
        );
    }

    fn add_remote_branch(&self, branch: &str) {
        let source = self.root.join("source");
        run_git(&source, &["branch", branch]);
        run_git(&source, &["push", "origin", branch]);
    }
}

impl Drop for CleanupFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn run_git(directory: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(directory)
        .args(args)
        .output()
        .expect("git command should run");
    assert!(
        output.status.success(),
        "git {:?} should succeed, stderr: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(windows)]
fn bash_executable() -> PathBuf {
    if let Some(program_files) = std::env::var_os("ProgramFiles") {
        let candidate = PathBuf::from(program_files)
            .join("Git")
            .join("bin")
            .join("bash.exe");
        if candidate.is_file() {
            return candidate;
        }
    }

    PathBuf::from("bash")
}

#[cfg(not(windows))]
fn bash_executable() -> PathBuf {
    PathBuf::from("bash")
}

#[test]
fn cleanup_uses_remote_default_when_local_origin_head_is_missing() {
    for default_branch in ["master", "main"] {
        let fixture = CleanupFixture::new(default_branch);
        let output = fixture.run_cleanup();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            output.status.success(),
            "cleanup should succeed without origin/HEAD and use origin/{default_branch}, stderr: {stderr}"
        );
        assert!(
            stdout.contains(&format!("release cleanup completed on {default_branch}")),
            "cleanup should report selected branch {default_branch}, stdout: {stdout}"
        );

        let remote_branch = Command::new("git")
            .current_dir(&fixture.checkout)
            .args([
                "ls-remote",
                "--exit-code",
                "--heads",
                "origin",
                "release/cleanup-test",
            ])
            .output()
            .expect("git should check the temporary remote branch");
        assert!(
            remote_branch.status.success(),
            "remote branch should be preserved by default, stderr: {}",
            String::from_utf8_lossy(&remote_branch.stderr)
        );
    }
}

#[test]
fn cleanup_preserves_nested_branch_name_from_remote_head() {
    let fixture = CleanupFixture::new("master");
    let default_branch = "release/cleanup-test";
    fixture.set_remote_head(default_branch);

    let output = fixture.run_cleanup();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "cleanup should use the full valid origin/HEAD branch name, stderr: {stderr}"
    );
    assert!(
        stdout.contains(&format!("release cleanup completed on {default_branch}")),
        "cleanup should report selected branch {default_branch}, stdout: {stdout}"
    );
}

#[test]
fn cleanup_fails_closed_when_remote_head_is_invalid_and_multiple_branches_exist() {
    let fixture = CleanupFixture::new("master");
    fixture.add_remote_branch("main");
    fixture.set_remote_head("unpublished-default");

    let output = fixture.run_cleanup();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "cleanup should refuse to choose between origin/master and origin/main"
    );
    assert!(
        stderr.contains("origin does not advertise a branch for its remote HEAD"),
        "cleanup should explain the invalid remote default branch, stderr: {stderr}"
    );
    assert!(
        !stdout.contains("release cleanup completed"),
        "cleanup must stop before branch cleanup, stdout: {stdout}"
    );
}

#[test]
fn cleanup_prefers_remote_head_over_stale_local_origin_head() {
    let fixture = CleanupFixture::new("master");
    let default_branch = "main";
    fixture.add_remote_branch(default_branch);
    fixture.set_remote_head(default_branch);
    fixture.set_origin_head("master");

    let output = fixture.run_cleanup();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "cleanup should use actual remote HEAD over stale local origin/HEAD, stderr: {stderr}"
    );
    assert!(
        stdout.contains(&format!("release cleanup completed on {default_branch}")),
        "cleanup should report selected branch {default_branch}, stdout: {stdout}"
    );
}
