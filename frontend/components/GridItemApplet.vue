<template>
  <grid-item
      v-bind="$attrs"
      drag-allow-from=".vue-draggable-handle"
      drag-ignore-from=".no-drag">
    <div class="grid-item-content">
      <div class="grid-item-main">
        <span v-if="showTitle" class="title">{{ getTitle() }}</span>
        <div ref="draggableHandle" class="vue-draggable-handle" style="height: 24px;">
          <span class="remove-item" @click="removeItem()"><i class="bx bx-x"/></span>
          <span v-if="appletName == 'IFrame'" class="open-new-tab-item" @click="openNewTab()"><i class="bx bx-export"/></span>
          <span v-if="appletName == 'IFrame'" class="reload-iframe" @click="reload()"><i class="bx bx-refresh"/></span>
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
      this.appletData.appletName = this.$refs.applet.currentAppletName;
      this.currentAppletData.appletName = this.$refs.applet.currentAppletName;
      for(let key in this.$refs.applet.$attrs){
        this.currentAppletData[key] = this.$refs.applet.$attrs[key];
      }
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
  box-shadow: rgba(0, 0, 0, 0.1) 0px 10px 15px -3px,
  rgba(0, 0, 0, 0.05) 0px 4px 6px -2px;
  border: 1px solid black;
  border-radius: 0.5rem;
}

.vue-grid-item {
  background: var(--fg);
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
  margin: auto;
  height: 100%;
  width: 100%;
  background: var(--fg);
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
}

.vue-draggable-handle {
  width: 100%;
  background: var(--cl);
  box-sizing: border-box;
  cursor: grab;
  .remove-item{
    position: absolute;
    right: 8px;
    top: 0px;
    font-size: 18px;
    cursor: pointer;
    color: var(--fg)
  }
  .open-new-tab-item {
    position: absolute;
    right: 32px;
    top: 2px;
    font-size: 14px;
    cursor: pointer;
    color: var(--fg)
  }
  .reload-iframe {
    position: absolute;
    right: 54px;
    top: 1px;
    font-size: 16px;
    cursor: pointer;
    color: var(--fg)
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