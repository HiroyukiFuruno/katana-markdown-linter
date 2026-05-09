mod config;
mod diagnostics;
mod notifications;
mod requests;

use super::protocol::LspProtocol;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

pub(crate) struct LspServerRunner;

impl LspServerRunner {
    pub(crate) fn run_stdio() -> Result<(), String> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut reader = stdin.lock();
        let mut writer = stdout.lock();
        LspServer::default().run(&mut reader, &mut writer)
    }
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
        while let Some(message) = LspProtocol::read_message(reader)? {
            let outgoing = self.handle_message(message)?;
            for message in outgoing {
                LspProtocol::write_message(writer, &message)?;
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
}
