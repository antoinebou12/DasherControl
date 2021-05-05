<template>
  <div class="iframe-container">
    <iframe
        ref="iframe"
        class="borderless-iframe"
        :src="link"
        loading="lazy"
        :scrolling="scrolling"
        frameborder="0"
        marginheight="0"
        marginwidth="0"
        allowtransparency="true"
        sandbox="allow-forms allow-same-origin allow-scripts"
        allow="picture-in-picture; fullscreen; encrypted-media; autoplay"
    />
  </div>
</template>
<script>
export default {
  name: "BorderlessIFrame",
  props: {
    src: String,
  },
  data() {
    return {
      scrolling: "yes",
      link: this.src,
    }
  },
  methods:{
    openNewTab() {
      window.open(this.link, '_blank').focus()
    },
    reload() {
      this.$refs.iframe.src += '';
    }
  }
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

/* Turn on custom 8px wide scrollbar */
::-webkit-scrollbar {
  width: 8px; /* 1px wider than Lion. */
  /* This is more usable for users trying to click it. */
  background-color: rgba(0, 0, 0, 0);
  -webkit-border-radius: 100px;
}

/* hover effect for both scrollbar area, and scrollbar 'thumb' */
::-webkit-scrollbar:hover {
  background-color: rgba(0, 0, 0, 0.09);
}

/* The scrollbar 'thumb' ...that marque oval shape in a scrollbar */
::-webkit-scrollbar-thumb:vertical {
  /* This is the EXACT color of Mac OS scrollbars.
     Yes, I pulled out digital color meter */
  background: rgba(0, 0, 0, 0.5);
  -webkit-border-radius: 100px;
}

::-webkit-scrollbar-thumb:vertical:active {
  background: rgba(0, 0, 0, 0.61); /* Some darker color when you click it */
  -webkit-border-radius: 100px;
}
</style>
