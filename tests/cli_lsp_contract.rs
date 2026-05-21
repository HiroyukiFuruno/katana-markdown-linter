use serde_json::Value;
use std::fs::File;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static LSP_INPUT_COUNTER: AtomicU64 = AtomicU64::new(0);

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

#[test]
fn lsp_respects_workspace_config() {
    let temp_dir = std::env::temp_dir().join(format!(
        "kml-test-workspace-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let config_path = temp_dir.join(".markdownlint.json");
    std::fs::write(&config_path, r#"{ "MD018": false }"#).unwrap();

    let md_path = temp_dir.join("bad.md");
    let md_uri = format!("file://{}", md_path.to_str().unwrap());

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
                    "uri": md_uri,
                    "languageId": "markdown",
                    "version": 1,
                    "text": "#Title\n"
                }
            }),
        )),
        frame(request(2, "shutdown", serde_json::json!(null))),
        frame(notification("exit", serde_json::json!(null))),
    ]
    .join("");

    let output = run_lsp(&input);
    assert!(output.status.success());
    let messages = decode_messages(&output.stdout);

    let diagnostics = messages
        .iter()
        .find(|message| message["method"] == "textDocument/publishDiagnostics")
        .expect("diagnostics notification should be sent");

    let diagnostic_codes = diagnostics["params"]["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array")
        .iter()
        .map(|diagnostic| diagnostic["code"].as_str().unwrap())
        .collect::<Vec<_>>();

    // MD018 (no-atx-spacing) should be disabled by config
    assert!(
        !diagnostic_codes.contains(&"MD018"),
        "MD018 should be disabled by config, but found in {:?}",
        diagnostic_codes
    );

    let _ = std::fs::remove_dir_all(temp_dir);
}

#[test]
fn lsp_respects_jsonc_config() {
    let temp_dir = std::env::temp_dir().join(format!(
        "kml-test-workspace-jsonc-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let config_path = temp_dir.join(".markdownlint.jsonc");
    // JSONC with comments
    std::fs::write(
        &config_path,
        r#"{
        // Disable MD018
        "MD018": false
    }"#,
    )
    .unwrap();

    let md_path = temp_dir.join("bad.md");
    let md_uri = format!("file://{}", md_path.to_str().unwrap());

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
                    "uri": md_uri,
                    "languageId": "markdown",
                    "version": 1,
                    "text": "#Title\n"
                }
            }),
        )),
        frame(request(2, "shutdown", serde_json::json!(null))),
        frame(notification("exit", serde_json::json!(null))),
    ]
    .join("");

    let output = run_lsp(&input);
    assert!(output.status.success());
    let messages = decode_messages(&output.stdout);

    let diagnostics = messages
        .iter()
        .find(|message| message["method"] == "textDocument/publishDiagnostics")
        .expect("diagnostics notification should be sent");

    let diagnostic_codes = diagnostics["params"]["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array")
        .iter()
        .map(|diagnostic| diagnostic["code"].as_str().unwrap())
        .collect::<Vec<_>>();

    assert!(
        !diagnostic_codes.contains(&"MD018"),
        "MD018 should be disabled by jsonc config, but found in {:?}",
        diagnostic_codes
    );

    let _ = std::fs::remove_dir_all(temp_dir);
}

#[test]
fn lsp_re_diagnoses_on_watched_config_change() {
    let temp_dir = std::env::temp_dir().join(format!(
        "kml-test-workspace-watch-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let config_path = temp_dir.join(".markdownlint.json");
    // Initially enable MD018 (default)
    std::fs::write(&config_path, r#"{ "MD018": true }"#).unwrap();

    let md_path = temp_dir.join("bad.md");
    let md_uri = format!("file://{}", md_path.to_str().unwrap());
    let config_uri = format!("file://{}", config_path.to_str().unwrap());

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
                    "uri": md_uri,
                    "languageId": "markdown",
                    "version": 1,
                    "text": "#Title\n"
                }
            }),
        )),
        // Change config to disable MD018
        {
            std::fs::write(&config_path, r#"{ "MD018": false }"#).unwrap();
            frame(notification(
                "workspace/didChangeWatchedFiles",
                serde_json::json!({
                    "changes": [
                        { "uri": config_uri, "type": 2 } // Changed
                    ]
                }),
            ))
        },
        frame(request(2, "shutdown", serde_json::json!(null))),
        frame(notification("exit", serde_json::json!(null))),
    ]
    .join("");

    let output = run_lsp(&input);
    assert!(output.status.success());
    let messages = decode_messages(&output.stdout);

    let diagnostics_notifications: Vec<_> = messages
        .iter()
        .filter(|message| message["method"] == "textDocument/publishDiagnostics")
        .collect();

    // Should have 2 diagnostics notifications: one from didOpen, one from watched files change
    assert!(diagnostics_notifications.len() >= 2);

    let last_diagnostics = diagnostics_notifications.last().unwrap();
    let diagnostic_codes = last_diagnostics["params"]["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array")
        .iter()
        .map(|diagnostic| diagnostic["code"].as_str().unwrap())
        .collect::<Vec<_>>();

    assert!(
        !diagnostic_codes.contains(&"MD018"),
        "MD018 should be disabled after watched file change, but found in {:?}",
        diagnostic_codes
    );

    let _ = std::fs::remove_dir_all(temp_dir);
}

