<template>
  <div class="grid-edit-menu-container">
    <WorkspaceSelector
      ref="workspaceSelector"
      @selected="workspaceSelector"
      @update-name="updateWorkspace"
    />
    <div v-if="token !== ''" class="grid-edit-menu" role="toolbar" aria-label="Workspace layout">
      <v-btn
        color="primary"
        icon
        title="Add applet"
        aria-label="Add applet to layout"
        class="grid-edit-menu__btn"
        @click="gridLayout.addNewItem()"
      >
        <i class="bx bx-add-to-queue grid-edit-menu__icon" aria-hidden="true" />
      </v-btn>
      <v-btn
        color="primary"
        icon
        title="Remove selected"
        aria-label="Remove selected applet"
        class="grid-edit-menu__btn"
        @click="gridLayout.removeItem()"
      >
        <i class="bx bx-x grid-edit-menu__icon" aria-hidden="true" />
      </v-btn>
      <v-btn
        color="primary"
        icon
        title="Lock layout"
        aria-label="Lock layout"
        class="grid-edit-menu__btn"
        @click="gridLayout.lockGridLayout()"
      >
        <i class="bx bx-lock-alt grid-edit-menu__icon" aria-hidden="true" />
      </v-btn>
      <v-btn
        color="primary"
        icon
        title="Unlock layout"
        aria-label="Unlock layout"
        class="grid-edit-menu__btn"
        @click="gridLayout.unlockGridLayout()"
      >
        <i class="bx bx-lock-open-alt grid-edit-menu__icon" aria-hidden="true" />
      </v-btn>
      <v-btn
        color="primary"
        icon
        title="Save workspace"
        aria-label="Save workspace"
        class="grid-edit-menu__btn"
        @click="saveWorkspace"
      >
        <i class="bx bx-save grid-edit-menu__icon" aria-hidden="true" />
      </v-btn>
      <v-btn
        color="primary"
        icon
        title="Update / export workspace"
        aria-label="Update or export workspace"
        class="grid-edit-menu__btn"
        @click="updateWorkspace"
      >
        <i class="bx bx-export grid-edit-menu__icon" aria-hidden="true" />
      </v-btn>
    </div>
  </div>
</template>

<script>
import { mapState } from "pinia";
import WorkspaceSelector from "./WorkspaceSelector.vue";
import { useWorkspaceAppearanceStore } from "../stores/workspaceAppearance";
import { useUserStore } from "../stores/user";

export default {
  name: "GridEditMenu",
  components: {
    WorkspaceSelector,
  },
  props: {
    gridLayout: { type: Object, required: true },
  },
  computed: {
    ...mapState(useUserStore, ["token"]),
  },
  data: () => ({
    sidebarActive: true,
  }),
  methods: {
    workspaceSelector() {
      const ws = this.$refs.workspaceSelector;
      const row = ws.selected;
      if (row && row.id != null && row.id !== -1) {
        useWorkspaceAppearanceStore().hydrateFromApiRow(row);
      }
      this.gridLayout.setWorkspaceLayout(ws.selected.id);
      this.gridLayout.workspace_id = ws.selected.id;
      this.gridLayout.workspace_name = row && row.name != null ? String(row.name) : "";
    },
    saveWorkspace() {
      const ws = this.$refs.workspaceSelector;
      this.gridLayout.saveWorkspaceLayout(ws.workspaceName);
      ws.get_workspaces();
    },
    updateWorkspace() {
      const ws = this.$refs.workspaceSelector;
      this.gridLayout.updateWorkspaceLayout(ws.selected.id, ws.workspaceName);
      ws.get_workspaces();
    },
  },
};
</script>

<style lang="scss" scoped>
.grid-edit-menu-container {
  margin: 0 16px;
}

.grid-edit-menu {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px;
  margin: 8px 0 20px;
  padding: 12px 14px;
  border-radius: 8px;
  border: 1px solid var(--ui-chrome-border);
  background: var(--ui-chrome-bg);
  color: var(--ui-chrome-fg);
  box-shadow: 0 4px 18px rgba(0, 0, 0, 0.32);
}

.grid-edit-menu__icon {
  font-size: 20px;
  line-height: 1;
  color: #ffffff;
}

.grid-edit-menu__btn {
  width: 40px;
  height: 40px;
}
</style>
