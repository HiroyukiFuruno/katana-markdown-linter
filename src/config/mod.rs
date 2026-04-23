use std::fs;
use std::path::Path;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownLintConfig {
    pub raw: String,
}

impl MarkdownLintConfig {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let raw = fs::read_to_string(path)
            .map_err(|err| Error::new(format!("failed to read config: {err}")))?;
        Ok(Self { raw })
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        fs::write(path, &self.raw)
            .map_err(|err| Error::new(format!("failed to write config: {err}")))
    }
}
