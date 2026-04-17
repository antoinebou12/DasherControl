<template>
  <div class="create-applet-container grid dc-create-panel">
    <v-card class="dc-create-card" variant="outlined" rounded="lg">
      <v-card-text>
        <div class="select-applet-form">
          <div class="select-applet-form__row d-flex flex-wrap align-start justify-space-between ga-3">
            <div
              class="catalog-wrap"
              data-testid="create-applet-select"
              role="region"
              aria-label="Applet catalog"
            >
              <v-expansion-panels v-model="expandedNames" multiple variant="accordion" class="dc-applet-catalog">
                <v-expansion-panel v-for="cat in createPickerCatalog" :key="cat.categoryKey" :value="cat.categoryKey">
                  <v-expansion-panel-title class="text-body-2 font-weight-semibold">
                    {{ cat.categoryLabel }}
                  </v-expansion-panel-title>
                  <v-expansion-panel-text class="pa-0">
                    <div class="applet-row-list">
                      <v-btn
                        v-for="a in cat.applets"
                        :key="a.id"
                        variant="text"
                        block
                        size="small"
                        class="applet-row justify-start"
                        :class="{ 'applet-row--selected': appletName === a.id }"
                        data-testid="create-applet-catalog-item"
                        :aria-pressed="appletName === a.id"
                        @click="pickApplet(a.id)"
                      >
                        {{ a.label }}
                      </v-btn>
                    </div>
                  </v-expansion-panel-text>
                </v-expansion-panel>
              </v-expansion-panels>
            </div>
            <v-btn color="primary" data-testid="create-applet-add" aria-label="Add applet" @click="newApplet">
              Add
            </v-btn>
          </div>
          <div class="select-applet-form__settings d-flex justify-center mt-3">
            <component :is="settingForm" ref="settingForm" />
          </div>
        </div>
      </v-card-text>
    </v-card>
  </div>
</template>

<script>
import { createNotification } from "../../../utils";
import CreateNewSetting from "./CreateNewSetting.vue";
import { getAppletDescriptor, getCreatePickerCatalog } from "../appletRegistryCore.js";

export default {
  name: "CreateNew",
  data() {
    return {
      appletName: "",
      settingForm: null,
      expandedNames: getCreatePickerCatalog().map((c) => c.categoryKey),
    };
  },
  computed: {
    createPickerCatalog() {
      return getCreatePickerCatalog();
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
    pickApplet(id) {
      this.appletName = id;
    },
    newApplet() {
      if (!this.appletName || !String(this.appletName).trim()) {
        createNotification(null, "Add applet", "Choose an applet first.", "danger");
        return;
      }
      this.$emit("confirm-applet", {
        appletName: this.appletName,
        settingFormRef: this.$refs.settingForm,
      });
    },
  },
};
</script>

<style lang="scss" scoped>
.create-applet-container.dc-create-panel {
  background: var(--create-panel-bg);
  color: var(--create-panel-fg);
  border-radius: 10px;
  padding: 12px 12px 14px;
  box-sizing: border-box;
  min-height: 100%;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.04);
}

.dc-create-card {
  max-width: 600px;
  margin: 0 auto;
  background: var(--surface-glass) !important;
  border-color: var(--create-border) !important;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.select-applet-form {
  width: 100%;
}

.select-applet-form__row {
  width: 100%;
  align-items: flex-start;
}

.catalog-wrap {
  flex: 1 1 280px;
  min-width: 0;
  max-height: 280px;
  overflow-y: auto;
  border-radius: 8px;
  border: 1px solid var(--create-border);
  background: var(--create-surface-solid);
}

.applet-row-list {
  padding: 4px 0 6px 10px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.applet-row {
  justify-content: flex-start !important;
  font-weight: 500;
  color: #111827 !important;
  border-radius: 6px;
}

.applet-row:hover {
  background: color-mix(in srgb, var(--ui-accent) 12%, transparent) !important;
}

.applet-row--selected {
  background: color-mix(in srgb, var(--ui-accent) 22%, transparent) !important;
  color: #0f172a !important;
}

.select-applet-form__settings {
  margin-top: 12px;
  width: 100%;
}

.create-applet-container :deep(.dc-applet-catalog .v-expansion-panel-title) {
  min-height: 40px;
  padding: 8px 12px;
  font-size: 13px;
  font-weight: 600;
  background: color-mix(in srgb, var(--create-panel-bg) 55%, transparent);
  color: rgba(17, 24, 39, 0.85);
}

.create-applet-container :deep(.dc-applet-catalog .v-expansion-panel-text__wrapper) {
  border-top: 1px solid var(--create-border);
}
</style>
