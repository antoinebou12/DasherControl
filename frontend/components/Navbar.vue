<template>
  <header class="dc-navbar">
    <div class="dc-navbar__left">
      <img src="/imgs/favicon/favicon-32x32.png" alt="" width="28" height="28" />
    </div>
    <div class="dc-navbar__right">
      <n-button v-if="isLogin" quaternary circle @click="emit('changeActive', 'Settings')">
        <template #icon><i class="bx bx-cog" /></template>
      </n-button>
      <n-button v-if="!isLogin" quaternary @click="emit('changeActive', 'Login')">Login</n-button>
      <n-button v-if="!isLogin" type="primary" @click="emit('changeActive', 'SignUp')">Sign Up</n-button>
      <n-button v-if="isLogin" quaternary @click="logout">Logout</n-button>
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
  padding: 8px 16px;
  margin-bottom: 48px;
  background: rgba(0, 0, 0, 0.2);
  border-bottom: 1px solid var(--cl);
}
.dc-navbar__right {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
