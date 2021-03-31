<template>
  <component :is="applet" v-bind="$attrs"/>
</template>

<script>
import CreateNew from "./CreateNew/CreateNew.vue";
import Editor from "./Editor/Editor.vue";
import BorderlessIFrame from "./IFrame/BorderlessIFrame.vue";

export default {
  inheritAttrs: false,
  name: "Applet",
  components: {
    CreateNew,
    Editor,
    BorderlessIFrame,
  },
  props: {
    appletName: String,
  },
  data() {
    return {
      applet: CreateNew,
    }
  },
  created() {
    this.changeApplet(this.appletName)
  },
  watch: {
    appletName: function (newVal, oldVal){
      this.changeApplet(newVal)
      this.$forceUpdate()
    },
  },
  methods: {
    changeApplet(appletName) {
      console.log(appletName)
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
      }
    }


  },
}
</script>

<style scoped>

</style>