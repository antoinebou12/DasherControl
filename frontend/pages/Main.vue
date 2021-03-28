
<template>
  <div class="main">
<!--   <Navbar />-->
<!--   <GridLayout id="GridLayout"/>-->
    <Login></Login>
  </div>
</template>

<script>

import GridLayout from '../components/GridLayout.vue'
import Navbar from '../components/Navbar.vue'
import Login from './Login.vue'
export default {
  components: {
    GridLayout,
    Navbar,
    Login
  },
  props: {
      msg: String,
  },
};
</script>

<style>
#GridLayout {
  margin-top: 64px;
}
</style>