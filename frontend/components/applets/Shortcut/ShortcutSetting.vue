<template>
  <div class="center content-inputs">
    <vs-input ref="src" v-model="link" placeholder="link"></vs-input>
    <vs-input ref="img_link" v-model="img_link" placeholder="image"></vs-input>
    <vs-input ref="title" v-model="title" placeholder="title"></vs-input>
    <vs-input ref="description" v-model="description" placeholder="description"></vs-input>
  </div>
</template>

<script>
export default {
  name: "ShortcutSetting",
  data() {
    return {
      link: '',
      img_link: '',
      title: '',
      description: ''

    }
  },
  methods: {
  }
}
</script>

<style scoped>

</style>