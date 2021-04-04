<template>
  <div class="terminal-container">
    <div ref="terminal" id="terminal-content"></div>
  </div>
</template>

<script>
import { Terminal } from 'xterm';
import { AttachAddon } from 'xterm-addon-attach'
import { FitAddon } from 'xterm-addon-fit'
import { WebLinksAddon } from 'xterm-addon-web-links'
// import { SearchAddon } from 'xterm-addon-search'
export default {
  name: "Terminal",
  props: {
    username: '',
    password: '',
  },
  data() {
    return {
      terminal: new Terminal({ cursorBlink: true }),
    }
  },
  methods: {
    loadAddon() {
      const webLinksAddon = new WebLinksAddon();
      const fitAddon = new FitAddon();
      const attachAddon = new AttachAddon();
      this.terminal.loadAddon(webLinksAddon);
      this.terminal.loadAddon(fitAddon);
      this.terminal.loadAddon(attachAddon);
    },
    loadTerminal() {
      this.terminal.open(this.$refs.terminal)
      this.terminal.fit();

    }
  }
}
</script>

<style scoped>

</style>