//! Bounded offline Language Server Protocol (M13a + M13b / ADR-017 / ADR-058).
//!
//! Stdio JSON-RPC only. **Product-path diagnostics are primary** (ADR-058 /
//! AE-SEED codes via seed product path). Symbols/format/hover/definition still
//! use bootstrap AST. No product AETH emission; no server-side disk writes of
//! source or artifacts.
//!
//! M13b: optional project file enables cross-file definition/hover for
//! `import unit "…" as alias` → `call alias.weave`.

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use aether_core::{
    compile_source, format_source, lsp_product_diagnostics_primary, parse_project_document,
    product_diagnostics, product_surface_symbols, product_surface_symbols_without_bootstrap,
    structural_document_json, validate_unit_path, Diagnostic, LANGUAGE_NAME, LANGUAGE_VERSION,
};
use serde_json::{json, Value};

#[derive(Debug, Default)]
struct ServerState {
    /// uri -> full text
    documents: BTreeMap<String, String>,
    /// Optional aether.project.json path (CLI flag or initializationOptions).
    project_file: Option<PathBuf>,
    /// Directory containing the project file.
    project_root: Option<PathBuf>,
    shutdown_requested: bool,
}

/// Run the LSP event loop on stdin/stdout until exit.
pub fn run(project_file: Option<PathBuf>) -> Result<(), String> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = io::stdout();
    let mut state = ServerState {
        project_file: project_file.clone(),
        project_root: project_file
            .as_ref()
            .and_then(|path| path.parent().map(Path::to_path_buf)),
        ..ServerState::default()
    };

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
        if parsed.get("method").and_then(Value::as_str) == Some("exit") {
            break;
        }
    }
    let _ = state.shutdown_requested;
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

fn handle_message(state: &mut ServerState, message: &Value) -> Option<Vec<String>> {
    let method = message.get("method").and_then(Value::as_str);
    let id = message.get("id").cloned();

    match method {
        Some("initialize") => {
            if let Some(project) = message
                .pointer("/params/initializationOptions/projectFile")
                .and_then(Value::as_str)
            {
                let path = PathBuf::from(project);
                if path.is_file() {
                    state.project_root = path.parent().map(Path::to_path_buf);
                    state.project_file = Some(path);
                }
            } else if state.project_file.is_none() {
                if let Some(root_uri) = message
                    .pointer("/params/rootUri")
                    .and_then(Value::as_str)
                    .and_then(uri_to_path)
                {
                    let candidate = root_uri.join("aether.project.json");
                    if candidate.is_file() {
                        state.project_root = Some(root_uri);
                        state.project_file = Some(candidate);
                    }
                }
            }
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
            Some(vec![response_ok(id, document_symbols(&text))])
        }
        Some("textDocument/formatting") => {
            let uri = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)?;
            let text = state.documents.get(uri).cloned().unwrap_or_default();
            Some(vec![response_ok(id, formatting_edits(&text))])
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
            let hover = hover_at(
                &text,
                line,
                character,
                state.project_root.as_deref(),
                state.project_file.as_deref(),
            );
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
            let location = definition_at(
                uri,
                &text,
                line,
                character,
                state.project_root.as_deref(),
                state.project_file.as_deref(),
                &state.documents,
            );
            Some(vec![response_ok(id, location)])
        }
        Some(other) if id.is_some() => Some(vec![response_error(
            id,
            -32601,
            format!("method not found: {other}"),
        )]),
        Some(_) => None,
        None => None,
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
    // Multi-module sources reject single-file compile by design; surface honesty.
    if text
        .lines()
        .any(|line| line.trim_start().starts_with("import unit "))
    {
        return vec![json!({
            "range": {
                "start": { "line": 0, "character": 0 },
                "end": { "line": 0, "character": 1 },
            },
            "severity": 3,
            "source": "aether",
            "code": "AE-SEED-012",
            "message": "import unit: multi-module product path is `aether project build` (host elaborate + seed emit; ADR-056). Single-file product/LSP cannot elaborate imports.",
        })];
    }
    // ADR-058: product diagnostics primary (seed path AE-SEED codes).
    if lsp_product_diagnostics_primary() {
        let product = product_diagnostics(text);
        if product.is_empty() {
            return Vec::new();
        }
        return product.iter().map(lsp_product_diagnostic).collect();
    }
    match compile_source(text) {
        Ok(_) => Vec::new(),
        Err(error) => {
            let diagnostic = error.diagnostic();
            vec![lsp_bootstrap_diagnostic(&diagnostic)]
        }
    }
}

