
<template>
  <div class="main">
    <Navbar ref="navbar" @changeActive="manageNav" />
    <GridLayoutMain ref="gridLayout"/>
    <Login ref="Login"/>
    <SignUp ref="SignUp"/>
  </div>
</template>

<script>

import GridLayoutMain from '../components/GridLayoutMain.vue'
import Navbar from '../components/Navbar.vue'
import Login from './Login.vue'
import SignUp from './SignUp.vue'
export default {
  components: {
    GridLayoutMain,
    Navbar,
    Login,
    SignUp
  },
  props: {
      msg: String,
  },
  methods: {
    manageNav(e){
      if(e === "Login") {
        this.$refs.Login.showDialog();
      } else if(e === "SignUp") {
        this.$refs.SignUp.showDialog();
      }
    }
  }
};
</script>

<style>
</style>