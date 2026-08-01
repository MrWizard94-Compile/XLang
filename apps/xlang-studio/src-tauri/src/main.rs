use std::env;
use std::fmt::Write as _;
use std::time::Duration;

use aether_core::{
    apply_structural_edit as apply_structural_edit_core, canonical_ast, compile_with_seed,
    run_bytecode, structural_document_json, Diagnostic, DIAGNOSTIC_SCHEMA_VERSION,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use url::Url;

const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";
const DEFAULT_OLLAMA_MODEL: &str = "qwen2.5:3b";
const MAX_COMPILE_SOURCE_BYTES: usize = 1_000_000;
const MAX_REVIEW_SOURCE_BYTES: usize = 16_000;
const MAX_ARTIFACT_PREVIEW_BYTES: usize = 256;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CompileResponse {
    success: bool,
    artifact: Option<String>,
    runtime_output: Option<String>,
    exit_code: Option<i64>,
    diagnostic: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthoringSpan {
    line: usize,
    column: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthoringDiagnostic {
    schema: &'static str,
    code: &'static str,
    span: AuthoringSpan,
    message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StructuralAuthoringResponse {
    success: bool,
    source: Option<String>,
    document: Option<String>,
    operation_count: Option<usize>,
    diagnostic: Option<AuthoringDiagnostic>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OllamaStatus {
    available: bool,
    endpoint: String,
    default_model: String,
    models: Vec<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReviewResponse {
    model: String,
    content: String,
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<TagModel>,
}

#[derive(Deserialize)]
struct TagModel {
    name: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: ChatMessageResponse,
}

#[derive(Deserialize)]
struct ChatMessageResponse {
    content: String,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    stream: bool,
    messages: [ChatMessage<'a>; 2],
    options: ChatOptions,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct ChatOptions {
    temperature: f32,
    num_predict: u16,
}

fn configured_model() -> String {
    env::var("XLANG_OLLAMA_MODEL")
        .ok()
        .filter(|model| validate_model(model).is_ok())
        .unwrap_or_else(|| DEFAULT_OLLAMA_MODEL.to_owned())
}

fn validate_model(model: &str) -> Result<(), String> {
    if model.is_empty() || model.len() > 128 || model.trim() != model {
        return Err("Choose a valid local Ollama model name.".to_owned());
    }

    if model.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, ':' | '.' | '_' | '-')
    }) {
        Ok(())
    } else {
        Err(
            "Model names may contain only letters, numbers, colon, dot, underscore, and hyphen."
                .to_owned(),
        )
    }
}

fn ollama_endpoint() -> Result<String, String> {
    let configured = env::var("XLANG_OLLAMA_URL").unwrap_or_else(|_| DEFAULT_OLLAMA_URL.to_owned());
    let parsed = Url::parse(&configured)
        .map_err(|_| "XLANG_OLLAMA_URL must be a valid loopback HTTP URL.".to_owned())?;

    let loopback_host = matches!(parsed.host_str(), Some("127.0.0.1" | "localhost" | "::1"));
    if parsed.scheme() != "http"
        || !loopback_host
        || parsed.path() != "/"
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
    {
        return Err(
            "XLANG_OLLAMA_URL must be a credential-free loopback HTTP base URL.".to_owned(),
        );
    }

    Ok(configured.trim_end_matches('/').to_owned())
}

fn ollama_client() -> Result<Client, String> {
    Client::builder()
        .timeout(Duration::from_secs(45))
        .build()
        .map_err(|_| "Could not create the local Ollama client.".to_owned())
}

fn failed_compile(diagnostic: String) -> CompileResponse {
    CompileResponse {
        success: false,
        artifact: None,
        runtime_output: None,
        exit_code: None,
        diagnostic: Some(diagnostic),
    }
}

fn authoring_diagnostic(diagnostic: &Diagnostic) -> AuthoringDiagnostic {
    AuthoringDiagnostic {
        schema: DIAGNOSTIC_SCHEMA_VERSION,
        code: diagnostic.code,
        span: AuthoringSpan {
            line: diagnostic.span.line,
            column: diagnostic.span.column,
        },
        message: diagnostic.message.clone(),
    }
}

fn failed_authoring(diagnostic: &Diagnostic) -> StructuralAuthoringResponse {
    StructuralAuthoringResponse {
        success: false,
        source: None,
        document: None,
        operation_count: None,
        diagnostic: Some(authoring_diagnostic(diagnostic)),
    }
}

fn source_limit_authoring_response() -> StructuralAuthoringResponse {
    StructuralAuthoringResponse {
        success: false,
        source: None,
        document: None,
        operation_count: None,
        diagnostic: Some(AuthoringDiagnostic {
            schema: DIAGNOSTIC_SCHEMA_VERSION,
            code: "AE-SOURCE-001",
            span: AuthoringSpan { line: 1, column: 1 },
            message: format!(
                "Source is too large for Aether Studio. Keep it within {MAX_COMPILE_SOURCE_BYTES} bytes."
            ),
        }),
    }
}

fn artifact_summary(bytecode: &[u8], ast: &str) -> String {
    let preview_length = bytecode.len().min(MAX_ARTIFACT_PREVIEW_BYTES);
    let mut summary = format!("AETH artifact\n{} byte(s)\n\n", bytecode.len());

    for (row, chunk) in bytecode[..preview_length].chunks(16).enumerate() {
        let offset = row * 16;
        write!(&mut summary, "{offset:04X}:").expect("writing to a String cannot fail");
        for byte in chunk {
            write!(&mut summary, " {byte:02X}").expect("writing to a String cannot fail");
        }
        summary.push('\n');
    }

    if bytecode.len() > preview_length {
        let remaining = bytecode.len() - preview_length;
        writeln!(&mut summary, "... {remaining} byte(s) omitted")
            .expect("writing to a String cannot fail");
    }

    summary.push_str("\nCanonical AST\n");
    summary.push_str(ast);
    summary
}

fn compile_response(source: &str) -> CompileResponse {
    if source.len() > MAX_COMPILE_SOURCE_BYTES {
        return failed_compile(format!(
            "Source is too large for Aether Studio. Keep it within {MAX_COMPILE_SOURCE_BYTES} bytes."
        ));
    }

    let output = match compile_with_seed(source) {
        Ok(output) => output,
        Err(error) => return failed_compile(error.to_string()),
    };
    let artifact = artifact_summary(&output.bytecode, &canonical_ast(&output.program));

    match run_bytecode(&output.bytecode) {
        Ok(runtime) => CompileResponse {
            success: true,
            artifact: Some(artifact),
            runtime_output: Some(runtime.stdout),
            exit_code: Some(runtime.exit_code),
            diagnostic: None,
        },
        Err(error) => failed_compile(format!("verified Aether artifact could not run: {error}")),
    }
}

fn inspect_structure_response(source: &str) -> StructuralAuthoringResponse {
    if source.len() > MAX_COMPILE_SOURCE_BYTES {
        return source_limit_authoring_response();
    }

    match structural_document_json(source) {
        Ok(document) => StructuralAuthoringResponse {
            success: true,
            source: None,
            document: Some(document),
            operation_count: None,
            diagnostic: None,
        },
        Err(error) => failed_authoring(&error.diagnostic()),
    }
}

fn apply_structural_edit_response(source: &str, edit: &str) -> StructuralAuthoringResponse {
    if source.len() > MAX_COMPILE_SOURCE_BYTES {
        return source_limit_authoring_response();
    }

    let edited = match apply_structural_edit_core(source, edit) {
        Ok(edited) => edited,
        Err(error) => return failed_authoring(error.diagnostic()),
    };
    if let Err(error) = compile_with_seed(&edited.source) {
        return failed_authoring(&error.diagnostic());
    }

    StructuralAuthoringResponse {
        success: true,
        source: Some(edited.source),
        document: Some(edited.document_json),
        operation_count: Some(edited.operation_count),
        diagnostic: None,
    }
}

#[tauri::command]
fn compile_source(source: String) -> CompileResponse {
    compile_response(&source)
}

#[tauri::command]
fn inspect_structure(source: String) -> StructuralAuthoringResponse {
    inspect_structure_response(&source)
}

#[tauri::command]
fn apply_structural_edit(source: String, edit: String) -> StructuralAuthoringResponse {
    apply_structural_edit_response(&source, &edit)
}

#[tauri::command]
async fn ollama_status() -> OllamaStatus {
    let default_model = configured_model();
    let endpoint = match ollama_endpoint() {
        Ok(endpoint) => endpoint,
        Err(error) => {
            return OllamaStatus {
                available: false,
                endpoint: DEFAULT_OLLAMA_URL.to_owned(),
                default_model,
                models: Vec::new(),
                error: Some(error),
            };
        }
    };

    let client = match ollama_client() {
        Ok(client) => client,
        Err(error) => {
            return OllamaStatus {
                available: false,
                endpoint,
                default_model,
                models: Vec::new(),
                error: Some(error),
            };
        }
    };

    let tags = match client.get(format!("{endpoint}/api/tags")).send().await {
        Ok(response) => match response.error_for_status() {
            Ok(response) => response
                .json::<TagsResponse>()
                .await
                .map_err(|_| "Docker Ollama returned an invalid model inventory.".to_owned()),
            Err(_) => Err("Docker Ollama did not accept the status request.".to_owned()),
        },
        Err(_) => {
            Err("Docker Ollama is unavailable at the configured loopback endpoint.".to_owned())
        }
    };

    match tags {
        Ok(tags) => {
            let mut models: Vec<String> = tags.models.into_iter().map(|model| model.name).collect();
            models.sort_unstable();
            models.dedup();
            OllamaStatus {
                available: true,
                endpoint,
                default_model,
                models,
                error: None,
            }
        }
        Err(error) => OllamaStatus {
            available: false,
            endpoint,
            default_model,
            models: Vec::new(),
            error: Some(error),
        },
    }
}

#[tauri::command]
async fn review_source(source: String, model: String) -> Result<ReviewResponse, String> {
    validate_model(&model)?;
    if source.trim().is_empty() {
        return Err("Enter source before requesting a review.".to_owned());
    }
    if source.len() > MAX_REVIEW_SOURCE_BYTES {
        return Err(
            "Source is too large for a local review request. Keep it within 16,000 bytes."
                .to_owned(),
        );
    }

    let endpoint = ollama_endpoint()?;
    let client = ollama_client()?;
    let system = "You are a careful local reviewer for Aether 0.6. Analyze only the supplied source. Do not execute code and do not claim that it compiles unless a compiler diagnostic is supplied. Keep the review under 300 words. Check the exact world declaration; record declarations after world and before weaves; named weave signatures; Text/Whole/Truth/Bytes, declared record types, Arena, BufferWhole, and BufferTruth; primitive-only immutable record fields; make constructor order; field borrow projection; bind and revise rules; choose/otherwise and while blocks; terminal yield; shallow prefix expressions; explicit borrow or move for owner values; bounded text and byte operations; and canonical two-space indentation. For M2 resources, check that one positive bounded arena is rooted in main, Buffer uses Whole or Truth, access is only an Arena operation/call argument, allocation and append move and restore the same mutable Buffer binding, lookup borrows an allocated buffer into a mutable Whole or Truth binding, and every resource choose is terminal with explicit outcomes. Buffer ownership cannot cross a weave result or the host ABI. Aether has a verified self-hosting Seed Profile, but the Rust bootstrap remains the invalid-source diagnostic authority.";
    let request = ChatRequest {
        model: &model,
        stream: false,
        messages: [
            ChatMessage {
                role: "system",
                content: system,
            },
            ChatMessage {
                role: "user",
                content: &source,
            },
        ],
        options: ChatOptions {
            temperature: 0.2,
            num_predict: 350,
        },
    };

    let response = client
        .post(format!("{endpoint}/api/chat"))
        .json(&request)
        .send()
        .await
        .map_err(|_| {
            "Docker Ollama is unavailable at the configured loopback endpoint.".to_owned()
        })?
        .error_for_status()
        .map_err(|_| "Docker Ollama rejected the review request.".to_owned())?;
    let response = response
        .json::<ChatResponse>()
        .await
        .map_err(|_| "Docker Ollama returned an invalid review response.".to_owned())?;

    Ok(ReviewResponse {
        model,
        content: response.message.content.trim().to_owned(),
    })
}

fn main() {
    tauri::Builder::default()
        .setup(|application| {
            if let Some(window) = application.get_webview_window("main") {
                window.set_title("Aether Studio")?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            compile_source,
            inspect_structure,
            apply_structural_edit,
            ollama_status,
            review_source
        ])
        .run(tauri::generate_context!())
        .expect("error while running Aether Studio");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_response_contains_verified_artifact_and_runtime_result() {
        let source =
            "world studio\n\nweave main [] -> Whole:\n  speak \"Aether Studio\\n\"\n  yield 7\n";
        let response = compile_response(source);

        assert!(response.success);
        assert!(response
            .artifact
            .as_deref()
            .is_some_and(|artifact| artifact.contains("AETH artifact")));
        assert_eq!(response.runtime_output.as_deref(), Some("Aether Studio\n"));
        assert_eq!(response.exit_code, Some(7));
        assert!(response.diagnostic.is_none());
    }

    #[test]
    fn compile_response_runs_the_seed_hosted_m2_resource_surface() {
        let response = compile_response(include_str!("../../../../examples/arena-buffer.ae"));

        assert!(response.success);
        assert!(response
            .artifact
            .as_deref()
            .is_some_and(|artifact| artifact.contains("AETH artifact")));
        assert_eq!(response.exit_code, Some(7));
        assert!(response.diagnostic.is_none());
    }

    #[test]
    fn compile_response_surfaces_aether_diagnostics() {
        let response = compile_response("let total = 42;");

        assert!(!response.success);
        assert!(response.artifact.is_none());
        assert!(response
            .diagnostic
            .as_deref()
            .is_some_and(|diagnostic| diagnostic.contains("world")));
    }

    #[test]
    fn structure_response_returns_the_versioned_local_semantic_document() {
        let response = inspect_structure_response(
            "world studio_structure\n\nweave main [] -> Whole:\n  yield 0\n",
        );

        assert!(response.success);
        assert!(response.source.is_none());
        assert!(response.diagnostic.is_none());
        assert!(response
            .document
            .as_deref()
            .is_some_and(|document| document.contains("aether.ast/v1")));
    }

    #[test]
    fn structural_edit_response_seed_validates_before_returning_canonical_source() {
        let source = "world studio_edit\n\nweave main [] -> Whole:\n  yield 0\n";
        let edit = r#"{
  "protocol": "aether.edit/v1",
  "schema": "aether.ast/v1",
  "baseSource": "world studio_edit\n\nweave main [] -> Whole:\n  yield 0\n",
  "operations": [{
    "op": "replace",
    "target": "weave:main",
    "declaration": {
      "kind": "Weave",
      "name": "main",
      "parameters": [],
      "result": "Whole",
      "body": [{
        "kind": "Yield",
        "value": {"kind": "Atom", "atom": {"kind": "Whole", "value": 11}}
      }]
    }
  }]
}"#;

        let response = apply_structural_edit_response(source, edit);

        assert!(response.success);
        assert_eq!(
            response.source.as_deref(),
            Some("world studio_edit\n\nweave main [] -> Whole:\n  yield 11\n")
        );
        assert_eq!(response.operation_count, Some(1));
        assert!(response.diagnostic.is_none());
    }

    #[test]
    fn stale_structural_edit_returns_machine_readable_diagnostic_without_source() {
        let response = apply_structural_edit_response(
            "world studio_stale\n\nweave main [] -> Whole:\n  yield 0\n",
            r#"{"protocol":"aether.edit/v1","schema":"aether.ast/v1","baseSource":"world other\n\nweave main [] -> Whole:\n  yield 0\n","operations":[{"op":"delete","target":"weave:main"}]}"#,
        );

        assert!(!response.success);
        assert!(response.source.is_none());
        assert_eq!(
            response
                .diagnostic
                .as_ref()
                .map(|diagnostic| diagnostic.code),
            Some("AE-EDIT-003")
        );
        assert_eq!(
            response
                .diagnostic
                .as_ref()
                .map(|diagnostic| diagnostic.span.line),
            Some(1)
        );
    }
}
