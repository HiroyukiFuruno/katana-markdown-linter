use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn public_markdown_docs_are_english_only() {
    let guard = PublicMarkdownLanguageGuard::new();

    assert_no_violations("public-markdown-docs-english", guard.violations());
}

struct PublicMarkdownLanguageGuard {
    files: Vec<PathBuf>,
}

impl PublicMarkdownLanguageGuard {
    fn new() -> Self {
        let mut files = vec![workspace_root().join("README.md")];
        files.extend(markdown_files(&workspace_root().join("docs")));
        files.sort();

        Self { files }
    }

    fn violations(&self) -> Vec<String> {
        self.files
            .iter()
            .flat_map(|path| self.file_violations(path))
            .collect()
    }

    fn file_violations(&self, path: &Path) -> Vec<String> {
        let content = fs::read_to_string(path).expect("public Markdown file should be readable");
        content
            .lines()
            .enumerate()
            .filter(|(_, line)| contains_japanese_text(line))
            .map(|(line_idx, line)| {
                format!(
                    "{}:{}: public docs must be English; found `{}`",
                    relative_path(path).display(),
                    line_idx + 1,
                    line
                )
            })
            .collect()
    }
}

fn markdown_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_markdown_files(root, &mut files);
    files
}

fn collect_markdown_files(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).expect("public docs directory should be readable") {
        let path = entry.expect("public docs entry should be readable").path();
        if path.is_dir() {
            collect_markdown_files(&path, files);
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            files.push(path);
        }
    }
}

fn contains_japanese_text(line: &str) -> bool {
    line.chars().any(|character| {
        matches!(
            character,
            '\u{3040}'..='\u{30ff}' | '\u{3400}'..='\u{9fff}' | '\u{f900}'..='\u{faff}'
        )
    })
}

fn relative_path(path: &Path) -> PathBuf {
    path.strip_prefix(workspace_root())
        .expect("path should be inside workspace")
        .to_path_buf()
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
