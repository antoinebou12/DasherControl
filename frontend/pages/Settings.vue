<template>
  <div class="settings-container">
    <div class="center">
      <vs-dialog ref="settings_dialog" v-model="show" class="dialog-settings dark">
        <div v-if="loading" class="vs-dialog__loading">
          <div class="vs-dialog__loading__load"></div>
        </div>
        <template #header>
          <h4 class="not-margin">
            <b>Settings</b>
          </h4>
        </template>
        <div class="settings">
          <div class="flex">
            <b>Connected User</b>: {{ this.$store.state.user.username || this.$store.state.user.email || "Not Connected" }}
          </div>
        </div>
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

<style lang="scss">
.settings-container {
  width: 100%;

  .flex {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
}

</style>