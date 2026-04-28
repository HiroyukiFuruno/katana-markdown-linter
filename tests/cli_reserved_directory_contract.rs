use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn check_skips_reserved_directories_by_default() {
    let workspace = ReservedWorkspace::new("reserved-check-default");

    let output = run_kml([
        "check",
        "--output",
        "json",
        "--config",
        workspace.config_text().as_str(),
        workspace.root_text().as_str(),
    ]);

    assert_eq!(output.status.code(), Some(1));
    let report = stdout_json(&output);
    let paths = report_paths(&report);
    assert_eq!(paths, vec![workspace.docs_file_text()]);
}

#[test]
fn check_can_include_reserved_directories_with_opt_in() {
    let workspace = ReservedWorkspace::new("reserved-check-opt-in");

    let output = run_kml([
        "check",
        "--include-reserved",
        "--output",
        "json",
        "--config",
        workspace.config_text().as_str(),
        workspace.root_text().as_str(),
    ]);

    assert_eq!(output.status.code(), Some(1));
    let report = stdout_json(&output);
    let paths = report_paths(&report);
    assert_eq!(
        paths,
        vec![workspace.docs_file_text(), workspace.reserved_file_text()]
    );
}

#[test]
fn fix_does_not_rewrite_reserved_directories_by_default() {
    let workspace = ReservedWorkspace::new("reserved-fix-default");

    let output = run_kml([
        "fix",
        "--output",
        "json",
        "--config",
        workspace.config_text().as_str(),
        workspace.root_text().as_str(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(read_file(workspace.docs_file()), "# Title\n");
    assert_eq!(read_file(workspace.reserved_file()), "#Title\n");
}

#[test]
fn fmt_does_not_rewrite_reserved_directories_by_default() {
    let workspace = ReservedWorkspace::new("reserved-fmt-default");
    fs::write(workspace.docs_file(), "# Title\r\nText\n\n\n")
        .expect("docs file should be rewritten for fmt fixture");
    fs::write(workspace.reserved_file(), "# Title\r\nText\n\n\n")
        .expect("reserved file should be rewritten for fmt fixture");

    let output = run_kml(["fmt", "--output", "json", workspace.root_text().as_str()]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(read_file(workspace.docs_file()), "# Title\n\nText\n");
    assert_eq!(
        read_file(workspace.reserved_file()),
        "# Title\r\nText\n\n\n"
    );
}

fn run_kml<const N: usize>(args: [&str; N]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(args)
        .output()
        .expect("kml should run")
}

fn stdout_json(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout should be json: {error}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn report_paths(report: &Value) -> Vec<String> {
    report["files"]
        .as_array()
        .expect("files should be an array")
        .iter()
        .map(|file| {
            file["path"]
                .as_str()
                .expect("file path should be a string")
                .to_string()
        })
        .collect()
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("fixture should be readable")
}

struct ReservedWorkspace {
    root: PathBuf,
    docs_file: PathBuf,
    reserved_file: PathBuf,
    config: PathBuf,
}

impl ReservedWorkspace {
    fn new(name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "katana-markdown-linter-{name}-{}-{nanos}",
            std::process::id()
        ));
        let docs_dir = root.join("docs");
        let reserved_dir = root.join("node_modules").join("package");
        fs::create_dir_all(&docs_dir).expect("docs directory should be created");
        fs::create_dir_all(&reserved_dir).expect("reserved directory should be created");

        let docs_file = docs_dir.join("bad.md");
        let reserved_file = reserved_dir.join("bad.md");
        let config = root.join(".markdownlint.json");
        fs::write(&docs_file, "#Title\n").expect("docs file should be written");
        fs::write(&reserved_file, "#Title\n").expect("reserved file should be written");
        fs::write(&config, "{ \"default\": false, \"MD018\": true }\n")
            .expect("config should be written");

        Self {
            root,
            docs_file,
            reserved_file,
            config,
        }
    }

    fn root_text(&self) -> String {
        self.root.display().to_string()
    }

    fn docs_file(&self) -> &Path {
        &self.docs_file
    }

    fn docs_file_text(&self) -> String {
        self.docs_file.display().to_string()
    }

    fn reserved_file(&self) -> &Path {
        &self.reserved_file
    }

    fn reserved_file_text(&self) -> String {
        self.reserved_file.display().to_string()
    }

    fn config_text(&self) -> String {
        self.config.display().to_string()
    }
}

impl Drop for ReservedWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
