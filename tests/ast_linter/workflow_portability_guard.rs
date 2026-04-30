use std::path::PathBuf;

#[test]
fn ci_workflows_keep_windows_and_cache_strategy_explicit() {
    let guard = WorkflowPortabilityGuard::new();

    assert_no_violations("workflow-portability", guard.violations());
}

struct WorkflowPortabilityGuard {
    ci: String,
    attributes: String,
    preflight: String,
    release: String,
    homebrew_formula: String,
    homebrew_tap_updater: String,
    release_verifier: String,
}

impl WorkflowPortabilityGuard {
    fn new() -> Self {
        Self {
            ci: read_workspace_file(".github/workflows/test-and-build.yml"),
            attributes: read_workspace_file(".gitattributes"),
            preflight: read_workspace_file(".github/workflows/release-preflight.yml"),
            release: read_workspace_file(".github/workflows/release.yml"),
            homebrew_formula: read_workspace_file("scripts/release/homebrew_formula.py"),
            homebrew_tap_updater: read_workspace_file("scripts/release/update_homebrew_tap.py"),
            release_verifier: read_workspace_file("scripts/release/verify-release-published.sh"),
        }
    }

    fn violations(&self) -> Vec<String> {
        let mut violations = Vec::new();
        self.require_ci_windows_matrix(&mut violations);
        self.require_cross_platform_commands(&mut violations);
        self.require_windows_line_ending_guard(&mut violations);
        self.require_unified_rust_cache(&mut violations);
        self.require_homebrew_tap_update(&mut violations);
        violations
    }

    fn require_ci_windows_matrix(&self, violations: &mut Vec<String>) {
        // The matrix is now conditional: 3 platforms on PR, ubuntu-only on push-to-main.
        // Both platforms must still be present in the expression.
        for platform in ["macos-latest", "windows-latest", "ubuntu-latest"] {
            require_contains(
                violations,
                ".github/workflows/test-and-build.yml",
                &self.ci,
                platform,
            );
        }
        require_contains(
            violations,
            ".github/workflows/test-and-build.yml",
            &self.ci,
            "Run Windows workspace test",
        );
    }

    fn require_cross_platform_commands(&self, violations: &mut Vec<String>) {
        for required in [
            "cargo check --workspace --locked",
            "cargo test --workspace --locked",
            "cargo fmt --all -- --check",
            "if: matrix.os == 'windows-latest'",
            "if: matrix.os != 'windows-latest'",
        ] {
            require_contains(
                violations,
                ".github/workflows/test-and-build.yml",
                &self.ci,
                required,
            );
        }
    }

    fn require_windows_line_ending_guard(&self, violations: &mut Vec<String>) {
        require_contains(
            violations,
            ".gitattributes",
            &self.attributes,
            "*.rs text eol=lf",
        );
    }

    fn require_unified_rust_cache(&self, violations: &mut Vec<String>) {
        require_absent(
            violations,
            ".github/workflows/test-and-build.yml",
            &self.ci,
            "actions/cache",
        );
        for (path, content, shared_key) in [
            (
                ".github/workflows/test-and-build.yml",
                &self.ci,
                "ci-${{ matrix.os }}-stable-default",
            ),
            (
                ".github/workflows/release-preflight.yml",
                &self.preflight,
                "release-preflight-ubuntu-stable-all-features",
            ),
            (
                ".github/workflows/release.yml",
                &self.release,
                "release-ubuntu-stable-publish",
            ),
        ] {
            require_contains(violations, path, content, "uses: Swatinem/rust-cache@v2");
            require_contains(violations, path, content, "shared-key:");
            require_contains(violations, path, content, shared_key);
        }
    }

    fn require_homebrew_tap_update(&self, violations: &mut Vec<String>) {
        for required in [
            "HOMEBREW_KATANA_GIT_TOKEN: ${{ secrets.HOMEBREW_KATANA_GIT_TOKEN }}",
            "scripts/release/update_homebrew_tap.py",
            "target/homebrew/kml@${{ steps.version.outputs.version_bare }}.rb",
        ] {
            require_contains(
                violations,
                ".github/workflows/release.yml",
                &self.release,
                required,
            );
        }
        for required in [
            "versioned_formula = \"@\" in self.formula_name",
            "keg_only :versioned_formula",
        ] {
            require_contains(
                violations,
                "scripts/release/homebrew_formula.py",
                &self.homebrew_formula,
                required,
            );
        }
        for required in [
            "HOMEBREW_KATANA_GIT_TOKEN",
            "HiroyukiFuruno/homebrew-katana",
            "path=f\"{self.formula_dir}/kml.rb\"",
            "versioned_name = f\"kml@{self.version_bare}.rb\"",
        ] {
            require_contains(
                violations,
                "scripts/release/update_homebrew_tap.py",
                &self.homebrew_tap_updater,
                required,
            );
        }
        for required in [
            "HOMEBREW_TAP_REPO",
            "raw.githubusercontent.com",
            "Formula/kml@${VERSION_BARE}.rb",
            "Homebrew tap formula mismatch",
        ] {
            require_contains(
                violations,
                "scripts/release/verify-release-published.sh",
                &self.release_verifier,
                required,
            );
        }
    }
}

fn require_contains(violations: &mut Vec<String>, path: &str, content: &str, required: &str) {
    if !content.contains(required) {
        violations.push(format!("{path}: missing `{required}`"));
    }
}

fn require_absent(violations: &mut Vec<String>, path: &str, content: &str, forbidden: &str) {
    if content.contains(forbidden) {
        violations.push(format!("{path}: remove `{forbidden}`"));
    }
}

fn read_workspace_file(path: &str) -> String {
    std::fs::read_to_string(workspace_root().join(path)).expect("workspace file should be readable")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn assert_no_violations(name: &str, violations: Vec<String>) {
    if violations.is_empty() {
        return;
    }

    panic!(
        "{name} failed with {} violation(s):\n{}",
        violations.len(),
        violations.join("\n")
    );
}
