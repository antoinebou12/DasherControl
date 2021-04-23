
<template>
  <div class="main">
    <Navbar ref="navbar" @changeActive="manageNav" />
    <GridLayoutMain ref="gridLayout"/>
    <Login ref="Login"/>
    <SignUp ref="SignUp"/>
    <Settings ref="Settings"/>
  </div>
</template>

<script>

import GridLayoutMain from '../components/GridLayoutMain.vue'
import Navbar from '../components/Navbar.vue'
import Login from './Login.vue'
import SignUp from './SignUp.vue'
import Settings from './Settings.vue'

export default {
  components: {
    GridLayoutMain,
    Navbar,
    Login,
    SignUp,
    Settings
  },
  props: {
      msg: String,
  },
  methods: {
    manageNav(e) {
      switch (e) {
        case "Login":
          this.$refs.Login.showDialog();
          break;
        case "SignUp":
          this.$refs.SignUp.showDialog();
          break;
        case "Settings":
          this.$refs.Settings.showDialog();
          break;
      }
    }
  },
};
</script>

<style>
</style>