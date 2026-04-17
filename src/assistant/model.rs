use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AssistantProvider {
    Openai,
    Ollama,
    Anthropic,
    Google,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatBody {
    pub provider: AssistantProvider,
    pub model: Option<String>,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub include_metrics: bool,
    #[serde(default)]
    pub workspace_context: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AssistantStatus {
    pub openai: bool,
    pub anthropic: bool,
    pub google: bool,
    /// Ollama is always offered (no API key); base URL defaults if unset.
    pub ollama: bool,
}
