
<template>
  <div class="main">
   <Navbar />
   <GridLayout />

  </div>
</template>

<script>

import GridLayout from '../components/GridLayout.vue'
import Navbar from '../components/Navbar.vue'
import Editor from '../components/applets/Editor.vue'
export default {
  components: {
    GridLayout,
    Navbar,
    Editor
  },
  props: {
      msg: String,
  },
};
</script>

<style>
</style>