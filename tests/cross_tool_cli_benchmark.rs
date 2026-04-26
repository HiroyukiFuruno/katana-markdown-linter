use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "kml-cross-tool-test-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory should be created");
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

fn script_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/bench/cross-tool-cli-benchmark.py")
}

fn write_corpus(root: &Path) -> (PathBuf, PathBuf) {
    let clean = root.join("clean");
    let dirty = root.join("dirty");
    fs::create_dir_all(&clean).expect("clean corpus should be created");
    fs::create_dir_all(&dirty).expect("dirty corpus should be created");
    fs::write(clean.join("doc.md"), "# Clean\n\nText.\n")
        .expect("clean document should be written");
    fs::write(dirty.join("doc.md"), "#Dirty\n").expect("dirty document should be written");
    (clean, dirty)
}

fn mock_kml_path(root: &Path) -> PathBuf {
    root.join(mock_kml_filename())
}

#[cfg(windows)]
fn mock_kml_filename() -> &'static str {
    "kml-mock.cmd"
}

#[cfg(not(windows))]
fn mock_kml_filename() -> &'static str {
    "kml-mock"
}

#[cfg(not(windows))]
fn write_mock_kml(path: &Path, clean_fails: bool) {
    let clean_exit = if clean_fails { 1 } else { 0 };
    let script = format!(
        r#"#!/bin/sh
if [ "${{1:-}}" = "--version" ]; then
  echo "kml-mock 1.0.0"
  exit 0
fi

fix=0
target=""
for arg in "$@"; do
  if [ "$arg" = "--fix" ]; then
    fix=1
  elif [ "$arg" = "check" ]; then
    true
  elif [ "${{arg#--}}" != "$arg" ]; then
    true
  else
    target="$arg"
  fi
done

if [ "$fix" = "1" ]; then
  if [ -f "$target/doc.md" ]; then
    printf '\nfixed\n' >> "$target/doc.md"
  fi
  exit 1
fi

case "$target" in
  *clean*) exit {clean_exit} ;;
  *dirty*) exit 1 ;;
  *) exit 0 ;;
esac
"#
    );
    fs::write(path, script).expect("mock kml should be written");
    make_executable(path);
}

#[cfg(windows)]
fn write_mock_kml(path: &Path, clean_fails: bool) {
    let clean_exit = if clean_fails { 1 } else { 0 };
    let script = format!(
        r#"@echo off
setlocal EnableExtensions
if "%~1"=="--version" (
  echo kml-mock 1.0.0
  exit /b 0
)

set "fix=0"
set "target="
:parse
if "%~1"=="" goto done_parse
if "%~1"=="--fix" (
  set "fix=1"
  shift
  goto parse
)
if "%~1"=="check" (
  shift
  goto parse
)
set "arg=%~1"
if "%arg:~0,2%"=="--" (
  shift
  goto parse
)
set "target=%~1"
shift
goto parse

:done_parse
if "%fix%"=="1" (
  if exist "%target%\doc.md" (
    echo.>>"%target%\doc.md"
    echo fixed>>"%target%\doc.md"
  )
  exit /b 1
)

echo %target% | findstr /I "clean" >nul
if %ERRORLEVEL%==0 exit /b {clean_exit}
echo %target% | findstr /I "dirty" >nul
if %ERRORLEVEL%==0 exit /b 1
exit /b 0
"#
    );
    fs::write(path, script).expect("mock kml should be written");
    make_executable(path);
}

#[cfg(not(windows))]
fn write_content_aware_mock_kml(path: &Path) {
    let script = r#"#!/bin/sh
if [ "${1:-}" = "--version" ]; then
  echo "kml-mock 1.0.0"
  exit 0
fi

fix=0
target=""
skip=0
for arg in "$@"; do
  if [ "$skip" = "1" ]; then
    skip=0
    continue
  fi
  case "$arg" in
    --fix) fix=1 ;;
    check) true ;;
    --output|--config) skip=1 ;;
    --*) true ;;
    *) target="$arg" ;;
  esac
done

if [ "$fix" = "1" ]; then
  printf '\nfixed\n' >> "$target/doc.md"
  exit 0
fi

if grep -q "fixed" "$target/doc.md"; then
  exit 0
fi
exit 1
"#;
    fs::write(path, script).expect("content-aware mock should be written");
    make_executable(path);
}

