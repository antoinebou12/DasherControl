
<template>
  <div class="main">
   {{ msg }}
  </div>
</template>

<script>
export default {
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