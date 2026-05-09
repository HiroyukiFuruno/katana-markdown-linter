use super::config::{is_config_file, text_document_uri};
use super::LspServer;
use serde_json::Value;

impl LspServer {
    pub(super) fn handle_notification(
        &mut self,
        method: &str,
        params: Value,
    ) -> Result<Vec<Value>, String> {
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

        let config_changed = changes.iter().any(|change| {
            change
                .get("uri")
                .and_then(Value::as_str)
                .is_some_and(is_config_file)
        });
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
}
