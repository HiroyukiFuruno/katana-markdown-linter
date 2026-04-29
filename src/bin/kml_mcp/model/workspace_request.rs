use rmcp::schemars;
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct FileRequest {
    pub(crate) path: String,
    pub(crate) config_path: Option<String>,
    pub(crate) locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DirectoryRequest {
    pub(crate) path: String,
    pub(crate) config_path: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) respect_gitignore: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct FixFileApplyRequest {
    pub(crate) path: String,
    pub(crate) config_path: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) apply: bool,
}
