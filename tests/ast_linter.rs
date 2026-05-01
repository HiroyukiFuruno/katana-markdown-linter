use ignore::{WalkBuilder, WalkState};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};

#[path = "ast_linter/documentation_language_guard.rs"]
mod documentation_language_guard;
#[path = "ast_linter/i18n_guard.rs"]
mod i18n_guard;
#[path = "ast_linter/open_spec_command_portability_guard.rs"]
mod open_spec_command_portability_guard;
#[path = "ast_linter/workflow_portability_guard.rs"]
mod workflow_portability_guard;

#[test]
fn ast_linter_parses_rule_doc_fixture() {
    let document = katana_markdown_linter::upstream::parse_rule_document(
        r#"# `MD001` - Heading levels should only increment by one level at a time

Tags: `headings`

Aliases: `heading-increment`

Parameters:

- `front_matter_title`: RegExp for matching title in front matter (`string`,
  default `^\s*title\s*[:=]`)
"#,
    )
    .expect("fixture should parse");

    assert_eq!(document.id, "MD001");
    assert_eq!(document.name, "heading-increment");
    assert_eq!(document.properties[0].key, "front_matter_title");
}

#[test]
fn ast_linter_no_lazy_macros_in_source() {
    let root = workspace_root();
    let violations = scan_rust_sources(
        &[root.join("src"), root.join("tests"), root.join("build.rs")],
        |path, line_idx, line| {
            // Skip this file itself — it defines the banned token strings as literals.
            // Normalize separators for Windows compatibility.
            let path_normalized = path.to_string_lossy().replace('\\', "/");
            if path_normalized.ends_with("tests/ast_linter.rs")
                || path_normalized.contains("tests/ast_linter/")
            {
                return None;
            }
            let banned = ["todo!(", "unimplemented!(", "dbg!("];
            banned
                .iter()
                .find(|token| line.contains(**token))
                .map(|token| {
                    format!(
                        "{}:{}: remove lazy macro `{}` and implement the behavior",
                        path.display(),
                        line_idx + 1,
                        token.trim_end_matches('(')
                    )
                })
        },
    );

    assert_no_violations("lazy-macros", violations);
}

#[test]
fn ast_linter_cli_directory_walk_uses_parallel_ignore_walker() {
    let required: &[&str] = &[
        "use ignore::{WalkBuilder, WalkState};",
        "WalkBuilder::new(dir)",
        ".build_parallel()",
        ".require_git(false)",
    ];
    // Use scan_rust_sources so this test survives future splits of src/cli.rs into sub-modules.
    // Task 2.2: filter to src/cli* paths only to avoid false-positives from test helpers that
    // also use WalkBuilder.
    let found: Arc<Mutex<BTreeSet<&str>>> = Arc::new(Mutex::new(BTreeSet::new()));
    {
        let found = found.clone();
        scan_rust_sources(
            &[workspace_root().join("src")],
            move |path, _line_idx, line| {
                let path_normalized = path.to_string_lossy().replace('\\', "/");
                if !path_normalized.contains("/cli") {
                    return None;
                }
                for token in required {
                    if line.contains(token) {
                        found.lock().unwrap().insert(token);
                    }
                }
                None
            },
        );
    }
    let found = found.lock().unwrap();
    let violations = required
        .iter()
        .filter(|token| !found.contains(*token))
        .map(|token| format!("src/cli*: missing `{token}`"))
        .collect();

    assert_no_violations("cli-parallel-walker", violations);
}