#[cfg(windows)]
fn write_content_aware_mock_kml(path: &Path) {
    let script = r#"@echo off
setlocal EnableExtensions
if "%~1"=="--version" (
  echo kml-mock 1.0.0
  exit /b 0
)

set "fix=0"
set "target="
set "skip=0"
:parse
if "%~1"=="" goto done_parse
if "%skip%"=="1" (
  set "skip=0"
  shift
  goto parse
)
if "%~1"=="--fix" (
  set "fix=1"
  shift
  goto parse
)
if "%~1"=="check" (
  shift
  goto parse
)
if "%~1"=="--output" (
  set "skip=1"
  shift
  goto parse
)
if "%~1"=="--config" (
  set "skip=1"
  shift
  goto parse
)
set "arg=%~1"
if "%arg:~0,2%"=="--" (
  shift
  goto parse
)
set "target=%~1"
shift
goto parse

:done_parse
if "%fix%"=="1" (
  echo.>>"%target%\doc.md"
  echo fixed>>"%target%\doc.md"
  exit /b 0
)

findstr /C:"fixed" "%target%\doc.md" >nul
if %ERRORLEVEL%==0 exit /b 0
exit /b 1
"#;
    fs::write(path, script).expect("content-aware mock should be written");
    make_executable(path);
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    let mut permissions = fs::metadata(path)
        .expect("mock file metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("mock file should be executable");
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}

fn run_python(args: &[String]) -> std::process::Output {
    Command::new("python3")
        .args(args)
        .output()
        .expect("python3 should execute benchmark script")
}

#[test]
fn missing_optional_tools_are_reported_as_skipped_cases() {
    let dir = TestDir::new("missing-tools");
    let (clean, dirty) = write_corpus(dir.path());
    let mock = mock_kml_path(dir.path());
    write_mock_kml(&mock, false);
    let output = dir.path().join("report.json");
    let summary = dir.path().join("summary.md");

    let result = run_python(&[
        script_path().display().to_string(),
        "--no-hyperfine".to_string(),
        "--runs".to_string(),
        "1".to_string(),
        "--warmup".to_string(),
        "0".to_string(),
        "--tools".to_string(),
        "kml,mado,rumdl".to_string(),
        "--kml".to_string(),
        mock.display().to_string(),
        "--mado".to_string(),
        dir.path().join("missing-mado").display().to_string(),
        "--rumdl".to_string(),
        dir.path().join("missing-rumdl").display().to_string(),
        "--mode".to_string(),
        "default".to_string(),
        "--workflow".to_string(),
        "check".to_string(),
        "--clean-corpus".to_string(),
        clean.display().to_string(),
        "--dirty-corpus".to_string(),
        dirty.display().to_string(),
        "--output".to_string(),
        output.display().to_string(),
        "--summary".to_string(),
        summary.display().to_string(),
    ]);
    assert!(
        result.status.success(),
        "benchmark should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );

    let report: Value =
        serde_json::from_str(&fs::read_to_string(output).expect("report should be readable"))
            .expect("report should be json");
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["timing_method"], "fallback");
    assert_eq!(report["runs"], 1);
    assert_eq!(report["warmup"], 0);
    assert!(fs::read_to_string(summary)
        .expect("summary should be readable")
        .contains("Cross-Tool CLI Benchmark"));

    let cases = report["cases"]
        .as_array()
        .expect("cases should be an array");
    assert!(cases.iter().any(|case| {
        case["tool"] == "kml"
            && case["status"] == "measured"
            && case["tool_version"] == "kml-mock 1.0.0"
            && case["mode"] == "default"
            && case["workflow"] == "check"
            && case["timing_method"] == "fallback"
    }));
    assert!(cases.iter().any(|case| {
        case["tool"] == "mado"
            && case["status"] == "skipped"
            && case["skip_reason"]
                .as_str()
                .expect("skip reason should be a string")
                .contains("binary was not found")
    }));
    assert!(cases.iter().any(|case| {
        case["tool"] == "rumdl"
            && case["status"] == "skipped"
            && case["skip_reason"]
                .as_str()
                .expect("skip reason should be a string")
                .contains("binary was not found")
    }));
}

#[test]
fn dirty_check_violation_exit_code_is_normalized() {
    let dir = TestDir::new("dirty-normalized");
    let (_clean, dirty) = write_corpus(dir.path());
    let mock = mock_kml_path(dir.path());
    write_mock_kml(&mock, false);

    let result = run_python(&[
        script_path().display().to_string(),
        "run-case".to_string(),
        "--tool".to_string(),
        "kml".to_string(),
        "--binary".to_string(),
        mock.display().to_string(),
        "--mode".to_string(),
        "default".to_string(),
        "--workflow".to_string(),
        "check".to_string(),
        "--corpus-kind".to_string(),
        "dirty".to_string(),
        "--corpus".to_string(),
        dirty.display().to_string(),
    ]);
    assert!(
        result.status.success(),
        "dirty violation exit should be normalized\nstderr:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn clean_check_non_zero_exit_is_a_failure() {
    let dir = TestDir::new("clean-failure");
    let (clean, _dirty) = write_corpus(dir.path());
    let mock = mock_kml_path(dir.path());
    write_mock_kml(&mock, true);

    let result = run_python(&[
        script_path().display().to_string(),
        "run-case".to_string(),
        "--tool".to_string(),
        "kml".to_string(),
        "--binary".to_string(),
        mock.display().to_string(),
        "--mode".to_string(),
        "default".to_string(),
        "--workflow".to_string(),
        "check".to_string(),
        "--corpus-kind".to_string(),
        "clean".to_string(),
        "--corpus".to_string(),
        clean.display().to_string(),
    ]);
    assert!(!result.status.success(), "clean non-zero exit should fail");
}

#[test]
fn fix_workflow_uses_temporary_workspace_copy() {
    let dir = TestDir::new("fix-copy");
    let (_clean, dirty) = write_corpus(dir.path());
    let source = dirty.join("doc.md");
    let before = fs::read_to_string(&source).expect("source should be readable");
    let mock = mock_kml_path(dir.path());
    write_mock_kml(&mock, false);

    let result = run_python(&[
        script_path().display().to_string(),
        "run-case".to_string(),
        "--tool".to_string(),
        "kml".to_string(),
        "--binary".to_string(),
        mock.display().to_string(),
        "--mode".to_string(),
        "default".to_string(),
        "--workflow".to_string(),
        "fix".to_string(),
        "--corpus-kind".to_string(),
        "dirty".to_string(),
        "--corpus".to_string(),
        dirty.display().to_string(),
    ]);
    assert!(
        result.status.success(),
        "fix violation exit should be normalized\nstderr:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let after = fs::read_to_string(source).expect("source should be readable after fix case");
    assert_eq!(after, before, "fix benchmark must not mutate source corpus");
}

#[test]
fn fix_workflow_reports_post_fix_validation() {
    let dir = TestDir::new("fix-validation");
    let (clean, dirty) = write_corpus(dir.path());
    let source = dirty.join("doc.md");
    let before = fs::read_to_string(&source).expect("source should be readable");
    let mock = mock_kml_path(dir.path());
    write_content_aware_mock_kml(&mock);
    let output = dir.path().join("report.json");
    let summary = dir.path().join("summary.md");

    let result = run_python(&[
        script_path().display().to_string(),
        "--no-hyperfine".to_string(),
        "--runs".to_string(),
        "1".to_string(),
        "--warmup".to_string(),
        "0".to_string(),
        "--tools".to_string(),
        "kml".to_string(),
        "--kml".to_string(),
        mock.display().to_string(),
        "--mode".to_string(),
        "default".to_string(),
        "--workflow".to_string(),
        "fix".to_string(),
        "--clean-corpus".to_string(),
        clean.display().to_string(),
        "--dirty-corpus".to_string(),
        dirty.display().to_string(),
        "--output".to_string(),
        output.display().to_string(),
        "--summary".to_string(),
        summary.display().to_string(),
    ]);
    assert!(
        result.status.success(),
        "benchmark should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );

    let report: Value =
        serde_json::from_str(&fs::read_to_string(output).expect("report should be readable"))
            .expect("report should be json");
    let case = &report["cases"]
        .as_array()
        .expect("cases should be an array")[0];
    assert_eq!(case["fix_validation"]["status"], "passed");
    assert_eq!(case["fix_validation"]["before_check_exit_code"], 1);
    assert_eq!(case["fix_validation"]["after_check_exit_code"], 0);
    assert_eq!(case["fix_validation"]["source_changed"], false);
    assert!(fs::read_to_string(summary)
        .expect("summary should be readable")
        .contains("fix validation: passed"));
    let after = fs::read_to_string(source).expect("source should be readable after benchmark");
    assert_eq!(
        after, before,
        "benchmark validation must not mutate source corpus"
    );
}
