<template>
  <n-config-provider :theme="darkTheme" :theme-overrides="naiveThemeOverrides">
    <div id="app" class="dc-root">
      <Main msg="" />
    </div>
  </n-config-provider>
</template>

<script setup lang="ts">
import { NConfigProvider, darkTheme } from "naive-ui";
import { computed, onMounted } from "vue";
import Main from "./pages/Main.vue";
import { useUserStore } from "./stores/user";
import { useWorkspaceAppearanceStore } from "./stores/workspaceAppearance";
import { createNotification } from "./utils";
import {
  resolveWorkspaceAccent,
  workspaceAccentHoverFromAccent,
  workspaceAccentPressedFromAccent,
} from "./utils/workspaceTheme";

const user = useUserStore();
const appearance = useWorkspaceAppearanceStore();

const naiveThemeOverrides = computed(() => {
  const accent = resolveWorkspaceAccent(appearance.$state);
  return {
    common: {
      primaryColor: accent,
      primaryColorHover: workspaceAccentHoverFromAccent(accent),
      primaryColorPressed: workspaceAccentPressedFromAccent(accent),
      primaryColorSuppl: accent,
      borderRadius: "8px",
    },
  };
});

onMounted(async () => {
  appearance.preview();
  const params = new URLSearchParams(window.location.search);
  if (params.get("oauth_error")) {
    createNotification(null, "Sign-in failed", "OAuth could not complete. Check server logs and OAuth app settings.", "danger");
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
  /* Applets: white surface, black type (independent of workspace theme) */
  --applet-bg: #ffffff;
  --applet-fg: #000000;
  --applet-border: #000000;
  --applet-chrome-bg: #16161d;
  --applet-chrome-fg: #ffffff;
  /* Shell chrome (navbar, guest bar, toolbar): dark dashboard */
  --ui-chrome-bg: #1e1e26;
  --ui-chrome-fg: #f4f4f5;
  --ui-chrome-border: #2a2a35;
  --ui-accent: #2563eb;
  --ui-accent-hover: #3b82f6;
  --ui-accent-pressed: #1d4ed8;
  --ui-danger: #ef4444;
  /* Create-applet / Shortcut panel (dark strip; applet tile content stays --applet-bg) */
  --create-panel-bg: #2c2e3a;
  --create-panel-fg: #f4f4f5;
  --surface-glass: rgba(30, 32, 40, 0.72);
  --create-surface-solid: rgba(248, 250, 252, 0.96);
  --create-border: rgba(255, 255, 255, 0.12);
}
html {
  background: var(--bg);
  font-family: "Poppins", system-ui, sans-serif;
}
#app,
.dc-root {
  width: 100%;
  min-height: 100vh;
}
</style>