#[test]
fn ast_linter_fixture_matrix_covers_active_rule_catalog() {
    let matrix = rule_fixture_matrix();
    let matrix_rules = matrix["rules"]
        .as_array()
        .expect("fixture matrix rules should be an array");
    let matrix_ids = matrix_rules
        .iter()
        .filter_map(|rule| rule["rule_id"].as_str())
        .collect::<BTreeSet<_>>();
    let catalog = katana_markdown_linter::rule_catalog();
    let catalog_ids = catalog
        .active_rules()
        .map(|rule| rule.id.as_str())
        .collect::<BTreeSet<_>>();

    let mut violations = Vec::new();
    for missing in catalog_ids.difference(&matrix_ids) {
        violations.push(format!(
            "tests/fixtures/rule-fixture-matrix.json: missing fixture entry for {missing}"
        ));
    }
    for stale in matrix_ids.difference(&catalog_ids) {
        violations.push(format!(
            "tests/fixtures/rule-fixture-matrix.json: stale fixture entry for {stale}"
        ));
    }
    for field in [
        "rule_id",
        "aliases",
        "tags",
        "parameters",
        "fixable",
        "check_pass",
        "check_fail",
        "fix",
        "config_valid",
        "config_invalid",
        "edge",
        "manual_required",
    ] {
        for rule in matrix_rules {
            if rule.get(field).is_none() {
                violations.push(format!(
                    "tests/fixtures/rule-fixture-matrix.json: {} missing `{field}`",
                    rule["rule_id"].as_str().unwrap_or("<unknown>")
                ));
            }
        }
    }

    assert_no_violations("fixture-matrix-coverage", violations);
}

#[test]
fn ast_linter_upstream_drift_gate_is_wired_to_make_and_release_workflows() {
    let makefile = read_workspace_file("Makefile");
    let release = read_workspace_file(".github/workflows/release.yml");
    let preflight = read_workspace_file(".github/workflows/release-preflight.yml");
    let upstream_drift = read_workspace_file("src/upstream/drift.rs");
    let required = [
        (
            "Makefile",
            &makefile,
            "upstream_default_branch_drift_has_no_unknown_items",
        ),
        ("Makefile", &makefile, "-- --ignored"),
        (
            ".github/workflows/release.yml",
            &release,
            "KML_UPSTREAM_MARKDOWNLINT_DOC_DIR",
        ),
        (
            ".github/workflows/release.yml",
            &release,
            "make upstream-drift",
        ),
        (
            ".github/workflows/release-preflight.yml",
            &preflight,
            "KML_UPSTREAM_MARKDOWNLINT_DOC_DIR",
        ),
        (
            ".github/workflows/release-preflight.yml",
            &preflight,
            "make upstream-drift",
        ),
        (
            "src/upstream/drift.rs",
            &upstream_drift,
            "assert_no_unknown_drift",
        ),
        (
            "src/upstream/drift.rs",
            &upstream_drift,
            "known_current_drift_allowlist",
        ),
    ];
    let violations = required
        .iter()
        .filter(|(_, content, required)| !content.contains(*required))
        .map(|(path, _, required)| format!("{path}: missing `{required}`"))
        .collect();

    assert_no_violations("upstream-drift-gate-wiring", violations);
}

#[test]
fn ast_linter_release_workflow_requires_github_verified_signed_tag() {
    let makefile = read_workspace_file("Makefile");
    let workflow = read_workspace_file(".github/workflows/release.yml");
    let verifier = read_workspace_file("scripts/release/verify-tag-verified.sh");
    let tag_guard = read_workspace_file("scripts/release/assert-tag-safe.sh");
    let required = [
        (
            ".github/workflows/release.yml",
            &workflow,
            "Verify release tag is GitHub Verified",
        ),
        (
            ".github/workflows/release.yml",
            &workflow,
            "scripts/release/verify-tag-verified.sh",
        ),
        (".github/workflows/release.yml", &workflow, "--verify-tag"),
        ("Makefile", &makefile, "RELEASE_TAGGER_NAME"),
        ("Makefile", &makefile, "RELEASE_TAGGER_EMAIL"),
        ("Makefile", &makefile, "GIT_COMMITTER_NAME"),
        (
            "scripts/release/verify-tag-verified.sh",
            &verifier,
            "git cat-file -t",
        ),
        (
            "scripts/release/verify-tag-verified.sh",
            &verifier,
            "git tag -v",
        ),
        (
            "scripts/release/verify-tag-verified.sh",
            &verifier,
            ".verification.verified",
        ),
        (
            "scripts/release/verify-tag-verified.sh",
            &verifier,
            "Use a tagger identity that GitHub can associate with the signing key.",
        ),
        ("Makefile", &makefile, "scripts/release/assert-tag-safe.sh"),
        (
            "scripts/release/assert-tag-safe.sh",
            &tag_guard,
            "refusing to overwrite a released tag",
        ),
    ];
    let violations = required
        .iter()
        .filter(|(_, content, required)| !content.contains(*required))
        .map(|(path, _, required)| format!("{path}: missing `{required}`"))
        .collect();

    assert_no_violations("release-github-verified-tag", violations);
}

