use crate::model::{
    validate_config, CheckTextRequest, CheckTextResponse, ConfigValidateRequest,
    ConfigValidateResponse, FixTextRequest, FixTextResponse, RuleDocRequest, RuleDocResponse,
    RuleGetRequest, RuleListRequest, RuleListResponse, RuleMetadata,
};
use crate::shared::{catalog_rules, diagnostics, request_locale};
use katana_markdown_linter::{fix, lint, upstream};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, Json, ServerHandler,
};

#[derive(Debug, Clone)]
pub(super) struct KmlMcpRemoteServer {
    tool_router: ToolRouter<Self>,
}

impl KmlMcpRemoteServer {
    pub(super) fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for KmlMcpRemoteServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router(router = tool_router)]
impl KmlMcpRemoteServer {
    #[tool(
        name = "check_text",
        description = "Lint supplied Markdown text and return structured diagnostics."
    )]
    async fn check_text(
        &self,
        Parameters(request): Parameters<CheckTextRequest>,
    ) -> Result<Json<CheckTextResponse>, String> {
        let options = katana_markdown_linter::LintOptions::default();
        let diagnostics = lint(&request.content, &options).map_err(|err| err.to_string())?;
        Ok(Json(CheckTextResponse::from_results(
            diagnostics,
            request_locale(request.locale.as_deref()),
        )))
    }

    #[tool(
        name = "fix_text",
        description = "Apply safe Markdown fixes to supplied text and return fixed content without writing files."
    )]
    async fn fix_text(
        &self,
        Parameters(request): Parameters<FixTextRequest>,
    ) -> Result<Json<FixTextResponse>, String> {
        let options = katana_markdown_linter::LintOptions::default();
        let locale = request_locale(request.locale.as_deref());
        let fix_result = fix(&request.content, &options).map_err(|err| err.to_string())?;
        let remaining = lint(&fix_result.content, &options).map_err(|err| err.to_string())?;
        Ok(Json(FixTextResponse {
            content: fix_result.content,
            applied_fixes: fix_result.applied_fixes,
            remaining_issue_count: remaining.len(),
            remaining_diagnostics: diagnostics(remaining, locale),
        }))
    }

    #[tool(
        name = "config_validate",
        description = "Validate a markdownlint-compatible JSON config object."
    )]
    async fn config_validate(
        &self,
        Parameters(request): Parameters<ConfigValidateRequest>,
    ) -> Json<ConfigValidateResponse> {
        let locale = request_locale(request.locale.as_deref());
        let errors = validate_config(request.config, locale);
        Json(ConfigValidateResponse {
            valid: errors.is_empty(),
            error_count: errors.len(),
            errors,
        })
    }

    #[tool(
        name = "rule_list",
        description = "List markdownlint-compatible rule metadata known to kml."
    )]
    async fn rule_list(
        &self,
        Parameters(request): Parameters<RuleListRequest>,
    ) -> Json<RuleListResponse> {
        let locale = request_locale(request.locale.as_deref());
        let rules = catalog_rules(locale);
        Json(RuleListResponse {
            locale: locale.code().to_string(),
            count: rules.len(),
            rules,
        })
    }

    #[tool(
        name = "rule_get",
        description = "Get metadata for a single markdownlint-compatible rule ID."
    )]
    async fn rule_get(
        &self,
        Parameters(request): Parameters<RuleGetRequest>,
    ) -> Result<Json<RuleMetadata>, String> {
        let rule_id = request.rule_id.to_ascii_uppercase();
        let locale = request_locale(request.locale.as_deref());
        catalog_rules(locale)
            .into_iter()
            .find(|rule| rule.id == rule_id)
            .map(Json)
            .ok_or_else(|| format!("unknown rule: {}", request.rule_id))
    }

    #[tool(
        name = "rule_doc_get",
        description = "Get full Markdown documentation for a rule, supporting multiple locales."
    )]
    async fn rule_doc_get(
        &self,
        Parameters(request): Parameters<RuleDocRequest>,
    ) -> Result<Json<RuleDocResponse>, String> {
        let locale = request_locale(request.locale.as_deref());
        let content = upstream::get_rule_documentation(&request.rule_id, locale)?;
        Ok(Json(RuleDocResponse {
            rule_id: request.rule_id,
            locale: locale.code().to_string(),
            content,
        }))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for KmlMcpRemoteServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "kml-mcp-remote",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "Text-only Streamable HTTP adapter for katana-markdown-linter. \
                 Workspace tools are intentionally absent; use kml-mcp over stdio \
                 for workspace reads, previews, or explicit file writes.",
            )
    }
}
