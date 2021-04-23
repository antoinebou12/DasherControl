<template>
  <div class="settings">
    <div class="center">
      <vs-dialog ref="settings_dialog" v-model="show" class="dialog-login dark">
        <div v-if="loading" class="vs-dialog__loading">
          <div class="vs-dialog__loading__load"></div>
        </div>
        <template #header>
          <h4 class="not-margin">
            <b>Settings</b>
          </h4>
        </template>
      </vs-dialog>
    </div>
  </div>
</template>

<script>
import axios from "axios";

export default {
  name: "Settings",
  data: () => ({
    show: false,
    loading: false,
  }),
  methods: {
    showDialog() {
      this.show = true;
    },
    hideDialog() {
      this.show = false;
    },
    loadingDialog(state) {
      if (state === true) {
        this.$refs.settings_dialog.$el.classList.add("vs-dialog--loading");
        this.loading = true;
      } else {
        this.$refs.settings_dialog.$el.classList.remove("vs-dialog--loading");
        this.loading = false;
      }
    },
    get_tenant_configuration() {
      axios({
        method: 'get',
        url: '/tenants/api/config',
      })
    }
  },
}
</script>

<style lang="scss" scoped>
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