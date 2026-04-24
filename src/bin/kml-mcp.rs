use katana_markdown_linter::{
    fix, lint, rule_catalog, ConfigError, ConfigErrorKind, LintOptions, LintResult,
    MarkdownLintConfig,
};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, Json, ServerHandler, ServiceExt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
struct KmlMcpServer {
    tool_router: ToolRouter<Self>,
}

impl KmlMcpServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
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
        let options = LintOptions::default();
        let diagnostics = lint(&request.content, &options).map_err(|err| err.to_string())?;
        Ok(Json(CheckTextResponse::from_results(diagnostics)))
    }

    #[tool(
        name = "fix_text",
        description = "Apply safe Markdown fixes to supplied text and return fixed content without writing files."
    )]
    async fn fix_text(
        &self,
        Parameters(request): Parameters<FixTextRequest>,
    ) -> Result<Json<FixTextResponse>, String> {
        let options = LintOptions::default();
        let fix_result = fix(&request.content, &options).map_err(|err| err.to_string())?;
        let remaining = lint(&fix_result.content, &options).map_err(|err| err.to_string())?;
        Ok(Json(FixTextResponse {
            content: fix_result.content,
            applied_fixes: fix_result.applied_fixes,
            remaining_issue_count: remaining.len(),
            remaining_diagnostics: remaining.into_iter().map(Diagnostic::from).collect(),
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
        let errors = validate_config(request.config);
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
    async fn rule_list(&self) -> Json<RuleListResponse> {
        let rules = catalog_rules();
        Json(RuleListResponse {
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
        catalog_rules()
            .into_iter()
            .find(|rule| rule.id == rule_id)
            .map(Json)
            .ok_or_else(|| format!("unknown rule: {}", request.rule_id))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for KmlMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("kml-mcp", env!("CARGO_PKG_VERSION")))
            .with_instructions(
                "Read-only, text-first MCP adapter for katana-markdown-linter. \
                 Use check_text, fix_text, config_validate, rule_list, and rule_get. \
                 This prototype does not read or write workspace files.",
            )
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CheckTextRequest {
    content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct FixTextRequest {
    content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ConfigValidateRequest {
    config: Value,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct RuleGetRequest {
    rule_id: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct CheckTextResponse {
    issue_count: usize,
    diagnostics: Vec<Diagnostic>,
}

impl CheckTextResponse {
    fn from_results(results: Vec<LintResult>) -> Self {
        Self {
            issue_count: results.len(),
            diagnostics: results.into_iter().map(Diagnostic::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct FixTextResponse {
    content: String,
    applied_fixes: usize,
    remaining_issue_count: usize,
    remaining_diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct Diagnostic {
    rule_id: String,
    rule_name: String,
    message: String,
    severity: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    fixable: bool,
    fix: Option<TextFix>,
}

impl From<LintResult> for Diagnostic {
    fn from(result: LintResult) -> Self {
        let fixable = result.fix.is_some();
        Self {
            rule_id: result.rule_id,
            rule_name: result.rule_name,
            message: result.message,
            severity: match result.severity {
                katana_markdown_linter::Severity::Error => "error",
                katana_markdown_linter::Severity::Warning => "warning",
                katana_markdown_linter::Severity::Info => "info",
            }
            .to_string(),
            line: result.line,
            column: result.column,
            end_line: result.end_line,
            end_column: result.end_column,
            fixable,
            fix: result.fix.map(|fix| TextFix {
                start_line: fix.range.start_line,
                start_column: fix.range.start_column,
                end_line: fix.range.end_line,
                end_column: fix.range.end_column,
                replacement: fix.replacement,
            }),
        }
    }
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct TextFix {
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    replacement: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct ConfigValidateResponse {
    valid: bool,
    error_count: usize,
    errors: Vec<ConfigValidationError>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct ConfigValidationError {
    rule_id: Option<String>,
    property: Option<String>,
    kind: String,
    message: String,
    expected: Option<String>,
    actual: Option<String>,
    allowed: Vec<String>,
}

impl From<ConfigError> for ConfigValidationError {
    fn from(error: ConfigError) -> Self {
        let (kind, expected, actual, allowed) = match error.kind {
            ConfigErrorKind::InvalidRoot => ("invalid_root", None, None, Vec::new()),
            ConfigErrorKind::UnknownRule => ("unknown_rule", None, None, Vec::new()),
            ConfigErrorKind::UnknownProperty => ("unknown_property", None, None, Vec::new()),
            ConfigErrorKind::InvalidType { expected, actual } => (
                "invalid_type",
                Some(expected.to_string()),
                Some(actual.to_string()),
                Vec::new(),
            ),
            ConfigErrorKind::InvalidEnumValue { allowed, actual } => (
                "invalid_enum_value",
                None,
                Some(actual),
                allowed.into_iter().map(str::to_string).collect(),
            ),
        };

        Self {
            rule_id: error.rule_id,
            property: error.property,
            kind: kind.to_string(),
            message: error.message,
            expected,
            actual,
            allowed,
        }
    }
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct RuleListResponse {
    count: usize,
    rules: Vec<RuleMetadata>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct RuleMetadata {
    id: String,
    name: String,
    description: String,
    docs_url: String,
    fixable: bool,
    implemented_check: bool,
    lifecycle: String,
}

fn catalog_rules() -> Vec<RuleMetadata> {
    let catalog = rule_catalog();
    catalog
        .active
        .into_iter()
        .map(|rule| RuleMetadata {
            id: rule.id,
            name: rule.name,
            description: rule.description,
            docs_url: rule.docs_url,
            fixable: rule.fixable,
            implemented_check: rule.implemented_check,
            lifecycle: "active".to_string(),
        })
        .chain(catalog.deprecated.into_iter().map(|rule| RuleMetadata {
            id: rule.id,
            name: rule.name,
            description: rule.description,
            docs_url: rule.docs_url,
            fixable: rule.fixable,
            implemented_check: rule.implemented_check,
            lifecycle: "deprecated".to_string(),
        }))
        .chain(catalog.removed.into_iter().map(|rule| RuleMetadata {
            id: rule.id,
            name: rule.name,
            description: rule.description,
            docs_url: rule.docs_url,
            fixable: rule.fixable,
            implemented_check: rule.implemented_check,
            lifecycle: "removed".to_string(),
        }))
        .collect()
}

fn validate_config(raw: Value) -> Vec<ConfigValidationError> {
    MarkdownLintConfig { raw }
        .validate(
            &katana_markdown_linter::rules::markdown::MarkdownLinterOps::get_user_configurable_rules(),
        )
        .into_iter()
        .map(Into::into)
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = KmlMcpServer::new().serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::handler::server::wrapper::Parameters;
    use serde_json::json;

    #[tokio::test]
    async fn check_text_reports_structured_diagnostics() {
        let server = KmlMcpServer::new();
        let Json(response) = server
            .check_text(Parameters(CheckTextRequest {
                content: "# title\n\n### skipped\n".to_string(),
            }))
            .await
            .expect("check_text should succeed");

        assert!(response.issue_count > 0);
        assert!(response
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.rule_id == "MD001"));
    }

    #[tokio::test]
    async fn fix_text_returns_fixed_content_without_file_tools() {
        let server = KmlMcpServer::new();
        let Json(response) = server
            .fix_text(Parameters(FixTextRequest {
                content: "#Title\n".to_string(),
            }))
            .await
            .expect("fix_text should succeed");

        assert!(response.applied_fixes > 0);
        assert_eq!(response.content, "# Title\n");
    }

    #[tokio::test]
    async fn config_validate_reports_structured_errors() {
        let server = KmlMcpServer::new();
        let Json(response) = server
            .config_validate(Parameters(ConfigValidateRequest {
                config: json!({ "MD999": true }),
            }))
            .await;

        assert!(!response.valid);
        assert_eq!(response.error_count, 1);
        assert_eq!(response.errors[0].kind, "unknown_rule");
        assert_eq!(response.errors[0].rule_id.as_deref(), Some("MD999"));
    }

    #[tokio::test]
    async fn rule_tools_expose_catalog_metadata() {
        let server = KmlMcpServer::new();
        let Json(list) = server.rule_list().await;
        assert!(list.rules.iter().any(|rule| rule.id == "MD001"));

        let Json(rule) = server
            .rule_get(Parameters(RuleGetRequest {
                rule_id: "md001".to_string(),
            }))
            .await
            .expect("MD001 should exist");
        assert_eq!(rule.id, "MD001");
    }

    #[test]
    fn tool_list_excludes_workspace_file_write_tools() {
        let server = KmlMcpServer::new();
        let tool_names = server
            .tool_router
            .list_all()
            .into_iter()
            .map(|tool| tool.name.to_string())
            .collect::<Vec<_>>();

        assert!(tool_names.contains(&"check_text".to_string()));
        assert!(tool_names.contains(&"fix_text".to_string()));
        assert!(tool_names.contains(&"config_validate".to_string()));
        assert!(tool_names.contains(&"rule_list".to_string()));
        assert!(tool_names.contains(&"rule_get".to_string()));
        assert!(!tool_names.contains(&"fix_files".to_string()));
    }
}
