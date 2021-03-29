
<template>
  <div id="app" vs-theme="dark">
    <Main msg=""/>
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
@font-face {
  font-family: 'Poppins';
  src: url('public/fonts/Poppins-Regular.ttf');
  font-weight: normal;
  font-style: normal;
}
:root {
  --darcula-bg: #282a36;
  --darcula-bg-rgb: 40, 42, 54;
  --darcula-fg: #f8f8f2;
  --darcula-fg-rgb: 248, 248, 242;
  --darcula-comment: #6272a4;
  --darcula-cl: #44475a;
}
html {
  background: var(--darcula-bg);
  font-family: 'Poppins';
}
#app {
  width: 100%;
  height: 100%;
}
</style>