
<template>
  <div class="main">
   {{ msg }}
   <GridLayout />
  </div>
</template>

<script>
import GridLayout from '../components/GridLayout.vue'
export default {
  components: {
    GridLayout
  },
  props: {
      msg: String,
  },
};
</script>

<style>
#app {
  font-size: 18px;
  font-family: 'Roboto', sans-serif;
  color: blue;
}
</style>