<template>
  <component ref="applet" :is="applet" v-bind="$attrs"/>
</template>

<script>
import CreateNew from "./CreateNew/CreateNew.vue";
import Editor from "./Editor/Editor.vue";
import BorderlessIFrame from "./IFrame/BorderlessIFrame.vue";
import Shortcut from "./Shortcut/Shortcut.vue";
import DockerManager from "./DockerManager/DockerManager.vue";
import Terminal from "./Terminal/Terminal.vue";

export default {
  inheritAttrs: false,
  name: "Applet",
  components: {
    CreateNew,
    Editor,
    BorderlessIFrame,
    Terminal
  },
  props: {
    appletName: String,
  },
  data() {
    return {
      currentAppletName: this.appletName,
      applet: CreateNew,
    }
  },
  created() {
    this.changeApplet(this.appletName)
  },
  watch: {
    appleName: function(newVal){
      this.currentAppletName = newVal
      this.changeApplet(newVal)
      this.$forceUpdate()
    },
    currentAppletName: function (newVal){
      this.appleName = newVal
      this.changeApplet(newVal)
      this.$emit("changeApplet")
      this.$forceUpdate()
    },
  },
  methods: {
    changeApplet(appletName) {
      switch (appletName) {
        case "Choose Applets":
          this.applet = CreateNew
          break;
        case "CreateNew":
          this.applet = CreateNew
          break;
        case "Editor":
          this.applet = Editor
          break;
        case "IFrame":
          this.applet = BorderlessIFrame
          break;
        case "Shortcut":
          this.applet = Shortcut
          break;
        case "Shortcut":
          this.applet = Shortcut
          break;
        case "DockerManager":
          this.applet = DockerManager
          break;
        case "Terminal":
          this.applet = Terminal
          break;
      }
    }


  },
}
</script>

<style scoped>

</style>