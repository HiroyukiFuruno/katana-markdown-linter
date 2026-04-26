use super::support::{catalog_rules, diagnostics, file_check_response, request_locale};
use super::KmlMcpServer;
use crate::model::{
    validate_config, CheckTextRequest, CheckTextResponse, ConfigValidateRequest,
    ConfigValidateResponse, DirectoryCheckResponse, DirectoryRequest, FileCheckResponse,
    FileFixApplyResponse, FileFixPreviewResponse, FileRequest, FixFileApplyRequest, FixTextRequest,
    FixTextResponse, RuleDocRequest, RuleDocResponse, RuleGetRequest, RuleListRequest,
    RuleListResponse, RuleMetadata, WorkspaceToolError,
};
use katana_markdown_linter::{fix, lint, upstream};
use rmcp::Json;

impl KmlMcpServer {
    pub(super) fn handle_check_text(
        &self,
        request: CheckTextRequest,
    ) -> Result<Json<CheckTextResponse>, String> {
        let options = katana_markdown_linter::LintOptions::default();
        let diagnostics = lint(&request.content, &options).map_err(|err| err.to_string())?;
        Ok(Json(CheckTextResponse::from_results(
            diagnostics,
            request_locale(request.locale.as_deref()),
        )))
    }

    pub(super) fn handle_fix_text(
        &self,
        request: FixTextRequest,
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

    pub(super) fn handle_config_validate(
        &self,
        request: ConfigValidateRequest,
    ) -> Json<ConfigValidateResponse> {
        let locale = request_locale(request.locale.as_deref());
        let errors = validate_config(request.config, locale);
        Json(ConfigValidateResponse {
            valid: errors.is_empty(),
            error_count: errors.len(),
            errors,
        })
    }

    pub(super) fn handle_rule_list(&self, request: RuleListRequest) -> Json<RuleListResponse> {
        let locale = request_locale(request.locale.as_deref());
        let rules = catalog_rules(locale);
        Json(RuleListResponse {
            locale: locale.code().to_string(),
            count: rules.len(),
            rules,
        })
    }

    pub(super) fn handle_rule_get(
        &self,
        request: RuleGetRequest,
    ) -> Result<Json<RuleMetadata>, String> {
        let rule_id = request.rule_id.to_ascii_uppercase();
        let locale = request_locale(request.locale.as_deref());
        catalog_rules(locale)
            .into_iter()
            .find(|rule| rule.id == rule_id)
            .map(Json)
            .ok_or_else(|| format!("unknown rule: {}", request.rule_id))
    }

    pub(super) fn handle_rule_doc_get(
        &self,
        request: RuleDocRequest,
    ) -> Result<Json<RuleDocResponse>, String> {
        let locale = request_locale(request.locale.as_deref());
        let content = upstream::get_rule_documentation(&request.rule_id, locale)?;
        Ok(Json(RuleDocResponse {
            rule_id: request.rule_id,
            locale: locale.code().to_string(),
            content,
        }))
    }

    pub(super) fn handle_check_file(
        &self,
        request: FileRequest,
    ) -> Result<Json<FileCheckResponse>, String> {
        let locale = request_locale(request.locale.as_deref());
        let result = self
            .workspace
            .lint_file(&request.path, request.config_path.as_deref())?;
        Ok(Json(file_check_response(&self.workspace, result, locale)))
    }

    pub(super) fn handle_check_directory(
        &self,
        request: DirectoryRequest,
    ) -> Result<Json<DirectoryCheckResponse>, String> {
        let locale = request_locale(request.locale.as_deref());
        let result = self.workspace.lint_directory(
            &request.path,
            request.config_path.as_deref(),
            request.respect_gitignore.unwrap_or(true),
        )?;
        let files = result
            .files
            .into_iter()
            .map(|file| file_check_response(&self.workspace, file, locale))
            .collect::<Vec<_>>();
        let issue_count = files.iter().map(|file| file.issue_count).sum();
        let errors = result
            .errors
            .into_iter()
            .map(|error| WorkspaceToolError {
                path: self.workspace.display_relative(&error.path),
                message: error.message,
            })
            .collect();
        Ok(Json(DirectoryCheckResponse {
            path: request.path,
            file_count: files.len(),
            issue_count,
            files,
            errors,
        }))
    }

    pub(super) fn handle_fix_file_preview(
        &self,
        request: FileRequest,
    ) -> Result<Json<FileFixPreviewResponse>, String> {
        let locale = request_locale(request.locale.as_deref());
        let preview = self
            .workspace
            .preview_fix(&request.path, request.config_path.as_deref())?;
        let remaining_issue_count = preview.remaining.len();
        Ok(Json(FileFixPreviewResponse {
            path: self.workspace.display_relative(&preview.path),
            changed: preview.changed,
            applied_fixes: preview.applied_fixes,
            diff: preview.diff,
            remaining_issue_count,
            remaining_diagnostics: diagnostics(preview.remaining, locale),
        }))
    }

    pub(super) fn handle_fix_file_apply(
        &self,
        request: FixFileApplyRequest,
    ) -> Result<Json<FileFixApplyResponse>, String> {
        if !request.apply {
            return Err("fix_file_apply requires apply: true".to_string());
        }
        let locale = request_locale(request.locale.as_deref());
        let result = self
            .workspace
            .apply_fix(&request.path, request.config_path.as_deref())?;
        let remaining_issue_count = result.remaining.len();
        Ok(Json(FileFixApplyResponse {
            path: self.workspace.display_relative(&result.path),
            changed: result.changed,
            applied_fixes: result.applied_fixes,
            remaining_issue_count,
            remaining_diagnostics: diagnostics(result.remaining, locale),
        }))
    }
}
