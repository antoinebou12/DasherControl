<template>
  <div class="shortcut-setting center content-inputs">
    <div class="shortcut-setting__stack d-flex flex-column ga-2">
      <v-text-field v-model="link" data-testid="shortcut-link" label="link" variant="outlined" density="compact" hide-details />
      <v-text-field
        v-model="img_link"
        data-testid="shortcut-image-url"
        label="image URL"
        variant="outlined"
        density="compact"
        hide-details
      />
      <v-text-field v-model="title" data-testid="shortcut-title" label="title" variant="outlined" density="compact" hide-details />
      <v-textarea
        v-model="description"
        data-testid="shortcut-description"
        label="description"
        variant="outlined"
        density="compact"
        rows="4"
        auto-grow
        hide-details
        class="shortcut-setting__desc"
      />
    </div>
  </div>
</template>

<script>
export default {
  name: "ShortcutSetting",
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
