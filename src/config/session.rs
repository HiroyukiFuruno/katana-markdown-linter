use super::{ConfigLoader, MarkdownLintConfig};
use crate::{ConfigError, LintOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct PreparedConfig {
    pub(crate) source: Option<PathBuf>,
    pub(crate) validation_errors: Vec<ConfigError>,
    pub(crate) lint_options: LintOptions,
}

impl PreparedConfig {
    fn new(source: Option<PathBuf>, config: MarkdownLintConfig) -> Self {
        Self {
            source,
            validation_errors: config.validate_against_schema(),
            lint_options: config.to_lint_options(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum LookupKey {
    Explicit,
    Directory(PathBuf),
}

pub(crate) struct ConfigSession {
    explicit: Option<PathBuf>,
    resolved_sources: HashMap<LookupKey, Result<Option<PathBuf>, String>>,
    prepared: HashMap<Option<PathBuf>, Result<Arc<PreparedConfig>, String>>,
}

impl ConfigSession {
    pub(crate) fn new(explicit: Option<&Path>) -> Self {
        Self {
            explicit: explicit.map(Path::to_path_buf),
            resolved_sources: HashMap::new(),
            prepared: HashMap::new(),
        }
    }

    pub(crate) fn explicit_path(&self) -> Option<&Path> {
        self.explicit.as_deref()
    }

    pub(crate) fn load_for_path(&mut self, path: &Path) -> Result<Arc<PreparedConfig>, String> {
        let key = match &self.explicit {
            Some(_) => LookupKey::Explicit,
            None => {
                LookupKey::Directory(path.parent().unwrap_or_else(|| Path::new("")).to_path_buf())
            }
        };
        let explicit = self.explicit.as_deref();
        let source = self
            .resolved_sources
            .entry(key)
            .or_insert_with(|| ConfigLoader::resolve_source(path, explicit))
            .clone()?;

        self.prepared
            .entry(source.clone())
            .or_insert_with(|| {
                ConfigLoader::load_config_from_source(source.as_deref())
                    .map(|config| Arc::new(PreparedConfig::new(source.clone(), config)))
            })
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn reuses_prepared_config_per_source_and_refreshes_for_new_session() {
        let dir = TestDir::new();
        let first_file = dir.path().join("first.md");
        let second_file = dir.path().join("second.md");
        let config_path = dir.path().join(".markdownlint.json");
        fs::write(&first_file, "#Title\n").unwrap();
        fs::write(&second_file, "#Title\n").unwrap();
        fs::write(&config_path, r#"{"default":false}"#).unwrap();

        let mut session = ConfigSession::new(None);
        let first = session.load_for_path(&first_file).unwrap();
        let second = session.load_for_path(&second_file).unwrap();
        assert!(Arc::ptr_eq(&first, &second));
        assert!(!first.lint_options.rules["MD001"].enabled);

        fs::write(&config_path, r#"{"default":true}"#).unwrap();
        let mut next_session = ConfigSession::new(None);
        let refreshed = next_session.load_for_path(&first_file).unwrap();
        assert!(!Arc::ptr_eq(&first, &refreshed));
        assert!(refreshed.lint_options.rules["MD001"].enabled);
    }

    #[test]
    fn resolves_nearest_child_config_before_parent_config() {
        let dir = TestDir::new();
        let child = dir.path().join("child");
        fs::create_dir_all(&child).unwrap();
        let parent_config = dir.path().join(".markdownlint.json");
        let child_config = child.join(".markdownlint.json");
        let parent_file = dir.path().join("parent.md");
        let child_file = child.join("child.md");
        fs::write(&parent_config, r#"{"default":false,"MD018":true}"#).unwrap();
        fs::write(&child_config, r#"{"default":false}"#).unwrap();
        fs::write(&parent_file, "#Title\n").unwrap();
        fs::write(&child_file, "#Title\n").unwrap();

        let mut session = ConfigSession::new(None);
        let parent = session.load_for_path(&parent_file).unwrap();
        let child = session.load_for_path(&child_file).unwrap();
        assert_eq!(parent.source.as_deref(), Some(parent_config.as_path()));
        assert_eq!(child.source.as_deref(), Some(child_config.as_path()));
        assert!(!Arc::ptr_eq(&parent, &child));
    }

    #[test]
    fn reuses_default_config_when_no_source_is_found() {
        let dir = TestDir::new();
        let first_file = dir.path().join("first.md");
        let second_file = dir.path().join("second.md");
        fs::write(&first_file, "#Title\n").unwrap();
        fs::write(&second_file, "#Title\n").unwrap();

        let mut session = ConfigSession::new(None);
        let first = session.load_for_path(&first_file).unwrap();
        let second = session.load_for_path(&second_file).unwrap();
        assert!(first.source.is_none());
        assert!(Arc::ptr_eq(&first, &second));
        assert!(first.validation_errors.is_empty());
    }

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
            let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "kml-config-session-{}-{nonce}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
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
}
