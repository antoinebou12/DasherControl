<template>
  <div
    class="floating-applet-shell"
    :style="shellStyle"
    @mousedown="onShellMouseDown"
  >
    <div
      class="floating-header"
      @mousedown.stop="startDrag"
    >
      <span class="dock-item" title="Dock to grid" @click.stop="dock"><i class="bx bx-dock-left"/></span>
      <span v-if="appletToolbar.showOpenInNewTab" class="open-new-tab-item" @click.stop="openNewTab"><i class="bx bx-export"/></span>
      <span v-if="appletToolbar.showReload" class="reload-iframe" @click.stop="reload"><i class="bx bx-refresh"/></span>
      <span
        class="remove-item"
        title="Remove applet"
        role="button"
        tabindex="0"
        aria-label="Remove applet"
        @mousedown.stop
        @click.stop="removeItem"
        @keydown.enter.prevent="removeItem"
        @keydown.space.prevent="removeItem"
      ><i class="bx bx-trash" aria-hidden="true"/></span>
    </div>
    <div class="floating-body no-drag">
      <Applet
        ref="applet"
        @changeApplet="changeApplet"
        :appletName="appletName"
        v-bind="currentAppletData"
      />
    </div>
    <div
      v-if="!gridlock"
      class="resize-handle"
      @mousedown.stop="startResize"
    />
  </div>
</template>

<script>
import Applet from "./applets/Applet.vue";
import { getAppletDescriptor, normalizeAppletId } from "./applets/appletRegistryCore.js";

export default {
  name: "FloatingAppletPanel",
  components: { Applet },
  props: {
    item: { type: Object, required: true },
    gridlock: { type: Boolean, default: false },
  },
  data() {
    return {
      currentAppletData: this.item.appletData || {},
      appletName: (this.item.appletData && this.item.appletData.appletName) || "CreateNew",
      dragState: null,
      resizeState: null,
    };
  },
  computed: {
    appletToolbar() {
      const d = getAppletDescriptor(normalizeAppletId(this.appletName));
      return d && d.toolbar ? d.toolbar : {};
    },
    shellStyle() {
      const z = this.item.floatZ != null ? this.item.floatZ : 2100;
      const l = this.item.floatLeft != null ? this.item.floatLeft : 0;
      const t = this.item.floatTop != null ? this.item.floatTop : 0;
      const w = this.item.floatWidth != null ? this.item.floatWidth : 400;
      const h = this.item.floatHeight != null ? this.item.floatHeight : 300;
      return {
        position: "fixed",
        left: `${l}px`,
        top: `${t}px`,
        width: `${w}px`,
        height: `${h}px`,
        zIndex: z,
      };
    },
  },
  watch: {
    "item.appletData": {
      deep: true,
      handler(newVal) {
        this.currentAppletData = newVal || {};
        this.appletName = (newVal && newVal.appletName) || "CreateNew";
      },
    },
  },
  methods: {
    dock() {
      this.$emit("dock");
    },
    removeItem() {
      this.$emit("removeItem", this.item.i);
    },
    onShellMouseDown() {
      this.$emit("focusFloat", this.item.i);
    },
    openNewTab() {
      if (this.item.appletData && this.item.appletData.src) {
        window.open(this.item.appletData.src, "_blank").focus();
      }
    },
    reload() {
      if (this.$refs.applet && this.$refs.applet.$refs.applet) {
        this.$refs.applet.$refs.applet.reload();
      }
    },
    changeApplet() {
      const inner = this.$refs.applet;
      if (!inner) return;
      if (!this.item.appletData) this.item.appletData = {};
      this.item.appletData.appletName = inner.currentAppletName;
      this.currentAppletData.appletName = inner.currentAppletName;
      this.appletName = inner.currentAppletName || "CreateNew";
      const merged = inner.getBindingsForPersist
        ? inner.getBindingsForPersist()
        : inner.$attrs;
      for (const key in merged) {
        this.item.appletData[key] = merged[key];
        this.currentAppletData[key] = merged[key];
      }
      if (inner.clearCreateOverrides) inner.clearCreateOverrides();
    },
    startDrag(ev) {
      if (this.gridlock) return;
      const startX = ev.clientX;
      const startY = ev.clientY;
      const startLeft = this.item.floatLeft;
      const startTop = this.item.floatTop;
      const onMove = (e) => {
        this.item.floatLeft = Math.round(startLeft + (e.clientX - startX));
        this.item.floatTop = Math.round(startTop + (e.clientY - startY));
      };
      const onUp = () => {
        document.removeEventListener("mousemove", onMove);
        document.removeEventListener("mouseup", onUp);
      };
      document.addEventListener("mousemove", onMove);
      document.addEventListener("mouseup", onUp);
    },
    startResize(ev) {
      if (this.gridlock) return;
      const startX = ev.clientX;
      const startY = ev.clientY;
      const startW = this.item.floatWidth;
      const startH = this.item.floatHeight;
      const onMove = (e) => {
        this.item.floatWidth = Math.max(220, Math.round(startW + (e.clientX - startX)));
        this.item.floatHeight = Math.max(160, Math.round(startH + (e.clientY - startY)));
      };
      const onUp = () => {
        document.removeEventListener("mousemove", onMove);
        document.removeEventListener("mouseup", onUp);
      };
      document.addEventListener("mousemove", onMove);
      document.addEventListener("mouseup", onUp);
    },
  },
};
</script>

<style lang="scss" scoped>
.floating-applet-shell {
  display: flex;
  flex-direction: column;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.18);
  border: 2px solid var(--applet-border);
  border-radius: 8px;
  background: var(--applet-bg);
  overflow: hidden;
}

.floating-header {
  flex: 0 0 28px;
  width: 100%;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.04) 0%, transparent 100%), var(--applet-chrome-bg);
  cursor: grab;
  position: relative;
  box-sizing: border-box;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.dock-item {
  position: absolute;
  left: 8px;
  top: 2px;
  font-size: 18px;
  cursor: pointer;
  color: var(--applet-chrome-fg);
}

.remove-item {
  position: absolute;
  right: 8px;
  top: 0px;
  font-size: 18px;
  cursor: pointer;
  color: var(--applet-chrome-fg);
  &:hover,
  &:focus-visible {
    color: #f87171;
  }
}

.open-new-tab-item {
  position: absolute;
  right: 32px;
  top: 2px;
  font-size: 14px;
  cursor: pointer;
  color: var(--applet-chrome-fg);
}

.reload-iframe {
  position: absolute;
  right: 54px;
  top: 1px;
  font-size: 16px;
  cursor: pointer;
  color: var(--applet-chrome-fg);
}

.floating-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
  color: var(--applet-fg);
  color-scheme: light;
  background: var(--applet-bg);
}

.resize-handle {
  position: absolute;
  right: 0;
  bottom: 0;
  width: 16px;
  height: 16px;
  cursor: se-resize;
  background: linear-gradient(135deg, transparent 50%, rgba(0, 0, 0, 0.25) 50%);
}
</style>
