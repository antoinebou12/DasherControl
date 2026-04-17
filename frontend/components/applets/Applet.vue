<template>
  <component ref="applet" :is="applet" v-bind="innerBindings"/>
</template>

<script>
import {
  getAppletComponent,
  getAppletDescriptor,
  normalizeAppletId,
} from "./appletRegistryCore.js";

export default {
  inheritAttrs: false,
  name: "Applet",
  props: {
    appletName: String,
  },
  data() {
    return {
      currentAppletName: this.appletName,
      applet: getAppletComponent("CreateNew"),
    };
  },
  computed: {
    resolvedAppletId() {
      return normalizeAppletId(this.currentAppletName || this.appletName);
    },
    innerBindings() {
      const base = { ...this.$attrs };
      if (!getAppletDescriptor(this.resolvedAppletId)) {
        base.unknownAppletId =
          this.currentAppletName || this.appletName || "(missing)";
      }
      return base;
    },
  },
  created() {
    this.changeApplet(this.appletName);
  },
  watch: {
    appletName: function (newVal) {
      this.currentAppletName = newVal;
      this.changeApplet(newVal);
      this.$forceUpdate();
    },
    currentAppletName: function (newVal) {
      this.changeApplet(newVal);
      this.$emit("changeApplet");
      this.$forceUpdate();
    },
  },
  methods: {
    changeApplet(appletName) {
      this.applet = getAppletComponent(appletName);
    },
  },
};
</script>

<style scoped>

</style>
