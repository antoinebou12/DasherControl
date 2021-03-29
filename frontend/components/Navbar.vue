<template>
  <div class="center navbar">
    <vs-navbar
      center
      v-model="active"
      color="dark">
      <template #left>
        <img src="/public/imgs/favicon/favicon-32x32.png" alt="" />
      </template>
      <template #right>
      <vs-navbar-item class="vs-navbar-item" @click="set_active('Settings')" id="settings" index="0">
        <i class="bx bx-cog"></i>
      </vs-navbar-item>
        <vs-button flat @click="set_active('Login')">Login</vs-button>
        <vs-button @click="set_active('SignUp')">Sign Up</vs-button>
      </template>
    </vs-navbar>
  </div>
</template>

<script>

export default {
  name: "Navbar",
  data: () => ({
    active: "Home",
  }),
  methods: {
      set_active(active){
        this.active = active
        this.$emit("changeActive", active)
      },
    }
};
</script>

<style lang="scss">
</style>
