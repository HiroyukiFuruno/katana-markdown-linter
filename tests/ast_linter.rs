use ignore::{WalkBuilder, WalkState};
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
fn ast_linter_release_workflow_requires_existing_signed_tag() {
    let workflow = read_workspace_file(".github/workflows/release.yml");
    let required = [
        "Verify release tag exists",
        "git cat-file -t",
        "must be an annotated signed tag",
        "--verify-tag",
    ];
    let violations = required
        .iter()
        .filter(|required| !workflow.contains(**required))
        .map(|required| format!(".github/workflows/release.yml: missing `{required}`"))
        .collect();

    assert_no_violations("release-signed-tag", violations);
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

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_workspace_file(path: &str) -> String {
    std::fs::read_to_string(workspace_root().join(path)).expect("workspace file should be readable")
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
