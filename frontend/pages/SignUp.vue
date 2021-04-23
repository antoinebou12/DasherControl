<template>
  <div class="signup">
    <div class="center">
      <vs-dialog v-model="show" class="dialog-login">
        <template #header>
          <h4 class="not-margin">
            <b>SignUp</b>
          </h4>
        </template>
        <div class="signup-form">
          <vs-input type="name" v-model="name" placeholder="Name">
          </vs-input>
          <vs-input v-model="email" placeholder="Email">
            <template #icon>
              @
            </template>
          </vs-input>
          <vs-input type="username" v-model="username" placeholder="Username">
            <template #icon>
              <i class='bx bxs-user'></i>
            </template>
          </vs-input>
          <vs-input type="password" v-model="password" placeholder="Password">
            <template #icon>
              <i class='bx bxs-lock'></i>
            </template>
          </vs-input>
          <vs-input type="password" v-model="password_confirmation" placeholder="Confirm Password">
            <template #icon>
              <i class='bx bxs-lock'></i>
            </template>
          </vs-input>
        </div>
        <template #footer>
          <div class="footer-dialog">
            <vs-button block @click="submitSignUp()">
              Sign Up
            </vs-button>
          </div>
        </template>
      </vs-dialog>
    </div>
  </div>
</template>


<script>
import axios from "axios";

export default {
  name: "SignUp",
  data: () => ({
    show: false,
    name: '',
    username: '',
    email: '',
    password: '',
    password_confirmation: '',
  }),
  methods: {
    showDialog() {
      this.show = true;
    },
    submitSignUp() {
      axios({
        method: 'post',
        url: '/tenants/api/create',
        headers: {
          'Content-Type': 'application/json'
        },
        data: {
          name: this.name,
          email: this.email,
          username: this.username,
          password: this.password,
          password_confirmation: this.password_confirmation,
          role: "tenant",
        }
      });
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

.signup-form {
  width: 100%;

  .flex {
    display: flex;
    align-items: center;
    justify-content: space-between;

    a {
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