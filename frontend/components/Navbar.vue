<template>
  <v-app-bar flat density="comfortable" color="surface" elevation="2" class="dc-navbar dc-navbar-bar border-b-thin">
    <template #prepend>
      <v-img
        src="/imgs/favicon/favicon-32x32.png"
        alt="DasherControl"
        width="28"
        height="28"
        max-width="28"
        class="mr-2 ml-1"
      />
    </template>
    <v-app-bar-title class="text-h6 font-weight-semibold">DasherControl</v-app-bar-title>
    <v-spacer />
    <v-btn
      icon="mdi-dock-window"
      variant="text"
      aria-label="Toggle tools panel"
      @click="shell.toggleRightPanel()"
    />
    <v-btn icon="mdi-cog" variant="text" title="Settings" aria-label="Open settings" @click="emit('changeActive', 'Settings')" />
    <v-btn v-if="!isLogin" variant="text" aria-label="Log in" @click="emit('changeActive', 'Login')">Login</v-btn>
    <v-btn v-if="!isLogin" color="primary" class="ml-1" aria-label="Sign up for an account" @click="emit('changeActive', 'SignUp')">
      Sign Up
    </v-btn>
    <v-btn v-if="isLogin" variant="text" aria-label="Log out" @click="logout">Logout</v-btn>
  </v-app-bar>
</template>

<script setup lang="ts">
import axios from "axios";
import { computed } from "vue";
import { useUiShellStore } from "../stores/uiShell";
import { useUserStore } from "../stores/user";

const emit = defineEmits<{ changeActive: [value: string] }>();
const user = useUserStore();
const shell = useUiShellStore();
const isLogin = computed(() => user.token !== "");

function logout() {
  axios({ method: "post", url: "/tenants/api/logout" }).then(() => {
    user.setUser({ email: "", username: "" });
    user.setToken("");
  });
}
</script>

<style scoped lang="scss">
.dc-navbar-bar {
  border-color: var(--ui-chrome-border) !important;
  color: var(--ui-chrome-fg);
}
</style>
