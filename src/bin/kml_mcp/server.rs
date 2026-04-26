#[path = "server/commands.rs"]
mod commands;
#[path = "server/runtime.rs"]
mod runtime;
#[path = "server/support.rs"]
mod support;

#[cfg(test)]
#[path = "server_tests.rs"]
mod tests;

use crate::model::{
    CheckTextRequest, CheckTextResponse, ConfigValidateRequest, ConfigValidateResponse,
    DirectoryCheckResponse, DirectoryRequest, FileCheckResponse, FileFixApplyResponse,
    FileFixPreviewResponse, FileRequest, FixFileApplyRequest, FixTextRequest, FixTextResponse,
    RuleDocRequest, RuleDocResponse, RuleGetRequest, RuleListRequest, RuleListResponse,
    RuleMetadata,
};
use crate::workspace::Workspace;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, Json, ServerHandler,
};
pub(crate) use runtime::run_from_env;

#[derive(Debug, Clone)]
pub(crate) struct KmlMcpServer {
    tool_router: ToolRouter<Self>,
    workspace: Workspace,
}

impl KmlMcpServer {
    pub(crate) fn new() -> Self {
        let workspace = Workspace::current().expect("workspace root should resolve");
        Self::with_workspace(workspace)
    }

    fn with_workspace(workspace: Workspace) -> Self {
        Self {
            tool_router: Self::tool_router(),
            workspace,
        }
    }
}

impl Default for KmlMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router(router = tool_router)]
impl KmlMcpServer {
    #[tool(
        name = "check_text",
        description = "Lint supplied Markdown text and return structured diagnostics."
    )]
    async fn check_text(
        &self,
        Parameters(request): Parameters<CheckTextRequest>,
    ) -> Result<Json<CheckTextResponse>, String> {
        self.handle_check_text(request)
    }

    #[tool(
        name = "fix_text",
        description = "Apply safe Markdown fixes to supplied text and return fixed content without writing files."
    )]
    async fn fix_text(
        &self,
        Parameters(request): Parameters<FixTextRequest>,
    ) -> Result<Json<FixTextResponse>, String> {
        self.handle_fix_text(request)
    }

    #[tool(
        name = "config_validate",
        description = "Validate a markdownlint-compatible JSON config object."
    )]
    async fn config_validate(
        &self,
        Parameters(request): Parameters<ConfigValidateRequest>,
    ) -> Json<ConfigValidateResponse> {
        self.handle_config_validate(request)
    }

    #[tool(
        name = "rule_list",
        description = "List markdownlint-compatible rule metadata known to kml."
    )]
    async fn rule_list(
        &self,
        Parameters(request): Parameters<RuleListRequest>,
    ) -> Json<RuleListResponse> {
        self.handle_rule_list(request)
    }

    #[tool(
        name = "rule_get",
        description = "Get metadata for a single markdownlint-compatible rule ID."
    )]
    async fn rule_get(
        &self,
        Parameters(request): Parameters<RuleGetRequest>,
    ) -> Result<Json<RuleMetadata>, String> {
        self.handle_rule_get(request)
    }

    #[tool(
        name = "rule_doc_get",
        description = "Get full Markdown documentation for a rule, supporting multiple locales."
    )]
    async fn rule_doc_get(
        &self,
        Parameters(request): Parameters<RuleDocRequest>,
    ) -> Result<Json<RuleDocResponse>, String> {
        self.handle_rule_doc_get(request)
    }

    #[tool(
        name = "check_file",
        description = "Lint a Markdown file inside the configured workspace root."
    )]
    async fn check_file(
        &self,
        Parameters(request): Parameters<FileRequest>,
    ) -> Result<Json<FileCheckResponse>, String> {
        self.handle_check_file(request)
    }

    #[tool(
        name = "check_directory",
        description = "Lint Markdown files under a workspace directory."
    )]
    async fn check_directory(
        &self,
        Parameters(request): Parameters<DirectoryRequest>,
    ) -> Result<Json<DirectoryCheckResponse>, String> {
        self.handle_check_directory(request)
    }

    #[tool(
        name = "fix_file_preview",
        description = "Preview safe fixes for a workspace file and return a text diff without writing."
    )]
    async fn fix_file_preview(
        &self,
        Parameters(request): Parameters<FileRequest>,
    ) -> Result<Json<FileFixPreviewResponse>, String> {
        self.handle_fix_file_preview(request)
    }

    #[tool(
        name = "fix_file_apply",
        description = "Apply safe fixes to a workspace file only when apply is true."
    )]
    async fn fix_file_apply(
        &self,
        Parameters(request): Parameters<FixFileApplyRequest>,
    ) -> Result<Json<FileFixApplyResponse>, String> {
        self.handle_fix_file_apply(request)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for KmlMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("kml-mcp", env!("CARGO_PKG_VERSION")))
            .with_instructions(
                "Workspace-scoped MCP adapter for katana-markdown-linter. \
                 Use check_text, fix_text, config_validate, rule_list, rule_get, \
                 check_file, check_directory, fix_file_preview, and fix_file_apply. \
                 File writes require the explicit fix_file_apply tool with apply: true.",
            )
    }
}
