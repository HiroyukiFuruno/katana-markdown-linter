#[path = "workspace/diff.rs"]
mod diff;
#[path = "workspace/path.rs"]
mod path;
#[path = "workspace/walk.rs"]
mod walk;

use diff::unified_line_diff;
use katana_markdown_linter::{LintOptions, MarkdownLintConfig, MarkdownLinter};
use path::{clean_relative_path, read_markdown_file, reject_symlink_components};
use std::path::{Path, PathBuf};
use walk::collect_markdown_files;

#[derive(Debug, Clone)]
pub(crate) struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub(crate) fn current() -> Result<Self, String> {
        let root = std::env::current_dir().map_err(|error| error.to_string())?;
        Self::new(root)
    }

    pub(crate) fn new(root: PathBuf) -> Result<Self, String> {
        let root = root
            .canonicalize()
            .map_err(|error| format!("failed to resolve workspace root: {error}"))?;
        Ok(Self { root })
    }

    pub(crate) fn resolve_existing(&self, relative_path: &str) -> Result<PathBuf, String> {
        let clean = clean_relative_path(relative_path)?;
        let path = self.root.join(&clean);
        reject_symlink_components(&self.root, &clean)?;
        if !path.exists() {
            return Err(format!("workspace path does not exist: {relative_path}"));
        }
        Ok(path)
    }

    pub(crate) fn display_relative(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string()
    }

    pub(crate) fn lint_file(
        &self,
        relative_path: &str,
        config_path: Option<&str>,
    ) -> Result<FileLint, String> {
        let path = self.resolve_existing(relative_path)?;
        let content = read_markdown_file(&path)?;
        let options = self.load_options(config_path)?;
        let diagnostics =
            MarkdownLinter::lint(&content, &options).map_err(|error| error.to_string())?;
        Ok(FileLint { path, diagnostics })
    }

    pub(crate) fn lint_directory(
        &self,
        relative_path: &str,
        config_path: Option<&str>,
        respect_gitignore: bool,
    ) -> Result<DirectoryLint, String> {
        let directory = self.resolve_existing(relative_path)?;
        if !directory.is_dir() {
            return Err(format!("{relative_path} is not a directory"));
        }

        let options = self.load_options(config_path)?;
        let mut files = Vec::new();
        let mut errors = Vec::new();
        for path in collect_markdown_files(&directory, respect_gitignore)? {
            match read_markdown_file(&path).and_then(|content| {
                MarkdownLinter::lint(&content, &options).map_err(|error| error.to_string())
            }) {
                Ok(diagnostics) => files.push(FileLint { path, diagnostics }),
                Err(message) => errors.push(FileError { path, message }),
            }
        }
        Ok(DirectoryLint { files, errors })
    }

    pub(crate) fn preview_fix(
        &self,
        relative_path: &str,
        config_path: Option<&str>,
    ) -> Result<FileFixPreview, String> {
        let path = self.resolve_existing(relative_path)?;
        let content = read_markdown_file(&path)?;
        let options = self.load_options(config_path)?;
        let fixed = MarkdownLinter::fix(&content, &options).map_err(|error| error.to_string())?;
        let remaining =
            MarkdownLinter::lint(&fixed.content, &options).map_err(|error| error.to_string())?;
        Ok(FileFixPreview {
            path,
            diff: unified_line_diff(&content, &fixed.content),
            changed: content != fixed.content,
            applied_fixes: fixed.applied_fixes,
            remaining,
            fixed_content: fixed.content,
        })
    }

    pub(crate) fn apply_fix(
        &self,
        relative_path: &str,
        config_path: Option<&str>,
    ) -> Result<FileFixApply, String> {
        let preview = self.preview_fix(relative_path, config_path)?;
        if preview.changed {
            std::fs::write(&preview.path, &preview.fixed_content)
                .map_err(|error| format!("failed to write fixed file: {error}"))?;
        }
        Ok(FileFixApply {
            path: preview.path,
            changed: preview.changed,
            applied_fixes: preview.applied_fixes,
            remaining: preview.remaining,
        })
    }

    fn load_options(&self, config_path: Option<&str>) -> Result<LintOptions, String> {
        let config = if let Some(config_path) = config_path {
            MarkdownLintConfig::load(&self.resolve_existing(config_path)?)
        } else {
            MarkdownLintConfig::load(&self.default_config_path())
        }
        .map_err(|error| error.to_string())?;

        let errors = config.validate_cached_rules();
        if !errors.is_empty() {
            return Err(errors[0].to_string());
        }
        Ok(config.to_lint_options())
    }

    fn default_config_path(&self) -> PathBuf {
        let json = self.root.join(".markdownlint.json");
        if json.exists() {
            return json;
        }
        self.root.join(".markdownlint.jsonc")
    }
}

#[derive(Debug)]
pub(crate) struct FileLint {
    pub(crate) path: PathBuf,
    pub(crate) diagnostics: Vec<katana_markdown_linter::LintResult>,
}

#[derive(Debug)]
pub(crate) struct DirectoryLint {
    pub(crate) files: Vec<FileLint>,
    pub(crate) errors: Vec<FileError>,
}

#[derive(Debug)]
pub(crate) struct FileError {
    pub(crate) path: PathBuf,
    pub(crate) message: String,
}

#[derive(Debug)]
pub(crate) struct FileFixPreview {
    pub(crate) path: PathBuf,
    pub(crate) changed: bool,
    pub(crate) applied_fixes: usize,
    pub(crate) diff: String,
    pub(crate) remaining: Vec<katana_markdown_linter::LintResult>,
    fixed_content: String,
}

#[derive(Debug)]
pub(crate) struct FileFixApply {
    pub(crate) path: PathBuf,
    pub(crate) changed: bool,
    pub(crate) applied_fixes: usize,
    pub(crate) remaining: Vec<katana_markdown_linter::LintResult>,
}
