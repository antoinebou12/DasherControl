<template>
  <div class="iframe-container">
    <iframe
      :src="src"
      :width="width"
      :height="height"
      loading="lazy"
      :scrolling="scrolling"
      frameborder="0"
      marginheight="0"
      marginwidth="0"
      sandbox="allow-forms allow-same-origin"
    />
  </div>
</template>
<script>
export default {
  name: "IFrame",
  components: {},
  props: {
    name: String,
    src: String,
    width: Number,
    height: Number,
  },
  data() {
    return {
      width: 800,
      height: 800,
      scrolling: "no"
    };
  },
};
</script>

<style lang="scss" scoped>

</style>
