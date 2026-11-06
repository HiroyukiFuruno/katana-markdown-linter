use super::LspServer;
use crate::lsp::{document, protocol::LspProtocol};
use serde_json::{json, Value};

impl LspServer {
    pub(super) fn publish_diagnostics(&mut self, uri: &str) -> Result<Vec<Value>, String> {
        let Some(content) = self.documents.get(uri) else {
            return Ok(Vec::new());
        };
        match self.get_lint_options(uri) {
            Ok(options) => {
                let payload = document::LspDocumentOps::diagnostics(uri, content, &options)?;
                self.last_config_error = None;
                Ok(vec![publish_notification(payload)])
            }
            Err(err) => Ok(self.config_error_notifications(uri, err)),
        }
    }

    fn config_error_notifications(&mut self, uri: &str, err: String) -> Vec<Value> {
        let message = format!("Configuration error: {err}");
        let mut notifications = vec![publish_notification(config_error_payload(uri, &message))];

        /* WHY: showMessage mirrors the diagnostic, but repeats are suppressed to avoid editor noise. */
        if self.last_config_error.as_ref() != Some(&err) {
            notifications.push(LspProtocol::notification(
                "window/showMessage",
                json!({ "type": 1, "message": message }),
            ));
            self.last_config_error = Some(err);
        }
        notifications
    }
}

fn publish_notification(payload: Value) -> Value {
    LspProtocol::notification("textDocument/publishDiagnostics", payload)
}

/* WHY: Configuration errors must be visible in editors instead of hidden behind default config. */
fn config_error_payload(uri: &str, message: &str) -> Value {
    json!({
        "uri": uri,
        "diagnostics": [{
            "range": {
                "start": { "line": 0, "character": 0 },
                "end": { "line": 0, "character": 0 }
            },
            "severity": 1,
            "code": "config-error",
            "source": "kml",
            "message": message
        }]
    })
}
