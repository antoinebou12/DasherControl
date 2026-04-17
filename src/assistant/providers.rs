use std::env;

use reqwest::Client;
use serde_json::{json, Value};

use super::model::AssistantProvider;

fn trim_env(name: &str) -> Option<String> {
    env::var(name).ok().and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() {
            None
        } else {
            Some(t)
        }
    })
}

pub fn google_api_key() -> Option<String> {
    trim_env("GEMINI_API_KEY").or_else(|| trim_env("GOOGLE_API_KEY"))
}

pub fn ollama_base_url() -> String {
    trim_env("OLLAMA_BASE_URL")
        .unwrap_or_else(|| "http://127.0.0.1:11434".to_string())
        .trim_end_matches('/')
        .to_string()
}

pub fn default_ollama_model() -> String {
    trim_env("OLLAMA_MODEL").unwrap_or_else(|| "llama3.2".to_string())
}

pub async fn chat_openai_compatible(
    client: &Client,
    url: &str,
    api_key: Option<&str>,
    model: &str,
    messages: Vec<Value>,
) -> Result<String, String> {
    let mut req = client.post(url).json(&json!({
        "model": model,
        "messages": messages,
    }));
    if let Some(k) = api_key {
        req = req.bearer_auth(k);
    }
    let res = req.send().await.map_err(|e| e.to_string())?;
    let status = res.status();
    let body: Value = res.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(body.to_string());
    }
    body
        .pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("unexpected OpenAI-shaped response: {body}"))
}

pub async fn chat_openai(
    client: &Client,
    api_key: &str,
    model: &str,
    messages: Vec<Value>,
) -> Result<String, String> {
    chat_openai_compatible(
        client,
        "https://api.openai.com/v1/chat/completions",
        Some(api_key),
        model,
        messages,
    )
    .await
}

pub async fn chat_ollama(
    client: &Client,
    base: &str,
    model: &str,
    messages: Vec<Value>,
) -> Result<String, String> {
    let url = format!("{base}/v1/chat/completions");
    chat_openai_compatible(client, &url, None, model, messages).await
}

pub async fn chat_anthropic(
    client: &Client,
    api_key: &str,
    model: &str,
    system: Option<String>,
    messages: Vec<Value>,
) -> Result<String, String> {
    let mut body = json!({
        "model": model,
        "max_tokens": 4096,
        "messages": messages,
    });
    if let Some(s) = system {
        body["system"] = json!(s);
    }
    let res = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = res.status();
    let v: Value = res.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(v.to_string());
    }
    v.pointer("/content/0/text")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("unexpected Anthropic response: {v}"))
}

/// Gemini `generateContent`. `contents` entries must be `{ "role": "user"|"model", "parts": [{"text": "..."}] }`.
pub async fn chat_google(
    client: &Client,
    api_key: &str,
    model: &str,
    system_instruction: Option<String>,
    contents: Vec<Value>,
) -> Result<String, String> {
    let mut body = json!({ "contents": contents });
    if let Some(s) = system_instruction {
        body["systemInstruction"] = json!({
            "parts": [{"text": s}]
        });
    }
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model,
        urlencoding::encode(api_key)
    );
    let res = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = res.status();
    let v: Value = res.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(v.to_string());
    }
    v.pointer("/candidates/0/content/parts/0/text")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("unexpected Gemini response: {v}"))
}

pub fn resolve_model(provider: AssistantProvider, model: Option<String>) -> String {
    match provider {
        AssistantProvider::Openai => model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
        AssistantProvider::Ollama => model.unwrap_or_else(default_ollama_model),
        AssistantProvider::Anthropic => {
            model.unwrap_or_else(|| "claude-3-5-haiku-20241022".to_string())
        }
        AssistantProvider::Google => model.unwrap_or_else(|| "gemini-1.5-flash".to_string()),
    }
}

pub fn require_key(provider: AssistantProvider) -> Result<(), String> {
    match provider {
        AssistantProvider::Openai => {
            trim_env("OPENAI_API_KEY").ok_or_else(|| "OPENAI_API_KEY is not set".to_string())?;
        }
        AssistantProvider::Anthropic => {
            trim_env("ANTHROPIC_API_KEY").ok_or_else(|| "ANTHROPIC_API_KEY is not set".to_string())?;
        }
        AssistantProvider::Google => {
            google_api_key().ok_or_else(|| {
                "GEMINI_API_KEY or GOOGLE_API_KEY is not set".to_string()
            })?;
        }
        AssistantProvider::Ollama => {}
    }
    Ok(())
}

pub async fn run_chat(
    client: &Client,
    provider: AssistantProvider,
    model: String,
    openai_messages: Vec<Value>,
    anthropic_messages: Vec<Value>,
    anthropic_system: Option<String>,
    gemini_messages: Vec<Value>,
    gemini_system: Option<String>,
) -> Result<String, String> {
    match provider {
        AssistantProvider::Openai => {
            let key = trim_env("OPENAI_API_KEY").ok_or_else(|| "OPENAI_API_KEY missing".to_string())?;
            chat_openai(client, &key, &model, openai_messages).await
        }
        AssistantProvider::Ollama => {
            let base = ollama_base_url();
            chat_ollama(client, &base, &model, openai_messages).await
        }
        AssistantProvider::Anthropic => {
            let key =
                trim_env("ANTHROPIC_API_KEY").ok_or_else(|| "ANTHROPIC_API_KEY missing".to_string())?;
            chat_anthropic(client, &key, &model, anthropic_system, anthropic_messages).await
        }
        AssistantProvider::Google => {
            let key = google_api_key().ok_or_else(|| "GEMINI_API_KEY missing".to_string())?;
            chat_google(client, &key, &model, gemini_system, gemini_messages).await
        }
    }
}
