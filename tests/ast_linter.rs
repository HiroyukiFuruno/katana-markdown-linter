use ignore::{WalkBuilder, WalkState};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

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
    let violations = scan_rust_sources(&[workspace_root().join("src")], |path, line_idx, line| {
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
    });

    assert_no_violations("lazy-macros", violations);
}

#[test]
fn ast_linter_cli_directory_walk_uses_parallel_ignore_walker() {
    let cli = read_workspace_file("src/cli.rs");
    let required = [
        "use ignore::{WalkBuilder, WalkState};",
        "WalkBuilder::new(dir)",
        ".build_parallel()",
        ".require_git(false)",
    ];
    let violations = required
        .iter()
        .filter(|required| !cli.contains(**required))
        .map(|required| format!("src/cli.rs: missing `{required}`"))
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
    let upstream = read_workspace_file("src/upstream.rs");
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
        ("src/upstream.rs", &upstream, "assert_no_unknown_drift"),
        (
            "src/upstream.rs",
            &upstream,
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
    let published_verifier = read_workspace_file("scripts/release/verify-release-published.sh");
    let required = [
        ("Makefile", &makefile, "release-check: fmt-check lint ast-lint release-test dogfood coverage-blocking examples mcp-build"),
        ("Makefile", &makefile, "release-test:"),
        ("Makefile", &makefile, "cargo test --all-features --locked"),
        ("Makefile", &makefile, "scripts/release/assert-crate-not-published.sh"),
        ("Makefile", &makefile, "release-verify:"),
        ("Makefile", &makefile, "scripts/release/verify-release-published.sh"),
        (".github/workflows/release.yml", &workflow, "run: make lint"),
        (".github/workflows/release.yml", &workflow, "run: make examples"),
        (".github/workflows/release.yml", &workflow, "run: make mcp-build"),
        (".github/workflows/release.yml", &workflow, "scripts/release/assert-crate-not-published.sh"),
        (".github/workflows/release.yml", &workflow, "--title \"$TAG\""),
        (".github/workflows/release-preflight.yml", &preflight, "run: make lint"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make examples"),
        (".github/workflows/release-preflight.yml", &preflight, "run: make mcp-build"),
        ("docs/release-runbook.md", &runbook, "make release-verify VERSION=vX.Y.Z"),
        ("docs/release-runbook.md", &runbook, "Do not rewrite a tag unless no GitHub Release exists"),
        ("docs/quality-gates.md", &quality, "release retry helpers must refuse remote tag overwrites"),
        ("docs/quality-gates.md", &quality, "already exists on crates.io"),
        ("scripts/release/release-notes.sh", &release_notes, "CHANGELOG.md is missing a non-empty section"),
        ("scripts/release/assert-crate-not-published.sh", &crate_guard, "Bump Cargo.toml before dispatching"),
        ("scripts/release/verify-release-published.sh", &published_verifier, "GitHub Release title mismatch"),
        ("scripts/release/verify-release-published.sh", &published_verifier, "github_release_title="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "github_release_target="),
        ("scripts/release/verify-release-published.sh", &published_verifier, "crates_io_version="),
    ];
    let violations = required
        .iter()
        .filter(|(_, content, required)| !content.contains(*required))
        .map(|(path, _, required)| format!("{path}: missing `{required}`"))
        .collect();

    assert_no_violations("release-local-ci-parity-and-retry-safety", violations);
}

#[test]
fn ast_linter_readme_rule_map_matches_public_catalog() {
    let readme = read_workspace_file("README.md");
    let mut violations = Vec::new();

    if !readme.contains("## Rule Map") {
        violations.push("README.md: missing `## Rule Map`".to_string());
    }

    for rule in katana_markdown_linter::available_rules() {
        let safe_fix = if rule.fixable { "yes" } else { "no" };
        let row = format!("| `{}` | {} |", rule.id, safe_fix);
        if !readme.contains(&row) {
            violations.push(format!("README.md: missing rule map row `{row}`"));
        }
    }

    assert_no_violations("readme-rule-map", violations);
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
        "pub fn available_rules() -> Vec<RuleMeta>",
        "pub fn implemented_rules() -> Vec<RuleMeta>",
        "pub fn missing_rules() -> Vec<RuleMeta>",
        "pub fn rule_catalog() -> catalog::RuleCatalog",
        "pub use config::{ConfigError, ConfigErrorKind, MarkdownLintConfig};",
        "pub use i18n::{",
        "localized_rule_description",
        "resolve_locale_code",
        "resolve_locale_code_or",
        "Locale, LocaleError",
        "LocalizedDiagnostic",
        "pub use types::{Fix, FixResult, LintOptions, LintResult, Range, RuleConfig, RuleMeta, Severity};",
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
