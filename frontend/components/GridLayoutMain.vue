<template>
  <div class="grid-layout-container">
    <GridEditMenu :grid-layout="gridSelf" />
    <WorkspaceGrid
      v-model:layout="dockedLayout"
      :col-num="12"
      :row-height="24"
      :is-draggable="draggable"
      :is-resizable="resizable"
      :responsive="responsive"
      :vertical-compact="verticalCompact"
      :use-css-transforms="useCssTransforms"
      :margin="[1, 1]"
    >
      <GridItemApplet
        v-for="item in dockedLayout"
        :key="item.i"
        :static="item.static"
        :x="item.x"
        :y="item.y"
        :w="item.w"
        :h="item.h"
        :i="item.i"
        :applet-data="item.appletData"
        :extra="item.extra"
        :id="item.i"
        ref=""
        @remove-item="removeItem"
        @pop-out="onPopOut"
      />
    </WorkspaceGrid>
    <FloatingAppletPanel
        v-for="item in floatingLayout"
        :key="'float-' + item.i"
        :item="item"
        :gridlock="gridlock"
        @dock="dockFloat(item)"
        @removeItem="removeItem"
        @focusFloat="bringFloatToFront"
    />
  </div>
</template>

<script>
import { watch } from "vue";
import { storeToRefs } from "pinia";
import GridItemApplet from "./GridItemApplet.vue";
import GridEditMenu from "./GridEditMenu.vue";
import WorkspaceGrid from "./WorkspaceGrid.vue";
import FloatingAppletPanel from "./FloatingAppletPanel.vue";
import Applet from "./applets/Applet.vue"
import axios from "axios";
import { createNotification } from "../utils";
import { buildGuestDashboardLayout } from "../utils/guestDashboard";
import { useUserStore } from "../stores/user";
import { appearancePayloadForApi } from "../utils/workspaceTheme";
import { useWorkspaceAppearanceStore } from "../stores/workspaceAppearance";
import { useWorkspaceContextStore } from "../stores/workspaceContext";

