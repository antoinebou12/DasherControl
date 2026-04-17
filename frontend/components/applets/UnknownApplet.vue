<template>
  <div class="unknown-applet">
    <p class="unknown-applet__title">Unknown applet</p>
    <p class="unknown-applet__id">{{ unknownAppletId }}</p>
    <p class="unknown-applet__hint">This type is not registered. It may have been removed or the workspace was saved with a newer version.</p>
  </div>
</template>

<script>
export default {
  name: "UnknownApplet",
  props: {
    unknownAppletId: {
      type: String,
      default: "",
    },
  },
};
</script>

<style scoped>
.unknown-applet {
  padding: 1rem;
  color: var(--fg, #333);
  background: var(--darcule-bg, #f5f5f5);
  height: 100%;
  box-sizing: border-box;
  font-size: 14px;
}

.unknown-applet__title {
  font-weight: 600;
  margin: 0 0 0.5rem;
}

.unknown-applet__id {
  font-family: monospace;
  margin: 0 0 0.75rem;
}

.unknown-applet__hint {
  margin: 0;
  opacity: 0.85;
}
</style>
