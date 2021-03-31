<template>
  <div class="shortcut-container">
  <a :href="link">
  <vs-card>
    <template #title>
      <h3>{{ title }}</h3>
    </template>
    <template #img>
      <img :src="img_link" alt="">
    </template>
    <template #text>
      <p>
        {{ description }}
      </p>
    </template>
  </vs-card>
  </a>
  </div>
</template>

<script>
export default {
  name: "Shortcut",
  props: {
    link: {type: String, default: "https://source.unsplash.com/random"},
    img_link: {type: String, default: "https://source.unsplash.com/random"},
    title: {type: String, default: "Shortcut"},
    description: {type: String, default: ""}
  }
}
</script>

<style lang="scss" >
  .shortcut-container {
    width: 100%;
    height: 100%;
    overflow: hidden;

    .vs-card {
      border-radius: 0 !important;
      max-width: none !important;
      --webkit-transform: unset !important;
      transform: unset !important;
      height: 600px;

      img {
        --webkit-transform: unset !important;
        transform: unset !important;
      }
    }

  }
</style>