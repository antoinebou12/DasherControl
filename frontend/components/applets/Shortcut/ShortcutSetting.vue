<template>
  <div class="shortcut-setting center content-inputs">
    <n-space vertical :size="9" class="shortcut-setting__stack">
      <n-input v-model:value="link" data-testid="shortcut-link" placeholder="link" size="small" round />
      <n-input v-model:value="img_link" data-testid="shortcut-image-url" placeholder="image URL" size="small" round />
      <n-input v-model:value="title" data-testid="shortcut-title" placeholder="title" size="small" round />
      <n-input
        v-model:value="description"
        data-testid="shortcut-description"
        placeholder="description"
        type="textarea"
        round
        :autosize="{ minRows: 4, maxRows: 10 }"
        class="shortcut-setting__desc"
      />
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

<style scoped lang="scss">
.shortcut-setting.content-inputs {
  padding: 4px 4px 0;
  width: 100%;
}

.shortcut-setting__stack {
  width: 100%;
  max-width: 420px;
}

.shortcut-setting__desc {
  min-height: 96px;
}
</style>
