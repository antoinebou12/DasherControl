<template>
  <div class="main">
    <h1>{{ msg }}</h1>
  </div>
</template>

<script>
export default {
  name: "Index",
  props: {
    msg: String
  }
};
</script>
