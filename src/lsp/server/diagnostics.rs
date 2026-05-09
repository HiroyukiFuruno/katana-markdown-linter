use super::LspServer;
use crate::lsp::{document, protocol::LspProtocol};
use serde_json::{json, Value};

impl LspServer {
    pub(super) fn publish_diagnostics(&mut self, uri: &str) -> Result<Vec<Value>, String> {
        let Some(content) = self.documents.get(uri) else {
            return Ok(Vec::new());
        };
        let mut notifications = Vec::new();
        match self.get_lint_options(uri) {
            Ok(options) => {
                notifications.push(LspProtocol::notification(
                    "textDocument/publishDiagnostics",
                    document::LspDocumentOps::diagnostics(uri, content, &options)?,
                ));
                self.last_config_error = None;
            }
            Err(err) => {
                /* WHY: Configuration errors must be visible in editors instead of hidden behind default config. */
                let config_diagnostic = json!({
                    "range": {
                        "start": { "line": 0, "character": 0 },
                        "end": { "line": 0, "character": 0 }
                    },
                    "severity": 1,
                    "code": "config-error",
                    "source": "kml",
                    "message": format!("Configuration error: {}", err)
                });

                notifications.push(LspProtocol::notification(
                    "textDocument/publishDiagnostics",
                    json!({
                        "uri": uri,
                        "diagnostics": [config_diagnostic]
                    }),
                ));

                /* WHY: showMessage mirrors the diagnostic, but repeats are suppressed to avoid editor noise. */
                if self.last_config_error.as_ref() != Some(&err) {
                    notifications.push(LspProtocol::notification(
                        "window/showMessage",
                        json!({
                            "type": 1,
                            "message": format!("Configuration error: {}", err)
                        }),
                    ));
                    self.last_config_error = Some(err);
                }
            }
        }
        Ok(notifications)
    }
}
