<template>
  <div class="center content-inputs">
    <v-text-field v-model="src" label="URL" variant="outlined" density="comfortable" hide-details class="iframe-url-field" />
  </div>
</template>

<script>
export default {
  name: "IFrameSetting",
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
.iframe-url-field {
  max-width: 420px;
  width: 100%;
}
</style>
