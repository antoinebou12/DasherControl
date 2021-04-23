<template>
  <div class="create-applet-container grid">
    <div class="select-applet-form">
      <vs-row align="center" justify="center">
        <vs-select placeholder="Choose Applets" v-model="appletName">
          <vs-option label="IFrame" value="IFrame">IFrame</vs-option>
          <vs-option label="Editor" value="Editor">Editor</vs-option>
          <vs-option label="Shortcut" value="Shortcut">Shortcut</vs-option>
          <vs-option label="DockerManager" value="DockerManager">DockerManager</vs-option>
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
import CreateNewSetting from './CreateNewSetting.vue'
import IFrameSetting from '../IFrame/IFrameSetting.vue'
import ShortcutSetting from '../Shortcut/ShortcutSetting.vue'
import DockerManagerSetting from '../DockerManager/DockerManagerSetting.vue'

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
        case "Shortcut":
          this.settingForm = ShortcutSetting
          break;
        case "DockerManager":
          this.settingForm = DockerManagerSetting
          break;
        default:
          this.settingForm = CreateNewSetting
  }
},
  },
  methods: {
    newApplet() {
      this.$parent.currentAppletName = this.appletName;
      switch (this.appletName) {
        case "IFrame":
          this.$parent.$attrs.src = this.$refs.settingForm.src
          break;
        case "Shortcut":
          this.$parent.$attrs.src = this.$refs.settingForm.src
          this.$parent.$attrs.img_link = this.$refs.settingForm.img_link
          this.$parent.$attrs.title = this.$refs.settingForm.title
          this.$parent.$attrs.description = this.$refs.settingForm.description
          break;
        case "DockerManager":
          break;
      }
    }
  }
}
</script>

<style lang="scss" scoped>
.create-applet-container {
  background: var(--darcule-bg);
  color: var(--fg);

  vs-select {
    color: var(--fg)
  }

  .select-applet-form {
    width: 100%;
    align-items: center;
  }
}

</style>