<template>
  <Index msg="Welcome to Your Bruhzone" />
  <GridManager />
</template>

<script>
import Index from "./components/Index.vue";
import GridManager from "./components/GridManager.vue";

export default {
  name: "App",
  components: {
    Index,
    GridManager
  }
};
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
