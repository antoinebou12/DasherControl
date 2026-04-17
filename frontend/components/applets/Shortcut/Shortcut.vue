<template>
  <div class="shortcut-container">
    <a :href="link" class="shortcut-link">
      <n-card class="shortcut-card" :bordered="false">
        <template #header>
          <h3>{{ title }}</h3>
        </template>
        <template #cover>
          <img :src="img_link" alt="" class="shortcut-cover" />
        </template>
        <p class="shortcut-desc">{{ description }}</p>
      </n-card>
    </a>
  </div>
</template>

<script>
import { NCard } from "naive-ui";

export default {
  name: "Shortcut",
  components: { NCard },
  props: {
    link: { type: String, default: "" },
    img_link: { type: String, default: "https://source.unsplash.com/random" },
    title: { type: String, default: "Shortcut" },
    description: { type: String, default: "" },
  },
};
</script>

<style lang="scss">
.shortcut-container {
  width: 100%;
  height: 100%;
  overflow: hidden;

  .shortcut-link {
    text-decoration: none;
    color: inherit;
    display: block;
    height: 100%;
  }

  .shortcut-card {
    border-radius: 0 !important;
    max-width: none !important;
    transform: unset !important;
    height: 600px;

    :deep(.n-card-cover) {
      border-radius: 0;
    }

    .shortcut-cover {
      transform: unset !important;
      border-radius: 0;
      width: 100%;
      object-fit: cover;
      max-height: 320px;
    }

    h3 {
      margin: 0;
    }

    .shortcut-desc {
      margin: 0.5rem 0 0;
    }
  }
}
</style>
