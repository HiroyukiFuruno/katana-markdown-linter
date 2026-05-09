use super::LspServer;
use crate::config::ConfigLoader;
use crate::lsp::LspUri;
use crate::{FormatOptions, LintOptions};
use serde_json::Value;

impl LspServer {
    pub(super) fn get_lint_options(&self, uri: &str) -> Result<LintOptions, String> {
        let path = LspUri::path(uri);
        let config = ConfigLoader::load_effective_config(&path, None)?;
        Ok(config.to_lint_options())
    }

    pub(super) fn get_format_options(&self, _uri: &str) -> Result<FormatOptions, String> {
        Ok(FormatOptions::default())
    }

    pub(super) fn get_layout_lint_options(&self, uri: &str) -> Result<LintOptions, String> {
        let path = LspUri::path(uri);
        let config = ConfigLoader::load_effective_config(&path, None)?;
        Ok(crate::MarkdownFormatter::layout_lint_options_from(
            &config.to_lint_options(),
        ))
    }

    pub(super) fn document_uri_and_content<'a>(
        &'a self,
        params: &'a Value,
    ) -> Option<(&'a str, &'a str)> {
        let uri = text_document_uri(params)?;
        self.documents
            .get(uri)
            .map(|content| (uri, content.as_str()))
    }
}

pub(super) fn is_config_file(uri: &str) -> bool {
    let path = LspUri::path(uri);
    let Some(filename) = path.file_name().and_then(|file| file.to_str()) else {
        return false;
    };
    filename == ".markdownlint.json" || filename == ".markdownlint.jsonc"
}

pub(super) fn text_document_uri(params: &Value) -> Option<&str> {
    params
        .get("textDocument")
        .and_then(|document| document.get("uri"))
        .and_then(Value::as_str)
}
