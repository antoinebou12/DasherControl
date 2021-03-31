<template>
  <div class="grid-layout-container">
    <GridEditMenu :gridLayout="this"/>
    <grid-layout
        ref="gridLayout"
        :layout.sync="layout"
        :col-num="12"
        :row-height="24"
        :is-draggable="draggable"
        :is-resizable="resizable"
        :responsive="responsive"
        :vertical-compact="verticalCompact"
        :use-css-transforms="useCssTransforms"
        :margin="[1, 1]">
      <GridItemApplet v-for="item in layout" :key="item.i"
                :static="item.static"
                :x="item.x"
                :y="item.y"
                :w="item.w"
                :h="item.h"
                :i="item.i"
                :appletData="item.appletData"
                :extra="item.extra">
      </GridItemApplet>
    </grid-layout>
  </div>
</template>

<script>
import { GridLayout, GridItem } from "vue-grid-layout";
import GridItemApplet from "./GridItemApplet.vue";
import GridEditMenu from "./GridEditMenu.vue";
import Applet from "./applets/Applet.vue"

export default {
  name: "GridLayoutMain",
  components: {
    GridLayout,
    GridItem,
    GridEditMenu,
    GridItemApplet,
    Applet
  },
  data: () => ({
    layout: [
      {x: 0, y: 0, w: 6, h: 12, i: '0', static: false, appletData: { appletName: 'Editor'}, extra: {title: '0'}},
      {x: 6, y: 0, w: 6, h: 12, i: '1', static: false, appletData: { appletName: 'IFrame', src: 'https://www.chess.com/daily_puzzle'}, extra: {title: '1'}},
      {x: 0, y: 6, w: 6, h: 12, i: '2', static: false, appletData: { appletName: 'CreateNew'}, extra: {title: '2'}},
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
  }),
  mounted() {
    this.index = this.layout.length;
  },
  methods: {
    addItem: function () {
      // Add a new item. It must have a unique key!
      this.layout.push({
        x: (this.layout.length * 2) % (this.colNum || 12),
        y: this.layout.length + (this.colNum || 12),
        w: 6,
        h: 6,
        i: this.index,
        static: false,
        title: this.index,
        draggable: true,
        resizable: true,
        appletData: { appletName: 'CreateNew'}, extra: {title: this.index}
      });
      // Increment the counter to ensure key is always unique.
      this.index++;
    },
    removeItem: function (val) {
      const index = this.layout.map(item => item.i).indexOf(val);
      this.layout.splice(index, 1);
    },
    lockGridLayout: function () {
      this.gridlock = true
      this.draggable = false
      this.resizable = false
      for (const item in this.layout) {
        this.layout[item].static = true;
      }
    },
    unlockGridLayout: function () {
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

<style lang="scss" scoped>
.grid-layout-container {
  background: var(--darcula-bg);
}

.vue-grid-layout {
  background: var(--darcula-bg);
}
</style>
