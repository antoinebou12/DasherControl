<template>
  <n-modal
    v-model:show="show"
    preset="card"
    title="Settings"
    :style="{ width: 'min(640px, 92vw)' }"
    class="settings-modal"
  >
    <p class="user-line">
      <strong>Connected user</strong>:
      {{ user.username || user.email || "Not connected" }}
    </p>

    <n-tabs v-model:value="activeTab" type="line" animated class="settings-tabs">
      <n-tab-pane name="workspace" tab="Workspace look">
        <n-alert type="info" class="settings-alert" :bordered="false">
          Theme, icon, and background are stored on the workspace. Use the toolbar
          <strong>Save</strong> or <strong>Export</strong> (update) after changing colors here so the server keeps them
          with your grid.
        </n-alert>

        <n-form class="theme-form" label-placement="left" label-width="108">
          <n-form-item label="Icon">
            <n-input
              :value="appearanceStore.workspace_icon"
              placeholder="Emoji (e.g. 🧩) or Boxicons (e.g. bx-grid-alt)"
              clearable
              @update:value="onIcon"
            />
          </n-form-item>
          <n-form-item label="Preset">
            <n-select
              :value="appearanceStore.theme_preset"
              :options="presetOptions"
              @update:value="onPreset"
            />
          </n-form-item>
          <n-form-item label="Background URL">
            <n-input
              :value="appearanceStore.background_image_url ?? ''"
              type="textarea"
              placeholder="https://… (optional image behind the grid)"
              :autosize="{ minRows: 2, maxRows: 4 }"
              clearable
              @update:value="onBgUrl"
            />
          </n-form-item>
          <n-form-item label="BG hex">
            <n-input
              :value="appearanceStore.bg_color ?? ''"
              placeholder="Optional override (#282a36)"
              clearable
              @update:value="(v) => patchColor('bg_color', v)"
            />
          </n-form-item>
          <n-form-item label="FG hex">
            <n-input
              :value="appearanceStore.fg_color ?? ''"
              placeholder="Optional text / foreground"
              clearable
              @update:value="(v) => patchColor('fg_color', v)"
            />
          </n-form-item>
          <n-form-item label="Accent hex">
            <n-input
              :value="appearanceStore.accent_color ?? ''"
              placeholder="Optional accent / primary"
              clearable
              @update:value="(v) => patchColor('accent_color', v)"
            />
          </n-form-item>
          <n-form-item label="Panel hex">
            <n-input
              :value="appearanceStore.panel_color ?? ''"
              placeholder="Optional panels / borders"
              clearable
              @update:value="(v) => patchColor('panel_color', v)"
            />
          </n-form-item>
        </n-form>

        <n-space justify="end" style="margin-top: 12px">
          <n-button quaternary @click="resetColorsToPreset">Match preset colors</n-button>
          <n-button quaternary @click="appearanceStore.resetToDefaults">Reset to Dracula</n-button>
        </n-space>
      </n-tab-pane>

      <n-tab-pane name="ai" tab="AI assistant">
        <n-alert type="warning" class="settings-alert" :bordered="false">
          API keys stay on the server in <code>.env</code> (never in the browser). See
          <code>.env.example</code> for <code>OPENAI_API_KEY</code>, <code>ANTHROPIC_API_KEY</code>,
          <code>GEMINI_API_KEY</code> or <code>GOOGLE_API_KEY</code>, and optional
          <code>OLLAMA_BASE_URL</code> / <code>OLLAMA_MODEL</code>. Restart the backend after changing keys.
        </n-alert>

        <n-spin :show="aiStatusLoading">
          <n-form class="theme-form" label-placement="left" label-width="140">
            <n-form-item label="Default provider">
              <n-select
                :value="assistantStore.defaultProvider"
                :options="providerOptions"
                style="width: 100%"
                @update:value="onAssistantProvider"
              />
            </n-form-item>
            <n-form-item label="Default model">
              <n-input
                :value="assistantStore.defaultModel"
                placeholder="Leave empty for built-in defaults per provider"
                clearable
                @update:value="onAssistantModel"
              />
            </n-form-item>
            <n-form-item label="Include server stats">
              <n-switch :value="assistantStore.defaultIncludeMetrics" @update:value="onAssistantIncludeMetrics" />
              <span class="hint">When on, each chat can send a live metrics snapshot (toggle in the applet too).</span>
            </n-form-item>
            <n-form-item v-if="aiStatus" label="Server keys">
              <ul class="status-list">
                <li>OpenAI: {{ aiStatus.openai ? "configured" : "not set" }}</li>
                <li>Anthropic: {{ aiStatus.anthropic ? "configured" : "not set" }}</li>
                <li>Google Gemini: {{ aiStatus.google ? "configured" : "not set" }}</li>
                <li>Ollama: {{ aiStatus.ollama ? "available" : "off" }} (no key; uses OLLAMA_BASE_URL)</li>
              </ul>
            </n-form-item>
          </n-form>
        </n-spin>

        <n-space justify="end" style="margin-top: 12px">
          <n-button quaternary @click="resetAssistantDefaults">Reset assistant defaults</n-button>
        </n-space>
      </n-tab-pane>
    </n-tabs>
  </n-modal>