#[test]
fn ast_linter_release_local_ci_parity_and_retry_safety() {
    let makefile = read_workspace_file("Makefile");
    let workflow = read_workspace_file(".github/workflows/release.yml");
    let preflight = read_workspace_file(".github/workflows/release-preflight.yml");
    let runbook = read_workspace_file("docs/release-runbook.md");
    let quality = read_workspace_file("docs/quality-gates.md");
    let release_notes = read_workspace_file("scripts/release/release-notes.sh");
    let crate_guard = read_workspace_file("scripts/release/assert-crate-not-published.sh");
    let tag_verifier = read_workspace_file("scripts/release/verify-tag-verified.sh");
    let published_verifier = read_workspace_file("scripts/release/verify-release-published.sh");
    let wrapper_smoke = read_workspace_file("scripts/release/smoke-wrappers.sh");
    let npm_installer = read_workspace_file("wrappers/npm/lib/installer.js");
    let python_installer =
        read_workspace_file("wrappers/python/src/katana_markdown_linter/installer.py");
    let task_ledger_verifier = read_workspace_file("scripts/release/verify-task-ledger.py");
    let release_target_verifier = read_workspace_file("scripts/release/verify-release-target.py");
    let release_recovery = read_workspace_file("scripts/release/recover-accidental-release.py");
    let answer_runner = read_workspace_file("scripts/ci/document_answer_fix_runner.py");
    let answer_validator = read_workspace_file("scripts/ci/document_answer_validator.py");
    let required = [
        ("Makefile", &makefile, "release-target-check:"),
        ("Makefile", &makefile, "scripts/release/verify-release-target.py"),
        ("Makefile", &makefile, "release-recovery-plan:"),
        ("Makefile", &makefile, "release-recover:"),
        ("Makefile", &makefile, "scripts/release/recover-accidental-release.py"),
        ("Makefile", &makefile, "release-check: release-target-check fmt-check lint ast-lint release-test dogfood coverage-blocking examples mcp-build mcp-stdio-smoke mcp-remote-build mcp-remote-smoke mcpb-smoke server-json-validate action-smoke binary-smoke homebrew-formula-check wrapper-smoke npm-package-check pypi-package-check wrapper-publish-gate document-answer-fix"),
        ("Makefile", &makefile, "binary-smoke: binary-package"),
        ("Makefile", &makefile, "homebrew-formula-check: homebrew-formula"),
        ("Makefile", &makefile, "wrapper-smoke: binary-package"),
        ("Makefile", &makefile, "npm-package-check:"),
        ("Makefile", &makefile, "scripts/release/verify-npm-package.js"),
        ("Makefile", &makefile, "pypi-package-check:"),
        ("Makefile", &makefile, "scripts/release/verify-pypi-package.py"),
        ("Makefile", &makefile, "document-answer-fix:"),
        ("Makefile", &makefile, "mcp-release-build:"),
        ("Makefile", &makefile, "mcp-remote-build:"),
        ("Makefile", &makefile, "mcp-remote-smoke:"),
        ("Makefile", &makefile, "mcpb-package: mcp-release-build"),
        ("Makefile", &makefile, "mcpb-smoke: mcpb-package"),
        ("Makefile", &makefile, "server-json-validate: mcp-server-json"),
        ("Makefile", &makefile, "release-task-ledger-check:"),
        ("Makefile", &makefile, "release-test:"),
        ("Makefile", &makefile, "cargo test --all-features --locked"),
        ("Makefile", &makefile, "scripts/release/assert-crate-not-published.sh"),
        ("Makefile", &makefile, "scripts/release/verify-task-ledger.py"),
        ("Makefile", &makefile, "release-verify:"),
        ("Makefile", &makefile, "scripts/release/verify-release-published.sh"),
        ("Makefile", &makefile, "publish_npm_wrapper=true"),
        ("Makefile", &makefile, "publish_pypi_wrapper=true"),
        (".github/workflows/release.yml", &workflow, "run: make lint"),
        (".github/workflows/release.yml", &workflow, "run: make examples"),
        (".github/workflows/release.yml", &workflow, "run: make mcp-build"),
        (".github/workflows/release.yml", &workflow, "run: make mcp-remote-smoke"),
        (".github/workflows/release.yml", &workflow, "MCPB smoke"),
        (".github/workflows/release.yml", &workflow, "MCP Registry metadata"),
        (".github/workflows/release.yml", &workflow, "run: make document-answer-fix"),
        (".github/workflows/release.yml", &workflow, "Binary artifact (${{ matrix.target }})"),
        (".github/workflows/release.yml", &workflow, "macos-15-intel"),
        (".github/workflows/release.yml", &workflow, "Generate Homebrew formula"),
        (".github/workflows/release.yml", &workflow, "Publish npm wrapper"),
        (".github/workflows/release.yml", &workflow, "node-version: \"24\""),
        (
            ".github/workflows/release.yml",
            &workflow,
            "Verify npm trusted publishing context",
        ),
        (
            ".github/workflows/release.yml",
            &workflow,
            "run: make npm-package-check",
        ),
        (
            ".github/workflows/release.yml",
            &workflow,
            "run: make pypi-package-check",
        ),
        (
            ".github/workflows/release.yml",
            &workflow,
            "inputs.publish_npm_wrapper == true",
        ),
        (
            ".github/workflows/release.yml",
            &workflow,
            "inputs.publish_pypi_wrapper == true",
        ),
        (
            ".github/workflows/release.yml",
            &workflow,
            "npm publish --tag latest --access public --provenance",
        ),
        (".github/workflows/release.yml", &workflow, "Publish PyPI wrapper"),
        (".github/workflows/release.yml", &workflow, "environment: pypi"),
        (".github/workflows/release.yml", &workflow, "pypa/gh-action-pypi-publish@release/v1"),
        (".github/workflows/release.yml", &workflow, "Publish MCP Registry metadata"),
        (".github/workflows/release.yml", &workflow, "mcp-publisher login github-oidc"),
        (".github/workflows/release.yml", &workflow, "gh release upload \"$TAG\" \"$PACKAGE_PATH\" \"$CHECKSUM_PATH\" \"$MCPB_PATH\" \"$MCPB_CHECKSUM_PATH\" \"$SERVER_JSON_PATH\" \"${binary_assets[@]}\" --clobber"),
        (".github/workflows/release.yml", &workflow, "scripts/release/assert-crate-not-published.sh"),
        (".github/workflows/release.yml", &workflow, "--title \"$TAG\""),
        (".github/workflows/release.yml", &workflow, "Release target check"),
        (".github/workflows/release.yml", &workflow, "GH_TOKEN: ${{ github.token }}"),
        (".github/workflows/release.yml", &workflow, "make release-target-check VERSION=\"${{ steps.version.outputs.version }}\""),
        (".github/workflows/release-preflight.yml", &preflight, "Release target check"),
        (".github/workflows/release-preflight.yml", &preflight, "permissions:"),
        (".github/workflows/release-preflight.yml", &preflight, "contents: read"),
        (".github/workflows/release-preflight.yml", &preflight, "GH_TOKEN: ${{ github.token }}"),
        (".github/workflows/release-preflight.yml", &preflight, "startsWith(github.head_ref, 'release/v')"),
        (".github/workflows/release-preflight.yml", &preflight, "make release-target-check VERSION=\"${GITHUB_HEAD_REF#release/}\""),
        (".github/workflows/release-preflight.yml", &preflight, "run: make lint"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make examples"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make mcp-build"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make mcp-remote-smoke"),
        (".github/workflows/release-preflight.yml", &preflight, "MCPB smoke"),
        (".github/workflows/release-preflight.yml", &preflight, "MCP Registry metadata"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make document-answer-fix"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make binary-smoke"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make homebrew-formula-check"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make wrapper-smoke"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make npm-package-check"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make pypi-package-check"),
        ("scripts/ci/document_answer_fix_runner.py", &answer_runner, "AnswerValidationRunner"),
        (
            "scripts/ci/document_answer_validator.py",
            &answer_validator,
            "answer_diagnostics",
        ),
        (
            "scripts/ci/document_answer_validator.py",
            &answer_validator,
            "answer fixture changes when fixed again",
        ),
        ("docs/release-runbook.md", &runbook, "make release-verify VERSION=vX.Y.Z"),
        ("docs/release-runbook.md", &runbook, "make release-target-check VERSION=vX.Y.Z"),
        ("docs/release-runbook.md", &runbook, "make release-recovery-plan BAD_VERSION=v0.18.7"),
        ("docs/release-runbook.md", &runbook, "KML_RELEASE_RECOVERY_CONFIRM=v0.18.7 make release-recover BAD_VERSION=v0.18.7"),
        ("docs/release-runbook.md", &runbook, "release target follows the published stable release line"),
        ("docs/release-runbook.md", &runbook, "make mcpb-smoke VERSION=vX.Y.Z"),
        ("docs/release-runbook.md", &runbook, "make binary-smoke VERSION=vX.Y.Z"),
        ("docs/release-runbook.md", &runbook, "make homebrew-formula-check VERSION=vX.Y.Z"),
        ("docs/release-runbook.md", &runbook, "mcp-publisher login github-oidc"),
        ("docs/release-runbook.md", &runbook, "Do not rewrite a tag unless no GitHub Release exists"),
        ("docs/quality-gates.md", &quality, "release retry helpers must refuse remote tag overwrites"),
        ("docs/quality-gates.md", &quality, "already exists on crates.io"),
        ("scripts/release/release-notes.sh", &release_notes, "CHANGELOG.md is missing a non-empty section"),
        ("scripts/release/assert-crate-not-published.sh", &crate_guard, "Bump Cargo.toml before dispatching"),
        ("scripts/release/verify-release-published.sh", &published_verifier, "assert_equal \"GitHub Release title\""),
        ("scripts/release/verify-tag-verified.sh", &tag_verifier, "git fetch --quiet origin \"refs/tags/${TAG}:refs/tags/${TAG}\""),
        ("scripts/release/verify-release-published.sh", &published_verifier, "release_asset_exists"),
        ("scripts/release/smoke-wrappers.sh", &wrapper_smoke, "reused an unversioned stale cache"),
        ("wrappers/npm/lib/installer.js", &npm_installer, "this.version, this.target"),
        ("wrappers/python/src/katana_markdown_linter/installer.py", &python_installer, "self.version / self.target"),
        ("scripts/release/verify-release-published.sh", &published_verifier, "github_release_title="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "github_release_target="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "GitHub Release is missing binary archive"),
        ("scripts/release/verify-release-published.sh", &published_verifier, "crates_io_version="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "npm_registry_version="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "pypi_registry_version="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "npm_wrapper_version="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "pypi_wrapper_version="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "homebrew_formula_path="),
        ("scripts/release/verify-task-ledger.py", &task_ledger_verifier, "Verify that the OpenSpec task ledger is release-ready."),
        ("scripts/release/verify-task-ledger.py", &task_ledger_verifier, "Release task ledger is not ready"),
        ("scripts/release/verify-task-ledger.py", &task_ledger_verifier, "品質評価スコア table is missing a 合計 row"),
        ("scripts/release/verify-release-target.py", &release_target_verifier, "a new minor line must start"),
        ("scripts/release/verify-release-target.py", &release_target_verifier, "KML_RELEASE_ALLOW_VERSION_LINE_OVERRIDE"),
        ("scripts/release/verify-release-target.py", &release_target_verifier, "api.github.com/repos"),
        ("scripts/release/verify-release-target.py", &release_target_verifier, "prerelease"),
        ("scripts/release/verify-release-target.py", &release_target_verifier, "draft"),
        ("scripts/release/recover-accidental-release.py", &release_recovery, "KML_RELEASE_RECOVERY_CONFIRM"),
        ("scripts/release/recover-accidental-release.py", &release_recovery, "\"dist-tag\""),
        ("scripts/release/recover-accidental-release.py", &release_recovery, "Yank PyPI release"),
    ];
    let violations = required
        .iter()
        .filter(|(_, content, required)| !content.contains(*required))
        .map(|(path, _, required)| format!("{path}: missing `{required}`"))
        .collect();

    assert_no_violations("release-local-ci-parity-and-retry-safety", violations);
}

#[test]
fn ast_linter_npm_wrapper_publish_is_tokenless() {
    let workflow = read_workspace_file(".github/workflows/release.yml");
    let violations = ["NPM_TOKEN", "NODE_AUTH_TOKEN"]
        .into_iter()
        .filter(|forbidden| workflow.contains(forbidden))
        .map(|forbidden| {
            format!(".github/workflows/release.yml: remove `{forbidden}` from npm wrapper publish")
        })
        .collect();

    assert_no_violations("npm-wrapper-tokenless-publish", violations);
}

#[test]
fn ast_linter_coverage_gate_counts_integration_tests() {
    let coverage = read_workspace_file("scripts/ci/coverage.sh");
    let violations = [
        (
            coverage.contains("cargo llvm-cov --no-report --jobs \"$JOBS\" --workspace -q"),
            "scripts/ci/coverage.sh: coverage must run workspace tests, including integration tests",
        ),
        (
            !coverage.contains("--workspace --lib --bins"),
            "scripts/ci/coverage.sh: remove --lib --bins so integration-test coverage is counted",
        ),
        (
            coverage.contains("Running workspace tests with llvm-cov"),
            "scripts/ci/coverage.sh: status text must describe the actual coverage scope",
        ),
    ]
    .into_iter()
    .filter_map(|(ok, message)| (!ok).then_some(message.to_string()))
    .collect();

    assert_no_violations("coverage-gate-integration-tests", violations);
}

#[test]
fn ast_linter_github_action_channel_is_wired() {
    let action = read_workspace_file("action.yml");
    let makefile = read_workspace_file("Makefile");
    let workflow = read_workspace_file(".github/workflows/test-and-build.yml");
    let preflight = read_workspace_file(".github/workflows/release-preflight.yml");
    let release = read_workspace_file(".github/workflows/release.yml");
    let install = read_workspace_file("scripts/action/install-kml.sh");
    let runner = read_workspace_file("scripts/action/run-kml.sh");
    let required = [
        ("action.yml", &action, "using: composite"),
        ("action.yml", &action, "KML_ACTION_INSTALL_SOURCE"),
        ("action.yml", &action, "scripts/action/install-kml.sh"),
        ("action.yml", &action, "scripts/action/run-kml.sh"),
        (
            "scripts/action/install-kml.sh",
            &install,
            "cargo install katana-markdown-linter",
        ),
        (
            "scripts/action/install-kml.sh",
            &install,
            "cargo install --path",
        ),
        ("scripts/action/install-kml.sh", &install, "kml.exe"),
        (
            "scripts/action/run-kml.sh",
            &runner,
            "append_multiline_args",
        ),
        ("Makefile", &makefile, "action-smoke:"),
        ("Makefile", &makefile, "KML_ACTION_INSTALL_SOURCE=path"),
        (
            ".github/workflows/test-and-build.yml",
            &workflow,
            "Run action smoke",
        ),
        (
            ".github/workflows/test-and-build.yml",
            &workflow,
            "'action.yml'",
        ),
        (
            ".github/workflows/release-preflight.yml",
            &preflight,
            "Action smoke",
        ),
        (".github/workflows/release.yml", &release, "Action smoke"),
    ];
    let violations = required
        .iter()
        .filter(|(_, content, required)| !content.contains(*required))
        .map(|(path, _, required)| format!("{path}: missing `{required}`"))
        .collect();

    assert_no_violations("github-action-channel", violations);
}

#[test]
fn ast_linter_readme_rule_map_matches_public_catalog() {
    let readme = read_workspace_file("README.md");
    let mut violations = Vec::new();

    if !readme.contains("## Rule Map") {
        violations.push("README.md: missing `## Rule Map`".to_string());
    }
    if !readme.contains("| Rule | Check | Fix (safe) | Fix (unsafe) |") {
        violations.push("README.md: missing rule map state columns".to_string());
    }

    let fixture_matrix: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/rule-fixture-matrix.json"))
            .expect("fixture matrix should parse");
    let rules = fixture_matrix["rules"]
        .as_array()
        .expect("fixture matrix rules should be an array");

    let active_rules = katana_markdown_linter::available_rules();
    let active_rule_ids = active_rules
        .iter()
        .map(|rule| rule.id.as_str())
        .collect::<BTreeSet<_>>();

    for rule in &active_rules {
        let safe_fix = rule_map_safe_fix_status(rules, rule.id.as_str());
        let unsafe_fix = rule_map_unsafe_fix_status(rules, rule.id.as_str());
        let row = format!(
            "| `{}` | Implemented | {} | {} |",
            rule.id, safe_fix, unsafe_fix
        );
        if !readme.contains(&row) {
            violations.push(format!("README.md: missing rule map row `{row}`"));
        }
    }

    let max_rule_number = active_rule_ids
        .iter()
        .filter_map(|rule_id| rule_id.strip_prefix("MD")?.parse::<u16>().ok())
        .max()
        .expect("active rules should contain MD-prefixed rule IDs");
    for number in 1..=max_rule_number {
        let rule_id = format!("MD{number:03}");
        if active_rule_ids.contains(rule_id.as_str()) {
            continue;
        }
        let row = format!("| `{rule_id}` | Deleted | - | - |");
        if !readme.contains(&row) {
            violations.push(format!("README.md: missing deleted rule gap row `{row}`"));
        }
    }

    assert_no_violations("readme-rule-map", violations);
}

fn rule_map_safe_fix_status(rules: &[Value], rule_id: &str) -> &'static str {
    let has_safe_fix = rules
        .iter()
        .find(|entry| entry["rule_id"].as_str() == Some(rule_id))
        .and_then(|entry| entry["fix"].as_array())
        .is_some_and(|fixes| !fixes.is_empty());

    if rule_id == "MD056" {
        "Implemented (pads short rows; overflow rows remain diagnostic-only)"
    } else if has_safe_fix {
        "Implemented subset"
    } else {
        "Diagnostic only"
    }
}

