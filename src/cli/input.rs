use super::args::Cli;
use glob::{glob, Pattern};
use ignore::{WalkBuilder, WalkState};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InputExpandError {
    Filesystem(String),
    Glob(String),
}

impl From<String> for InputExpandError {
    fn from(value: String) -> Self {
        Self::Filesystem(value)
    }
}

pub(crate) fn expand_inputs(cli: &Cli) -> Result<Vec<PathBuf>, InputExpandError> {
    let inputs = &cli.inputs;
    if inputs.is_empty() {
        return filter_paths(
            markdown_files_in_dir(
                &env::current_dir().map_err(|err| InputExpandError::Filesystem(err.to_string()))?,
                cli.respect_gitignore,
                cli.include_reserved,
            )
            .map_err(InputExpandError::Filesystem)?,
            cli,
            false,
        );
    }

    let mut paths = Vec::new();
    for input in inputs {
        if has_glob_chars(input) {
            for entry in glob(input).map_err(|err| InputExpandError::Glob(err.to_string()))? {
                match entry {
                    Ok(path) => paths.push(path),
                    Err(err) => return Err(InputExpandError::Glob(err.to_string())),
                }
            }
        } else {
            paths.push(PathBuf::from(input));
        }
    }

    let mut expanded = Vec::new();
    for path in paths {
        if path.is_dir() {
            let respect_gitignore = cli.respect_gitignore && !cli.include_ignored;
            expanded.extend(
                markdown_files_in_dir(&path, respect_gitignore, cli.include_reserved)
                    .map_err(InputExpandError::Filesystem)?,
            );
        } else {
            expanded.push(path);
        }
    }
    expanded.sort();
    expanded.dedup();
    filter_paths(expanded, cli, true)
}

fn has_glob_chars(input: &str) -> bool {
    input.contains('*') || input.contains('?') || input.contains('[')
}

fn markdown_files_in_dir(
    dir: &Path,
    respect_gitignore: bool,
    include_reserved: bool,
) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect_markdown_files(dir, &mut paths, respect_gitignore, include_reserved)?;
    paths.sort();
    Ok(paths)
}

fn collect_markdown_files(
    dir: &Path,
    paths: &mut Vec<PathBuf>,
    respect_gitignore: bool,
    include_reserved: bool,
) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let mut builder = WalkBuilder::new(dir);
    builder
        .hidden(false)
        .parents(true)
        .ignore(respect_gitignore)
        .git_ignore(respect_gitignore)
        .git_global(respect_gitignore)
        .git_exclude(respect_gitignore)
        .require_git(false);
    if !include_reserved {
        builder.filter_entry(|entry| !is_reserved_directory(entry.path()));
    }
    let walker = builder.build_parallel();

    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |entry| {
            match entry {
                Ok(entry) => {
                    let path = entry.into_path();
                    if path.is_file() && is_markdown_file(&path) {
                        let _ = tx.send(Ok(path));
                    }
                }
                Err(err) => {
                    let _ = tx.send(Err(err.to_string()));
                }
            }
            WalkState::Continue
        })
    });
    drop(tx);

    for result in rx {
        match result {
            Ok(path) => paths.push(path),
            Err(err) => return Err(format!("{}: {err}", dir.display())),
        }
    }
    Ok(())
}

fn filter_paths(
    paths: Vec<PathBuf>,
    cli: &Cli,
    explicit: bool,
) -> Result<Vec<PathBuf>, InputExpandError> {
    let includes = compile_patterns(&cli.include)?;
    let excludes = compile_patterns(&cli.exclude)?;
    Ok(paths
        .into_iter()
        .filter(|path| {
            let text = path.to_string_lossy();
            let included =
                includes.is_empty() || includes.iter().any(|pattern| pattern.matches(&text));
            let excluded = excludes.iter().any(|pattern| pattern.matches(&text));
            included && (!excluded || (explicit && !cli.force_exclude))
        })
        .collect())
}

fn compile_patterns(patterns: &[String]) -> Result<Vec<Pattern>, InputExpandError> {
    patterns
        .iter()
        .map(|pattern| Pattern::new(pattern).map_err(|err| InputExpandError::Glob(err.to_string())))
        .collect()
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            let extension = extension.to_ascii_lowercase();
            extension == "md" || extension == "markdown"
        })
        .unwrap_or(false)
}

fn is_reserved_directory(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(is_reserved_directory_name)
}

