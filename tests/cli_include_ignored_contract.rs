use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn explicit_ignored_directory_can_be_included_for_fixing() {
    let workspace = IgnoredWorkspace::new();

    let default_output = run_kml([
        "check",
        "--output",
        "json",
        "--config",
        workspace.config_text().as_str(),
        workspace.root_text().as_str(),
    ]);
    assert_eq!(
        report_paths(&stdout_json(&default_output)),
        vec![workspace.docs_text()]
    );

    let fix_output = run_kml([
        "fix",
        "--include-ignored",
        "--output",
        "json",
        "--config",
        workspace.config_text().as_str(),
        workspace.ignored_dir_text().as_str(),
    ]);
    assert!(fix_output.status.success());
    assert_eq!(read_file(&workspace.ignored_file), "# Title\n");
    assert_eq!(read_file(&workspace.docs_file), "#Title\n");
}

fn run_kml<const N: usize>(args: [&str; N]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(args)
        .output()
        .expect("kml should run")
}

fn stdout_json(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("stdout should be json")
}

fn report_paths(report: &Value) -> Vec<String> {
    report["files"]
        .as_array()
        .expect("files should be an array")
        .iter()
        .map(|file| {
            file["path"]
                .as_str()
                .expect("path should be string")
                .to_string()
        })
        .collect()
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("fixture should be readable")
}

struct IgnoredWorkspace {
    root: PathBuf,
    ignored_dir: PathBuf,
    ignored_file: PathBuf,
    docs_file: PathBuf,
    config: PathBuf,
}

impl IgnoredWorkspace {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "katana-markdown-linter-ignored-dir-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time should be available")
                .as_nanos()
        ));
        let ignored_dir = root.join(".agents");
        let ignored_file = ignored_dir.join("bad.md");
        let docs_file = root.join("docs/bad.md");
        let config = root.join(".markdownlint.json");
        create_bad_markdown(&ignored_file);
        create_bad_markdown(&docs_file);
        fs::write(root.join(".gitignore"), ".agents/\n").expect("gitignore should be written");
        fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
            .expect("config should be written");
        Self {
            root,
            ignored_dir,
            ignored_file,
            docs_file,
            config,
        }
    }

    fn root_text(&self) -> String {
        self.root.display().to_string()
    }

    fn ignored_dir_text(&self) -> String {
        self.ignored_dir.display().to_string()
    }

    fn docs_text(&self) -> String {
        self.docs_file.display().to_string()
    }

    fn config_text(&self) -> String {
        self.config.display().to_string()
    }
}

impl Drop for IgnoredWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn create_bad_markdown(path: &Path) {
    fs::create_dir_all(path.parent().expect("path should have parent"))
        .expect("directory should be created");
    fs::write(path, "#Title\n").expect("fixture should be written");
}
