use std::env;
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use url::Url;
use xlang_core::compile_source as compile;

const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";
const DEFAULT_OLLAMA_MODEL: &str = "qwen2.5:3b";
const MAX_REVIEW_SOURCE_BYTES: usize = 16_000;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CompileResponse {
    success: bool,
    ast: Option<String>,
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
        Err("Model names may contain only letters, numbers, colon, dot, underscore, and hyphen.".to_owned())
    }
}

fn ollama_endpoint() -> Result<String, String> {
    let configured = env::var("XLANG_OLLAMA_URL")
        .unwrap_or_else(|_| DEFAULT_OLLAMA_URL.to_owned());
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
        return Err("XLANG_OLLAMA_URL must be a credential-free loopback HTTP base URL.".to_owned());
    }

    Ok(configured.trim_end_matches('/').to_owned())
}

fn ollama_client() -> Result<Client, String> {
    Client::builder()
        .timeout(Duration::from_secs(45))
        .build()
        .map_err(|_| "Could not create the local Ollama client.".to_owned())
}

#[tauri::command]
fn compile_source(source: String) -> CompileResponse {
    match compile(&source) {
        Ok(program) => CompileResponse {
            success: true,
            ast: Some(format!("{program:#?}")),
            diagnostic: None,
        },
        Err(error) => CompileResponse {
            success: false,
            ast: None,
            diagnostic: Some(error.to_string()),
        },
    }
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
        Err(_) => Err("Docker Ollama is unavailable at the configured loopback endpoint.".to_owned()),
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
        return Err("Source is too large for a local review request. Keep it within 16,000 bytes.".to_owned());
    }

    let endpoint = ollama_endpoint()?;
    let client = ollama_client()?;
    let system = "You are a careful local reviewer for the XLang bootstrap language. Analyze only the supplied source. Do not claim that source compiles unless a compiler diagnostic is supplied, do not execute code, and keep the review under 300 words. Focus on likely syntax, type, and language-boundary issues.";
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
        .map_err(|_| "Docker Ollama is unavailable at the configured loopback endpoint.".to_owned())?
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
                window.set_title("XLang Studio")?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            compile_source,
            ollama_status,
            review_source
        ])
        .run(tauri::generate_context!())
        .expect("error while running XLang Studio");
}
