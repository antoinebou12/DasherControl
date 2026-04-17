<template>
  <div class="shortcut-container">
    <a :href="link" class="shortcut-link">
      <v-card class="shortcut-card" flat>
        <v-card-title class="py-2">
          <h3 class="text-h6">{{ title }}</h3>
        </v-card-title>
        <v-img :src="img_link" alt="" cover height="320" class="shortcut-cover" />
        <v-card-text>
          <p class="shortcut-desc">{{ description }}</p>
        </v-card-text>
      </v-card>
    </a>
  </div>
</template>

<script>
export default {
  name: "Shortcut",
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

    .shortcut-cover {
      border-radius: 0;
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
