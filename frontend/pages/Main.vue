
<template>
  <div class="main">
   <Navbar />
   <GridLayout />

  </div>
</template>

<script>

import GridLayout from '../components/GridLayout.vue'
import Navbar from '../components/Navbar.vue'
export default {
  components: {
    GridLayout,
    Navbar
  },
  props: {
      msg: String,
  },
};
</script>

<style>
</style>