fn rule_map_unsafe_fix_status(rules: &[Value], rule_id: &str) -> &'static str {
    let has_unsafe_fix = rules
        .iter()
        .find(|entry| entry["rule_id"].as_str() == Some(rule_id))
        .and_then(|entry| entry.get("unsafe_fix"))
        .and_then(Value::as_array)
        .is_some_and(|fixes| !fixes.is_empty());

    let has_safe_fix = rules
        .iter()
        .find(|entry| entry["rule_id"].as_str() == Some(rule_id))
        .and_then(|entry| entry["fix"].as_array())
        .is_some_and(|fixes| !fixes.is_empty());

    if has_unsafe_fix {
        "Implemented subset"
    } else if has_safe_fix {
        "Not applicable"
    } else if has_manual_required_reason(rules, rule_id) {
        "Manual intent required"
    } else {
        "Needs triage"
    }
}

fn has_manual_required_reason(rules: &[Value], rule_id: &str) -> bool {
    rules
        .iter()
        .find(|entry| entry["rule_id"].as_str() == Some(rule_id))
        .and_then(|entry| entry["manual_required"].as_array())
        .is_some_and(|reasons| !reasons.is_empty())
}

#[test]
fn ast_linter_markdown_rule_catalog_has_unique_rule_ids() {
    let rules = katana_markdown_linter::rule_catalog();
    let mut seen = std::collections::BTreeSet::new();
    let duplicates = rules
        .active_rules()
        .filter_map(|entry| {
            if seen.insert(entry.id.as_str()) {
                None
            } else {
                Some(format!("{}: duplicate markdown rule id", entry.id))
            }
        })
        .collect();

    assert_no_violations("markdown-rule-ids", duplicates);
}

