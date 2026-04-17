<template>
  <v-layout class="dc-main-layout" full-height>
    <v-navigation-drawer
      v-model="shell.rightPanelOpen"
      location="right"
      width="320"
      temporary
      scrim
    >
      <v-toolbar density="compact" color="surface">
        <v-toolbar-title class="text-subtitle-1">Tools</v-toolbar-title>
        <v-spacer />
        <v-btn icon="mdi-close" variant="text" aria-label="Close panel" @click="shell.setRightPanel(false)" />
      </v-toolbar>
      <div class="pa-3 text-body-2 text-medium-emphasis">
        Optional side rail for inspector or shortcuts (extensible).
      </div>
    </v-navigation-drawer>

    <v-main class="dc-main-surface pa-0 d-flex flex-column flex-grow-1 min-height-0">
      <Navbar ref="navRef" @change-active="manageNav" />

      <v-expand-transition>
        <div v-if="!user.token" class="px-4 pb-3">
          <v-alert
            type="info"
            variant="tonal"
            density="comfortable"
            rounded="lg"
            class="dc-guest-alert"
            role="region"
            aria-label="Account"
          >
            <div class="d-flex flex-column flex-md-row align-md-center justify-space-between ga-3">
              <span class="dc-guest-bar__text" aria-live="polite">
                You are not signed in. Log in to sync workspaces and use account features.
              </span>
              <div class="d-flex ga-2 flex-wrap">
                <v-btn color="primary" aria-label="Log in" @click="loginRef?.showDialog()">Log in</v-btn>
                <v-btn variant="outlined" aria-label="Sign up for an account" @click="signUpRef?.showDialog()">
                  Sign up
                </v-btn>
              </div>
            </div>
          </v-alert>
        </div>
      </v-expand-transition>

      <div class="flex-grow-1 min-height-0">
        <GridLayoutMain ref="gridRef" />
      </div>

      <Login ref="loginRef" @login="onLogin" @sign-up="onSignUp" />
      <SignUp ref="signUpRef" />
      <Settings ref="settingsRef" @request-login="loginRef?.showDialog()" @request-sign-up="signUpRef?.showDialog()" />
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
import { provide, ref } from "vue";
import GridLayoutMain from "../components/GridLayoutMain.vue";
import Navbar from "../components/Navbar.vue";
import { useUiShellStore } from "../stores/uiShell";
import { useUserStore } from "../stores/user";
import Login from "./Login.vue";
import SignUp from "./SignUp.vue";
import Settings from "./Settings.vue";

const user = useUserStore();
const shell = useUiShellStore();

defineProps<{ msg?: string }>();

const navRef = ref<InstanceType<typeof Navbar> | null>(null);
const gridRef = ref<InstanceType<typeof GridLayoutMain> | null>(null);
const loginRef = ref<InstanceType<typeof Login> | null>(null);
const signUpRef = ref<InstanceType<typeof SignUp> | null>(null);
const settingsRef = ref<InstanceType<typeof Settings> | null>(null);

provide("dcOpenLogin", () => loginRef.value?.showDialog());
provide("dcOpenSignUp", () => signUpRef.value?.showDialog());

function manageNav(e: string) {
  if (e === "Login") loginRef.value?.showDialog();
  if (e === "SignUp") signUpRef.value?.showDialog();
  if (e === "Settings") settingsRef.value?.showDialog();
}

function onLogin() {
  /* reserved */
}
function onSignUp() {
  signUpRef.value?.showDialog();
}
</script>

<style scoped lang="scss">
.dc-main-layout {
  background: var(--bg);
  color: var(--fg);
}
.dc-main-surface {
  background: transparent;
}
.dc-guest-bar__text {
  font-size: 0.95rem;
  line-height: 1.45;
  max-width: 52ch;
  font-weight: 500;
}
.dc-guest-alert {
  border: 1px solid var(--ui-chrome-border);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.28);
}
</style>
