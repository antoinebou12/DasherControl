<template>
  <component
    ref="applet"
    :is="applet"
    v-bind="innerBindings"
    @confirm-applet="onConfirmApplet"
  />
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
      /** Merged into child props; Vue 3 $attrs is read-only so applyCreate cannot mutate attrs. */
      createOverrides: {},
    };
  },
  computed: {
    resolvedAppletId() {
      return normalizeAppletId(this.currentAppletName || this.appletName);
    },
    innerBindings() {
      const base = { ...this.$attrs, ...this.createOverrides };
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
    applyCreateAttrs(attrs) {
      if (!attrs) return;
      this.createOverrides = { ...this.createOverrides, ...attrs };
    },
    getBindingsForPersist() {
      return { ...this.$attrs, ...this.createOverrides };
    },
    clearCreateOverrides() {
      this.createOverrides = {};
    },
    changeApplet(appletName) {
      this.applet = getAppletComponent(appletName);
    },
    onConfirmApplet(payload) {
      const appletName = payload?.appletName;
      const settingFormRef = payload?.settingFormRef;
      if (!appletName || !String(appletName).trim()) return;
      const descriptor = getAppletDescriptor(appletName);
      if (descriptor && typeof descriptor.applyCreate === "function") {
        descriptor.applyCreate(this, settingFormRef ?? null);
      }
      this.currentAppletName = appletName;
    },
  },
};
</script>

<style scoped>

</style>
