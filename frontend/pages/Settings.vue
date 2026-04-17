<template>
  <v-dialog v-model="show" max-width="680" scrollable transition="dialog-bottom-transition">
    <v-card rounded="lg">
      <v-card-title class="text-h6">Settings</v-card-title>
      <v-card-text>
        <v-alert v-if="!user.token" type="warning" variant="tonal" density="comfortable" class="mb-3">
          <div class="font-weight-medium mb-1">Not signed in</div>
          <div>Sign in to sync workspaces and use features that require a session.</div>
          <div class="d-flex ga-2 mt-3 flex-wrap">
            <v-btn color="primary" size="small" @click="emit('requestLogin')">Login</v-btn>
            <v-btn variant="tonal" size="small" @click="emit('requestSignUp')">Sign up</v-btn>
          </div>
        </v-alert>

        <p class="user-line">
          <strong>Connected user</strong>:
          {{ user.username || user.email || "Not connected" }}
        </p>

        <v-tabs v-model="activeTab" color="primary" class="mb-2">
          <v-tab value="workspace">Workspace look</v-tab>
          <v-tab value="ai">AI assistant</v-tab>
        </v-tabs>

        <v-window v-model="activeTab">
          <v-window-item value="workspace">
            <v-alert type="info" variant="tonal" density="comfortable" class="mb-3">
              Theme, icon, and background are stored on the workspace. Use the toolbar
              <strong>Save</strong> or <strong>Export</strong> (update) after changing colors here so the server keeps them
              with your grid.
            </v-alert>

            <v-row dense>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">Icon</v-col>
              <v-col cols="12" sm="8">
                <v-text-field
                  :model-value="appearanceStore.workspace_icon"
                  placeholder="Emoji or Boxicons class"
                  variant="outlined"
                  density="compact"
                  hide-details="auto"
                  clearable
                  @update:model-value="onIcon"
                />
              </v-col>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">Preset</v-col>
              <v-col cols="12" sm="8">
                <v-select
                  :model-value="appearanceStore.theme_preset"
                  :items="presetOptions"
                  item-title="label"
                  item-value="value"
                  variant="outlined"
                  density="compact"
                  hide-details="auto"
                  @update:model-value="onPreset"
                />
              </v-col>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">Background URL</v-col>
              <v-col cols="12" sm="8">
                <v-textarea
                  :model-value="appearanceStore.background_image_url ?? ''"
                  placeholder="https://… (optional image behind the grid)"
                  variant="outlined"
                  density="compact"
                  rows="2"
                  auto-grow
                  hide-details="auto"
                  clearable
                  @update:model-value="onBgUrl"
                />
              </v-col>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">BG hex</v-col>
              <v-col cols="12" sm="8">
                <v-text-field
                  :model-value="appearanceStore.bg_color ?? ''"
                  placeholder="Optional override (#282a36)"
                  variant="outlined"
                  density="compact"
                  hide-details="auto"
                  clearable
                  @update:model-value="(v) => patchColor('bg_color', v)"
                />
              </v-col>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">FG hex</v-col>
              <v-col cols="12" sm="8">
                <v-text-field
                  :model-value="appearanceStore.fg_color ?? ''"
                  placeholder="Optional text / foreground"
                  variant="outlined"
                  density="compact"
                  hide-details="auto"
                  clearable
                  @update:model-value="(v) => patchColor('fg_color', v)"
                />
              </v-col>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">Accent hex</v-col>
              <v-col cols="12" sm="8">
                <v-text-field
                  :model-value="appearanceStore.accent_color ?? ''"
                  placeholder="Optional accent / primary"
                  variant="outlined"
                  density="compact"
                  hide-details="auto"
                  clearable
                  @update:model-value="(v) => patchColor('accent_color', v)"
                />
              </v-col>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">Panel hex</v-col>
              <v-col cols="12" sm="8">
                <v-text-field
                  :model-value="appearanceStore.panel_color ?? ''"
                  placeholder="Optional panels / borders"
                  variant="outlined"
                  density="compact"
                  hide-details="auto"
                  clearable
                  @update:model-value="(v) => patchColor('panel_color', v)"
                />
              </v-col>
            </v-row>

            <div class="d-flex justify-end ga-2 flex-wrap mt-3">
              <v-btn variant="text" @click="resetColorsToPreset">Match preset colors</v-btn>
              <v-btn variant="text" @click="appearanceStore.resetToDefaults">Reset to Dracula</v-btn>
            </div>
          </v-window-item>

          <v-window-item value="ai">
            <v-alert type="warning" variant="tonal" density="comfortable" class="mb-3">
              API keys stay on the server in <code>.env</code> (never in the browser). See
              <code>.env.example</code> for <code>OPENAI_API_KEY</code>, <code>ANTHROPIC_API_KEY</code>,
              <code>GEMINI_API_KEY</code> or <code>GOOGLE_API_KEY</code>, and optional
              <code>OLLAMA_BASE_URL</code> / <code>OLLAMA_MODEL</code>. Restart the backend after changing keys.
            </v-alert>

            <v-progress-linear v-if="aiStatusLoading" indeterminate color="primary" class="mb-2" />

            <v-row dense>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">Default provider</v-col>
              <v-col cols="12" sm="8">
                <v-select
                  :model-value="assistantStore.defaultProvider"
                  :items="providerOptions"
                  item-title="label"
                  item-value="value"
                  variant="outlined"
                  density="compact"
                  hide-details="auto"
                  @update:model-value="onAssistantProvider"
                />
              </v-col>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">Default model</v-col>
              <v-col cols="12" sm="8">
                <v-text-field
                  :model-value="assistantStore.defaultModel"
                  placeholder="Leave empty for built-in defaults per provider"
                  variant="outlined"
                  density="compact"
                  hide-details="auto"
                  clearable
                  @update:model-value="onAssistantModel"
                />
              </v-col>
              <v-col cols="12" sm="4" class="text-body-2 text-medium-emphasis pt-sm-2">Include server stats</v-col>
              <v-col cols="12" sm="8">
                <div class="d-flex align-center flex-wrap ga-2">
                  <v-switch
                    :model-value="assistantStore.defaultIncludeMetrics"
                    color="primary"
                    density="compact"
                    hide-details
                    inset
                    @update:model-value="onAssistantIncludeMetrics"
                  />
                  <span class="hint">When on, each chat can send a live metrics snapshot (toggle in the applet too).</span>
                </div>
              </v-col>
              <v-col v-if="aiStatus" cols="12">
                <div class="text-body-2 text-medium-emphasis mb-1">Server keys</div>
                <ul class="status-list">
                  <li>OpenAI: {{ aiStatus.openai ? "configured" : "not set" }}</li>
                  <li>Anthropic: {{ aiStatus.anthropic ? "configured" : "not set" }}</li>
                  <li>Google Gemini: {{ aiStatus.google ? "configured" : "not set" }}</li>
                  <li>Ollama: {{ aiStatus.ollama ? "available" : "off" }} (no key; uses OLLAMA_BASE_URL)</li>
                </ul>
              </v-col>
            </v-row>

            <div class="d-flex justify-end mt-3">
              <v-btn variant="text" @click="resetAssistantDefaults">Reset assistant defaults</v-btn>
            </div>
          </v-window-item>
        </v-window>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import axios from "axios";
