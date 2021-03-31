<template>
  <div class="create-applet-container">
    <vs-select ref="selectApplet" placeholder="Choose Applets" v-model="appletName">
      <vs-option label="IFrame" value="IFrame">IFrame</vs-option>
      <vs-option label="Editor" value="Editor">Editor</vs-option>
    </vs-select>
    <vs-button @click="newApplet()"> +</vs-button>
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
      this.$parent.appletName = this.$refs.selectApplet.value;
      this.$parent.$forceUpdate()
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
}

</style>