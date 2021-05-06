<template>
  <div class="login">
    <div class="center">
      <vs-dialog ref="login_dialog" v-model="show" class="dialog-login dark">
        <div v-if="loading" class="vs-dialog__loading">
          <div class="vs-dialog__loading__load"></div>
        </div>
        <template #header>
          <h4 class="not-margin">
            <b>Login</b>
          </h4>
        </template>
        <div class="login-form">
          <vs-input v-model="username" placeholder="Email/Username">
            <template #icon>
              <i class='bx bxs-user'></i>
            </template>
          </vs-input>
          <vs-input type="password" v-model="password" placeholder="Password" v-on:keyup.enter="submitLogIn()">
            <template #icon>
              <i class='bx bxs-lock'></i>
            </template>
          </vs-input>
          <div class="flex">
            <vs-checkbox v-model="remember">Remember me</vs-checkbox>
            <a href="#">Forgot Password?</a>
          </div>
        </div>

        <template #footer>
          <div class="footer-dialog">
            <vs-button @click="submitLogIn()" block>
              Sign In
            </vs-button>
            <div class="new">
              New Here? <a @click="">Create New Account</a>
            </div>
          </div>
        </template>
      </vs-dialog>
    </div>
  </div>
</template>


<script>
import axios from "axios";
import {createNotification} from "../utils";

export default {
  name: "Login",
  data: () => ({
    show: false,
    username: '',
    password: '',
    remember: false,
    loading: false,
  }),
  methods: {
    showDialog(){
      this.show = true;
    },
    hideDialog(){
      this.show = false;
    },
    loadingDialog(state){
      if (state === true){
        this.$refs.login_dialog.$el.classList.add("vs-dialog--loading");
        this.loading = true;
      } else {
        this.$refs.login_dialog.$el.classList.remove("vs-dialog--loading");
        this.loading = false;
      }
    },
    submitLogIn(){
      let self = this;
      this.loadingDialog(true)
      axios({
          method: 'post',
          url: '/tenants/api/login',
          headers: {
            'Content-Type': 'application/json'
          },
          data: {
            email: this.username,
            username: this.username,
            password: this.password
          }
      }).then((response) => {
        this.$emit('login')
        this.$store.commit("setUser",
        {
          'username': this.username,
          'email': this.email,
        })
        this.$store.dispatch('getToken')
        this.loadingDialog(false)
        this.hideDialog();
      }).catch(function (error) {
        self.loadingDialog(false)
        createNotification(
            self,
            "Bad login",
            error.response.data,
            'danger')
      });
    },
    signUp() {
      this.$emit("signUp")
    }
  }

};
</script>

<style lang="scss">
.dialog-title {
  margin: 0px;
  font-weight: normal;
  padding: 10px;
}

.login-form {
  width: 100%;

  .flex {
    display: flex;
    align-items: center;
    justify-content: space-between;

    a {
      color: #b3b8d5;
      font-size: 0.8rem;
      opacity: 0.7;

      &:hover {
        opacity: 1;
      }
    }
  }

  .vs-checkbox-label {
    font-size: 0.8rem;
  }

  .vs-input-content {
    margin: 10px 0px;
    width: calc(100%);

    .vs-input {
      width: 100%;
      background: var(--bg);
      color: var(--fg);
    }
  }
}

.footer-dialog {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  width: calc(100%);

  .new {
    margin: 20px 0px 0px;
    padding: 0px;
    font-size: 0.7rem;

    a {
      color: var(--cl);
      margin-left: 6px;
      &:hover {
        text-decoration: underline;
      }
    }
  }

  .vs-button {
    margin: 0px;
  }
}
</style>