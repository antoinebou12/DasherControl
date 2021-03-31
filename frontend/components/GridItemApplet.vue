<template>
  <grid-item
      v-bind="$attrs"
      drag-allow-from=".vue-draggable-handle"
      drag-ignore-from=".no-drag">
    <div class="grid-item-content">
      <div class="grid-item-main">
        <span v-if="showTitle" class="title">{{ getTitle() }}</span>
        <div class="vue-draggable-handle"></div>
        <div class="grid-item-main no-drag">
          <Applet :appletName="appletData.appletName || 'CreateNew' " v-bind="appletData"></Applet>
        </div>
      </div>
    </div>
  </grid-item>
</template>
<script>
import {GridItem} from "vue-grid-layout"
import Applet from "./applets/Applet.vue"

export default {
  name: "GridItemApplet",
  components: {
    GridItem,
    Applet
  },
  props: {
    appletData: {type: Object, required: false, default: {}},
    extra: {type: Object, required: false, default: {}},
  },
  watch: {

  },
  data() {
    return {
      title: this.extra.title || "Title",
      showTitle: false
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

  }
}
</script>
<style lang="scss">
.vue-grid-item:not(.vue-grid-placeholder) {
  justify-content: center;
  display: flex;
  box-shadow: rgba(0, 0, 0, 0.1) 0px 10px 15px -3px,
  rgba(0, 0, 0, 0.05) 0px 4px 6px -2px;
  border: 1px solid black;
  border-radius: 0.5rem;
}

.vue-grid-item {
  background: var(--darcula-fg);
}

.vue-grid-item .resizing {
  opacity: 0.9;
}

.vue-grid-item .static {
  background: #cce;
}

.vue-grid-item .grid-item-content {
  font-size: 24px;
  box-sizing: border-box;
  overflow: auto;
  margin: auto;
  height: 100%;
  width: 100%;
  background: var(--darcula-fg);
}

.vue-grid-item .cssTransforms {
  transition-property: transform, -webkit-transform;
  left: 0;
  right: auto;
}

.vue-grid-item .no-drag {
  height: 100%;
  width: 100%;
  margin-top: 20px;
}

.vue-grid-item .minMax {
  font-size: 12px;
}

.vue-grid-item .add {
  cursor: pointer;
}

.grid-item-main {
  overflow: hidden;
  height: 100%;
  width: 100%;
}

.vue-draggable-handle {
  position: absolute;
  width: 100%;
  height: 25px;
  top: 0;
  left: 0;
  background: var(--darcula-bg);
  box-sizing: border-box;
  cursor: grab;
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