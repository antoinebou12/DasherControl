<template>
  <div class="login">
    <div class="center">
      <vs-dialog v-model="show" class="dialog-login dark">
        <template #header>
          <h4 class="not-margin">
            <b>Login</b>
          </h4>
        </template>
        <div class="login-form">
          <vs-input v-model="email" placeholder="Email">
            <template #icon>
              @
            </template>
          </vs-input>
          <vs-input type="password" v-model="password" placeholder="Password">
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
            <vs-button block>
              Sign In
            </vs-button>

            <div class="new">
              New Here? <a href="#">Create New Account</a>
            </div>
          </div>
        </template>
      </vs-dialog>
    </div>
  </div>
</template>


<script>


export default {
  name: "Login",
  data: () => ({
    show: false,
    email: '',
    password: '',
    remember: false
  }),
  methods: {
    showDialog(){
      this.show = true;
    },
    hideDialog(){
      this.show = false;
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
      color: var(--darcula-cl);
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
      color: var(--darcula-cl);
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
      color: var(--darcula-cl);
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