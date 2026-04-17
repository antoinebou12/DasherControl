<template>
  <div class="main">
    <Navbar ref="navRef" @change-active="manageNav" />
    <GridLayoutMain ref="gridRef" />
    <Login ref="loginRef" @login="onLogin" @sign-up="onSignUp" />
    <SignUp ref="signUpRef" />
    <Settings ref="settingsRef" />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import GridLayoutMain from "../components/GridLayoutMain.vue";
import Navbar from "../components/Navbar.vue";
import Login from "./Login.vue";
import SignUp from "./SignUp.vue";
import Settings from "./Settings.vue";

defineProps<{ msg?: string }>();

const navRef = ref<InstanceType<typeof Navbar> | null>(null);
const gridRef = ref<InstanceType<typeof GridLayoutMain> | null>(null);
const loginRef = ref<InstanceType<typeof Login> | null>(null);
const signUpRef = ref<InstanceType<typeof SignUp> | null>(null);
const settingsRef = ref<InstanceType<typeof Settings> | null>(null);

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

<style scoped></style>
