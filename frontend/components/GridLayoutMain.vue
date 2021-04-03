<template>
  <div class="grid-layout-container">
    <GridEditMenu :gridLayout="this" />
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
      <GridItemApplet
          v-for="item in layout" :key="item.i"
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
import axios from "axios";

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
      {x: 0, y: 0, w: 6, h: 12, i: 0, static: false, appletData: { appletName: 'Editor'}, extra: {title: '0'}},
      {x: 6, y: 0, w: 6, h: 12, i: 1, static: false, appletData: { appletName: 'IFrame', src: 'https://www.chess.com/daily_puzzle'}, extra: {title: '1'}},
      {x: 0, y: 6, w: 6, h: 12, i: 2, static: false, appletData: { appletName: 'CreateNew'}, extra: {title: '2'}},
    ],
    draggable: true,
    resizable: true,
    verticalCompact: true,
    useCssTransforms: true,
    responsive: true,
    // colNum: 12,
    // rowHeight: 24,
    index: 0,
    gridlock: false,
    workspace_id: 0,
  }),
  watch: {
  },
  mounted() {
    this.index = this.layout.length;
  },
  methods: {
    addNewItem: function () {
      // Add a new item. It must have a unique key!
      this.layout.push({
        x: (this.layout.length * 2) % (this.colNum || 12),
        y: this.layout.length + (this.colNum || 12),
        w: 6,
        h: 6,
        i: this.index,
        static: false,
        draggable: true,
        resizable: true,
        appletData: { appletName: 'CreateNew'}, extra: {title: this.index}
      });
      // Increment the counter to ensure key is always unique.
      this.index++;
    },
    addItemCustom: function (x, y, w, h, isStatic, draggable, resizable, appletData, extra) {
      // Add a new item. It must have a unique key!
      this.layout.push({
        x: x,
        y: y,
        w: w,
        h: h,
        i: this.index,
        static: isStatic,
        draggable: draggable,
        resizable: resizable,
        appletData: appletData,
        extra: extra
      });
      // Increment the counter to ensure key is always unique.
      this.index++;
    },
    removeItem: function (val) {
      const index = this.layout.map(item => item.i).indexOf(val);
      this.layout.splice(index, 1);
      this.index--;
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
    },
    saveWorkspaceLayout(){
      let applets_layout = []
      for (let i=0;i < this.layout.length;i++) {
        let applet = {}
        applet.name = this.layout[i].i.toString()
        applet.position_x = this.layout[i].x
        applet.position_y = this.layout[i].y
        applet.width = this.layout[i].w
        applet.height = this.layout[i].h
        applet.editable = this.layout[i].static
        applet.applet_data = JSON.stringify(this.layout[i].appletData)
        applets_layout.push(applet)
      }

      axios({
        method: 'post',
        url: '/workspaces/api/create',
        headers: {
          'Content-Type': 'application/json'
        },
        data: {
          name: "workspace",
          display_order: 0,
          tenant_id: 1,
          applets: applets_layout
        }
      });
    },
    setWorkspaceLayout(id) {
      axios({
        method: 'get',
        url: '/workspaces/api/' + id,
      }).then((response) => {
        this.layout = []
        this.index = 0
        for (let i=0;i < response.data.length;i++) {
          this.addItemCustom(
              response.data[i].position_x,
              response.data[i].position_y,
              response.data[i].width,
              response.data[i].height,
              response.data[i].editable,
              response.data[i].editable,
              response.data[i].editable,
              JSON.parse(response.data[i].applet_data),
              {}
          )
        }
        this.$emit("setWorkspace")
      });
    },
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
