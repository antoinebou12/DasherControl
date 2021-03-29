
<template>
  <div class="main">
    <Navbar ref="navbar" @changeActive="manageNav" />
    <GridLayout ref="gridLayout"/>
    <Login ref="Login"/>
    <SignUp ref="SignUp"/>
  </div>
</template>

<script>

import GridLayout from '../components/GridLayout.vue'
import Navbar from '../components/Navbar.vue'
import Login from './Login.vue'
import SignUp from './SignUp.vue'
export default {
  components: {
    GridLayout,
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
#GridLayout {
  margin-top: 64px;
}
</style>