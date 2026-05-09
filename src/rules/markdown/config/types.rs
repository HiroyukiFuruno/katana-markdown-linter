use serde_json::Value;

/// Why: markdownlint config helper needs a stable JSON representation.
#[derive(Debug, Clone, PartialEq)]
pub struct MarkdownLintConfig {
    pub raw: Value,
}

impl Default for MarkdownLintConfig {
    fn default() -> Self {
        Self {
            raw: serde_json::json!({ "default": true }),
        }
    }
}