import { ref, watch } from "vue";
import { useAssistantSettingsStore, type AssistantProviderId } from "../stores/assistantSettings";
import { useUserStore } from "../stores/user";
import { useWorkspaceAppearanceStore } from "../stores/workspaceAppearance";
import { THEME_PRESETS } from "../utils/workspaceTheme";

const emit = defineEmits<{ requestLogin: []; requestSignUp: [] }>();

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

function onIcon(v: string | null) {
  appearanceStore.workspace_icon = v ?? "";
  appearanceStore.preview();
}

function onPreset(v: string | null) {
  if (v) {
    appearanceStore.theme_preset = v;
    appearanceStore.preview();
  }
}

function onBgUrl(v: string | null) {
  const t = (v ?? "").trim();
  appearanceStore.background_image_url = t.length ? t : null;
  appearanceStore.preview();
}

function patchColor(key: "bg_color" | "fg_color" | "accent_color" | "panel_color", v: string | null) {
  const t = (v ?? "").trim();
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

function onAssistantProvider(v: AssistantProviderId | null) {
  if (v) {
    assistantStore.defaultProvider = v;
    assistantStore.save();
  }
}

function onAssistantModel(v: string | null) {
  assistantStore.defaultModel = v ?? "";
  assistantStore.save();
}

function onAssistantIncludeMetrics(v: boolean | null) {
  assistantStore.defaultIncludeMetrics = !!v;
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
.hint {
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
