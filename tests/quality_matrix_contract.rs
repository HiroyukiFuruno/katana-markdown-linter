use katana_markdown_linter::{
    FormatOptions, MarkdownFormatter, MarkdownLintConfig, MarkdownLinter,
};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Deserialize)]
struct Matrix {
    cases: Vec<MatrixCase>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
enum MatrixMode {
    Formatter,
    StyleOnly,
    Fix,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum LineEnding {
    #[default]
    Lf,
    Crlf,
}

#[derive(Deserialize)]
struct MatrixCase {
    id: String,
    mode: MatrixMode,
    config: String,
    input: String,
    answer: String,
    #[serde(default)]
    remaining_diagnostics: BTreeMap<String, usize>,
    #[serde(default)]
    line_ending: LineEnding,
}

#[test]
fn quality_matrix_matches_independent_answers_across_library_file_and_stdin() {
    let root = workspace_root().join("tests/fixtures/quality-matrix");
    let matrix: Matrix = serde_json::from_slice(
        &fs::read(root.join("matrix.json")).expect("quality matrix should be readable"),
    )
    .expect("quality matrix should parse");

    let modes = matrix
        .cases
        .iter()
        .map(|case| case.mode)
        .collect::<HashSet<_>>();
    assert_eq!(
        modes,
        HashSet::from([
            MatrixMode::Formatter,
            MatrixMode::StyleOnly,
            MatrixMode::Fix
        ])
    );
    let ids = matrix
        .cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<HashSet<_>>();
    assert_eq!(ids.len(), matrix.cases.len(), "matrix IDs must be unique");
    for required_id in [
        "md007-indent-four",
        "md007-disabled",
        "enabled-true-md049",
        "escaped-unicode-crlf-nesting",
        "flanking-literals-unicode-crlf",
        "md050-styleunderscore-nested-triple",
        "md049-md050-styleunderscore-nested-triple",
        "reference-style-preservation",
    ] {
        assert!(
            ids.contains(required_id),
            "missing required matrix case: {required_id}"
        );
    }

    for case in matrix.cases {
        let input = fs::read(root.join(&case.input)).expect("matrix input should be readable");
        let expected = fs::read(root.join(&case.answer)).expect("matrix answer should be readable");
        for bytes in [&input, &expected] {
            match case.line_ending {
                LineEnding::Lf => assert!(!bytes.contains(&b'\r'), "{} must use LF", case.id),
                LineEnding::Crlf => {
                    assert!(bytes.windows(2).any(|pair| pair == b"\r\n"));
                    assert!(
                        bytes.iter().enumerate().all(|(index, byte)| {
                            *byte != b'\n' || (index > 0 && bytes[index - 1] == b'\r')
                        }),
                        "{} must use CRLF throughout",
                        case.id
                    );
                }
            }
        }
        let config_path = root.join(&case.config);
        let raw_config: Value = serde_json::from_slice(
            &fs::read(&config_path).expect("matrix config should be readable"),
        )
        .expect("matrix config should parse");
        let config = MarkdownLintConfig { raw: raw_config };
        let options = config.to_lint_options();

        let mut library_output = String::from_utf8(input.clone()).expect("input should be UTF-8");
        for pass in 1..=3 {
            library_output = MarkdownLinter::fix(&library_output, &options)
                .unwrap_or_else(|error| panic!("{} library pass {pass}: {error}", case.id))
                .content;
            assert_eq!(
                library_output.as_bytes(),
                expected,
                "{} library pass {pass} differs from independent answer",
                case.id
            );
        }
        let final_diagnostics = MarkdownLinter::lint(&library_output, &options)
            .unwrap_or_else(|error| panic!("{} final library lint: {error}", case.id));
        let actual_counts = final_diagnostics.iter().fold(
            BTreeMap::<String, usize>::new(),
            |mut counts, diagnostic| {
                *counts.entry(diagnostic.rule_id.clone()).or_default() += 1;
                assert_ne!(
                    diagnostic.fix.as_ref().map(|fix| fix.safety),
                    Some(katana_markdown_linter::FixSafety::Safe),
                    "{} must not leave safe fixes after convergence",
                    case.id
                );
                counts
            },
        );
        assert_eq!(
            actual_counts, case.remaining_diagnostics,
            "{} final diagnostics differ from the independent matrix expectation",
            case.id
        );
        let expected_exit = if case.remaining_diagnostics.is_empty() {
            0
        } else {
            1
        };

        let temp = TestDir::new(&case.id);
        let file = temp.path().join("sample.md");
        fs::write(&file, &input).expect("file input should be written");
        let mut stdin_input = input.clone();
        for pass in 1..=3 {
            let output = run_kml(&[
                "check",
                "--fix",
                "--config",
                path(&config_path),
                "--output",
                "json",
                path(&file),
            ]);
            assert_eq!(
                output.status.code(),
                Some(expected_exit),
                "{} file pass {pass}: {}",
                case.id,
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(
                fs::read(&file).expect("fixed file should be readable"),
                expected,
                "{} file pass {pass} differs from independent answer",
                case.id
            );

            let stdin_output = run_kml_stdin(
                &["check", "--fix", "--stdin", "--config", path(&config_path)],
                &stdin_input,
            );
            assert_eq!(
                stdin_output.status.code(),
                Some(expected_exit),
                "{} stdin pass {pass}: {}",
                case.id,
                String::from_utf8_lossy(&stdin_output.stderr)
            );
            assert_eq!(
                stdin_output.stdout.as_slice(),
                expected.as_slice(),
                "{} stdin pass {pass} differs from independent answer",
                case.id
            );
            stdin_input = stdin_output.stdout;
        }

        if case.mode == MatrixMode::Formatter {
            let layout_options = MarkdownFormatter::layout_lint_options_from(&options);
            let mut formatted = String::from_utf8(input.clone()).expect("input should be UTF-8");
            for pass in 1..=3 {
                formatted = MarkdownFormatter::format_markdown_with_lint_options(
                    &formatted,
                    &FormatOptions::default(),
                    &layout_options,
                )
                .unwrap_or_else(|error| panic!("{} library format pass {pass}: {error}", case.id))
                .content;
                assert_eq!(
                    formatted.as_bytes(),
                    expected,
                    "{} library format pass {pass} differs from independent answer",
                    case.id
                );
            }

            fs::write(&file, &input).expect("formatter file input should be reset");
            let mut stdin_formatted = input.clone();
            for pass in 1..=3 {
                let output = run_kml(&["fmt", "--config", path(&config_path), path(&file)]);
                assert_eq!(
                    output.status.code(),
                    Some(0),
                    "{} fmt file pass {pass}: {}",
                    case.id,
                    String::from_utf8_lossy(&output.stderr)
                );
                assert_eq!(
                    fs::read(&file).expect("formatted file should be readable"),
                    expected,
                    "{} fmt file pass {pass} differs from independent answer",
                    case.id
                );
                let stdin_output = run_kml_stdin(
                    &["fmt", "--stdin", "--config", path(&config_path)],
                    &stdin_formatted,
                );
                assert_eq!(
                    stdin_output.status.code(),
                    Some(0),
                    "{} fmt stdin pass {pass}: {}",
                    case.id,
                    String::from_utf8_lossy(&stdin_output.stderr)
                );
                assert_eq!(
                    stdin_output.stdout.as_slice(),
                    expected.as_slice(),
                    "{} fmt stdin pass {pass} differs from independent answer",
                    case.id
                );
                stdin_formatted = stdin_output.stdout;
            }
        }

        if case.mode == MatrixMode::StyleOnly {
            assert_ne!(
                input, expected,
                "{} style-only fixture must exercise an actual marker change",
                case.id
            );
            assert_eq!(
                parsed_events(&String::from_utf8(input).unwrap()),
                parsed_events(&String::from_utf8(expected).unwrap()),
                "{} style-only fix changed parsed Markdown meaning",
                case.id
            );
        }
    }
}

fn parsed_events(source: &str) -> Vec<String> {
    pulldown_cmark::Parser::new(source)
        .map(|event| format!("{event:?}"))
        .collect()
}

fn run_kml(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_kml"))
        .args(args)
        .output()
        .expect("kml should start")
}

fn run_kml_stdin(args: &[&str], content: &[u8]) -> std::process::Output {
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
        .write_all(content)
        .expect("stdin should be written");
    child.wait_with_output().expect("kml should finish")
}

fn path(path: &Path) -> &str {
    path.to_str().expect("temporary path should be UTF-8")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

struct TestDir(PathBuf);

impl TestDir {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("kml-quality-matrix-{}-{name}", std::process::id()));
        fs::create_dir_all(&path).expect("temporary test directory should be created");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
