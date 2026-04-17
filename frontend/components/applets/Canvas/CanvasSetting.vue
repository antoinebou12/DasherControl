<template>
  <div class="canvas-setting">
    <n-form-item label="Mode" label-placement="left">
      <n-select
        v-model:value="canvasMode"
        :options="modeOptions"
        style="min-width: 200px"
      />
    </n-form-item>
  </div>
</template>

<script>
import { NFormItem, NSelect } from "naive-ui";

export default {
  name: "CanvasSetting",
  components: { NFormItem, NSelect },
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