#[test]
fn lsp_config_rule_options_reflect_in_diagnostics_and_quick_fixes() {
    let temp_dir = std::env::temp_dir().join(format!(
        "kml-test-workspace-options-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let config_path = temp_dir.join(".markdownlint.json");
    // Configure MD049 (emphasis-style) to 'underscore'
    std::fs::write(&config_path, r#"{ "MD049": { "style": "underscore" } }"#).unwrap();

    let md_path = temp_dir.join("style.md");
    let md_uri = format!("file://{}", md_path.to_str().unwrap());

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
                    "uri": md_uri,
                    "languageId": "markdown",
                    "version": 1,
                    "text": "*Asterisk emphasis*\n"
                }
            }),
        )),
        frame(request(
            2,
            "textDocument/codeAction",
            serde_json::json!({
                "textDocument": { "uri": md_uri },
                "range": {
                    "start": { "line": 0, "character": 0 },
                    "end": { "line": 0, "character": 1 }
                },
                "context": { "diagnostics": [] }
            }),
        )),
        frame(request(3, "shutdown", serde_json::json!(null))),
        frame(notification("exit", serde_json::json!(null))),
    ]
    .join("");

    let output = run_lsp(&input);
    assert!(output.status.success());
    let messages = decode_messages(&output.stdout);

    let diagnostics = messages
        .iter()
        .find(|message| message["method"] == "textDocument/publishDiagnostics")
        .expect("diagnostics notification should be sent");

    let md049_diagnostic = diagnostics["params"]["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["code"] == "MD049")
        .expect("MD049 diagnostic should be present for asterisk when underscore is configured");

    assert!(md049_diagnostic["message"]
        .as_str()
        .unwrap()
        .contains("Emphasis style"));

    let code_actions = response(&messages, 2);
    let md049_action = code_actions["result"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["title"].as_str().unwrap().contains("MD049"))
        .expect("MD049 quick fix should be present");

    let changes = &md049_action["edit"]["changes"][&md_uri];
    assert_eq!(changes[0]["newText"], "_Asterisk emphasis_");

    let _ = std::fs::remove_dir_all(temp_dir);
}

#[test]
fn lsp_formatting_returns_error_on_malformed_config() {
    let temp_dir = std::env::temp_dir().join(format!(
        "kml-test-workspace-fmt-error-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let config_path = temp_dir.join(".markdownlint.json");
    std::fs::write(&config_path, r#"{ "MD018": "#).unwrap(); // Malformed

    let md_path = temp_dir.join("bad.md");
    let md_uri = format!("file://{}", md_path.to_str().unwrap());

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
                    "uri": md_uri,
                    "languageId": "markdown",
                    "version": 1,
                    "text": "#Title\n"
                }
            }),
        )),
        frame(request(
            2,
            "textDocument/formatting",
            serde_json::json!({
                "textDocument": { "uri": md_uri },
                "options": { "tabSize": 2, "insertSpaces": true }
            }),
        )),
        frame(request(3, "shutdown", serde_json::json!(null))),
        frame(notification("exit", serde_json::json!(null))),
    ]
    .join("");

    let output = run_lsp(&input);
    assert!(output.status.success());
    let messages = decode_messages(&output.stdout);

    let formatting_response = response(&messages, 2);
    assert!(formatting_response.get("error").is_some());
    assert!(formatting_response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Configuration error"));

    let _ = std::fs::remove_dir_all(temp_dir);
}

#[test]
fn lsp_handles_malformed_config() {
    let temp_dir = std::env::temp_dir().join(format!(
        "kml-test-workspace-malformed-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let config_path = temp_dir.join(".markdownlint.json");
    std::fs::write(&config_path, r#"{ "MD018": "#).unwrap(); // Malformed JSON

    let md_path = temp_dir.join("bad.md");
    let md_uri = format!("file://{}", md_path.to_str().unwrap());

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
                    "uri": md_uri,
                    "languageId": "markdown",
                    "version": 1,
                    "text": "#Title\n"
                }
            }),
        )),
        frame(request(2, "shutdown", serde_json::json!(null))),
        frame(notification("exit", serde_json::json!(null))),
    ]
    .join("");

    let output = run_lsp(&input);
    // It should not crash (status success)
    assert!(output.status.success());
    let messages = decode_messages(&output.stdout);

    // Should receive window/showMessage notification for the error
    let show_message = messages
        .iter()
        .find(|message| message["method"] == "window/showMessage")
        .expect("window/showMessage notification should be sent for malformed config");
    assert_eq!(show_message["params"]["type"], 1); // Error
    assert!(show_message["params"]["message"]
        .as_str()
        .unwrap()
        .contains("Configuration error"));

    let diagnostics = messages
        .iter()
        .find(|message| message["method"] == "textDocument/publishDiagnostics")
        .expect("diagnostics notification should be sent");

    let diagnostic_codes = diagnostics["params"]["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array")
        .iter()
        .map(|diagnostic| diagnostic["code"].as_str().unwrap())
        .collect::<Vec<_>>();

    // Should NOT fallback to default. It should report config-error and NO MD018.
    assert!(
        diagnostic_codes.contains(&"config-error"),
        "config-error diagnostic should be present, but found in {:?}",
        diagnostic_codes
    );
    assert!(
        !diagnostic_codes.contains(&"MD018"),
        "MD018 should NOT be present when config is malformed, but found in {:?}",
        diagnostic_codes
    );

    let _ = std::fs::remove_dir_all(temp_dir);
}

fn run_lsp(input: &str) -> std::process::Output {
    let input_id = LSP_INPUT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let input_path = std::env::temp_dir().join(format!(
        "katana-markdown-linter-lsp-{}-{}-{}.input",
        std::process::id(),
        input_id,
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
