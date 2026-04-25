use katana_markdown_linter::{
    catalog::RuleCatalogEntry, fix, lint, rule_catalog, ConfigError, LintOptions, LintResult,
    Locale, MarkdownLintConfig,
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
        let options = LintOptions::default();
        let locale = request_locale(request.locale.as_deref());
        let fix_result = fix(&request.content, &options).map_err(|err| err.to_string())?;
        let remaining = lint(&fix_result.content, &options).map_err(|err| err.to_string())?;
        Ok(Json(FixTextResponse {
            content: fix_result.content,
            applied_fixes: fix_result.applied_fixes,
            remaining_issue_count: remaining.len(),
            remaining_diagnostics: remaining
                .into_iter()
                .map(|diagnostic| Diagnostic::from_result(diagnostic, locale))
                .collect(),
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
    locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct FixTextRequest {
    content: String,
    locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ConfigValidateRequest {
    config: Value,
    locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct RuleListRequest {
    locale: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct RuleGetRequest {
    rule_id: String,
    locale: Option<String>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct CheckTextResponse {
    issue_count: usize,
    diagnostics: Vec<Diagnostic>,
}

impl CheckTextResponse {
    fn from_results(results: Vec<LintResult>, locale: Locale) -> Self {
        Self {
            issue_count: results.len(),
            diagnostics: results
                .into_iter()
                .map(|result| Diagnostic::from_result(result, locale))
                .collect(),
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
    message_id: String,
    message_params: std::collections::BTreeMap<String, String>,
    severity: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    fixable: bool,
    fix: Option<TextFix>,
}

impl Diagnostic {
    fn from_result(result: LintResult, locale: Locale) -> Self {
        let fixable = result.fix.is_some();
        let message = katana_markdown_linter::i18n::render_message(
            locale,
            result.message_id.as_str(),
            &result.message_params,
            result.message.as_str(),
        );
        Self {
            rule_id: result.rule_id,
            rule_name: result.rule_name,
            message,
            message_id: result.message_id,
            message_params: result.message_params,
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
    message_id: String,
    message_params: std::collections::BTreeMap<String, String>,
    expected: Option<String>,
    actual: Option<String>,
    allowed: Vec<String>,
}

impl ConfigValidationError {
    fn from_error(error: ConfigError, locale: Locale) -> Self {
        Self {
            rule_id: error.rule_id.clone(),
            property: error.property.clone(),
            kind: error.kind_code().to_string(),
            message: error.localized_message(locale),
            message_id: error.message_id().to_string(),
            message_params: error.message_params(),
            expected: error.expected().map(str::to_string),
            actual: error.actual().map(str::to_string),
            allowed: error.allowed().into_iter().map(str::to_string).collect(),
        }
    }
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct RuleListResponse {
    locale: String,
    count: usize,
    rules: Vec<RuleMetadata>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct RuleMetadata {
    id: String,
    name: String,
    description: String,
    english_description: String,
    docs_url: String,
    fixable: bool,
    implemented_check: bool,
    lifecycle: String,
    locale: String,
}

impl RuleMetadata {
    fn from_entry(rule: RuleCatalogEntry, lifecycle: &str, locale: Locale) -> Self {
        let locale_code = locale.code();
        let description = rule.localized_description(locale_code);
        Self {
            id: rule.id,
            name: rule.name,
            description,
            english_description: rule.description,
            docs_url: rule.docs_url,
            fixable: rule.fixable,
            implemented_check: rule.implemented_check,
            lifecycle: lifecycle.to_string(),
            locale: locale_code.to_string(),
        }
    }
}

fn catalog_rules(locale: Locale) -> Vec<RuleMetadata> {
    let catalog = rule_catalog();
    catalog
        .active
        .into_iter()
        .map(|rule| RuleMetadata::from_entry(rule, "active", locale))
        .chain(
            catalog
                .deprecated
                .into_iter()
                .map(|rule| RuleMetadata::from_entry(rule, "deprecated", locale)),
        )
        .chain(
            catalog
                .removed
                .into_iter()
                .map(|rule| RuleMetadata::from_entry(rule, "removed", locale)),
        )
        .collect()
}

fn validate_config(raw: Value, locale: Locale) -> Vec<ConfigValidationError> {
    MarkdownLintConfig { raw }
        .validate_cached_rules()
        .into_iter()
        .map(|error| ConfigValidationError::from_error(error, locale))
        .collect()
}

fn request_locale(locale: Option<&str>) -> Locale {
    locale
        .map(katana_markdown_linter::resolve_locale_code)
        .unwrap_or(Locale::En)
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
                locale: Some("ja-JP".to_string()),
            }))
            .await
            .expect("check_text should succeed");

        assert!(response.issue_count > 0);
        let diagnostic = response
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.rule_id == "MD001")
            .expect("MD001 should be reported");
        assert_eq!(diagnostic.message_id, "rule.MD001.heading_increment");
        assert!(diagnostic.message.contains("見出しレベル"));
    }

    #[tokio::test]
    async fn fix_text_returns_fixed_content_without_file_tools() {
        let server = KmlMcpServer::new();
        let Json(response) = server
            .fix_text(Parameters(FixTextRequest {
                content: "#Title\n".to_string(),
                locale: Some("ja".to_string()),
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
                locale: Some("ja".to_string()),
            }))
            .await;

        assert!(!response.valid);
        assert_eq!(response.error_count, 1);
        assert_eq!(response.errors[0].kind, "unknown_rule");
        assert_eq!(response.errors[0].rule_id.as_deref(), Some("MD999"));
        assert_eq!(response.errors[0].message_id, "config.unknown_rule");
        assert_eq!(
            response.errors[0].message,
            "未知の markdownlint rule です: MD999"
        );
    }

    #[tokio::test]
    async fn rule_tools_expose_catalog_metadata() {
        let server = KmlMcpServer::new();
        let Json(list) = server
            .rule_list(Parameters(RuleListRequest {
                locale: Some("ja".to_string()),
            }))
            .await;
        assert_eq!(list.locale, "ja");
        assert!(list.rules.iter().any(|rule| rule.id == "MD001"));
        assert!(list.rules.iter().any(|rule| {
            rule.id == "MD003" && rule.description == "見出しのスタイルを統一してください"
        }));

        let Json(rule) = server
            .rule_get(Parameters(RuleGetRequest {
                rule_id: "md001".to_string(),
                locale: Some("ja-JP".to_string()),
            }))
            .await
            .expect("MD001 should exist");
        assert_eq!(rule.id, "MD001");
        assert_eq!(rule.locale, "ja");
        assert_ne!(rule.description, rule.english_description);
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
