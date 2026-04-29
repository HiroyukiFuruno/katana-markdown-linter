use serde_json::{json, Value};
use std::io::{BufRead, Write};

pub(crate) fn read_message(reader: &mut impl BufRead) -> Result<Option<Value>, String> {
    let Some(length) = read_content_length(reader)? else {
        return Ok(None);
    };
    let mut payload = vec![0; length];
    reader
        .read_exact(&mut payload)
        .map_err(|err| format!("failed to read LSP payload: {err}"))?;
    serde_json::from_slice(&payload)
        .map(Some)
        .map_err(|err| format!("failed to parse LSP payload: {err}"))
}

pub(crate) fn write_message(writer: &mut impl Write, message: &Value) -> Result<(), String> {
    let payload = serde_json::to_vec(message).map_err(|err| err.to_string())?;
    write!(writer, "Content-Length: {}\r\n\r\n", payload.len()).map_err(|err| err.to_string())?;
    writer.write_all(&payload).map_err(|err| err.to_string())?;
    writer.flush().map_err(|err| err.to_string())
}

pub(crate) fn response(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    })
}

pub(crate) fn notification(method: &str, params: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params
    })
}

fn read_content_length(reader: &mut impl BufRead) -> Result<Option<usize>, String> {
    let mut content_length = None;
    loop {
        let mut line = String::new();
        let read = reader
            .read_line(&mut line)
            .map_err(|err| format!("failed to read LSP header: {err}"))?;
        if read == 0 {
            return Ok(None);
        }

        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            return content_length
                .map(Some)
                .ok_or_else(|| "missing Content-Length header".to_string());
        }

        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.eq_ignore_ascii_case("Content-Length") {
            content_length = Some(
                value
                    .trim()
                    .parse::<usize>()
                    .map_err(|err| format!("invalid Content-Length header: {err}"))?,
            );
        }
    }
}
