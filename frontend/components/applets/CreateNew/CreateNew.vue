<template>
  <div class="create-applet-container grid">
    <div class="select-applet-form">
      <n-space justify="center" align="center" wrap>
        <n-select
          v-model:value="appletName"
          :options="selectOptions"
          placeholder="Choose applet"
          style="min-width: 220px"
        />
        <n-button type="primary" @click="newApplet">+</n-button>
      </n-space>
      <n-space justify="center" style="margin-top: 12px">
        <component :is="settingForm" ref="settingForm" />
      </n-space>
    </div>
  </div>
</template>

<script>
import { NButton, NSelect, NSpace } from "naive-ui";
import CreateNewSetting from "./CreateNewSetting.vue";
import { getAppletDescriptor, listAppletsForCreate } from "../appletRegistryCore.js";

export default {
  name: "CreateNew",
  components: { NButton, NSelect, NSpace },
  data() {
    return {
      appletName: "",
      settingForm: null,
    };
  },
  computed: {
    appletChoices() {
      return listAppletsForCreate();
    },
    selectOptions() {
      return this.appletChoices.map((c) => ({ label: c.label, value: c.applet }));
    },
  },
  watch: {
    appletName(newVal) {
      const descriptor = getAppletDescriptor(newVal);
      if (descriptor && descriptor.settingComponent != null) {
        this.settingForm = descriptor.settingComponent;
      } else {
        this.settingForm = CreateNewSetting;
      }
    },
  },
  methods: {
    newApplet() {
      this.$parent.currentAppletName = this.appletName;
      const descriptor = getAppletDescriptor(this.appletName);
      if (descriptor && typeof descriptor.applyCreate === "function") {
        descriptor.applyCreate(this.$parent, this.$refs.settingForm);
      }
    },
  },
};
</script>

<style lang="scss" scoped>
.create-applet-container {
  background: var(--darcule-bg, var(--bg));
  color: var(--fg);
  .select-applet-form {
    width: 100%;
  }
}
</style>
