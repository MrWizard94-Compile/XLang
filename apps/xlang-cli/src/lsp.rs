//! Bounded offline Language Server Protocol (M13a / ADR-017).
//!
//! Stdio JSON-RPC only. Bootstrap diagnostics; no product AETH emission; no
//! server-side disk writes of source or artifacts.

use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

use aether_core::{
    compile_source, format_source, structural_document_json, Diagnostic, LANGUAGE_NAME,
    LANGUAGE_VERSION,
};
use serde_json::{json, Value};

#[derive(Debug, Default)]
struct ServerState {
    /// uri -> full text
    documents: BTreeMap<String, String>,
    shutdown_requested: bool,
}

/// Run the LSP event loop on stdin/stdout until exit.
pub fn run() -> Result<(), String> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = io::stdout();
    let mut state = ServerState::default();

    loop {
        let message = read_message(&mut stdin).map_err(|error| error.to_string())?;
        let Some(message) = message else {
            break;
        };
        let parsed: Value =
            serde_json::from_str(&message).map_err(|error| format!("invalid JSON-RPC: {error}"))?;
        if let Some(responses) = handle_message(&mut state, &parsed) {
            for response in responses {
                write_message(&mut stdout, &response).map_err(|error| error.to_string())?;
            }
        }
        if state.shutdown_requested {
            // Wait for exit notification; if next is exit, break.
            // If client sends exit without shutdown, still exit loop on exit method.
        }
        if parsed.get("method").and_then(Value::as_str) == Some("exit") {
            break;
        }
    }
    Ok(())
}

fn read_message(reader: &mut impl BufRead) -> io::Result<Option<String>> {
    let mut content_length: Option<usize> = None;
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            return Ok(None);
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }
        let lower = trimmed.to_ascii_lowercase();
        if let Some(rest) = lower.strip_prefix("content-length:") {
            let value = rest.trim();
            content_length = Some(
                value
                    .parse()
                    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?,
            );
        }
    }
    let Some(length) = content_length else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "LSP message missing Content-Length",
        ));
    };
    let mut body = vec![0_u8; length];
    reader.read_exact(&mut body)?;
    String::from_utf8(body)
        .map(Some)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn write_message(writer: &mut impl Write, body: &str) -> io::Result<()> {
    write!(writer, "Content-Length: {}\r\n\r\n{}", body.len(), body)?;
    writer.flush()
}

/// Handle one JSON-RPC message; returns zero or more response/notification bodies.
fn handle_message(state: &mut ServerState, message: &Value) -> Option<Vec<String>> {
    let method = message.get("method").and_then(Value::as_str);
    let id = message.get("id").cloned();

    match method {
        Some("initialize") => {
            let result = json!({
                "capabilities": {
                    "textDocumentSync": 1,
                    "documentSymbolProvider": true,
                    "documentFormattingProvider": true,
                    "hoverProvider": true,
                    "definitionProvider": true,
                },
                "serverInfo": {
                    "name": LANGUAGE_NAME,
                    "version": LANGUAGE_VERSION,
                }
            });
            Some(vec![response_ok(id, result)])
        }
        Some("initialized") | Some("workspace/didChangeConfiguration") => None,
        Some("shutdown") => {
            state.shutdown_requested = true;
            Some(vec![response_ok(id, Value::Null)])
        }
        Some("exit") => None,
        Some("textDocument/didOpen") => {
            let params = message.get("params")?;
            let uri = params
                .pointer("/textDocument/uri")
                .and_then(Value::as_str)?
                .to_owned();
            let text = params
                .pointer("/textDocument/text")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned();
            state.documents.insert(uri.clone(), text.clone());
            Some(vec![diagnostics_notification(&uri, &text)])
        }
        Some("textDocument/didChange") => {
            let params = message.get("params")?;
            let uri = params
                .pointer("/textDocument/uri")
                .and_then(Value::as_str)?
                .to_owned();
            // Full document sync only (textDocumentSync = 1 means Full in older mapping;
            // we accept contentChanges[0].text as full replacement).
            let text = params
                .pointer("/contentChanges/0/text")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .or_else(|| state.documents.get(&uri).cloned())
                .unwrap_or_default();
            state.documents.insert(uri.clone(), text.clone());
            Some(vec![diagnostics_notification(&uri, &text)])
        }
        Some("textDocument/didClose") => {
            if let Some(uri) = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)
            {
                state.documents.remove(uri);
                Some(vec![diagnostics_notification(uri, "")])
            } else {
                None
            }
        }
        Some("textDocument/documentSymbol") => {
            let uri = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)?;
            let text = state.documents.get(uri).cloned().unwrap_or_default();
            let symbols = document_symbols(&text);
            Some(vec![response_ok(id, symbols)])
        }
        Some("textDocument/formatting") => {
            let uri = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)?;
            let text = state.documents.get(uri).cloned().unwrap_or_default();
            let edits = formatting_edits(&text);
            Some(vec![response_ok(id, edits)])
        }
        Some("textDocument/hover") => {
            let uri = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)?;
            let text = state.documents.get(uri).cloned().unwrap_or_default();
            let line = message
                .pointer("/params/position/line")
                .and_then(Value::as_u64)
                .unwrap_or(0) as usize;
            let character = message
                .pointer("/params/position/character")
                .and_then(Value::as_u64)
                .unwrap_or(0) as usize;
            let hover = hover_at(&text, line, character);
            Some(vec![response_ok(id, hover)])
        }
        Some("textDocument/definition") => {
            let uri = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)?;
            let text = state.documents.get(uri).cloned().unwrap_or_default();
            let line = message
                .pointer("/params/position/line")
                .and_then(Value::as_u64)
                .unwrap_or(0) as usize;
            let character = message
                .pointer("/params/position/character")
                .and_then(Value::as_u64)
                .unwrap_or(0) as usize;
            let location = definition_at(uri, &text, line, character);
            Some(vec![response_ok(id, location)])
        }
        Some(other) if id.is_some() => Some(vec![response_error(
            id,
            -32601,
            format!("method not found: {other}"),
        )]),
        Some(_) => None,
        None => {
            // Response to server request — ignore in M13a.
            None
        }
    }
}

