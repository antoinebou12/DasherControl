<template>
  <v-snackbar
    v-model="snackbarState.open"
    :color="snackbarState.color"
    :timeout="snackbarState.timeout"
    location="top"
    multi-line
  >
    <div class="dc-snack__title">{{ snackbarState.title }}</div>
    <div class="dc-snack__msg">{{ snackbarState.message }}</div>
    <template #actions>
      <v-btn variant="text" @click="snackbarState.open = false">Close</v-btn>
    </template>
  </v-snackbar>
</template>

<script setup lang="ts">
import { snackbarState } from "../utils/snackbarState";
</script>

<style scoped>
.dc-snack__title {
  font-weight: 600;
  margin-bottom: 4px;
}
.dc-snack__msg {
  font-size: 0.9rem;
  opacity: 0.95;
}
</style>
