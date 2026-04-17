import { defineStore } from "pinia";

const STORAGE_KEY = "dashercontrol_assistant_settings_v1";

export type AssistantProviderId = "openai" | "ollama" | "anthropic" | "google";

export type AssistantSettingsState = {
  defaultProvider: AssistantProviderId;
  defaultModel: string;
  defaultIncludeMetrics: boolean;
};

function load(): AssistantSettingsState {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) throw new Error("empty");
    const o = JSON.parse(raw) as Partial<AssistantSettingsState>;
    return {
      defaultProvider: normalizeProvider(o.defaultProvider),
      defaultModel: typeof o.defaultModel === "string" ? o.defaultModel : "",
      defaultIncludeMetrics: !!o.defaultIncludeMetrics,
    };
  } catch {
    return {
      defaultProvider: "ollama",
      defaultModel: "",
      defaultIncludeMetrics: false,
    };
  }
}

function normalizeProvider(v: unknown): AssistantProviderId {
  if (v === "openai" || v === "ollama" || v === "anthropic" || v === "google") {
    return v;
  }
  return "ollama";
}

function persist(state: AssistantSettingsState) {
  localStorage.setItem(
    STORAGE_KEY,
    JSON.stringify({
      defaultProvider: state.defaultProvider,
      defaultModel: state.defaultModel,
      defaultIncludeMetrics: state.defaultIncludeMetrics,
    })
  );
}

export const useAssistantSettingsStore = defineStore("assistantSettings", {
  state: (): AssistantSettingsState => load(),
  actions: {
    save() {
      persist(this.$state);
    },
    reset() {
      this.$patch({
        defaultProvider: "ollama",
        defaultModel: "",
        defaultIncludeMetrics: false,
      });
      this.save();
    },
  },
});
