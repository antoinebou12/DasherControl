<template>
  <n-modal v-model:show="show" preset="card" title="Sign up" :style="{ width: '440px' }">
    <form @submit.prevent="submitSignUp">
      <n-space vertical size="large">
        <n-input v-model:value="name" placeholder="Name" size="large" />
        <n-input v-model:value="email" placeholder="Email" size="large" />
        <n-input v-model:value="username" placeholder="Username" size="large" />
        <n-input v-model:value="password" type="password" show-password-on="click" placeholder="Password" size="large" />
        <n-input
          v-model:value="password_confirmation"
          type="password"
          show-password-on="click"
          placeholder="Confirm password"
          size="large"
        />
        <n-button type="primary" attr-type="submit" block size="large">Create account</n-button>
      </n-space>
    </form>
  </n-modal>
</template>

<script setup lang="ts">
import axios, { isAxiosError } from "axios";
import { NButton, NInput, NModal, NSpace } from "naive-ui";
import { ref } from "vue";
import { createNotification } from "../utils";

const show = ref(false);
const name = ref("");
const username = ref("");
const email = ref("");
const password = ref("");
const password_confirmation = ref("");

defineExpose({
  showDialog: () => {
    show.value = true;
  },
});

function signupErrorMessage(error: unknown): string {
  if (isAxiosError(error)) {
    const data = error.response?.data;
    if (data && typeof data === "object" && "error" in data) {
      return String((data as { error: string }).error);
    }
    if (error.response?.status === 409) {
      return "This email or username may already be in use, or the passwords do not match.";
    }
  }
  return "Could not create account.";
}

async function submitSignUp() {
  try {
    await axios.post(
      "/tenants/api/create",
      {
        name: name.value,
        email: email.value,
        username: username.value,
        role: "tenant",
        password: password.value,
        password_confirmation: password_confirmation.value,
      },
      { headers: { "Content-Type": "application/json" } },
    );
    show.value = false;
  } catch (error: unknown) {
    createNotification(null, "Sign up failed", signupErrorMessage(error), "danger");
  }
}
</script>
