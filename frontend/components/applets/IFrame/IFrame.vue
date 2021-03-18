<template>
  <div class="iframe-container">
    <iframe
      :src="src"
      :width="width"
      :height="height"
      :scrolling="scrolling"
      frameborder="0"
      marginheight="0"
      marginwidth="0"
    />
  </div>
</template>
<script>
export default {
  name: "IFrame",
  components: {},
  props: {
    name: String
  },
  data() {
    return {
      src:
        "https://icatcare.org/app/uploads/2018/07/Thinking-of-getting-a-cat.png",
      width: 400,
      height: 400,
      scrolling: "no"
    };
  },
};
</script>

<style lang="scss" scoped></style>
