<template>
  <header class="dc-navbar">
    <div class="dc-navbar__left">
      <img src="/imgs/favicon/favicon-32x32.png" alt="DasherControl" width="28" height="28" />
    </div>
    <div class="dc-navbar__right">
      <n-button
        quaternary
        circle
        title="Settings"
        aria-label="Open settings"
        @click="emit('changeActive', 'Settings')"
      >
        <template #icon><i class="bx bx-cog" aria-hidden="true" /></template>
      </n-button>
      <n-button v-if="!isLogin" quaternary aria-label="Log in" @click="emit('changeActive', 'Login')">
        Login
      </n-button>
      <n-button
        v-if="!isLogin"
        type="primary"
        aria-label="Sign up for an account"
        @click="emit('changeActive', 'SignUp')"
      >
        Sign Up
      </n-button>
      <n-button v-if="isLogin" quaternary aria-label="Log out" @click="logout">Logout</n-button>
    </div>
  </header>
</template>

<script setup lang="ts">
import axios from "axios";
import { NButton } from "naive-ui";
import { computed } from "vue";
import { useUserStore } from "../stores/user";

const emit = defineEmits<{ changeActive: [value: string] }>();
const user = useUserStore();
const isLogin = computed(() => user.token !== "");

function logout() {
  axios({ method: "post", url: "/tenants/api/logout" }).then(() => {
    user.setUser({ email: "", username: "" });
    user.setToken("");
  });
}
</script>

<style scoped lang="scss">
.dc-navbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 20px;
  margin-bottom: 16px;
  background: var(--ui-chrome-bg);
  color: var(--ui-chrome-fg);
  border-bottom: 1px solid var(--ui-chrome-border);
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.35);
}
.dc-navbar__right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.dc-navbar :deep(.n-button:not(.n-button--primary-type)) {
  --n-text-color: rgba(244, 244, 245, 0.92) !important;
  --n-text-color-hover: #ffffff !important;
  --n-text-color-pressed: rgba(244, 244, 245, 0.85) !important;
  --n-text-color-focus: #ffffff !important;
}

.dc-navbar :deep(.n-button.n-button--primary-type) {
  --n-color: var(--ui-accent) !important;
  --n-color-hover: var(--ui-accent-hover) !important;
  --n-color-pressed: var(--ui-accent-pressed) !important;
  --n-text-color: #ffffff !important;
  --n-text-color-hover: #ffffff !important;
  --n-border: none !important;
}
</style>