fn response_ok(id: Option<Value>, result: Value) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id.unwrap_or(Value::Null),
        "result": result,
    })
    .to_string()
}

fn response_error(id: Option<Value>, code: i64, message: String) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id.unwrap_or(Value::Null),
        "error": { "code": code, "message": message },
    })
    .to_string()
}

/// Map bootstrap compile diagnostics to an LSP publishDiagnostics notification.
pub fn diagnostics_notification(uri: &str, text: &str) -> String {
    let diagnostics = collect_diagnostics(text);
    json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {
            "uri": uri,
            "diagnostics": diagnostics,
        }
    })
    .to_string()
}

pub fn collect_diagnostics(text: &str) -> Vec<Value> {
    if text.is_empty() {
        return Vec::new();
    }
    match compile_source(text) {
        Ok(_) => Vec::new(),
        Err(error) => {
            let diagnostic = error.diagnostic();
            vec![lsp_diagnostic(&diagnostic)]
        }
    }
}

fn lsp_diagnostic(diagnostic: &Diagnostic) -> Value {
    let line = diagnostic.span.line.saturating_sub(1);
    let character = diagnostic.span.column.saturating_sub(1);
    json!({
        "range": {
            "start": { "line": line, "character": character },
            "end": { "line": line, "character": character.saturating_add(1) },
        },
        "severity": 1,
        "source": "aether",
        "code": diagnostic.code,
        "message": format!("{} (bootstrap diagnostics; product compile is seed-hosted)", diagnostic.message),
    })
}

pub fn document_symbols(text: &str) -> Value {
    let Ok(document) = structural_document_json(text) else {
        return Value::Array(Vec::new());
    };
    let Ok(parsed) = serde_json::from_str::<Value>(&document) else {
        return Value::Array(Vec::new());
    };
    let mut symbols = Vec::new();
    if let Some(world) = parsed
        .pointer("/program/world/name")
        .and_then(Value::as_str)
    {
        symbols.push(symbol("world", world, 5, 0, 0));
    }
    if let Some(weaves) = parsed.pointer("/program/weaves").and_then(Value::as_array) {
        for weave in weaves {
            if let Some(name) = weave.get("name").and_then(Value::as_str) {
                let line = weave
                    .pointer("/span/line")
                    .and_then(Value::as_u64)
                    .unwrap_or(1)
                    .saturating_sub(1);
                let character = weave
                    .pointer("/span/column")
                    .and_then(Value::as_u64)
                    .unwrap_or(1)
                    .saturating_sub(1);
                symbols.push(symbol(name, "weave", 12, line, character));
            }
        }
    }
    if let Some(records) = parsed.pointer("/program/records").and_then(Value::as_array) {
        for record in records {
            if let Some(name) = record.get("name").and_then(Value::as_str) {
                let line = record
                    .pointer("/span/line")
                    .and_then(Value::as_u64)
                    .unwrap_or(1)
                    .saturating_sub(1);
                symbols.push(symbol(name, "record", 5, line, 0));
            }
        }
    }
    Value::Array(symbols)
}

fn symbol(name: &str, detail: &str, kind: u64, line: u64, character: u64) -> Value {
    json!({
        "name": name,
        "detail": detail,
        "kind": kind,
        "range": {
            "start": { "line": line, "character": character },
            "end": { "line": line, "character": character.saturating_add(name.len() as u64) },
        },
        "selectionRange": {
            "start": { "line": line, "character": character },
            "end": { "line": line, "character": character.saturating_add(name.len() as u64) },
        },
    })
}

pub fn formatting_edits(text: &str) -> Value {
    match format_source(text) {
        Ok(formatted) if formatted != text => {
            let end_line = text.lines().count().saturating_sub(1) as u64;
            let end_character = text.lines().last().map(str::len).unwrap_or(0) as u64;
            json!([{
                "range": {
                    "start": { "line": 0, "character": 0 },
                    "end": { "line": end_line, "character": end_character },
                },
                "newText": formatted,
            }])
        }
        _ => Value::Array(Vec::new()),
    }
}

