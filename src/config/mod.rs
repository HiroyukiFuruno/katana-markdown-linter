mod schema;
mod session;

use std::path::{Path, PathBuf};

pub use crate::rules::markdown::config::*;
pub use schema::{ConfigSchema, MARKDOWNLINT_CONFIG_SCHEMA_ID};
pub(crate) use session::ConfigSession;

pub struct EffectiveConfig {
    pub config: MarkdownLintConfig,
    pub source: Option<PathBuf>,
}

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_effective_config(
        path: &Path,
        explicit: Option<&Path>,
    ) -> Result<MarkdownLintConfig, String> {
        Ok(Self::load_effective_config_with_source(path, explicit)?.config)
    }

    pub fn load_effective_config_with_source(
        path: &Path,
        explicit: Option<&Path>,
    ) -> Result<EffectiveConfig, String> {
        let source = Self::resolve_source(path, explicit)?;
        let config = Self::load_config_from_source(source.as_deref())?;
        Ok(EffectiveConfig { config, source })
    }

    pub(super) fn resolve_source(
        path: &Path,
        explicit: Option<&Path>,
    ) -> Result<Option<PathBuf>, String> {
        if let Some(explicit) = explicit {
            if !explicit.exists() {
                return Err(format!("config file not found: {}", explicit.display()));
            }
            return Ok(Some(explicit.to_path_buf()));
        }

        let mut current = path.parent();
        while let Some(dir) = current {
            let json = dir.join(".markdownlint.json");
            if json.exists() {
                return Ok(Some(json));
            }
            let jsonc = dir.join(".markdownlint.jsonc");
            if jsonc.exists() {
                return Ok(Some(jsonc));
            }
            current = dir.parent();
        }
        Ok(None)
    }

    pub(super) fn load_config_from_source(
        source: Option<&Path>,
    ) -> Result<MarkdownLintConfig, String> {
        source.map_or_else(
            || Ok(MarkdownLintConfig::default()),
            |path| MarkdownLintConfig::load(path).map_err(|err| err.to_string()),
        )
    }

    pub fn validate_effective_config(
        path: &Path,
        explicit: Option<&Path>,
    ) -> Result<Vec<crate::ConfigError>, String> {
        Ok(Self::load_effective_config(path, explicit)?.validate_against_schema())
    }
}
