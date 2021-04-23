
<template>
  <div id="app" vs-theme="dark">
    <Main msg=""/>
  </div>
</template>

<script>
import Main from './pages/Main.vue'
import axios from "axios";

export default {
  name: 'app',
  components: {
    Main
  },
  created() {
    axios({
      method: 'get',
      url: '/tenants/api/token',
    }).then((response) => {
      this.$store.commit("setToken", response.data)
    })
  },
  methods: {
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
  --bg: #282a36;
  --bg-rgb: 40, 42, 54;
  --fg: #f8f8f2;
  --fg-rgb: 248, 248, 242;
  --comment: #6272a4;
  --cl: #44475a;
}
html {
  background: var(--bg);
  font-family: 'Poppins';
}
#app {
  width: 100%;
  height: 100%;
}
</style>