fn lsp_product_diagnostic(diagnostic: &Diagnostic) -> Value {
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
        "message": format!("{} (product seed diagnostics; bootstrap: aether check)", diagnostic.message),
    })
}

fn lsp_bootstrap_diagnostic(diagnostic: &Diagnostic) -> Value {
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
        "message": format!("{} (bootstrap diagnostics)", diagnostic.message),
    })
}

pub fn document_symbols(text: &str) -> Value {
    // ADR-063: product-surface symbols without bootstrap AST when product accepts.
    if product_surface_symbols_without_bootstrap() {
        if let Ok(surface) = product_surface_symbols(text) {
            let mut symbols = Vec::new();
            for item in surface {
                let kind = match item.kind {
                    "world" | "record" => 5u64,
                    "weave" => 12,
                    _ => 13,
                };
                let detail = item.kind;
                symbols.push(symbol(
                    &item.name,
                    detail,
                    kind,
                    u64::from(item.line),
                    u64::from(item.column),
                ));
            }
            for (alias, path) in parse_imports(text) {
                symbols.push(symbol(&alias, &format!("import {path}"), 9, 0, 0));
            }
            return Value::Array(symbols);
        }
        // Product reject (e.g. raw import unit): still expose import aliases without AST.
        let imports = parse_imports(text);
        if !imports.is_empty() {
            let mut symbols = Vec::new();
            for (alias, path) in imports {
                symbols.push(symbol(&alias, &format!("import {path}"), 9, 0, 0));
            }
            return Value::Array(symbols);
        }
    }
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
    for (alias, path) in parse_imports(text) {
        symbols.push(symbol(&alias, &format!("import {path}"), 9, 0, 0));
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

/// Parse `import unit "path" as alias` lines.
pub fn parse_imports(source: &str) -> Vec<(String, String)> {
    let mut imports = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("import unit ") else {
            continue;
        };
        let rest = rest.trim();
        if !rest.starts_with('"') {
            continue;
        }
        let after = &rest[1..];
        let Some(end) = after.find('"') else {
            continue;
        };
        let path = after[..end].to_owned();
        if validate_unit_path(&path).is_err() {
            continue;
        }
        let tail = after[end + 1..].trim_start();
        let Some(alias) = tail.strip_prefix("as ") else {
            continue;
        };
        let alias = alias.trim();
        if !alias.is_empty() {
            imports.push((alias.to_owned(), path));
        }
    }
    imports
}

/// Word or `alias.weave` under cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorSymbol {
    Plain(String),
    Qualified { alias: String, name: String },
}

