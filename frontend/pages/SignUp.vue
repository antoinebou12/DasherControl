<template>
  <n-modal v-model:show="show" preset="card" title="Sign up" :style="{ width: '440px' }">
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
        @keyup.enter="submitSignUp"
      />
      <n-button type="primary" block size="large" @click="submitSignUp">Create account</n-button>
    </n-space>
  </n-modal>
</template>

<script setup lang="ts">
import axios from "axios";
import { NButton, NInput, NModal, NSpace } from "naive-ui";
import { ref } from "vue";

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

async function submitSignUp() {
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
}
</script>
