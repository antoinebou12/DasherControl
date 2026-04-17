<template>
  <div id="app" class="dc-root">
    <Main msg="" />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import Main from "./pages/Main.vue";
import { useUserStore } from "./stores/user";
import { createNotification } from "./utils";

const user = useUserStore();
onMounted(async () => {
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
