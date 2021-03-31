<template>
  <div class="grid-edit-menu" style="">
    <vs-button @click="gridLayout.addItem()"><i class="bx bx-add-to-queue"/></vs-button>
    <vs-button @click="gridLayout.removeItem()"><i class="bx bx-x"/></vs-button>
    <vs-button @click="gridLayout.lockGridLayout()"><i class="bx bx-lock-alt"/></vs-button>
    <vs-button @click="gridLayout.unlockGridLayout()"><i class="bx bx-lock-open-alt"/></vs-button>
<!--    <vs-button @click="gridLayout.saveGridlayout()"><i class="bx bx-save"/></vs-button>-->
<!--    <vs-sidebar absolute hover-expend reduce v-model="sidebarActive">-->
<!--      <vs-sidebar-item @click="gridLayout.addItem()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-add-to-queue"/>-->
<!--        </template>-->
<!--        Add Bloc-->
<!--      </vs-sidebar-item>-->
<!--      <vs-sidebar-item @click="gridLayout.removeItem()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-x"/>-->
<!--        </template>-->
<!--        Remove Bloc-->
<!--      </vs-sidebar-item>-->
<!--      <vs-sidebar-item @click="gridLayout.lockGridLayout()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-lock-alt"/>-->
<!--        </template>-->
<!--        Lock Grid-->
<!--      </vs-sidebar-item>-->
<!--      <vs-sidebar-item @click="gridLayout.unlockGridLayout()">-->
<!--        <template #icon>-->
<!--          <i class="bx bx-lock-open-alt"/>-->
<!--        </template>-->
<!--        Unlock Grid-->
<!--      </vs-sidebar-item>-->
<!--    </vs-sidebar>-->
  </div>
</template>

<script>

export default {
  name: "GridEditMenu",
  props: {
    gridLayout: Object
  },
  data: () => ({
    sidebarActive: ''
  })
}
</script>


<style lang="scss" scoped>
.grid-edit-menu {
  display: inline-flex;
  margin-top: 36px;
  margin-bottom: 24px;
}
</style>