#[test]
fn ast_linter_public_api_surface_is_explicit() {
    let lib = read_workspace_file("src/lib.rs");
    let required = [
        "pub fn lint(content: &str, options: &LintOptions) -> Result<Vec<LintResult>, Error>",
        "pub fn fix(content: &str, options: &LintOptions) -> Result<FixResult, Error>",
        "pub use formatter::{format_markdown, layout_lint_options, FormatOptions, FormatResult};",
        "pub fn available_rules() -> Vec<RuleMeta>",
        "pub fn localized_available_rules(language_code: &str) -> Vec<RuleMeta>",
        "pub fn implemented_rules() -> Vec<RuleMeta>",
        "pub fn missing_rules() -> Vec<RuleMeta>",
        "pub fn rule_catalog() -> catalog::RuleCatalog",
        "pub fn localized_rule_catalog(language_code: &str) -> catalog::RuleCatalog",
        "pub use config::{ConfigError, ConfigErrorKind, MarkdownLintConfig};",
        "pub use i18n::{",
        "has_rule_description_translation",
        "localized_rule_description",
        "resolve_locale_code",
        "resolve_locale_code_or",
        "supported_locales",
        "Locale, LocaleError",
        "LocalizedDiagnostic",
        "Fix, FixDetail, FixResult, FixSafety, LintOptions, LintResult, Range, RuleConfig, RuleMeta,",
    ];
    let mut violations = required
        .iter()
        .filter(|required| !lib.contains(**required))
        .map(|required| format!("src/lib.rs: public API surface missing `{required}`"))
        .collect::<Vec<_>>();

    let catalog = katana_markdown_linter::rule_catalog();
    if !catalog.active_rules().any(|rule| rule.id == "MD001") {
        violations.push("rule catalog: missing MD001 active rule".to_string());
    }
    if !catalog.active_rules().any(|rule| rule.id == "MD060") {
        violations.push("rule catalog: missing MD060 active rule".to_string());
    }
    if !katana_markdown_linter::missing_rules().is_empty() {
        violations
            .push("public API: missing_rules must stay empty after full rule parity".to_string());
    }

    assert_no_violations("public-api-surface", violations);
}