</template>

<script setup lang="ts">
import axios from "axios";
import {
  NAlert,
  NButton,
  NForm,
  NFormItem,
  NInput,
  NModal,
  NSelect,
  NSpace,
  NSpin,
  NSwitch,
  NTabPane,
  NTabs,
} from "naive-ui";
import { ref, watch } from "vue";
import {
  useAssistantSettingsStore,
  type AssistantProviderId,
} from "../stores/assistantSettings";
import { useUserStore } from "../stores/user";
import { useWorkspaceAppearanceStore } from "../stores/workspaceAppearance";
import { THEME_PRESETS } from "../utils/workspaceTheme";

const show = ref(false);
const activeTab = ref<"workspace" | "ai">("workspace");
const user = useUserStore();
const appearanceStore = useWorkspaceAppearanceStore();
const assistantStore = useAssistantSettingsStore();

const aiStatus = ref<{
  openai: boolean;
  anthropic: boolean;
  google: boolean;
  ollama: boolean;
} | null>(null);
const aiStatusLoading = ref(false);

const presetOptions = Object.keys(THEME_PRESETS).map((k) => ({
  label: k.charAt(0).toUpperCase() + k.slice(1),
  value: k,
}));

const providerOptions = [
  { label: "Ollama (local)", value: "ollama" as const },
  { label: "OpenAI", value: "openai" as const },
  { label: "Anthropic (Claude)", value: "anthropic" as const },
  { label: "Google Gemini", value: "google" as const },
];

function onIcon(v: string) {
  appearanceStore.workspace_icon = v;
  appearanceStore.preview();
}

function onPreset(v: string) {
  appearanceStore.theme_preset = v;
  appearanceStore.preview();
}

function onBgUrl(v: string) {
  const t = v.trim();
  appearanceStore.background_image_url = t.length ? t : null;
  appearanceStore.preview();
}

function patchColor(key: "bg_color" | "fg_color" | "accent_color" | "panel_color", v: string) {
  const t = v.trim();
  const val = t.length ? t : null;
  if (key === "bg_color") appearanceStore.bg_color = val;
  else if (key === "fg_color") appearanceStore.fg_color = val;
  else if (key === "accent_color") appearanceStore.accent_color = val;
  else appearanceStore.panel_color = val;
  appearanceStore.preview();
}

function resetColorsToPreset() {
  const preset = THEME_PRESETS[appearanceStore.theme_preset] || THEME_PRESETS.dracula;
  appearanceStore.$patch({
    bg_color: preset.bg,
    fg_color: preset.fg,
    accent_color: preset.accent,
    panel_color: preset.panel,
  });
  appearanceStore.preview();
}

function onAssistantProvider(v: AssistantProviderId) {
  assistantStore.defaultProvider = v;
  assistantStore.save();
}

function onAssistantModel(v: string) {
  assistantStore.defaultModel = v;
  assistantStore.save();
}

function onAssistantIncludeMetrics(v: boolean) {
  assistantStore.defaultIncludeMetrics = v;
  assistantStore.save();
}

function resetAssistantDefaults() {
  assistantStore.reset();
}

async function fetchAiStatus() {
  const token = user.token;
  if (!token) {
    aiStatus.value = null;
    return;
  }
  aiStatusLoading.value = true;
  try {
    const { data } = await axios.get("/assistant/api/status", {
      headers: { Authorization: `Bearer ${token}` },
    });
    aiStatus.value = data;
  } catch {
    aiStatus.value = null;
  } finally {
    aiStatusLoading.value = false;
  }
}

watch(show, (v) => {
  if (v) {
    fetchAiStatus();
  }
});

defineExpose({
  showDialog: () => {
    show.value = true;
  },
  hideDialog: () => {
    show.value = false;
  },
});
</script>

<style scoped>
.user-line {
  margin-bottom: 8px;
}
.settings-alert {
  margin-bottom: 12px;
}
.theme-form {
  margin-top: 8px;
}
.settings-tabs :deep(.n-tab-pane) {
  padding-top: 8px;
}
.hint {
  margin-left: 10px;
  font-size: 12px;
  opacity: 0.85;
}
.status-list {
  margin: 0;
  padding-left: 1.2rem;
  font-size: 13px;
  line-height: 1.5;
}
</style>
