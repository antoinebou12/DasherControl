<template>
  <div class="grid-edit-menu" style="">
    <vs-button @click="gridLayout.addItem()"><i class="bx bx-add-to-queue"/></vs-button>
    <vs-button @click="gridLayout.removeItem()"><i class="bx bx-x"/></vs-button>
    <vs-button @click="gridLayout.lockGridLayout()"><i class="bx bx-lock-alt"/></vs-button>
    <vs-button @click="gridLayout.unlockGridLayout()"><i class="bx bx-lock-open-alt"/></vs-button>
  </div>
</template>

<script>
export default {
  name: "GridEditMenu",
  props: {
    gridLayout: Object
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