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
      <vs-navbar-item class="vs-navbar-item" :active="active=='home'" id="home" index="0">
        <i class="bx bx-home"></i>
      </vs-navbar-item>
      <vs-navbar-item  class="vs-navbar-item" :active="active=='settings'" id="settings" index="1">
        <i class="bx bx-cog"></i>
      </vs-navbar-item>
        <vs-button flat>Login</vs-button>
        <vs-button>Sign Up</vs-button>
      </template>
    </vs-navbar>
  </div>
</template>

<script>

export default {
  name: "Navbar",
  data: () => ({
    active: "home"
  })
};
</script>

<style lang="scss">
.vs-navbar-item{
  color: rgba(255,255,255,.6) !important;
}
</style>
