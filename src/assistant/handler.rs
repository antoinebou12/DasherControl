use std::time::Duration;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};

use crate::assistant::model::{AssistantProvider, AssistantStatus, ChatBody, ChatResponse};
use crate::assistant::providers::{self, require_key, resolve_model, run_chat};
use crate::system::metrics::SystemMetricsState;
use crate::tenants::token::Claims;

const BASE_SYSTEM_PROMPT: &str = r#"You are DasherControl Assistant, helping users with a homelab dashboard.

DasherControl is a grid of applets. Builtin applet ids: CreateNew, Editor, IFrame, Shortcut, DockerManager, SystemMonitor, CpuMetrics, MemoryMetrics, DiskMetrics, NetworkMetrics, GpuMetrics, Terminal, CaddyConfigurator, Assistant, Canvas.

- To add an applet: use the Create new applet, pick a type, configure settings, then save the workspace from the toolbar.
- CaddyConfigurator calls POST /caddy/api/generate with JSON sites; it only generates Caddyfile text — it does not reload Caddy or obtain certificates. Users paste the result into their real Caddy config.
- SystemMonitor reads live server metrics from this host.

Be concise. If server metrics JSON is attached, summarize only what is present — do not invent numbers."#;

fn env_nonempty(name: &str) -> bool {
    std::env::var(name)
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

#[get("/api/status")]
pub fn status(_claims: Claims) -> Json<AssistantStatus> {
    Json(AssistantStatus {
        openai: env_nonempty("OPENAI_API_KEY"),
        anthropic: env_nonempty("ANTHROPIC_API_KEY"),
        google: providers::google_api_key().is_some(),
        ollama: true,
    })
}

fn normalize_role(r: &str) -> String {
    r.trim().to_ascii_lowercase()
}

fn build_context(
    body: &ChatBody,
    metrics_json: Option<String>,
) -> (String, Vec<ChatMessageLite>) {
    let mut system_chunks: Vec<String> = vec![BASE_SYSTEM_PROMPT.to_string()];
    if let Some(m) = metrics_json {
        system_chunks.push(format!("Current server metrics (JSON):\n{m}"));
    }
    if let Some(ws) = &body.workspace_context {
        if let Ok(s) = serde_json::to_string_pretty(ws) {
            system_chunks.push(format!("Current workspace context (JSON):\n{s}"));
        }
    }

    let mut convo: Vec<ChatMessageLite> = Vec::new();
    for m in &body.messages {
        let role = normalize_role(&m.role);
        let content = m.content.trim().to_string();
        if content.is_empty() {
            continue;
        }
        if role == "system" {
            system_chunks.push(content);
            continue;
        }
        if role != "user" && role != "assistant" {
            continue;
        }
        convo.push(ChatMessageLite { role, content });
    }

    (system_chunks.join("\n\n"), convo)
}

struct ChatMessageLite {
    role: String,
    content: String,
}

fn to_openai_messages(system: &str, convo: &[ChatMessageLite]) -> Vec<Value> {
    let mut out = vec![json!({
        "role": "system",
        "content": system,
    })];
    for m in convo {
        out.push(json!({
            "role": &m.role,
            "content": &m.content,
        }));
    }
    out
}

fn to_anthropic_messages(convo: &[ChatMessageLite]) -> Vec<Value> {
    convo
        .iter()
        .map(|m| {
            json!({
                "role": &m.role,
                "content": &m.content,
            })
        })
        .collect()
}

fn to_gemini_convo(convo: &[ChatMessageLite]) -> Vec<Value> {
    convo
        .iter()
        .map(|m| {
            json!({
                "role": if m.role == "assistant" { "model" } else { "user" },
                "parts": [{"text": &m.content}]
            })
        })
        .collect()
}

#[post("/api/chat", format = "json", data = "<body>")]
pub async fn chat(
    body: Json<ChatBody>,
    _claims: Claims,
    metrics_state: &State<SystemMetricsState>,
) -> Result<Json<ChatResponse>, (Status, Json<Value>)> {
    let body = body.into_inner();
    if body.messages.is_empty() {
        return Err((
            Status::BadRequest,
            Json(json!({ "error": "messages must not be empty" })),
        ));
    }

    if let Err(e) = require_key(body.provider) {
        return Err((Status::BadRequest, Json(json!({ "error": e }))));
    }

    let metrics_json = if body.include_metrics {
        Some(
            serde_json::to_string(&metrics_state.snapshot())
                .map_err(|_| (Status::InternalServerError, Json(json!({ "error": "metrics" }))))?,
        )
    } else {
        None
    };

    let (system_combined, convo) = build_context(&body, metrics_json);
    if convo.is_empty() {
        return Err((
            Status::BadRequest,
            Json(json!({ "error": "at least one user or assistant message is required" })),
        ));
    }

    let model = resolve_model(body.provider, body.model.clone());
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(json!({ "error": "http client" })),
            )
        })?;

    let openai_msgs = to_openai_messages(&system_combined, &convo);
    let anth_msgs = to_anthropic_messages(&convo);
    let gem_msgs = to_gemini_convo(&convo);

    let anthropic_system = if body.provider == AssistantProvider::Anthropic {
        Some(system_combined.clone())
    } else {
        None
    };
    let gemini_system = if body.provider == AssistantProvider::Google {
        Some(system_combined.clone())
    } else {
        None
    };

    let text = run_chat(
        &client,
        body.provider,
        model,
        openai_msgs,
        anth_msgs,
        anthropic_system,
        gem_msgs,
        gemini_system,
    )
    .await
    .map_err(|e| (Status::BadGateway, Json(json!({ "error": e }))))?;

    Ok(Json(ChatResponse { message: text }))
}
