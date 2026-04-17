<template>
  <div class="grid-edit-menu-container">
    <WorkspaceSelector
      ref="workspaceSelector"
      @selected="workspaceSelector"
      @update-name="updateWorkspace"
    />
    <div class="grid-edit-menu">
      <n-button quaternary circle @click="gridLayout.addNewItem()"><i class="bx bx-add-to-queue" /></n-button>
      <n-button quaternary circle @click="gridLayout.removeItem()"><i class="bx bx-x" /></n-button>
      <n-button quaternary circle @click="gridLayout.lockGridLayout()"><i class="bx bx-lock-alt" /></n-button>
      <n-button quaternary circle @click="gridLayout.unlockGridLayout()"><i class="bx bx-lock-open-alt" /></n-button>
      <n-button quaternary circle @click="saveWorkspace"><i class="bx bx-save" /></n-button>
      <n-button quaternary circle @click="updateWorkspace"><i class="bx bx-export" /></n-button>
    </div>
  </div>
</template>

<script>
import { NButton } from "naive-ui";
import WorkspaceSelector from "./WorkspaceSelector.vue";
import { useWorkspaceAppearanceStore } from "../stores/workspaceAppearance";

export default {
  name: "GridEditMenu",
  components: {
    WorkspaceSelector,
    NButton,
  },
  props: {
    gridLayout: { type: Object, required: true },
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
.grid-edit-menu {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin: 8px 0 16px;
}
</style>
