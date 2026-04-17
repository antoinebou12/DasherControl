<template>
  <div class="center content-inputs">
    <n-input v-model:value="src" placeholder="URL" style="max-width: 420px; width: 100%" />
  </div>
</template>

<script>
import { NInput } from "naive-ui";

export default {
  name: "IFrameSetting",
  components: { NInput },
  data() {
    return {
      src: "",
    };
  },
};
</script>

<style scoped>
.content-inputs {
  padding: 8px;
}
</style>
