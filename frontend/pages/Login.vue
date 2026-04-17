<template>
  <n-modal v-model:show="show" preset="card" title="Login" class="dc-modal" :style="{ width: '420px' }">
    <n-spin :show="loading">
      <n-space vertical size="large">
        <form @submit.prevent="submitLogIn">
          <n-space vertical size="large">
            <n-input v-model:value="username" placeholder="Email / Username" size="large">
              <template #prefix><i class="bx bxs-user" /></template>
            </n-input>
            <n-input
              v-model:value="password"
              type="password"
              show-password-on="click"
              placeholder="Password"
              size="large"
            >
              <template #prefix><i class="bx bxs-lock" /></template>
            </n-input>
            <n-space justify="space-between" align="center">
              <n-checkbox v-model:checked="remember">Remember me</n-checkbox>
              <a class="dc-muted" href="#">Forgot password?</a>
            </n-space>
            <n-button type="primary" attr-type="submit" block size="large">Sign in</n-button>
          </n-space>
        </form>
        <n-divider>or continue with</n-divider>
        <n-space vertical size="small">
          <n-button block size="large" quaternary @click="startOAuth('google')">
            <template #icon><i class="bx bxl-google" /></template>
            Google
          </n-button>
          <n-button block size="large" quaternary @click="startOAuth('github')">
            <template #icon><i class="bx bxl-github" /></template>
            GitHub
          </n-button>
        </n-space>
      </n-space>
    </n-spin>
  </n-modal>
</template>

<script setup lang="ts">
import axios from "axios";
import { NButton, NCheckbox, NDivider, NInput, NModal, NSpin, NSpace } from "naive-ui";
import { ref } from "vue";
import { useUserStore } from "../stores/user";
import { createNotification, formatAxiosError } from "../utils";

const emit = defineEmits<{ login: []; signUp: [] }>();

const show = ref(false);
const username = ref("");
const password = ref("");
const remember = ref(false);
const loading = ref(false);
const user = useUserStore();

defineExpose({
  showDialog: () => {
    show.value = true;
  },
  hideDialog: () => {
    show.value = false;
  },
});

function startOAuth(provider: "google" | "github") {
  window.location.assign(`/tenants/api/oauth/${provider}/login`);
}

async function submitLogIn() {
  loading.value = true;
  try {
    const { data } = await axios.post<{ email: string; username: string }>(
      "/tenants/api/login",
      {
        email: username.value,
        username: username.value,
        password: password.value,
      },
      { headers: { "Content-Type": "application/json" } },
    );
    emit("login");
    user.setUser({ username: data.username, email: data.email });
    await user.getToken();
    show.value = false;
  } catch (error: unknown) {
    createNotification(null, "Bad login", formatAxiosError(error), "danger");
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped lang="scss">
.dc-muted {
  color: var(--comment);
  font-size: 0.85rem;
}
</style>
