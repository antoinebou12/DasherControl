<template>
  <div class="center navbar">
    <vs-navbar
        center
        v-model="active"
        color="dark">
      <template #left>
        <img src="/public/imgs/favicon/favicon-32x32.png" alt=""/>
      </template>
      <template #right>
        <vs-navbar-item class="vs-navbar-item" @click="set_active('Settings')" id="settings" index="1">
            <i class="bx bx-cog"></i>
        </vs-navbar-item>
        <vs-button v-if="!is_login" flat @click="set_active('Login')">Login</vs-button>
<!--        <vs-button v-if="!is_login" @click="set_active('SignUp')">Sign Up</vs-button>-->
        <vs-button v-if="is_login"  flat @click="logout()">Logout</vs-button>
      </template>
    </vs-navbar>
  </div>
</template>

<script>
import Settings from '../pages/Settings.vue'
import axios from "axios";

export default {
  name: "Navbar",
  components: {
    Settings
  },
  data() {
    return {
      active: "Home",
      activeSettings: false,
    }
  },
  computed: {
    is_login: {
      get: function() {
        return this.$store.state.user.token !== ""
      },
      set : function(newVal) {
        this.is_login = newVal;
      }
    }
  },
  methods: {
    set_active(active) {
      this.active = active
      this.$emit("changeActive", active)
    },
    logout() {
      axios({
        method: 'post',
        url: '/tenants/api/logout',
      }).then((response) => {
        this.$store.commit('setUser', { 'email': '', 'username': ''})
        this.$store.commit('setToken', '')
      })
    }
  }
};
</script>

<style lang="scss">
.navbar {
  margin-bottom: 48px;

  button {
    border-radius: 0px;
  }
}
.content-tooltip {
  .body {
    display: flex;
    align-items: flex-start;
    justify-content: center;

    .vs-avatar-content {
      margin-top: -30px;
      border: 3px solid var(--vs-theme-layout);
      box-shadow: 0px 4px 15px 0px rgba(0, 0, 0, 0.1);
    }

    .text {
      display: flex;
      align-items: center;
      justify-content: center;
      flex-direction: column;
      font-size: 0.55rem;
      padding: 10px;
      font-weight: normal;

      span {
        font-weight: bold;
        font-size: 0.7rem;
      }
    }
  }

  footer {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  h4 {
    padding: 8px;
    margin: 0px;
    text-align: left;
  }

  p {
    text-align: left;
    margin: 0px;
    line-height: 1rem;
    padding: 0px 0px 5px 8px;
  }
}
</style>
