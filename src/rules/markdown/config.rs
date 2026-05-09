mod error;
mod jsonc;
mod load;
mod options;
mod property;
mod types;
mod validate;

pub use error::{ConfigError, ConfigErrorKind};
pub use types::MarkdownLintConfig;

#[cfg(test)]
mod tests;
