<template>
  <div class="grid-edit-menu" style="">
    <vs-button @click="gridLayout.addItem()"><i class="bx bx-add-to-queue"/></vs-button>
    <vs-button @click="gridLayout.removeItem()"><i class="bx bx-x"/></vs-button>
    <vs-button @click="gridLayout.lockGridLayout()"><i class="bx bx-lock-alt"/></vs-button>
    <vs-button @click="gridLayout.unlockGridLayout()"><i class="bx bx-lock-open-alt"/></vs-button>
    <vs-button @click="saveWorkspaceLayout()"><i class="bx bx-save"/></vs-button>
<!--    <vs-button @click="gridLayout.saveGridlayout()"><i class="bx bx-save"/></vs-button>-->
<!--    <vs-sidebar absolute hover-expend reduce v-model="sidebarActive">-->
<!--      <vs-sidebar-this.gridLayout.layout[i] @click="gridLayout.addthis.gridLayout.layout[i]()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-add-to-queue"/>-->
<!--        </template>-->
<!--        Add Bloc-->
<!--      </vs-sidebar-this.gridLayout.layout[i]>-->
<!--      <vs-sidebar-this.gridLayout.layout[i] @click="gridLayout.removethis.gridLayout.layout[i]()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-x"/>-->
<!--        </template>-->
<!--        Remove Bloc-->
<!--      </vs-sidebar-this.gridLayout.layout[i]>-->
<!--      <vs-sidebar-this.gridLayout.layout[i] @click="gridLayout.lockGridLayout()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-lock-alt"/>-->
<!--        </template>-->
<!--        Lock Grid-->
<!--      </vs-sidebar-this.gridLayout.layout[i]>-->
<!--      <vs-sidebar-this.gridLayout.layout[i] @click="gridLayout.unlockGridLayout()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-lock-open-alt"/>-->
<!--        </template>-->
<!--        Unlock Grid-->
<!--      </vs-sidebar-this.gridLayout.layout[i]>-->
<!--    </vs-sidebar>-->
  </div>
</template>

<script>
import axios from "axios";

export default {
  name: "GridEditMenu",
  props: {
    gridLayout: Object
  },
  data: () => ({
    sidebarActive: ''
  }),
  methods: {
    saveWorkspaceLayout(){
      let applets_layout = []
      for (let i=0;i < this.gridLayout.layout.length;i++){
        let applet = {}
        applet.name = this.gridLayout.layout[i].i
        applet.position_x = this.gridLayout.layout[i].x
        applet.position_y = this.gridLayout.layout[i].y
        applet.width = this.gridLayout.layout[i].w
        applet.height = this.gridLayout.layout[i].y
        applet.editable = this.gridLayout.layout[i].static
        applet.applet_data = this.gridLayout.layout[i].appletData.toString()
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
    }
  }
}
</script>


<style lang="scss" scoped>
.grid-edit-menu {
  display: inline-flex;
  margin-top: 36px;
  margin-bottom: 24px;
}
</style>