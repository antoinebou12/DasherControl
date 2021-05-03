<template>
  <div class="grid-edit-menu-container">
    <WorkspaceSelector ref="workspaceSelector" />
    <div class="grid-edit-menu">
      <vs-button @click="gridLayout.addNewItem()"><i class="bx bx-add-to-queue"/></vs-button>
      <vs-button @click="gridLayout.removeItem()"><i class="bx bx-x"/></vs-button>
      <vs-button @click="gridLayout.lockGridLayout()"><i class="bx bx-lock-alt"/></vs-button>
      <vs-button @click="gridLayout.unlockGridLayout()"><i class="bx bx-lock-open-alt"/></vs-button>
      <vs-button @click="saveWorkspace()"><i class="bx bx-save"/></vs-button>
      <vs-button @click="workspaceSelector()"><i class="bx bx-download"/></vs-button>
<!--    <vs-sidebar v-model="sidebarActive">-->
<!--      <vs-sidebar-item @click="gridLayout.addNewItem()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-add-to-queue"/>-->
<!--        </template>-->
<!--        Add-->
<!--      </vs-sidebar-item>-->
<!--      <vs-sidebar-item @click="gridLayout.removeItem()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-x"/>-->
<!--        </template>-->
<!--        Remove-->
<!--      </vs-sidebar-item>-->
<!--      <vs-sidebar-item @click="gridLayout.lockGridLayout()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-lock-alt"/>-->
<!--        </template>-->
<!--        Lock-->
<!--      </vs-sidebar-item>-->
<!--      <vs-sidebar-item @click="gridLayout.unlockGridLayout()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-lock-open-alt"/>-->
<!--        </template>-->
<!--        Unlock-->
<!--      </vs-sidebar-item>-->
<!--      <vs-sidebar-item @click="workspaceSelector()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-save"/>-->
<!--        </template>-->
<!--        Load-->
<!--      </vs-sidebar-item>-->
<!--      <vs-sidebar-item @click="saveWorkspace()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-download"/>-->
<!--        </template>-->
<!--        Save-->
<!--      </vs-sidebar-item>-->
<!--    </vs-sidebar>-->
    </div>
  </div>
</template>

<script>
import WorkspaceSelector from "./WorkspaceSelector.vue"

export default {
  name: "GridEditMenu",
  components: {
    WorkspaceSelector
  },
  props: {
    gridLayout: Object
  },
  data: () => ({
    sidebarActive: true
  }),
  methods: {
    workspaceSelector() {
      this.gridLayout.setWorkspaceLayout(this.$refs.workspaceSelector.selected.id);
      this.gridLayout.workspace_id = this.$refs.workspaceSelector.selected.id;
    },
    saveWorkspace(){
      this.gridLayout.saveWorkspaceLayout();
      this.$refs.workspaceSelector.get_workspaces();
      this.$refs.workspaceSelector.isVisible = true;
      this.$refs.workspaceSelector.$forceUpdate();
    }
  }
}
</script>


<style lang="scss" scoped>
.grid-edit-menu {
  display: inline-flex;
  margin-top: 36px;
  margin-bottom: 24px;

  button {
    border-radius: 0px;
  }
}
</style>