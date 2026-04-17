<template>
  <v-app class="dc-v-app">
    <DcSnackbar />
    <Main msg="" />
  </v-app>
</template>

<script setup lang="ts">
import { useTheme } from "vuetify";
import { onMounted, watch } from "vue";
import DcSnackbar from "./components/DcSnackbar.vue";
import Main from "./pages/Main.vue";
import { useUserStore } from "./stores/user";
import { useWorkspaceAppearanceStore } from "./stores/workspaceAppearance";
import { createNotification } from "./utils";
import { resolveWorkspaceAccent } from "./utils/workspaceTheme";

const user = useUserStore();
const appearance = useWorkspaceAppearanceStore();
const theme = useTheme();

function syncVuetifyPrimary() {
  const accent = resolveWorkspaceAccent(appearance.$state);
  const t = theme.themes.value.dcDark;
  if (t?.colors) t.colors.primary = accent;
}

watch(
  () => ({
    preset: appearance.theme_preset,
    accent: appearance.accent_color,
    bg: appearance.bg_color,
  }),
  syncVuetifyPrimary,
  { deep: true },
);

onMounted(async () => {
  syncVuetifyPrimary();
  appearance.preview();
  const params = new URLSearchParams(window.location.search);
  if (params.get("oauth_error")) {
    createNotification(
      null,
      "Sign-in failed",
      "OAuth could not complete. Check server logs and OAuth app settings.",
      "danger",
    );
    params.delete("oauth_error");
    const next = `${window.location.pathname}${params.toString() ? `?${params}` : ""}${window.location.hash}`;
    window.history.replaceState({}, "", next);
  }
  await user.getToken();
  if (user.token) await user.refreshProfile();
});
</script>

<style lang="scss">
@font-face {
  font-family: "Poppins";
  src: url("/fonts/Poppins-Regular.ttf");
  font-weight: normal;
  font-style: normal;
}
:root {
  --bg: #282a36;
  --bg-rgb: 40, 42, 54;
  --fg: #f8f8f2;
  --fg-rgb: 248, 248, 242;
  --comment: #6272a4;
  --cl: #44475a;
  --applet-bg: #ffffff;
  --applet-fg: #000000;
  --applet-border: #000000;
  --applet-chrome-bg: #16161d;
  --applet-chrome-fg: #ffffff;
  --ui-chrome-bg: #1e1e26;
  --ui-chrome-fg: #f4f4f5;
  --ui-chrome-border: #2a2a35;
  --ui-accent: #2563eb;
  --ui-accent-hover: #3b82f6;
  --ui-accent-pressed: #1d4ed8;
  --ui-danger: #ef4444;
  --create-panel-bg: #2c2e3a;
  --create-panel-fg: #f4f4f5;
  --surface-glass: rgba(30, 32, 40, 0.72);
  --create-surface-solid: rgba(248, 250, 252, 0.96);
  --create-border: rgba(255, 255, 255, 0.12);
  --workspace-bg-image: none;
}
html {
  background: var(--bg);
  font-family: "Poppins", system-ui, sans-serif;
}
.dc-v-app {
  width: 100%;
  min-height: 100vh;
}
</style>