fn is_reserved_directory_name(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | ".hg"
            | ".svn"
            | ".cache"
            | ".next"
            | ".turbo"
            | "node_modules"
            | "bower_components"
            | "target"
            | "dist"
            | "build"
            | "coverage"
            | "out"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_dir(name: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        env::temp_dir().join(format!(
            "katana-markdown-linter-cli-{name}-{}-{nanos}",
            std::process::id()
        ))
    }

    fn canonical_paths(paths: &[std::path::PathBuf]) -> Vec<std::path::PathBuf> {
        paths.iter().map(canonical_path).collect()
    }

    fn canonical_path(path: impl AsRef<Path>) -> std::path::PathBuf {
        path.as_ref()
            .canonicalize()
            .expect("path should canonicalize")
    }

    #[test]
    fn empty_input_discovers_markdown_files_from_current_dir() {
        let dir = test_dir("default-recursive-input");
        let nested = dir.join("docs");
        fs::create_dir_all(&nested).expect("test dir should be created");
        fs::write(dir.join("README.md"), "#Title\n").expect("markdown file should be written");
        fs::write(nested.join("guide.markdown"), "#Title\n")
            .expect("markdown file should be written");
        fs::write(nested.join("ignored.txt"), "#Title\n").expect("text file should be written");

        let original_dir = env::current_dir().expect("current dir should be available");
        env::set_current_dir(&dir).expect("current dir should be changed");
        let files =
            expand_inputs(&Cli::default()).expect("empty input should expand from current dir");
        env::set_current_dir(original_dir).expect("current dir should be restored");

        assert_eq!(
            canonical_paths(&files),
            vec![
                canonical_path(dir.join("README.md")),
                canonical_path(dir.join("docs").join("guide.markdown"))
            ]
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn directory_input_discovers_markdown_files() {
        let dir = test_dir("directory-input");
        fs::create_dir_all(&dir).expect("test dir should be created");
        fs::write(dir.join("bad.md"), "#Title\n").expect("markdown file should be written");
        fs::write(dir.join("ignored.txt"), "#Title\n").expect("text file should be written");

        let files = expand_inputs(&Cli {
            inputs: vec![dir.display().to_string()],
            ..Cli::default()
        })
        .expect("input should expand");

        assert_eq!(files, vec![dir.join("bad.md")]);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn directory_input_respects_gitignore() {
        let dir = test_dir("directory-gitignore");
        let ignored_dir = dir.join("ignored");
        fs::create_dir_all(&ignored_dir).expect("test dir should be created");
        fs::write(dir.join(".gitignore"), "ignored/\n").expect("gitignore should be written");
        fs::write(dir.join("kept.md"), "#Title\n").expect("markdown file should be written");
        fs::write(ignored_dir.join("skipped.md"), "#Title\n")
            .expect("markdown file should be written");

        let files = expand_inputs(&Cli {
            inputs: vec![dir.display().to_string()],
            ..Cli::default()
        })
        .expect("input should expand");

        assert_eq!(files, vec![dir.join("kept.md")]);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn no_ignore_discovers_gitignored_markdown_files() {
        let dir = test_dir("directory-no-ignore");
        let ignored_dir = dir.join("ignored");
        fs::create_dir_all(&ignored_dir).expect("test dir should be created");
        fs::write(dir.join(".gitignore"), "ignored/\n").expect("gitignore should be written");
        fs::write(dir.join("kept.md"), "#Title\n").expect("markdown file should be written");
        fs::write(ignored_dir.join("skipped.md"), "#Title\n")
            .expect("markdown file should be written");

        let files = expand_inputs(&Cli {
            inputs: vec![dir.display().to_string()],
            respect_gitignore: false,
            ..Cli::default()
        })
        .expect("input should expand");

        assert_eq!(files.len(), 2);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn force_exclude_applies_to_explicit_files() {
        let dir = test_dir("force-exclude");
        fs::create_dir_all(&dir).expect("test dir should be created");
        let file = dir.join("skip.md");
        fs::write(&file, "#Title\n").expect("markdown file should be written");

        let files = expand_inputs(&Cli {
            inputs: vec![file.display().to_string()],
            exclude: vec!["**/skip.md".to_string()],
            force_exclude: true,
            ..Cli::default()
        })
        .expect("input should expand");

        assert!(files.is_empty());
        let _ = fs::remove_dir_all(dir);
    }
}
