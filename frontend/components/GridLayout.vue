<template>
  <div class="grid-layout-container">
    <div style="display: inline-flex;">
      <vs-button @click="addItem">Add</vs-button>
      <vs-button @click="removeItem">Remove</vs-button>
      <vs-button @click="lockGridLayout">Lock</vs-button>
      <vs-button @click="unlockGridLayout">Unlock</vs-button>
    </div>
    <grid-layout
      :layout.sync="layout"
      :col-num="12"
      :row-height="24"
      :is-draggable="draggable"
      :is-resizable="resizable"
      :responsive="responsive"
      :vertical-compact="verticalCompact"
      :use-css-transforms="useCssTransforms"
      :margin="[1, 1]"
      >
      <grid-item
        v-for="item in layout"
        v-bind:key="item.i"
        :static="item.static"
        :x="item.x"
        :y="item.y"
        :w="item.w"
        :h="item.h"
        :i="item.i"
        drag-allow-from=".vue-draggable-handle"
        drag-ignore-from=".no-drag">
        <div class="grid-item-content">
          <span v-if="gridlock" class="text">{{ itemTitle(item) }}</span>
          <div class="vue-draggable-handle"></div>
          <div class="grid-item-main no-drag">
            <Editor v-if="item.i==0" class="editor_applet" />
            <IFrame style="width:100%; height:100%; " src="https://codepen.io/antoinebou13/full/xMXNyy" v-if="item.i==1" class="iframe_applet" />
            <IFrame style="width:100%; height:100%;" src="https://money.usnews.com/investing/stocks" v-if="item.i==2" class="iframe_applet" />
            <IFrame style="width:100%; height:100%;" src="https://www.youtube.com/embed/5qap5aO4i9A" v-if="item.i==3" class="iframe_applet" />
          </div>
        </div>
      </grid-item>
    </grid-layout>
  </div>
</template>

<script>
import { GridLayout, GridItem } from "vue-grid-layout";
import Editor from "./applets/Editor/Editor.vue";
import IFrame from "./applets/IFrame/IFrame.vue";
export default {
  components: {
    GridLayout,
    GridItem,
    Editor
  },
  data() {
    return {
      layout: [
        { x: 0, y: 0, w: 6, h: 12, i: "0", static: false },
        { x: 6, y: 0, w: 6, h: 12, i: "1", static: false },
        { x: 0, y: 6, w: 6, h: 12, i: "2", static: false },
        { x: 6, y: 6, w: 6, h: 12, i: "3", static: false },
      ],
      draggable: true,
      resizable: true,
      verticalCompact: true,
      useCssTransforms: true,
      responsive: true,
      // colNum: 12,
      // rowHeight: 24,
      index: 0,
      gridlock: false
    };
  },
  mounted() {
    // this.$gridlayout.load();
    this.index = this.layout.length;
  },
  methods: {
    itemTitle(item) {
      // # TODO change this
      let result = item.i;
      if (item.static) {
        result += " - Static";
      }
      return result;
    },
    addItem: function() {
      // Add a new item. It must have a unique key!
      this.layout.push({
        x: (this.layout.length * 2) % (this.colNum || 12),
        y: this.layout.length + (this.colNum || 12), // puts it at the bottom
        w: 6,
        h: 6,
        i: this.index
      });
      // Increment the counter to ensure key is always unique.
      this.index++;
    },
    removeItem: function(val) {
      const index = this.layout.map(item => item.i).indexOf(val);
      this.layout.splice(index, 1);
    },
    lockGridLayout: function() {
        this.gridlock = true
        this.draggable = false
        this.resizable = false
        for (const item in this.layout) {
            this.layout[item].static = true;
        }
    },
    unlockGridLayout: function() {
        this.gridlock = false
        this.draggable = true
        this.resizable = true
        for (const item in this.layout) {
            this.layout[item].static = false;
        }
    }
  }
};
</script>

<style scoped>
.grid-layout-container {
  background: var(--darcula-bg);
}
.vue-grid-layout {
    background: var(--darcula-bg);
}
.vue-grid-item:not(.vue-grid-placeholder) {
    justify-content: center;
    display: flex;
    box-shadow: rgb(0 0 0 / 10%) 0px 10px 15px -3px,
    rgb(0 0 0 / 5%) 0px 4px 6px -2px;
    border: 1px solid black;
    border-radius: 0.5rem;
}
.vue-grid-item{
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
    text-align: center;
    box-sizing: border-box;
    overflow: auto;
    margin: auto;
    height: 100%;
    width: 100%;
    background: var(--darcula-fg);
}

.vue-grid-item .cssTransforms {
    transition-property: transform,-webkit-transform;
    left: 0;
    right: auto;
}

.vue-grid-item .no-drag {
    height: 100%;
    width: 100%;
}
.vue-grid-item .minMax {
    font-size: 12px;
}
.vue-grid-item .add {
    cursor: pointer;
}
.vue-draggable-handle {
  position: absolute;
  width: 20px;
  height: 20px;
  top: 0;
  left: 0;
  background: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='10'><circle cx='5' cy='5' r='5' fill='#999999'/></svg>")
    no-repeat;
  background-position: bottom right;
  padding: 0 8px 8px 0;
  background-repeat: no-repeat;
  background-origin: content-box;
  box-sizing: border-box;
  cursor: pointer;
}

.grid-item-main {
  overflow: hidden;
}

.editor_applet {
  background: rgba(0, 128, 255, 0.1);
  margin: 24px 16px 16px 16px;
  text-align: left;

}

/* Turn on custom 8px wide scrollbar */
::-webkit-scrollbar {
  width: 8px; /* 1px wider than Lion. */
  /* This is more usable for users trying to click it. */
  background-color: rgba(0,0,0,0);
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
  background: rgba(0,0,0,0.5);
  -webkit-border-radius: 100px;
}
::-webkit-scrollbar-thumb:vertical:active {
  background: rgba(0,0,0,0.61); /* Some darker color when you click it */
  -webkit-border-radius: 100px;
}
</style>
