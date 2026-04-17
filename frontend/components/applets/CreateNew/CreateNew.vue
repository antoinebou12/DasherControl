<template>
  <div class="create-applet-container grid dc-create-panel">
    <n-card size="small" bordered class="dc-create-card" :segmented="{ content: true }">
      <div class="select-applet-form">
        <n-space justify="space-between" align="start" wrap :size="12" class="select-applet-form__row">
          <div
            class="catalog-wrap"
            data-testid="create-applet-select"
            role="region"
            aria-label="Applet catalog"
          >
            <n-collapse v-model:expanded-names="expandedNames" :accordion="false" class="dc-applet-catalog">
              <n-collapse-item
                v-for="cat in createPickerCatalog"
                :key="cat.categoryKey"
                :title="cat.categoryLabel"
                :name="cat.categoryKey"
              >
                <div class="applet-row-list">
                  <n-button
                    v-for="a in cat.applets"
                    :key="a.id"
                    text
                    block
                    size="small"
                    class="applet-row"
                    :class="{ 'applet-row--selected': appletName === a.id }"
                    data-testid="create-applet-catalog-item"
                    :aria-pressed="appletName === a.id"
                    @click="pickApplet(a.id)"
                  >
                    {{ a.label }}
                  </n-button>
                </div>
              </n-collapse-item>
            </n-collapse>
          </div>
          <n-button type="primary" data-testid="create-applet-add" aria-label="Add applet" @click="newApplet">
            Add
          </n-button>
        </n-space>
        <n-space justify="center" :size="0" class="select-applet-form__settings">
          <component :is="settingForm" ref="settingForm" />
        </n-space>
      </div>
    </n-card>
  </div>
</template>

<script>
import { NButton, NCard, NCollapse, NCollapseItem, NSpace } from "naive-ui";
import { createNotification } from "../../../utils";
import CreateNewSetting from "./CreateNewSetting.vue";
import { getAppletDescriptor, getCreatePickerCatalog } from "../appletRegistryCore.js";

export default {
  name: "CreateNew",
  components: { NButton, NCard, NCollapse, NCollapseItem, NSpace },
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
  border: 1px solid var(--create-border) !important;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  --n-border-color: var(--create-border) !important;
}

.dc-create-card :deep(.n-card__content) {
  padding: 14px 14px 16px !important;
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

.create-applet-container :deep(.dc-applet-catalog.n-collapse) {
  --n-title-text-color: rgba(17, 24, 39, 0.85);
  --n-text-color: #111827;
  --n-divider-color: var(--create-border);
}

.create-applet-container :deep(.dc-applet-catalog .n-collapse-item) {
  margin: 0;
}

.create-applet-container :deep(.dc-applet-catalog .n-collapse-item__header) {
  padding: 8px 12px;
  font-size: 13px;
  font-weight: 600;
  background: color-mix(in srgb, var(--create-panel-bg) 55%, transparent);
}

.create-applet-container :deep(.dc-applet-catalog .n-collapse-item__content-wrapper) {
  border-top: 1px solid var(--create-border);
}

.create-applet-container :deep(.dc-applet-catalog .n-collapse-item__content-inner) {
  padding-top: 0;
  padding-bottom: 4px;
}

.create-applet-container :deep(.n-input) {
  --n-color: var(--create-surface-solid) !important;
  --n-text-color: #111827 !important;
  --n-placeholder-color: rgba(17, 24, 39, 0.45) !important;
  --n-border: 1px solid var(--create-border) !important;
  --n-border-hover: 1px solid color-mix(in srgb, var(--ui-accent) 40%, var(--create-border)) !important;
  --n-border-focus: 1px solid var(--ui-accent) !important;
}

.create-applet-container :deep(.n-button.n-button--primary-type) {
  --n-color: var(--ui-accent) !important;
  --n-color-hover: var(--ui-accent-hover) !important;
  --n-color-pressed: var(--ui-accent-pressed) !important;
  --n-text-color: #ffffff !important;
  --n-border: none !important;
}
</style>
