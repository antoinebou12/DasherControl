<template>
  <v-dialog v-model="show" max-width="460" transition="dialog-bottom-transition">
    <v-card rounded="lg">
      <v-card-title class="text-h6">Login</v-card-title>
      <v-card-text>
        <v-form @submit.prevent="submitLogIn">
          <div class="d-flex flex-column ga-3">
            <v-text-field
              v-model="username"
              label="Email / Username"
              variant="outlined"
              density="comfortable"
              autocomplete="username"
              hide-details="auto"
            >
              <template #prepend-inner>
                <i class="bx bxs-user" aria-hidden="true" />
              </template>
            </v-text-field>
            <v-text-field
              v-model="password"
              label="Password"
              type="password"
              variant="outlined"
              density="comfortable"
              autocomplete="current-password"
              hide-details="auto"
            >
              <template #prepend-inner>
                <i class="bx bxs-lock" aria-hidden="true" />
              </template>
            </v-text-field>
            <div class="d-flex align-center justify-space-between flex-wrap ga-2">
              <v-checkbox v-model="remember" label="Remember me" density="compact" hide-details />
              <a class="dc-muted" href="#">Forgot password?</a>
            </div>
            <v-btn type="submit" color="primary" size="large" block :loading="loading">Sign in</v-btn>
          </div>
        </v-form>
        <v-divider class="my-4">or continue with</v-divider>
        <div class="d-flex flex-column ga-2">
          <v-btn block size="large" variant="tonal" @click="startOAuth('google')">
            <template #prepend>
              <i class="bx bxl-google" />
            </template>
            Google
          </v-btn>
          <v-btn block size="large" variant="tonal" @click="startOAuth('github')">
            <template #prepend>
              <i class="bx bxl-github" />
            </template>
            GitHub
          </v-btn>
        </div>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import axios from "axios";
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
