
<template>
  <div id="app">
    <Main msg="" />
  </div>
</template>

<script>
import Main from './pages/Main.vue'

export default {
  name: 'app',
  components: {
    Main
  }
}
</script>

<style lang="scss">
:root {
  --darcula-bg: #282a36;
  --darcula-fg: #f8f8f2;
}
html {
  background: var(--darcula-bg);
}
#app {
  width: 100%;
  height: 100%;
}
</style>