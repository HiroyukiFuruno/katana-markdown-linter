use rmcp::schemars;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CheckTextRequest {
    pub(crate) content: String,
    pub(crate) locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct FixTextRequest {
    pub(crate) content: String,
    pub(crate) locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ConfigValidateRequest {
    pub(crate) config: Value,
    pub(crate) locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RuleListRequest {
    pub(crate) locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RuleGetRequest {
    pub(crate) rule_id: String,
    pub(crate) locale: Option<String>,
}

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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RuleDocRequest {
    pub(crate) rule_id: String,
    pub(crate) locale: Option<String>,
}
