<template>
  <div class="create-applet-container grid">
    <div class="select-applet-form">
      <vs-row align="center" justify="center">
        <vs-select ref="selectApplet" placeholder="Choose Applets" v-model="appletName">
          <vs-option label="IFrame" value="IFrame">IFrame</vs-option>
          <vs-option label="Editor" value="Editor">Editor</vs-option>
          <vs-option label="Shortcut" value="Shortcut">Shortcut</vs-option>
        </vs-select>
        <vs-button @click="newApplet()"> +</vs-button>
      </vs-row>
    </div>
  </div>
</template>

<script>
export default {
  name: "CreateNew",
  data: () => ({
    appletName: ''
  }),
  methods: {
    newApplet() {
      this.$parent.currentAppletName = this.$refs.selectApplet.value;
    }
  }
}
</script>

<style lang="scss" scoped>
.create-applet-container {
  background: var(--darcule-bg);
  color: var(--darcula-fg);

  vs-select {
    color: var(--darcula-fg)
  }

  .select-applet-form {
    width: 100%;
    align-items: center;
  }
}

</style>