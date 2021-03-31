<template>
  <div class="iframe-container">
    <iframe
        class="borderless-iframe"
        :src="src"
        loading="lazy"
        :scrolling="scrolling"
        frameborder="0"
        marginheight="0"
        marginwidth="0"
        sandbox="allow-forms allow-same-origin allow-scripts"
    />
  </div>
</template>
<script>
export default {
  name: "BorderlessIFrame",
  props: {
    src: String,
  },
  data: () => ({
    scrolling: "no"
  }),
};
</script>

<style lang="scss" scoped>
.iframe-container {
  width: 100%;
  height: 100%;
}
.borderless-iframe {
  width: 100%;
  height: 100%;
}
</style>
