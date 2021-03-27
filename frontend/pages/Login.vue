
<template>
  <div class="main">
   <Navbar />
  </div>
</template>

<script>

import Navbar from '../components/Navbar.vue'
export default {
  components: {
    GridLayout,
    Navbar,
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