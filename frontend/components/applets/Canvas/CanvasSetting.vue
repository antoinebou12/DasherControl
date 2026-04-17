<template>
  <div class="canvas-setting d-flex align-center ga-3 flex-wrap">
    <span class="text-body-2 text-medium-emphasis">Mode</span>
    <v-select
      v-model="canvasMode"
      :items="modeOptions"
      item-title="label"
      item-value="value"
      variant="outlined"
      density="compact"
      hide-details
      min-width="200"
    />
  </div>
</template>

<script>
export default {
  name: "CanvasSetting",
  data() {
    return {
      canvasMode: "three",
      modeOptions: [
        { label: "2D canvas", value: "canvas2d" },
        { label: "Three.js (WebGL)", value: "three" },
      ],
    };
  },
};
</script>

<style scoped>
.canvas-setting {
  min-width: 280px;
}
</style>
