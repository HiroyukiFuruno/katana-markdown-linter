use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn fix_preserves_literal_blocks_after_container_labels() {
    let directory = TestDir::new("fix-labeled-blocks");
    let config = directory.path().join(".markdownlint.json");
    fs::write(&config, r#"{"default": false, "MD018": true}"#).unwrap();
    for literal in [
        "[^note]: ```text\n    #Heading\n",
        "[^脚注]: <pre>\n    #Heading\n    </pre>\n",
        "Term\n: ~~~text\n  #Heading\n",
    ] {
        let input = format!("{literal}\n#Heading\n");
        let output = run_kml(
            ["fix", "--stdin", "--config", file_text(&config).as_str()],
            &input,
        );
        assert!(output.status.success(), "{input:?}");
        assert_eq!(output.stdout, format!("{literal}\n# Heading\n").as_bytes());
    }
}

#[test]
fn formatter_entrypoints_preserve_literal_block_content() {
    let directory = TestDir::new("fmt-fence-eof");
    let path = directory.path().join("doc.md");
    for (source, answer) in [
        ("```text\npayload\n\n\n", "```text\npayload\n\n\n"),
        ("```text\npayload", "```text\npayload"),
        (
            "[^note]: ```text\n    payload",
            "[^note]: ```text\n    payload",
        ),
        ("~~~text\r\npayload\r\n\r\n", "~~~text\npayload\n\n"),
        (
            "> ```text\n> payload\n>\n>\n",
            "> ```text\n> payload\n>\n>\n",
        ),
        ("- ```text\n  payload\n\n\n", "- ```text\n  payload\n\n\n"),
        ("```text\npayload\n```\n\n\n", "```text\npayload\n```\n"),
        ("Text\n\n\n", "Text\n"),
        (
            "<pre>\nliteral\n\n\nend\n</pre>\n\n\n",
            "<pre>\nliteral\n\n\nend\n</pre>\n",
        ),
        ("<pre>\nliteral\n\n\n", "<pre>\nliteral\n\n\n"),
        ("<pre>\nliteral", "<pre>\nliteral"),
        (
            "<pre>\n# Heading\n-  item\n| a | b |\n|---|---|\n| 1 | 2 |\n</pre>\n",
            "<pre>\n# Heading\n-  item\n| a | b |\n|---|---|\n| 1 | 2 |\n</pre>\n",
        ),
        ("", ""),
    ] {
        let library =
            katana_markdown_linter::MarkdownFormatter::format_markdown(source, &Default::default())
                .unwrap();
        assert_eq!(library.content, answer, "{source:?}");
        let stdin = run_kml(["fmt", "--stdin"], source);
        assert!(
            stdin.status.success(),
            "{}",
            String::from_utf8_lossy(&stdin.stderr)
        );
        assert_eq!(stdin.stdout, answer.as_bytes(), "{source:?}");
        fs::write(&path, source).unwrap();
        let file = run_kml(["fmt", &file_text(&path)], "");
        assert!(
            file.status.success(),
            "{}",
            String::from_utf8_lossy(&file.stderr)
        );
        assert_eq!(fs::read(&path).unwrap(), answer.as_bytes(), "{source:?}");
    }
}

#[test]
fn stdin_json_respects_config_validation_for_check_fix_and_fmt() {
    let dir = TestDir::new("stdin-json-config");
    let config = dir.path().join(".markdownlint.json");
    fs::write(&config, "{ \"MD999\": true }\n").expect("config should be written");

    for command in ["check", "fix", "fmt"] {
        let output = run_kml(
            [
                command,
                "--stdin",
                "--output",
                "json",
                "--config",
                file_text(&config).as_str(),
            ],
            "# Title\n",
        );

        assert_eq!(output.status.code(), Some(2));
        let json = stdout_json(&output);
        assert_eq!(json["command"], command);
        assert_eq!(json["errors"][0]["message_id"], "config.unknown_rule");
    }
}

fn run_kml<const N: usize>(args: [&str; N], stdin: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("kml should start");
    child
        .stdin
        .as_mut()
        .expect("stdin should be piped")
        .write_all(stdin.as_bytes())
        .expect("stdin should be written");
    child.wait_with_output().expect("kml should finish")
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

fn file_text(path: &Path) -> String {
    path.display().to_string()
}

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "katana-markdown-linter-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test dir should be created");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
