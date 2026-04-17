<template>
  <div class="center content-inputs">
    <n-space vertical style="width: 100%; max-width: 420px">
      <n-input v-model:value="link" placeholder="link" />
      <n-input v-model:value="img_link" placeholder="image URL" />
      <n-input v-model:value="title" placeholder="title" />
      <n-input v-model:value="description" placeholder="description" type="textarea" />
    </n-space>
  </div>
</template>

<script>
import { NInput, NSpace } from "naive-ui";

export default {
  name: "ShortcutSetting",
  components: { NInput, NSpace },
  data() {
    return {
      link: "",
      img_link: "",
      title: "",
      description: "",
    };
  },
};
</script>

<style scoped>
.content-inputs {
  padding: 8px;
}
</style>
