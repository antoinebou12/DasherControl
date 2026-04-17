<template>
  <v-dialog v-model="show" max-width="480" transition="dialog-bottom-transition">
    <v-card rounded="lg">
      <v-card-title class="text-h6">Sign up</v-card-title>
      <v-card-text>
        <v-form @submit.prevent="submitSignUp">
          <div class="d-flex flex-column ga-3">
            <v-text-field v-model="name" label="Name" variant="outlined" density="comfortable" hide-details="auto" />
            <v-text-field v-model="email" label="Email" variant="outlined" density="comfortable" hide-details="auto" />
            <v-text-field
              v-model="username"
              label="Username"
              variant="outlined"
              density="comfortable"
              hide-details="auto"
            />
            <v-text-field
              v-model="password"
              label="Password"
              type="password"
              variant="outlined"
              density="comfortable"
              hide-details="auto"
            />
            <v-text-field
              v-model="password_confirmation"
              label="Confirm password"
              type="password"
              variant="outlined"
              density="comfortable"
              hide-details="auto"
            />
            <v-btn type="submit" color="primary" size="large" block>Create account</v-btn>
          </div>
        </v-form>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import axios, { isAxiosError } from "axios";
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
