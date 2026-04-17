<template>
  <div class="main">
    <Navbar ref="navRef" @change-active="manageNav" />
    <div v-if="!user.token" class="dc-guest-bar" role="region" aria-label="Account">
      <span class="dc-guest-bar__text" aria-live="polite">
        You are not signed in. Log in to sync workspaces and use account features.
      </span>
      <n-space :size="10">
        <n-button type="primary" aria-label="Log in" @click="loginRef?.showDialog()">Login</n-button>
        <n-button aria-label="Sign up for an account" @click="signUpRef?.showDialog()">Sign up</n-button>
      </n-space>
    </div>
    <GridLayoutMain ref="gridRef" />
    <Login ref="loginRef" @login="onLogin" @sign-up="onSignUp" />
    <SignUp ref="signUpRef" />
    <Settings ref="settingsRef" @request-login="loginRef?.showDialog()" @request-signup="signUpRef?.showDialog()" />
  </div>
</template>

<script setup lang="ts">
import { NButton, NSpace } from "naive-ui";
import { provide, ref } from "vue";
import GridLayoutMain from "../components/GridLayoutMain.vue";
import Navbar from "../components/Navbar.vue";
import { useUserStore } from "../stores/user";
import Login from "./Login.vue";
import SignUp from "./SignUp.vue";
import Settings from "./Settings.vue";

const user = useUserStore();

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
.dc-guest-bar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin: 0 16px 16px;
  padding: 14px 18px;
  background: var(--ui-chrome-bg);
  color: var(--ui-chrome-fg);
  border: 1px solid var(--ui-chrome-border);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.28);
}
.dc-guest-bar__text {
  font-size: 0.95rem;
  line-height: 1.45;
  max-width: 52ch;
  font-weight: 500;
}

.dc-guest-bar :deep(.n-button.n-button--default-type) {
  --n-text-color: rgba(244, 244, 245, 0.95) !important;
  --n-border: 1px solid var(--ui-chrome-border) !important;
  --n-color: transparent !important;
  --n-color-hover: rgba(255, 255, 255, 0.06) !important;
}

.dc-guest-bar :deep(.n-button.n-button--primary-type) {
  --n-color: var(--ui-accent) !important;
  --n-color-hover: var(--ui-accent-hover) !important;
  --n-color-pressed: var(--ui-accent-pressed) !important;
  --n-text-color: #ffffff !important;
  --n-border: none !important;
}
</style>
