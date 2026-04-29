use super::{document, protocol};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

pub(crate) fn run_stdio() -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();
    LspServer::default().run(&mut reader, &mut writer)
}

#[derive(Default)]
struct LspServer {
    documents: HashMap<String, String>,
    shutdown_requested: bool,
    exit_requested: bool,
}

impl LspServer {
    fn run(&mut self, reader: &mut impl BufRead, writer: &mut impl Write) -> Result<(), String> {
        while let Some(message) = protocol::read_message(reader)? {
            let outgoing = self.handle_message(message)?;
            for message in outgoing {
                protocol::write_message(writer, &message)?;
            }
            if self.shutdown_requested || self.exit_requested {
                break;
            }
        }
        Ok(())
    }

    fn handle_message(&mut self, message: Value) -> Result<Vec<Value>, String> {
        let method = message
            .get("method")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let id = message.get("id").cloned();
        let params = message.get("params").cloned().unwrap_or(Value::Null);

        if let Some(id) = id {
            return self.handle_request(id, method, params);
        }
        self.handle_notification(method, params)
    }

    fn handle_request(
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
            "textDocument/formatting" => {
                let Some(content) = self.document_content(&params) else {
                    return Ok(vec![protocol::response(id, json!([]))]);
                };
                document::formatting_edits(content)?
            }
            "textDocument/rangeFormatting" => {
                let Some(content) = self.document_content(&params) else {
                    return Ok(vec![protocol::response(id, json!([]))]);
                };
                document::range_formatting_edits(content, &params["range"])?
            }
            "textDocument/codeAction" => {
                let Some((uri, content)) = self.document_uri_and_content(&params) else {
                    return Ok(vec![protocol::response(id, json!([]))]);
                };
                document::code_actions(uri, content)?
            }
            _ => Value::Null,
        };
        Ok(vec![protocol::response(id, result)])
    }

    fn handle_notification(&mut self, method: &str, params: Value) -> Result<Vec<Value>, String> {
        match method {
            "initialized" => Ok(Vec::new()),
            "exit" => {
                self.exit_requested = true;
                Ok(Vec::new())
            }
            "textDocument/didOpen" => self.open_document(params),
            "textDocument/didChange" => self.change_document(params),
            "textDocument/didSave" => self.saved_document(params),
            _ => Ok(Vec::new()),
        }
    }

    fn initialize_result(&self) -> Value {
        json!({
            "capabilities": {
                "textDocumentSync": 1,
                "documentFormattingProvider": true,
                "documentRangeFormattingProvider": true,
                "codeActionProvider": true
            },
            "serverInfo": {
                "name": "katana-markdown-linter",
                "version": env!("CARGO_PKG_VERSION")
            }
        })
    }

    fn open_document(&mut self, params: Value) -> Result<Vec<Value>, String> {
        let Some(document) = params.get("textDocument") else {
            return Ok(Vec::new());
        };
        let Some(uri) = document.get("uri").and_then(Value::as_str) else {
            return Ok(Vec::new());
        };
        let content = document
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        self.documents.insert(uri.to_string(), content);
        self.publish_diagnostics(uri)
    }

    fn change_document(&mut self, params: Value) -> Result<Vec<Value>, String> {
        let Some(uri) = text_document_uri(&params) else {
            return Ok(Vec::new());
        };
        let Some(text) = params
            .get("contentChanges")
            .and_then(Value::as_array)
            .and_then(|changes| changes.first())
            .and_then(|change| change.get("text"))
            .and_then(Value::as_str)
        else {
            return Ok(Vec::new());
        };
        self.documents.insert(uri.to_string(), text.to_string());
        self.publish_diagnostics(uri)
    }

    fn saved_document(&mut self, params: Value) -> Result<Vec<Value>, String> {
        let Some(uri) = text_document_uri(&params) else {
            return Ok(Vec::new());
        };
        self.publish_diagnostics(uri)
    }

    fn publish_diagnostics(&self, uri: &str) -> Result<Vec<Value>, String> {
        let Some(content) = self.documents.get(uri) else {
            return Ok(Vec::new());
        };
        Ok(vec![protocol::notification(
            "textDocument/publishDiagnostics",
            document::diagnostics(uri, content)?,
        )])
    }

    fn document_content(&self, params: &Value) -> Option<&str> {
        let uri = text_document_uri(params)?;
        self.documents.get(uri).map(String::as_str)
    }

    fn document_uri_and_content<'a>(&'a self, params: &'a Value) -> Option<(&'a str, &'a str)> {
        let uri = text_document_uri(params)?;
        self.documents
            .get(uri)
            .map(|content| (uri, content.as_str()))
    }
}

fn text_document_uri(params: &Value) -> Option<&str> {
    params
        .get("textDocument")
        .and_then(|document| document.get("uri"))
        .and_then(Value::as_str)
}