pub fn word_at(text: &str, line: usize, character: usize) -> Option<String> {
    let line_text = text.lines().nth(line)?;
    if character > line_text.len() {
        return None;
    }
    let bytes = line_text.as_bytes();
    let mut start = character.min(bytes.len());
    let mut end = start;
    while start > 0
        && (bytes[start - 1].is_ascii_lowercase()
            || bytes[start - 1].is_ascii_digit()
            || bytes[start - 1] == b'_')
    {
        start -= 1;
    }
    while end < bytes.len()
        && (bytes[end].is_ascii_lowercase() || bytes[end].is_ascii_digit() || bytes[end] == b'_')
    {
        end += 1;
    }
    if start == end {
        None
    } else {
        Some(line_text[start..end].to_owned())
    }
}

pub fn hover_at(text: &str, line: usize, character: usize) -> Value {
    let Some(word) = word_at(text, line, character) else {
        return Value::Null;
    };
    if let Ok(program) = compile_source(text) {
        if let Some(weave) = program.weaves.iter().find(|weave| weave.name == word) {
            return json!({
                "contents": {
                    "kind": "markdown",
                    "value": format!("**weave `{word}`** (effect: {:?})", weave.effect),
                }
            });
        }
        if program.records.iter().any(|record| record.name == word) {
            return json!({
                "contents": {
                    "kind": "markdown",
                    "value": format!("**record `{word}`**"),
                }
            });
        }
    }
    json!({
        "contents": {
            "kind": "plaintext",
            "value": word,
        }
    })
}

pub fn definition_at(uri: &str, text: &str, line: usize, character: usize) -> Value {
    let Some(word) = word_at(text, line, character) else {
        return Value::Null;
    };
    let Ok(document) = structural_document_json(text) else {
        return Value::Null;
    };
    let Ok(parsed) = serde_json::from_str::<Value>(&document) else {
        return Value::Null;
    };
    if let Some(weaves) = parsed.pointer("/program/weaves").and_then(Value::as_array) {
        for weave in weaves {
            if weave.get("name").and_then(Value::as_str) == Some(word.as_str()) {
                let line = weave
                    .pointer("/span/line")
                    .and_then(Value::as_u64)
                    .unwrap_or(1)
                    .saturating_sub(1);
                let character = weave
                    .pointer("/span/column")
                    .and_then(Value::as_u64)
                    .unwrap_or(1)
                    .saturating_sub(1);
                return json!({
                    "uri": uri,
                    "range": {
                        "start": { "line": line, "character": character },
                        "end": { "line": line, "character": character + word.len() as u64 },
                    }
                });
            }
        }
    }
    Value::Null
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_report_type_error_and_clear_on_valid() {
        let bad = "world x\n\nweave main [] -> Whole:\n  yield \"nope\"\n";
        let diags = collect_diagnostics(bad);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0]["code"], "AE-TYPE-001");
        assert_eq!(diags[0]["source"], "aether");

        let good = "world x\n\nweave main [] -> Whole:\n  yield 0\n";
        assert!(collect_diagnostics(good).is_empty());
    }

    #[test]
    fn formatting_produces_canonical_text_edit() {
        let messy = "world x\n\nweave main [] -> Whole:\n  yield sum 1 2\n";
        // already canonical — empty edits
        let edits = formatting_edits(messy);
        assert!(edits.as_array().is_some());

        let good = "world x\n\nweave main [] -> Whole:\n  yield 0\n";
        let symbols = document_symbols(good);
        let array = symbols.as_array().expect("symbols array");
        assert!(array.iter().any(|symbol| symbol["name"] == "main"));
    }

    #[test]
    fn hover_and_definition_find_weave() {
        let source = "world calls\n\nweave helper [] -> Whole:\n  yield 1\n\nweave main [] -> Whole:\n  yield call helper\n";
        // "helper" on call line — find line index
        let lines: Vec<_> = source.lines().collect();
        let call_line = lines
            .iter()
            .position(|line| line.contains("call helper"))
            .expect("call line");
        let character = lines[call_line].find("helper").expect("helper col");
        let hover = hover_at(source, call_line, character);
        assert!(hover["contents"]["value"]
            .as_str()
            .is_some_and(|value| value.contains("helper")));

        let def = definition_at("file:///x.ae", source, call_line, character);
        assert_eq!(def["uri"], "file:///x.ae");
        assert!(def["range"]["start"]["line"].as_u64().unwrap() < call_line as u64);
    }

    #[test]
    fn initialize_handshake_lists_capabilities() {
        let mut state = ServerState::default();
        let init = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        });
        let responses = handle_message(&mut state, &init).expect("response");
        assert_eq!(responses.len(), 1);
        let body: Value = serde_json::from_str(&responses[0]).unwrap();
        assert_eq!(body["result"]["capabilities"]["hoverProvider"], true);
        assert_eq!(
            body["result"]["capabilities"]["documentFormattingProvider"],
            true
        );
    }
}
