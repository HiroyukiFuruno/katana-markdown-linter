use std::fs;
use std::path::Path;

#[test]
fn public_fix_entrypoints_delegate_convergence_to_one_engine() {
    for source in [
        include_str!("../src/linter/core.rs"),
        include_str!("../src/cli/workflow/common.rs"),
        include_str!("../src/formatter.rs"),
    ] {
        assert!(source.contains("FixEngine::run("));
        assert!(!source.contains("for _ in 0.."));
    }
}

#[test]
fn code_policy_is_shared() {
    for source in [
        include_str!("../src/rules/markdown/rules/md010.rs"),
        include_str!("../src/rules/markdown/rules/md034.rs"),
    ] {
        assert!(source.contains("ctx.is_inside_inline_code("));
        assert!(!source.contains("in_code ="));
    }
    let reversed_links = include_str!("../src/rules/markdown/rules/md011.rs");
    assert!(reversed_links.contains("ctx.overlaps_inline_code("));
    assert!(!reversed_links.contains("in_code ="));
}

#[test]
fn markdown_backend_is_confined_to_the_source_adapter() {
    fn inspect(path: &Path) {
        for entry in fs::read_dir(path).expect("source directory should exist") {
            let path = entry.expect("source entry should exist").path();
            if path.is_dir() {
                inspect(&path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let source = fs::read_to_string(&path).expect("source should be readable");
                if source.contains("pulldown_cmark::") {
                    assert!(
                        path.ends_with("rules/markdown/document/syntax.rs"),
                        "{} bypasses shared adapter",
                        path.display()
                    );
                }
            }
        }
    }
    inspect(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"));
}

#[test]
fn context_migrations_do_not_reparse_during_shared_evaluation() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/rules/markdown/rules");
    for name in [
        "md046.rs",
        "heading_style.rs",
        "md019.rs",
        "md018.rs",
        "md021.rs",
        "md020.rs",
        "heading_duplicates.rs",
        "spaces_in_emphasis.rs",
        "list_spacing.rs",
        "list_indent.rs",
        "md005.rs",
        "blockquote.rs",
        "whitespace/multiple_blanks.rs",
        "whitespace/blockquote.rs",
        "list/ul_style.rs",
    ] {
        let source = fs::read_to_string(root.join(name)).expect("migrated source should exist");
        let shared = source
            .split("fn evaluate_context(")
            .nth(1)
            .expect("rule must accept shared context");
        let implementation = shared.split("#[cfg(test)]").next().unwrap_or(shared);
        assert!(
            !implementation.contains("DocumentContext::new("),
            "{name} reparses shared evaluation"
        );
    }
}
