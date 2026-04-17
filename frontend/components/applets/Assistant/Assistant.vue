<template>
  <div class="assistant-applet">
    <div class="assistant-toolbar">
      <n-select
        v-model:value="provider"
        :options="providerOptions"
        size="small"
        style="width: 160px"
      />
      <n-input
        v-model:value="model"
        size="small"
        placeholder="Model (optional)"
        style="flex: 1; min-width: 120px"
      />
      <n-switch v-model:value="includeMetrics" size="small">
        <template #checked>Stats</template>
        <template #unchecked>Stats</template>
      </n-switch>
    </div>
    <n-scrollbar class="assistant-messages" trigger="hover">
      <div v-for="(m, idx) in messages" :key="idx" :class="['msg', m.role]">
        <span class="msg-role">{{ m.role }}</span>
        <pre class="msg-body">{{ m.content }}</pre>
      </div>
      <div v-if="!messages.length" class="empty">Ask about applets, server metrics, or Caddyfile generation.</div>
    </n-scrollbar>
    <div class="assistant-input-row">
      <n-input
        v-model:value="draft"
        type="textarea"
        placeholder="Message…"
        :autosize="{ minRows: 2, maxRows: 6 }"
        @keydown.enter.exact.prevent="send"
      />
      <n-button type="primary" :loading="loading" :disabled="!draft.trim()" @click="send">Send</n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import axios from "axios";
import { NButton, NInput, NScrollbar, NSelect, NSwitch } from "naive-ui";
import { onMounted, ref } from "vue";
import { useAssistantSettingsStore, type AssistantProviderId } from "../../../stores/assistantSettings";
import { useUserStore } from "../../../stores/user";
import { useWorkspaceContextStore } from "../../../stores/workspaceContext";
import { createNotification } from "../../../utils";

type ChatRole = "user" | "assistant" | "system";

type ChatMessage = { role: ChatRole; content: string };

const user = useUserStore();
const assistantDefaults = useAssistantSettingsStore();
const workspaceCtx = useWorkspaceContextStore();

const provider = ref<AssistantProviderId>("ollama");
const model = ref("");
const includeMetrics = ref(false);
const messages = ref<ChatMessage[]>([]);
const draft = ref("");
const loading = ref(false);

const providerOptions = [
  { label: "Ollama", value: "ollama" as const },
  { label: "OpenAI", value: "openai" as const },
  { label: "Anthropic", value: "anthropic" as const },
  { label: "Google", value: "google" as const },
];

onMounted(() => {
  provider.value = assistantDefaults.defaultProvider;
  model.value = assistantDefaults.defaultModel;
  includeMetrics.value = assistantDefaults.defaultIncludeMetrics;
});

function authHeaders() {
  const t = user.token;
  return t ? { Authorization: `Bearer ${t}` } : {};
}

function buildWorkspacePayload() {
  const name = (workspaceCtx.workspaceName || "").trim();
  const hasMeta =
    workspaceCtx.workspaceId > 0 ||
    name.length > 0 ||
    workspaceCtx.applets.length > 0;
  if (!hasMeta) {
    return undefined;
  }
  return {
    workspace_id: workspaceCtx.workspaceId,
    workspace_name: workspaceCtx.workspaceName,
    applets: workspaceCtx.applets,
  };
}

async function send() {
  const text = draft.value.trim();
  if (!text) return;
  if (!user.token) {
    createNotification(null, "AI assistant", "Sign in required", "danger");
    return;
  }

  const nextMessages: ChatMessage[] = [...messages.value, { role: "user", content: text }];
  messages.value = nextMessages;
  draft.value = "";
  loading.value = true;

  const apiMessages = nextMessages.map((m) => ({ role: m.role, content: m.content }));
  const body: Record<string, unknown> = {
    provider: provider.value,
    model: model.value.trim() || null,
    messages: apiMessages,
    include_metrics: includeMetrics.value,
  };
  const ws = buildWorkspacePayload();
  if (ws) {
    body.workspace_context = ws;
  }

  try {
    const { data } = await axios.post("/assistant/api/chat", body, {
      headers: {
        "Content-Type": "application/json",
        ...authHeaders(),
      },
    });
    const reply = typeof data?.message === "string" ? data.message : JSON.stringify(data);
    messages.value = [...messages.value, { role: "assistant", content: reply }];
  } catch (err: unknown) {
    const ax = err as { response?: { data?: unknown; status?: number } };
    const data = ax.response?.data as { error?: string } | undefined;
    const msg =
      (typeof data?.error === "string" && data.error) ||
      (ax.response?.status === 401 ? "Sign in required" : null) ||
      String(err);
    createNotification(null, "AI assistant", msg, "danger");
    messages.value = messages.value.slice(0, -1);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped lang="scss">
.assistant-applet {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 200px;
  background: var(--darcule-bg, var(--bg));
  color: var(--fg);
  padding: 8px;
  box-sizing: border-box;
}
.assistant-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  margin-bottom: 8px;
}
.assistant-messages {
  flex: 1;
  min-height: 120px;
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 8px;
  padding: 8px;
  margin-bottom: 8px;
}
.msg {
  margin-bottom: 12px;
}
.msg-role {
  display: block;
  font-size: 11px;
  text-transform: uppercase;
  opacity: 0.65;
  margin-bottom: 4px;
}
.msg-body {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: ui-monospace, monospace;
  font-size: 13px;
  line-height: 1.45;
}
.msg.user .msg-body {
  opacity: 0.95;
}
.msg.assistant .msg-body {
  opacity: 1;
}
.empty {
  font-size: 13px;
  opacity: 0.7;
  padding: 8px;
}
.assistant-input-row {
  display: flex;
  gap: 8px;
  align-items: flex-end;
}
</style>