pub fn symbol_at(text: &str, line: usize, character: usize) -> Option<CursorSymbol> {
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
            || bytes[start - 1] == b'_'
            || bytes[start - 1] == b'.')
    {
        start -= 1;
    }
    while end < bytes.len()
        && (bytes[end].is_ascii_lowercase()
            || bytes[end].is_ascii_digit()
            || bytes[end] == b'_'
            || bytes[end] == b'.')
    {
        end += 1;
    }
    if start == end {
        return None;
    }
    let token = &line_text[start..end];
    if let Some((alias, name)) = token.split_once('.') {
        if !alias.is_empty() && !name.is_empty() && !name.contains('.') {
            return Some(CursorSymbol::Qualified {
                alias: alias.to_owned(),
                name: name.to_owned(),
            });
        }
    }
    // If cursor is on the name after `alias.`, token may be just the name —
    // look left for `alias.`
    if let Some(plain) = word_at(text, line, character) {
        if let Some(prefix) = line_text[..start].trim_end().strip_suffix('.') {
            let alias_end = prefix.len();
            let mut alias_start = alias_end;
            let pb = prefix.as_bytes();
            while alias_start > 0
                && (pb[alias_start - 1].is_ascii_lowercase()
                    || pb[alias_start - 1].is_ascii_digit()
                    || pb[alias_start - 1] == b'_')
            {
                alias_start -= 1;
            }
            if alias_start < alias_end {
                let alias = &prefix[alias_start..alias_end];
                return Some(CursorSymbol::Qualified {
                    alias: alias.to_owned(),
                    name: plain,
                });
            }
        }
        return Some(CursorSymbol::Plain(plain));
    }
    None
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

/// Read unit text from open documents or disk under project root (read-only).
fn load_unit_source(
    unit_path: &str,
    project_root: Option<&Path>,
    documents: &BTreeMap<String, String>,
) -> Option<String> {
    if let Some(root) = project_root {
        let full = join_unit(root, unit_path);
        let uri = path_to_uri(&full);
        if let Some(text) = documents.get(&uri) {
            return Some(text.clone());
        }
        // Also try normalized URI variants
        for (doc_uri, text) in documents {
            if uri_to_path(doc_uri).is_some_and(|path| path == full) {
                return Some(text.clone());
            }
        }
        fs::read_to_string(full).ok()
    } else {
        None
    }
}

fn join_unit(root: &Path, unit_path: &str) -> PathBuf {
    let mut path = root.to_path_buf();
    for segment in unit_path.split('/') {
        path.push(segment);
    }
    path
}

fn unit_listed_in_project(project_file: &Path, unit_path: &str) -> bool {
    let Ok(json) = fs::read_to_string(project_file) else {
        return false;
    };
    let Ok(document) = parse_project_document(&json) else {
        return false;
    };
    document.units.iter().any(|unit| unit.path == unit_path)
}

fn find_export_weave_line(source: &str, weave_name: &str) -> Option<(u64, u64)> {
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        let rest = trimmed
            .strip_prefix("export weave ")
            .or_else(|| trimmed.strip_prefix("weave "));
        if let Some(rest) = rest {
            if rest.starts_with(weave_name)
                && rest
                    .as_bytes()
                    .get(weave_name.len())
                    .is_some_and(|byte| *byte == b' ' || *byte == b'[')
            {
                let col = line.find(weave_name).unwrap_or(0) as u64;
                return Some((index as u64, col));
            }
        }
    }
    None
}

pub fn hover_at(
    text: &str,
    line: usize,
    character: usize,
    project_root: Option<&Path>,
    project_file: Option<&Path>,
) -> Value {
    let Some(symbol) = symbol_at(text, line, character) else {
        return Value::Null;
    };
    match symbol {
        CursorSymbol::Qualified { alias, name } => {
            let imports = parse_imports(text);
            let Some((_, unit_path)) = imports.into_iter().find(|(a, _)| a == &alias) else {
                return json!({
                    "contents": {
                        "kind": "markdown",
                        "value": format!("unknown import alias `{alias}`"),
                    }
                });
            };
            if let Some(project_file) = project_file {
                if !unit_listed_in_project(project_file, &unit_path) {
                    return json!({
                        "contents": {
                            "kind": "markdown",
                            "value": format!("import `{unit_path}` is not a project unit (path jail)"),
                        }
                    });
                }
            }
            let empty = BTreeMap::new();
            if let Some(lib_source) = load_unit_source(&unit_path, project_root, &empty) {
                if lib_source.contains(&format!("export weave {name}"))
                    || lib_source
                        .lines()
                        .any(|l| l.trim_start().starts_with(&format!("export weave {name} ")))
                {
                    return json!({
                        "contents": {
                            "kind": "markdown",
                            "value": format!("**export weave `{alias}.{name}`** from `{unit_path}`"),
                        }
                    });
                }
                return json!({
                    "contents": {
                        "kind": "markdown",
                        "value": format!("`{name}` is not an exported weave in `{unit_path}`"),
                    }
                });
            }
            json!({
                "contents": {
                    "kind": "markdown",
                    "value": format!("**import `{alias}`** → `{unit_path}` (open project to resolve)"),
                }
            })
        }
        CursorSymbol::Plain(word) => {
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
            if parse_imports(text).iter().any(|(alias, _)| alias == &word) {
                let path = parse_imports(text)
                    .into_iter()
                    .find(|(alias, _)| alias == &word)
                    .map(|(_, path)| path)
                    .unwrap_or_default();
                return json!({
                    "contents": {
                        "kind": "markdown",
                        "value": format!("**import alias `{word}`** → `{path}`"),
                    }
                });
            }
            json!({
                "contents": {
                    "kind": "plaintext",
                    "value": word,
                }
            })
        }
    }
}

