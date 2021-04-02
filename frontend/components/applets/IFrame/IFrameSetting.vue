<template>
  <div class="center content-inputs">
    <vs-input ref="src" v-model="src" placeholder="link"></vs-input>
  </div>
</template>

<script>
export default {
  name: "IFrameSetting",
  data() {
    return {
      src: ''
    }
  },
  methods: {
    get_src(){
      return this.$refs.src.value;
    }
  }

}
</script>

<style scoped>

</style>