use std::path::{Path, PathBuf};

#[test]
fn agent_instructions_use_repository_local_openspec_launcher() {
    let guard = OpenspecCommandPortabilityGuard::new();

    assert_no_violations("openspec-command-portability", guard.violations());
}

struct OpenspecCommandPortabilityGuard {
    workspace_root: PathBuf,
}

impl OpenspecCommandPortabilityGuard {
    fn new() -> Self {
        Self {
            workspace_root: workspace_root(),
        }
    }

    fn violations(&self) -> Vec<String> {
        let mut violations = Vec::new();
        self.require_launcher(&mut violations);
        self.require_agent_docs_to_use_launcher(&mut violations);
        violations
    }

    fn require_launcher(&self, violations: &mut Vec<String>) {
        let launcher = self.workspace_root.join("scripts/openspec");
        if !launcher.is_file() {
            violations.push("scripts/openspec: missing repository-local launcher".to_string());
            return;
        }

        let content = read_file(&launcher);
        for required in ["@fission-ai/openspec@1.3.1", "nvm.sh", "KML_OPENSPEC_BIN"] {
            if !content.contains(required) {
                violations.push(format!("scripts/openspec: missing `{required}`"));
            }
        }
    }

    fn require_agent_docs_to_use_launcher(&self, violations: &mut Vec<String>) {
        for path in self.agent_instruction_paths() {
            let content = read_file(&path);
            for forbidden in [
                "openspec --version",
                "openspec archive",
                "openspec instructions",
                "openspec list",
                "openspec new",
                "openspec schemas",
                "openspec status",
            ] {
                if contains_bare_command(&content, forbidden) {
                    violations.push(format!(
                        "{}: replace `{forbidden}` with `scripts/openspec ...`",
                        relative_path(&self.workspace_root, &path)
                    ));
                }
            }
        }
    }

    fn agent_instruction_paths(&self) -> Vec<PathBuf> {
        [
            ".agent",
            ".claude",
            ".codex",
            ".github/prompts",
            ".github/skills",
            ".opencode",
        ]
        .iter()
        .flat_map(|path| markdown_files_under(&self.workspace_root.join(path)))
        .collect()
    }
}

fn contains_bare_command(content: &str, command: &str) -> bool {
    content
        .match_indices(command)
        .any(|(index, _)| match content[..index].chars().next_back() {
            Some('/') => false,
            Some(character) => !character.is_ascii_alphanumeric() && character != '_',
            None => true,
        })
}

fn markdown_files_under(root: &Path) -> Vec<PathBuf> {
    if !root.is_dir() {
        return Vec::new();
    }

    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        let entries = path
            .read_dir()
            .unwrap_or_else(|_| panic!("{} should be readable", path.display()));
        for entry in entries {
            let child = entry.expect("directory entry should be readable").path();
            if child.is_dir() {
                pending.push(child);
                continue;
            }

            if child.extension().is_some_and(|extension| extension == "md") {
                files.push(child);
            }
        }
    }
    files
}

fn read_file(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("{} should be readable", path.display()))
}

fn relative_path(workspace_root: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_root)
        .expect("path should be under workspace root")
        .display()
        .to_string()
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
