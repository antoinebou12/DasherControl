<template>
  <div class="create-applet-container grid">
    <div class="select-applet-form">
      <vs-row align="center" justify="center">
        <vs-select placeholder="Choose Applets" v-model="appletName">
          <vs-option label="IFrame" value="IFrame">IFrame</vs-option>
          <vs-option label="Editor" value="Editor">Editor</vs-option>
          <vs-option label="Shortcut" value="Shortcut">Shortcut</vs-option>
        </vs-select>
        <vs-button @click="newApplet()">+</vs-button>
      </vs-row>
      <vs-row align="center" justify="center">
        <component ref="settingForm" :is="settingForm"></component>
      </vs-row>
    </div>
  </div>
</template>

<script>
import IFrameSetting from '../IFrame/IFrameSetting.vue'

export default {
  name: "CreateNew",
  data () {
    return {
      appletName: '',
      settingForm: null
    }
  },
  watch: {
    appletName : function(newVal) {
      switch (newVal) {
        case "IFrame":
          this.settingForm = IFrameSetting
          break;
  }
},
  },
  methods: {
    newApplet() {
      this.$parent.currentAppletName = this.appletName;
      switch (this.appletName) {
        case "IFrame":
          this.$parent.$attrs.src = this.$refs.settingForm.$refs.src.value
          break;
      }
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