use std::env;
use std::fmt::Write as _;
use std::time::Duration;

use aether_core::{canonical_ast, compile_to_bytecode, run_bytecode};
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

    let output = match compile_to_bytecode(source) {
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

#[tauri::command]
fn compile_source(source: String) -> CompileResponse {
    compile_response(&source)
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
    let system = "You are a careful local reviewer for the Aether 0.3 Forge Foundation language. Analyze only the supplied source. Do not execute code and do not claim that it compiles unless a compiler diagnostic is supplied. Keep the review under 300 words. Check the exact world declaration, named weave signatures, Text/Whole/Truth/Bytes types, bind and revise rules, choose/otherwise and while blocks, terminal yield, shallow prefix expressions, explicit borrow or move for Text and Bytes bindings, bytes hexadecimal literals, bounded byte operations, canonical two-space indentation, and language-boundary issues. Aether is not self-hosting yet.";
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
    fn compile_response_surfaces_aether_diagnostics() {
        let response = compile_response("let total = 42;");

        assert!(!response.success);
        assert!(response.artifact.is_none());
        assert!(response
            .diagnostic
            .as_deref()
            .is_some_and(|diagnostic| diagnostic.contains("world")));
    }
}