export default {
  name: "GridLayoutMain",
  components: {
    WorkspaceGrid,
    GridEditMenu,
    GridItemApplet,
    FloatingAppletPanel,
    Applet,
  },
  data: () => ({
    layout: [],
    draggable: true,
    resizable: true,
    verticalCompact: true,
    useCssTransforms: true,
    responsive: true,
    index: 0,
    gridlock: false,
    workspace_id: 0,
    floatZCounter: 2100,
    workspace_name: "",
  }),
  watch: {
    layout: {
      deep: true,
      handler() {
        this.syncWorkspaceContextStore();
      },
    },
    workspace_id() {
      this.syncWorkspaceContextStore();
    },
    workspace_name() {
      this.syncWorkspaceContextStore();
    },
  },
  computed: {
    /** Pass Options API instance to the toolbar (Vue templates do not expose `this`). */
    gridSelf() {
      return this;
    },
    /** Writable slice for vue-grid-layout (docked tiles only). */
    dockedLayout: {
      get() {
        return this.layout.filter((it) => !it.isFloating);
      },
      set(rows) {
        rows.forEach((cell) => {
          const t = this.layout.find((l) => l.i === cell.i);
          if (t && !t.isFloating) {
            t.x = cell.x;
            t.y = cell.y;
            t.w = cell.w;
            t.h = cell.h;
            if (typeof cell.static === "boolean") t.static = cell.static;
          }
        });
      },
    },
    floatingLayout() {
      return this.layout.filter((it) => it.isFloating);
    },
  },
  mounted() {
    this.index = this.layout.length;
    this.syncWorkspaceContextStore();
    const user = useUserStore();
    const { token, sessionResolved } = storeToRefs(user);
    this._guestWatchStop = watch(
      [sessionResolved, token],
      ([resolved, tok]) => {
        if (!resolved) return;
        if (!tok) this.applyGuestDashboard();
      },
      { immediate: true }
    );
  },
  beforeUnmount() {
    if (typeof this._guestWatchStop === "function") this._guestWatchStop();
  },
  methods: {
    applyGuestDashboard() {
      this.layout = buildGuestDashboardLayout().map((row) => ({ ...row }));
      this.index = 2;
      this.workspace_id = -1;
      this.workspace_name = "Guest";
      this.lockGridLayout();
      this.syncWorkspaceContextStore();
    },
    syncWorkspaceContextStore() {
      const applets = this.layout.map((it) => ({
        gridId: it.i,
        appletName:
          (it.appletData && it.appletData.appletName) ? it.appletData.appletName : "CreateNew",
      }));
      useWorkspaceContextStore().setSession(
        this.workspace_id,
        this.workspace_name || "",
        applets
      );
    },
    onPopOut({ id, rect }) {
      const item = this.layout.find((l) => l.i === id);
      if (!item || item.isFloating) return;
      item.isFloating = true;
      item.floatLeft = Math.round(rect.left);
      item.floatTop = Math.round(rect.top);
      item.floatWidth = Math.max(220, Math.round(rect.width));
      item.floatHeight = Math.max(160, Math.round(rect.height));
      item.floatZ = ++this.floatZCounter;
    },
    dockFloat(item) {
      if (item) item.isFloating = false;
    },
    bringFloatToFront(id) {
      const item = this.layout.find((l) => l.i === id);
      if (item && item.isFloating) {
        item.floatZ = ++this.floatZCounter;
      }
    },
    addNewItem: function () {
      this.layout.push({
        x: (this.layout.length * 2) % 12,
        y: this.layout.length + 12,
        w: 6,
        h: 6,
        i: this.index,
        static: false,
        draggable: true,
        resizable: true,
        appletData: { appletName: 'CreateNew'},
        extra: {title: this.index},
        isFloating: false,
        floatLeft: null,
        floatTop: null,
        floatWidth: null,
        floatHeight: null,
        floatZ: null,
      });
      this.index++;
    },
    addItemCustom: function (x, y, w, h, isStatic, draggable, resizable, appletData, extra, floatOpts) {
      floatOpts = floatOpts || {};
      const isFloating = !!floatOpts.isFloating;
      const entry = {
        x: x,
        y: y,
        w: w,
        h: h,
        i: this.index,
        static: isStatic,
        draggable: draggable,
        resizable: resizable,
        appletData: appletData,
        extra: extra || {},
        isFloating: isFloating,
        floatLeft: floatOpts.floatLeft != null ? floatOpts.floatLeft : null,
        floatTop: floatOpts.floatTop != null ? floatOpts.floatTop : null,
        floatWidth: floatOpts.floatWidth != null ? floatOpts.floatWidth : null,
        floatHeight: floatOpts.floatHeight != null ? floatOpts.floatHeight : null,
        floatZ: floatOpts.floatZ != null ? floatOpts.floatZ : null,
      };
      if (isFloating) {
        if (entry.floatLeft == null) entry.floatLeft = 80;
        if (entry.floatTop == null) entry.floatTop = 80;
        if (entry.floatWidth == null) entry.floatWidth = 480;
        if (entry.floatHeight == null) entry.floatHeight = 360;
        if (entry.floatZ == null) entry.floatZ = ++this.floatZCounter;
      }
      this.layout.push(entry);
      this.index++;
    },
    removeItem: function (val) {
      if (this.layout.length != 0) {
        const index = this.layout.map(item => item.i).indexOf(val);
        if (index >= 0) {
          this.layout.splice(index, 1);
        }
      }
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
    appletPayloadFromItem(item) {
      const applet = {
        name: item.i.toString(),
        position_x: item.x,
        position_y: item.y,
        width: item.w,
        height: item.h,
        editable: !item.static,
        applet_data: item.appletData,
        is_floating: !!item.isFloating,
        float_left: item.isFloating && item.floatLeft != null ? item.floatLeft : null,
        float_top: item.isFloating && item.floatTop != null ? item.floatTop : null,
        float_width: item.isFloating && item.floatWidth != null ? item.floatWidth : null,
        float_height: item.isFloating && item.floatHeight != null ? item.floatHeight : null,
        float_z: item.isFloating && item.floatZ != null ? item.floatZ : null,
      };
      return applet;
    },
    saveWorkspaceLayout(name="workspace"){
      const self = this;
      let applets_layout = []
      for (let i=0;i < this.layout.length;i++) {
        applets_layout.push(this.appletPayloadFromItem(this.layout[i]));
      }

      const appearance = appearancePayloadForApi(useWorkspaceAppearanceStore().$state);
      axios({
        method: 'post',
        url: '/workspaces/api/create',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${useUserStore().token}`,
        },
        data: {
          name: name || "Workspace",
          display_order: 0,
          applets: applets_layout,
          ...appearance,
        }
      }).then((response) => {
        createNotification(null, name + " Workspace Saved", "", "primary")
      }).catch(function (error) {
        createNotification(null, "Error while save workspace", error?.response?.data ?? error, "danger")
      });
    },
    updateWorkspaceLayout(id, name="workspace"){
      const self = this;
      let applets_layout = []
      for (let i=0;i < this.layout.length;i++) {
        applets_layout.push(this.appletPayloadFromItem(this.layout[i]));
      }

      const appearance = appearancePayloadForApi(useWorkspaceAppearanceStore().$state);
      axios({
        method: 'post',
        url: '/workspaces/api/update',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${useUserStore().token}`,
        },
        data: {
          id: id,
          name: name,
          display_order: 0,
          applets: applets_layout,
          ...appearance,
        }
      }).then((response) => {
        createNotification(null, name + " Workspace Updated", "", "primary")
      }).catch(function (error) {
        createNotification(null, "Error while updating workspace", error?.response?.data ?? error, "danger")
      });
    },
    setWorkspaceLayout(id) {
      const self = this;
      this.draggable = true;
      this.resizable = true;
      this.gridlock = false;
      this.layout = []
      this.index = 0
      axios({
        method: 'get',
        url: '/workspaces/api/' + id,
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${useUserStore().token}`,
        },
      }).then((response) => {
        for (let i=0;i < response.data.length;i++) {
          const r = response.data[i];
          this.addItemCustom(
              r.position_x,
              r.position_y,
              r.width,
              r.height,
              !r.editable,
              r.editable,
              r.editable,
              r.applet_data,
              {},
              {
                isFloating: r.is_floating,
                floatLeft: r.float_left,
                floatTop: r.float_top,
                floatWidth: r.float_width,
                floatHeight: r.float_height,
                floatZ: r.float_z,
              }
          )
        }
        this.$emit("setWorkspace");
        this.syncWorkspaceContextStore();
      }).catch(function (error) {
        createNotification(null, "Error while loading workspace", error?.response ?? error, "danger")
      });
    },
  }
};
</script>

<style lang="scss" scoped>
.grid-layout-container {
  background-color: var(--bg);
  background-image: var(--workspace-bg-image, none);
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  position: relative;
}

.vue-grid-layout {
  background: transparent;
}
</style>