pub fn definition_at(
    uri: &str,
    text: &str,
    line: usize,
    character: usize,
    project_root: Option<&Path>,
    project_file: Option<&Path>,
    documents: &BTreeMap<String, String>,
) -> Value {
    let Some(symbol) = symbol_at(text, line, character) else {
        return Value::Null;
    };
    match symbol {
        CursorSymbol::Qualified { alias, name } => {
            let imports = parse_imports(text);
            let Some((_, unit_path)) = imports.into_iter().find(|(a, _)| a == &alias) else {
                return Value::Null;
            };
            if let Some(project_file) = project_file {
                if !unit_listed_in_project(project_file, &unit_path) {
                    return Value::Null;
                }
            }
            let Some(lib_source) = load_unit_source(&unit_path, project_root, documents) else {
                return Value::Null;
            };
            // Only jump to exported weaves.
            let exported = lib_source.lines().any(|line| {
                let t = line.trim_start();
                t.starts_with(&format!("export weave {name} "))
                    || t.starts_with(&format!("export weave {name}["))
            });
            if !exported {
                return Value::Null;
            }
            let Some((line, character)) = find_export_weave_line(&lib_source, &name) else {
                return Value::Null;
            };
            let target_uri = project_root
                .map(|root| path_to_uri(&join_unit(root, &unit_path)))
                .unwrap_or_else(|| uri.to_owned());
            json!({
                "uri": target_uri,
                "range": {
                    "start": { "line": line, "character": character },
                    "end": { "line": line, "character": character + name.len() as u64 },
                }
            })
        }
        CursorSymbol::Plain(word) => {
            // Alias alone → jump to import line or unit file start
            if let Some((_, unit_path)) = parse_imports(text)
                .into_iter()
                .find(|(alias, _)| alias == &word)
            {
                if let Some(project_file) = project_file {
                    if !unit_listed_in_project(project_file, &unit_path) {
                        return Value::Null;
                    }
                }
                if let Some(root) = project_root {
                    let target = join_unit(root, &unit_path);
                    return json!({
                        "uri": path_to_uri(&target),
                        "range": {
                            "start": { "line": 0, "character": 0 },
                            "end": { "line": 0, "character": 1 },
                        }
                    });
                }
            }
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
    }
}

pub fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let path = uri.strip_prefix("file://")?;
    let path = if cfg!(windows) {
        let path = path.strip_prefix('/').unwrap_or(path);
        path.replace('/', "\\")
    } else {
        path.to_owned()
    };
    // Minimal percent-decoding for spaces
    let path = path
        .replace("%20", " ")
        .replace("%3A", ":")
        .replace("%3a", ":");
    Some(PathBuf::from(path))
}

