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

pub(crate) use config::{validate_config, ConfigValidateResponse};
pub(crate) use diagnostic::Diagnostic;
pub(crate) use request::{
    CheckTextRequest, ConfigValidateRequest, DirectoryRequest, FileRequest, FixFileApplyRequest,
    FixTextRequest, RuleDocRequest, RuleGetRequest, RuleListRequest,
};
pub(crate) use response::{
    CheckTextResponse, DirectoryCheckResponse, FileCheckResponse, FileFixApplyResponse,
    FileFixPreviewResponse, FixTextResponse, RuleDocResponse, WorkspaceToolError,
};
pub(crate) use rule::{RuleListResponse, RuleMetadata};
