#[path = "model/config.rs"]
mod config;
#[path = "model/diagnostic.rs"]
mod diagnostic;
#[path = "model/request.rs"]
mod request;
#[path = "model/response.rs"]
mod response;
#[path = "model/rule.rs"]
mod rule;
#[path = "model/workspace_request.rs"]
mod workspace_request;
#[path = "model/workspace_response.rs"]
mod workspace_response;

pub(super) use config::ConfigValidateResponse;
pub(super) use diagnostic::Diagnostic;
pub(super) use request::{
    CheckTextRequest, ConfigValidateRequest, FixTextRequest, RuleDocRequest, RuleGetRequest,
    RuleListRequest,
};
pub(super) use response::{CheckTextResponse, FixTextResponse, RuleDocResponse};
pub(super) use rule::{RuleListResponse, RuleMetadata};
pub(super) use workspace_request::{DirectoryRequest, FileRequest, FixFileApplyRequest};
pub(super) use workspace_response::{
    DirectoryCheckResponse, FileCheckResponse, FileFixApplyResponse, FileFixPreviewResponse,
    WorkspaceToolError,
};

pub(super) struct ConfigValidator;

impl ConfigValidator {
    pub(super) fn validate_config(
        raw: serde_json::Value,
        locale: katana_markdown_linter::Locale,
    ) -> Vec<config::ConfigValidationError> {
        config::validate_config(raw, locale)
    }
}
