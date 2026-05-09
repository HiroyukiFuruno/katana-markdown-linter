use super::LspServer;
use crate::lsp::{document, protocol::LspProtocol};
use serde_json::{json, Value};

const JSON_RPC_INTERNAL_ERROR: i32 = -32603;

impl LspServer {
    pub(super) fn handle_request(
        &mut self,
        id: Value,
        method: &str,
        params: Value,
    ) -> Result<Vec<Value>, String> {
        let result = match method {
            "initialize" => self.initialize_result(),
            "shutdown" => {
                self.shutdown_requested = true;
                Value::Null
            }
            "textDocument/formatting" => return self.handle_formatting_request(id, params),
            "textDocument/rangeFormatting" => {
                return self.handle_range_formatting_request(id, params);
            }
            "textDocument/codeAction" => return self.handle_code_action_request(id, params),
            _ => Value::Null,
        };
        Ok(vec![LspProtocol::response(id, result)])
    }

    fn handle_formatting_request(&self, id: Value, params: Value) -> Result<Vec<Value>, String> {
        let Some((uri, content)) = self.document_uri_and_content(&params) else {
            return Ok(vec![LspProtocol::response(id, json!([]))]);
        };
        let format_options = match self.get_format_options(uri) {
            Ok(options) => options,
            Err(err) => return Ok(vec![configuration_error(id, err)]),
        };
        let lint_options = match self.get_layout_lint_options(uri) {
            Ok(options) => options,
            Err(err) => return Ok(vec![configuration_error(id, err)]),
        };
        Ok(vec![LspProtocol::response(
            id,
            document::LspDocumentOps::formatting_edits_with_options(
                content,
                &format_options,
                &lint_options,
            )?,
        )])
    }

    fn handle_range_formatting_request(
        &self,
        id: Value,
        params: Value,
    ) -> Result<Vec<Value>, String> {
        let Some((uri, content)) = self.document_uri_and_content(&params) else {
            return Ok(vec![LspProtocol::response(id, json!([]))]);
        };
        let format_options = match self.get_format_options(uri) {
            Ok(options) => options,
            Err(err) => return Ok(vec![configuration_error(id, err)]),
        };
        let lint_options = match self.get_layout_lint_options(uri) {
            Ok(options) => options,
            Err(err) => return Ok(vec![configuration_error(id, err)]),
        };
        Ok(vec![LspProtocol::response(
            id,
            document::LspDocumentOps::range_formatting_edits_with_options(
                content,
                &params["range"],
                &format_options,
                &lint_options,
            )?,
        )])
    }

    fn handle_code_action_request(&self, id: Value, params: Value) -> Result<Vec<Value>, String> {
        let Some((uri, content)) = self.document_uri_and_content(&params) else {
            return Ok(vec![LspProtocol::response(id, json!([]))]);
        };
        let options = match self.get_lint_options(uri) {
            Ok(options) => options,
            Err(err) => return Ok(vec![configuration_error(id, err)]),
        };
        Ok(vec![LspProtocol::response(
            id,
            document::LspDocumentOps::code_actions(uri, content, &options)?,
        )])
    }
}

fn configuration_error(id: Value, err: String) -> Value {
    LspProtocol::error_response(
        id,
        JSON_RPC_INTERNAL_ERROR,
        &format!("Configuration error: {}", err),
    )
}
