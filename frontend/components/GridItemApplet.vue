<template>
  <grid-item
      v-bind="$attrs"
      drag-allow-from=".vue-draggable-handle"
      drag-ignore-from=".no-drag">
    <div class="grid-item-content">
      <div class="grid-item-main">
        <span v-if="showTitle" class="title">{{ getTitle() }}</span>
        <div ref="draggableHandle" class="vue-draggable-handle">
          <span
            class="remove-item"
            title="Remove applet"
            role="button"
            tabindex="0"
            aria-label="Remove applet"
            @mousedown.stop
            @click.stop="removeItem()"
            @keydown.enter.prevent="removeItem()"
            @keydown.space.prevent="removeItem()"
          ><i class="bx bx-trash" aria-hidden="true"/></span>
          <span v-if="appletToolbar.showOpenInNewTab" class="open-new-tab-item" @click="openNewTab()"><i class="bx bx-export"/></span>
          <span v-if="appletToolbar.showReload" class="reload-iframe" @click="reload()"><i class="bx bx-refresh"/></span>
        </div>
        <div ref="gridItemMain" class="grid-item-main no-drag">
          <Applet ref="applet" @changeApplet="changeApplet" :appletName="appletName" v-bind="currentAppletData"></Applet>
        </div>
      </div>
    </div>
  </grid-item>
</template>
<script>
import {GridItem} from "vue-grid-layout"
import Applet from "./applets/Applet.vue"
import { getAppletDescriptor, normalizeAppletId } from "./applets/appletRegistryCore.js"

export default {
  name: "GridItemApplet",
  components: {
    GridItem,
    Applet
  },
  props: {
    id: Number,
    appletData: {type: Object, required: false, default: {}},
    extra: {type: Object, required: false, default: {}},
  },
  data() {
    return {
      currentAppletData: this.appletData,
      appletName: this.appletData.appletName || 'CreateNew',
      title: this.extra.title || "Title",
      showTitle: false,
    }
  },
  computed: {
    appletToolbar() {
      const d = getAppletDescriptor(normalizeAppletId(this.appletName))
      return d && d.toolbar ? d.toolbar : {}
    },
  },
  watch : {
    appletData: function(newVal) {
      this.currentAppletData = newVal
      this.appletName = newVal.appletName
      this.appletName = this.currentAppletData.appletName || 'CreateNew';
    }
  },
  methods: {
    getTitle(){
      let result = this.index;
      if (this.static) {
        result += " - Static";
      }
      return result;
    },
    resizeEvent: function (i, newH, newW, newHPx, newWPx) {
    },
    containerResizedEvent: function (i, newH, newW, newHPx, newWPx) {
    },
    changeApplet() {
      const inner = this.$refs.applet;
      if (!inner) return;
      this.appletData.appletName = inner.currentAppletName;
      this.currentAppletData.appletName = inner.currentAppletName;
      const merged = inner.getBindingsForPersist
        ? inner.getBindingsForPersist()
        : inner.$attrs;
      for (const key in merged) {
        this.currentAppletData[key] = merged[key];
      }
      if (inner.clearCreateOverrides) inner.clearCreateOverrides();
    },
    removeItem() {
      this.$emit('removeItem', this.id);
    },
    openNewTab() {
      window.open(this.appletData.src, '_blank').focus();
    },
    reload() {
      this.$refs.applet.$refs.applet.reload();
    },
  }
}
</script>
<style lang="scss">

.vue-grid-item:not(.vue-grid-placeholder) {
  justify-content: center;
  display: flex;
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.12);
  border: 2px solid var(--applet-border);
  border-radius: 8px;
}

.vue-grid-item {
  background: var(--applet-bg);
}

.vue-grid-item .resizing {
  opacity: 0.95;
}

.vue-grid-item .static {
  background: var(--applet-bg);
}

.vue-grid-item .grid-item-content {
  font-size: 24px;
  box-sizing: border-box;
  margin: auto;
  height: 100%;
  width: 100%;
  background: var(--applet-bg);
  color: var(--applet-fg);
  color-scheme: light;
  overflow: hidden;
}

.vue-grid-item .cssTransforms {
  transition-property: transform, -webkit-transform;
  left: 0;
  right: auto;
}

.vue-grid-item .no-drag {
  height: 100%;
  width: 100%;
  overflow: auto;
}

.vue-grid-item .minMax {
  font-size: 12px;
}

.vue-grid-item .add {
  cursor: pointer;
}

.grid-item-main {
  height: 100%;
  width: 100%;
  color: var(--applet-fg);
}

.vue-grid-item .grid-item-main :deep(.n-input),
.vue-grid-item .grid-item-main :deep(.n-base-selection) {
  --n-text-color: var(--applet-fg) !important;
}

.vue-grid-item .grid-item-main :deep(.dc-create-panel .n-input),
.vue-grid-item .grid-item-main :deep(.dc-create-panel .n-base-selection) {
  --n-text-color: #111827 !important;
}

.vue-draggable-handle {
  position: relative;
  width: 100%;
  height: 26px;
  min-height: 26px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.04) 0%, transparent 100%), var(--applet-chrome-bg);
  box-sizing: border-box;
  cursor: grab;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  .remove-item{
    position: absolute;
    right: 8px;
    top: 1px;
    font-size: 18px;
    cursor: pointer;
    color: var(--applet-chrome-fg);
    opacity: 0.9;
    &:hover,
    &:focus-visible {
      opacity: 1;
      color: #f87171;
    }
  }
  .open-new-tab-item {
    position: absolute;
    right: 32px;
    top: 3px;
    font-size: 14px;
    cursor: pointer;
    color: var(--applet-chrome-fg);
    opacity: 0.9;
    &:hover {
      opacity: 1;
    }
  }
  .reload-iframe {
    position: absolute;
    right: 54px;
    top: 2px;
    font-size: 16px;
    cursor: pointer;
    color: var(--applet-chrome-fg);
    opacity: 0.9;
    &:hover {
      opacity: 1;
    }
  }
}

/* Turn on custom 8px wide scrollbar */
::-webkit-scrollbar {
  width: 8px; /* 1px wider than Lion. */
  /* This is more usable for users trying to click it. */
  background-color: rgba(0, 0, 0, 0);
  -webkit-border-radius: 100px;
}

/* hover effect for both scrollbar area, and scrollbar 'thumb' */
::-webkit-scrollbar:hover {
  background-color: rgba(0, 0, 0, 0.09);
}

/* The scrollbar 'thumb' ...that marque oval shape in a scrollbar */
::-webkit-scrollbar-thumb:vertical {
  /* This is the EXACT color of Mac OS scrollbars.
     Yes, I pulled out digital color meter */
  background: rgba(0, 0, 0, 0.5);
  -webkit-border-radius: 100px;
}

::-webkit-scrollbar-thumb:vertical:active {
  background: rgba(0, 0, 0, 0.61); /* Some darker color when you click it */
  -webkit-border-radius: 100px;
}
</style>
