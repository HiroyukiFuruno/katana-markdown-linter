use super::{document, protocol};
use crate::config::load_effective_config;
use crate::{FormatOptions, LintOptions};
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
    last_config_error: Option<String>,
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
                let Some((uri, content)) = self.document_uri_and_content(&params) else {
                    return Ok(vec![protocol::response(id, json!([]))]);
                };
                let format_options = match self.get_format_options(uri) {
                    Ok(options) => options,
                    Err(err) => {
                        return Ok(vec![protocol::error_response(
                            id,
                            -32603,
                            &format!("Configuration error: {}", err),
                        )]);
                    }
                };
                let lint_options = match self.get_layout_lint_options(uri) {
                    Ok(options) => options,
                    Err(err) => {
                        return Ok(vec![protocol::error_response(
                            id,
                            -32603,
                            &format!("Configuration error: {}", err),
                        )]);
                    }
                };
                document::formatting_edits_with_options(content, &format_options, &lint_options)?
            }
            "textDocument/rangeFormatting" => {
                let Some((uri, content)) = self.document_uri_and_content(&params) else {
                    return Ok(vec![protocol::response(id, json!([]))]);
                };
                let format_options = match self.get_format_options(uri) {
                    Ok(options) => options,
                    Err(err) => {
                        return Ok(vec![protocol::error_response(
                            id,
                            -32603,
                            &format!("Configuration error: {}", err),
                        )]);
                    }
                };
                let lint_options = match self.get_layout_lint_options(uri) {
                    Ok(options) => options,
                    Err(err) => {
                        return Ok(vec![protocol::error_response(
                            id,
                            -32603,
                            &format!("Configuration error: {}", err),
                        )]);
                    }
                };
                document::range_formatting_edits_with_options(
                    content,
                    &params["range"],
                    &format_options,
                    &lint_options,
                )?
            }
            "textDocument/codeAction" => {
                let Some((uri, content)) = self.document_uri_and_content(&params) else {
                    return Ok(vec![protocol::response(id, json!([]))]);
                };
                match self.get_lint_options(uri) {
                    Ok(options) => document::code_actions(uri, content, &options)?,
                    Err(err) => {
                        return Ok(vec![protocol::error_response(
                            id,
                            -32603,
                            &format!("Configuration error: {}", err),
                        )]);
                    }
                }
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
            "workspace/didChangeWatchedFiles" => self.watched_files_changed(params),
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

        if is_config_file(uri) {
            return self.re_diagnose_all();
        }

        self.publish_diagnostics(uri)
    }

    fn watched_files_changed(&mut self, params: Value) -> Result<Vec<Value>, String> {
        let Some(changes) = params.get("changes").and_then(Value::as_array) else {
            return Ok(Vec::new());
        };

        let mut config_changed = false;
        for change in changes {
            if let Some(uri) = change.get("uri").and_then(Value::as_str) {
                if is_config_file(uri) {
                    config_changed = true;
                    break;
                }
            }
        }

        if config_changed {
            return self.re_diagnose_all();
        }

        Ok(Vec::new())
    }

    fn re_diagnose_all(&mut self) -> Result<Vec<Value>, String> {
        let mut notifications = Vec::new();
        let uris: Vec<String> = self.documents.keys().cloned().collect();
        for open_uri in uris {
            notifications.extend(self.publish_diagnostics(&open_uri)?);
        }
        Ok(notifications)
    }

    fn publish_diagnostics(&mut self, uri: &str) -> Result<Vec<Value>, String> {
        let Some(content) = self.documents.get(uri) else {
            return Ok(Vec::new());
        };
        let mut notifications = Vec::new();
        match self.get_lint_options(uri) {
            Ok(options) => {
                notifications.push(protocol::notification(
                    "textDocument/publishDiagnostics",
                    document::diagnostics(uri, content, &options)?,
                ));
                self.last_config_error = None;
            }
            Err(err) => {
                // Surface configuration errors as a diagnostic at the top of the file
                // instead of silently falling back to default config.
                let config_diagnostic = json!({
                    "range": {
                        "start": { "line": 0, "character": 0 },
                        "end": { "line": 0, "character": 0 }
                    },
                    "severity": 1, // Error
                    "code": "config-error",
                    "source": "kml",
                    "message": format!("Configuration error: {}", err)
                });

                notifications.push(protocol::notification(
                    "textDocument/publishDiagnostics",
                    json!({
                        "uri": uri,
                        "diagnostics": [config_diagnostic]
                    }),
                ));

                // Notify via showMessage for visibility, but avoid repeating same message
                if self.last_config_error.as_ref() != Some(&err) {
                    notifications.push(protocol::notification(
                        "window/showMessage",
                        json!({
                            "type": 1, // Error
                            "message": format!("Configuration error: {}", err)
                        }),
                    ));
                    self.last_config_error = Some(err);
                }
            }
        }
        Ok(notifications)
    }

    fn get_lint_options(&self, uri: &str) -> Result<LintOptions, String> {
        let path = uri_path(uri);
        let config = load_effective_config(&path, None)?;
        Ok(config.to_lint_options())
    }

    fn get_format_options(&self, _uri: &str) -> Result<FormatOptions, String> {
        Ok(FormatOptions::default())
    }

    fn get_layout_lint_options(&self, uri: &str) -> Result<LintOptions, String> {
        let path = uri_path(uri);
        let config = load_effective_config(&path, None)?;
        Ok(crate::formatter::layout_lint_options_from(
            &config.to_lint_options(),
        ))
    }

    fn document_uri_and_content<'a>(&'a self, params: &'a Value) -> Option<(&'a str, &'a str)> {
        let uri = text_document_uri(params)?;
        self.documents
            .get(uri)
            .map(|content| (uri, content.as_str()))
    }
}

use super::uri_path;

fn is_config_file(uri: &str) -> bool {
    let path = uri_path(uri);
    let Some(filename) = path.file_name().and_then(|f| f.to_str()) else {
        return false;
    };
    filename == ".markdownlint.json" || filename == ".markdownlint.jsonc"
}

fn text_document_uri(params: &Value) -> Option<&str> {
    params
        .get("textDocument")
        .and_then(|document| document.get("uri"))
        .and_then(Value::as_str)
}
