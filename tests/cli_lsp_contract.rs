use serde_json::Value;
use std::fs::File;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn lsp_initialize_reports_diagnostics_and_formats_open_document() {
    let input = [
        frame(request(
            1,
            "initialize",
            serde_json::json!({ "capabilities": {} }),
        )),
        frame(notification(
            "textDocument/didOpen",
            serde_json::json!({
                "textDocument": {
                    "uri": "file:///workspace/bad.md",
                    "languageId": "markdown",
                    "version": 1,
                    "text": "#Title\r\nText\n\n\n"
                }
            }),
        )),
        frame(request(
            2,
            "textDocument/formatting",
            serde_json::json!({
                "textDocument": { "uri": "file:///workspace/bad.md" },
                "options": { "tabSize": 2, "insertSpaces": true }
            }),
        )),
        frame(request(
            3,
            "textDocument/rangeFormatting",
            serde_json::json!({
                "textDocument": { "uri": "file:///workspace/bad.md" },
                "range": {
                    "start": { "line": 1, "character": 0 },
                    "end": { "line": 4, "character": 0 }
                },
                "options": { "tabSize": 2, "insertSpaces": true }
            }),
        )),
        frame(request(
            4,
            "textDocument/codeAction",
            serde_json::json!({
                "textDocument": { "uri": "file:///workspace/bad.md" },
                "range": {
                    "start": { "line": 0, "character": 0 },
                    "end": { "line": 0, "character": 6 }
                },
                "context": { "diagnostics": [] }
            }),
        )),
        frame(request(5, "shutdown", serde_json::json!(null))),
        frame(notification("exit", serde_json::json!(null))),
    ]
    .join("");

    let output = run_lsp(&input);
    assert!(output.status.success());
    let messages = decode_messages(&output.stdout);

    let initialize = response(&messages, 1);
    assert_eq!(
        initialize["result"]["serverInfo"]["name"],
        "katana-markdown-linter"
    );
    assert_eq!(
        initialize["result"]["capabilities"]["documentFormattingProvider"],
        true
    );

    let diagnostics = messages
        .iter()
        .find(|message| message["method"] == "textDocument/publishDiagnostics")
        .expect("diagnostics notification should be sent");
    let diagnostic_codes = diagnostics["params"]["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array")
        .iter()
        .map(|diagnostic| diagnostic["code"].clone())
        .collect::<Vec<_>>();
    assert!(diagnostic_codes.contains(&Value::String("MD018".to_string())));

    let formatting = response(&messages, 2);
    assert_eq!(formatting["result"][0]["newText"], "#Title\nText\n");

    let range_formatting = response(&messages, 3);
    assert_eq!(range_formatting["result"][0]["range"]["start"]["line"], 1);
    assert_eq!(range_formatting["result"][0]["range"]["end"]["line"], 4);
    assert_eq!(range_formatting["result"][0]["newText"], "Text\n");

    let code_actions = response(&messages, 4);
    assert_eq!(code_actions["result"][0]["kind"], "quickfix");
    assert!(code_actions["result"][0]["edit"]["changes"]
        .get("file:///workspace/bad.md")
        .is_some());
    assert_eq!(response(&messages, 5)["result"], Value::Null);
}

fn run_lsp(input: &str) -> std::process::Output {
    let input_path = std::env::temp_dir().join(format!(
        "katana-markdown-linter-lsp-{}-{}.input",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos()
    ));
    std::fs::write(&input_path, input).expect("lsp input file should be written");

    let mut command = Command::new(env!("CARGO_BIN_EXE_kml"));
    command.arg("lsp");
    command.stdin(Stdio::from(
        File::open(&input_path).expect("lsp input file should open"),
    ));
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = command.output().expect("kml lsp should finish");
    let _ = std::fs::remove_file(input_path);
    output
}

fn request(id: i64, method: &str, params: Value) -> Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params
    })
}

fn notification(method: &str, params: Value) -> Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params
    })
}

fn frame(message: Value) -> String {
    let payload = serde_json::to_string(&message).expect("message should serialize");
    format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload)
}

fn decode_messages(output: &[u8]) -> Vec<Value> {
    let mut rest = output;
    let mut messages = Vec::new();
    while !rest.is_empty() {
        let header_end = rest
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .expect("header terminator should exist");
        let header = std::str::from_utf8(&rest[..header_end]).expect("header should be utf8");
        let length = header
            .lines()
            .find_map(|line| line.strip_prefix("Content-Length: "))
            .expect("content length should exist")
            .parse::<usize>()
            .expect("content length should be numeric");
        let payload_start = header_end + 4;
        let payload_end = payload_start + length;
        let payload = &rest[payload_start..payload_end];
        messages.push(serde_json::from_slice(payload).expect("payload should be json"));
        rest = &rest[payload_end..];
    }
    messages
}

fn response(messages: &[Value], id: i64) -> &Value {
    messages
        .iter()
        .find(|message| message["id"] == id)
        .expect("response should exist")
}