pub fn path_to_uri(path: &Path) -> String {
    let mut s = path.to_string_lossy().replace('\\', "/");
    if cfg!(windows) && !s.starts_with('/') {
        s = format!("/{s}");
    }
    format!("file://{s}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_report_type_error_and_clear_on_valid() {
        assert!(
            lsp_product_diagnostics_primary(),
            "ADR-058: product diagnostics primary"
        );
        let bad = "world x\n\nweave main [] -> Whole:\n  yield \"nope\"\n";
        let diags = collect_diagnostics(bad);
        assert_eq!(diags.len(), 1);
        // Product-primary: type/stack mismatch maps to AE-SEED-010 (not bootstrap AE-TYPE-001).
        assert_eq!(diags[0]["code"], "AE-SEED-010");
        assert!(diags[0]["message"]
            .as_str()
            .unwrap_or("")
            .contains("product seed diagnostics"));

        let good = "world x\n\nweave main [] -> Whole:\n  yield 0\n";
        assert!(collect_diagnostics(good).is_empty());

        let modules = "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  yield 0\n";
        let diags = collect_diagnostics(modules);
        assert_eq!(diags[0]["code"], "AE-SEED-012");
        assert_eq!(diags[0]["severity"], 3);
    }

    #[test]
    fn formatting_and_symbols() {
        let good = "world x\n\nweave main [] -> Whole:\n  yield 0\n";
        let symbols = document_symbols(good);
        let array = symbols.as_array().expect("symbols array");
        assert!(array.iter().any(|symbol| symbol["name"] == "main"));
    }

    #[test]
    fn hover_and_definition_find_local_weave() {
        let source = "world calls\n\nweave helper [] -> Whole:\n  yield 1\n\nweave main [] -> Whole:\n  yield call helper\n";
        let lines: Vec<_> = source.lines().collect();
        let call_line = lines
            .iter()
            .position(|line| line.contains("call helper"))
            .expect("call line");
        let character = lines[call_line].find("helper").expect("helper col");
        let hover = hover_at(source, call_line, character, None, None);
        assert!(hover["contents"]["value"]
            .as_str()
            .is_some_and(|value| value.contains("helper")));

        let docs = BTreeMap::new();
        let def = definition_at(
            "file:///x.ae",
            source,
            call_line,
            character,
            None,
            None,
            &docs,
        );
        assert_eq!(def["uri"], "file:///x.ae");
    }

    #[test]
    fn project_aware_definition_resolves_exported_import() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/project-modules");
        let project = root.join("aether.project.json");
        if !project.is_file() {
            return;
        }
        let main = fs::read_to_string(root.join("src/main.ae")).expect("main");
        let lines: Vec<_> = main.lines().collect();
        let call_line = lines
            .iter()
            .position(|line| line.contains("math.double"))
            .expect("call");
        let character = lines[call_line].find("double").expect("double");
        let docs = BTreeMap::new();
        let def = definition_at(
            "file:///main.ae",
            &main,
            call_line,
            character,
            Some(&root),
            Some(&project),
            &docs,
        );
        let uri = def["uri"].as_str().expect("uri");
        assert!(
            uri.contains("math.ae") || uri.ends_with("math.ae"),
            "uri was {uri}"
        );
        let hover = hover_at(&main, call_line, character, Some(&root), Some(&project));
        assert!(hover["contents"]["value"]
            .as_str()
            .is_some_and(|value| value.contains("export weave") && value.contains("double")));

        // Private weave must not resolve
        let private = main.replace("math.double", "math.secret");
        let def = definition_at(
            "file:///main.ae",
            &private,
            call_line,
            character,
            Some(&root),
            Some(&project),
            &docs,
        );
        assert!(def.is_null());
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
        let body: Value = serde_json::from_str(&responses[0]).unwrap();
        assert_eq!(body["result"]["capabilities"]["hoverProvider"], true);
    }

    #[test]
    fn parse_imports_extracts_alias_and_path() {
        let source = "world app\n\nimport unit \"lib/math.ae\" as math\n\nweave main [] -> Whole:\n  yield 0\n";
        let imports = parse_imports(source);
        assert_eq!(imports, vec![("math".to_owned(), "lib/math.ae".to_owned())]);
    }
}
