mod schema;

use std::path::{Path, PathBuf};

pub use crate::rules::markdown::config::*;
pub use schema::{ConfigSchema, MARKDOWNLINT_CONFIG_SCHEMA_ID};

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
        if let Some(path) = explicit {
            if !path.exists() {
                return Err(format!("config file not found: {}", path.display()));
            }
            let config = MarkdownLintConfig::load(path).map_err(|err| err.to_string())?;
            return Ok(EffectiveConfig {
                config,
                source: Some(path.to_path_buf()),
            });
        }

        let mut current = path.parent();
        while let Some(dir) = current {
            let json = dir.join(".markdownlint.json");
            if json.exists() {
                let config = MarkdownLintConfig::load(&json).map_err(|err| err.to_string())?;
                return Ok(EffectiveConfig {
                    config,
                    source: Some(json),
                });
            }
            let jsonc = dir.join(".markdownlint.jsonc");
            if jsonc.exists() {
                let config = MarkdownLintConfig::load(&jsonc).map_err(|err| err.to_string())?;
                return Ok(EffectiveConfig {
                    config,
                    source: Some(jsonc),
                });
            }
            current = dir.parent();
        }

        Ok(EffectiveConfig {
            config: MarkdownLintConfig::default(),
            source: None,
        })
    }

    pub fn validate_effective_config(
        path: &Path,
        explicit: Option<&Path>,
    ) -> Result<Vec<crate::ConfigError>, String> {
        Ok(Self::load_effective_config(path, explicit)?.validate_against_schema())
    }
}
