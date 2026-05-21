use std::path::PathBuf;

#[test]
fn mcp_wrapper_distribution_contract_is_wired_end_to_end() {
    let contract = McpWrapperDistributionContract::new();

    assert_no_violations("mcp-wrapper-distribution", contract.violations());
}

struct McpWrapperDistributionContract {
    binary_artifacts: String,
    binary_roles: String,
    npm_package: String,
    npm_verifier: String,
    pypi_package: String,
    pypi_verifier: String,
    wrapper_smoke: String,
    release_workflow: String,
    release_verifier: String,
}

impl McpWrapperDistributionContract {
    fn new() -> Self {
        Self {
            binary_artifacts: read_workspace_file("scripts/release/binary_artifacts.py"),
            binary_roles: read_workspace_file("scripts/release/binary_roles.py"),
            npm_package: read_workspace_file("wrappers/npm/package.json"),
            npm_verifier: read_workspace_file("scripts/release/verify-npm-package.js"),
            pypi_package: read_workspace_file("wrappers/python/pyproject.toml"),
            pypi_verifier: read_workspace_file("scripts/release/verify-pypi-package.py"),
            wrapper_smoke: read_workspace_file("scripts/release/smoke-wrappers.sh"),
            release_workflow: read_workspace_file(".github/workflows/release.yml"),
            release_verifier: read_workspace_file("scripts/release/verify-release-published.sh"),
        }
    }

    fn violations(&self) -> Vec<String> {
        let mut violations = Vec::new();
        self.require_binary_roles(&mut violations);
        self.require_npm_entrypoints(&mut violations);
        self.require_pypi_entrypoints(&mut violations);
        self.require_release_wiring(&mut violations);
        violations
    }

    fn require_binary_roles(&self, violations: &mut Vec<String>) {
        for required in [
            "--executable",
            "scripts/ci/mcp-stdio-smoke.py",
            "scripts/ci/mcp-remote-smoke.py",
        ] {
            require_contains(
                violations,
                "scripts/release/binary_artifacts.py",
                &self.binary_artifacts,
                required,
            );
        }
        for required in ["kml-mcp", "kml-mcp-remote", "--features", "mcp-remote"] {
            require_contains(
                violations,
                "scripts/release/binary_roles.py",
                &self.binary_roles,
                required,
            );
        }
    }

    fn require_npm_entrypoints(&self, violations: &mut Vec<String>) {
        for required in [
            r#""katana-markdown-linter": "bin/katana-markdown-linter.js""#,
            r#""kml": "bin/kml.js""#,
            r#""kml-mcp": "bin/kml-mcp.js""#,
            r#""kml-mcp-remote": "bin/kml-mcp-remote.js""#,
        ] {
            require_contains(
                violations,
                "wrappers/npm/package.json",
                &self.npm_package,
                required,
            );
        }
        for required in [
            "bin/kml-mcp.js",
            "bin/kml-mcp-remote.js",
            "bin/katana-markdown-linter.js",
        ] {
            require_contains(
                violations,
                "scripts/release/verify-npm-package.js",
                &self.npm_verifier,
                required,
            );
        }
    }

    fn require_pypi_entrypoints(&self, violations: &mut Vec<String>) {
        for required in [
            r#"kml = "katana_markdown_linter.cli:main""#,
            r#"kml-mcp = "katana_markdown_linter.cli:main_mcp""#,
            r#"kml-mcp-remote = "katana_markdown_linter.cli:main_mcp_remote""#,
        ] {
            require_contains(
                violations,
                "wrappers/python/pyproject.toml",
                &self.pypi_package,
                required,
            );
        }
        for required in [
            "kml-mcp = katana_markdown_linter.cli:main_mcp",
            "kml-mcp-remote = katana_markdown_linter.cli:main_mcp_remote",
        ] {
            require_contains(
                violations,
                "scripts/release/verify-pypi-package.py",
                &self.pypi_verifier,
                required,
            );
        }
    }

    fn require_release_wiring(&self, violations: &mut Vec<String>) {
        for required in ["kml-mcp", "kml-mcp-remote", "npx", "bunx", "uvx"] {
            require_contains(
                violations,
                "scripts/release/smoke-wrappers.sh",
                &self.wrapper_smoke,
                required,
            );
        }
        for required in [
            "target/binary/kml-mcp-${TAG}-*",
            "target/binary/kml-mcp-remote-${TAG}-*",
        ] {
            require_contains(
                violations,
                ".github/workflows/release.yml",
                &self.release_workflow,
                required,
            );
        }
        for required in [
            "verify_mcp_binary_assets",
            "smoke_current_platform_mcp_binaries",
            "smoke_npm_mcp_wrapper",
            "smoke_pypi_mcp_wrapper",
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