#[test]
fn ast_linter_i18n_translation_coverage_is_complete_for_supported_locales() {
    let mut violations = Vec::new();
    for locale in katana_markdown_linter::supported_locales() {
        if katana_markdown_linter::i18n::catalog_keys(katana_markdown_linter::Locale::En)
            != katana_markdown_linter::i18n::catalog_keys(*locale)
        {
            violations.push(format!(
                "i18n: catalog key set differs for {}",
                locale.code()
            ));
        }
    }

    for rule in katana_markdown_linter::available_rules() {
        for locale in katana_markdown_linter::supported_locales() {
            if *locale == katana_markdown_linter::Locale::En {
                continue;
            }
            if !katana_markdown_linter::has_rule_description_translation(&rule.id, *locale) {
                violations.push(format!(
                    "i18n: missing {} rule description for {}",
                    locale.code(),
                    rule.id
                ));
            }
            let localized = rule.localized_description(locale.code());
            if localized == rule.description {
                violations.push(format!(
                    "i18n: {} rule description is an English copy for {}",
                    locale.code(),
                    rule.id
                ));
            }
        }
    }

    assert_no_violations("i18n-translation-coverage", violations);
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_workspace_file(path: &str) -> String {
    std::fs::read_to_string(workspace_root().join(path)).expect("workspace file should be readable")
}

fn rule_fixture_matrix() -> Value {
    serde_json::from_str(&read_workspace_file(
        "tests/fixtures/rule-fixture-matrix.json",
    ))
    .expect("fixture matrix should be valid json")
}

fn scan_rust_sources<F>(roots: &[PathBuf], lint: F) -> Vec<String>
where
    F: Fn(&Path, usize, &str) -> Option<String> + Send + Sync,
{
    let (tx, rx) = mpsc::channel();

    for root in roots {
        let walker = WalkBuilder::new(root)
            .hidden(false)
            .ignore(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .require_git(false)
            .build_parallel();

        walker.run(|| {
            let tx = tx.clone();
            let lint = &lint;
            Box::new(move |entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("rs")
                    {
                        let Ok(content) = std::fs::read_to_string(path) else {
                            return WalkState::Continue;
                        };
                        for (line_idx, line) in content.lines().enumerate() {
                            if let Some(violation) = lint(path, line_idx, line) {
                                let _ = tx.send(violation);
                            }
                        }
                    }
                }
                WalkState::Continue
            })
        });
    }

    drop(tx);
    let mut violations = rx.into_iter().collect::<Vec<_>>();
    violations.sort();
    violations
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
