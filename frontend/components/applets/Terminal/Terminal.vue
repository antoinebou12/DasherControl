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
import { SearchAddon } from 'xterm-addon-search'
export default {
  name: "Terminal",
  props: {
    username: '',
    password: '',
  },
  data() {
    return {
      terminal: null,
      websocket: null
    }
  },
  mounted() {
    this.terminal = new Terminal({ cursorBlink: true })
    this.connectTerminal(this.username, this.password)
  },
  methods: {
    loadAddon() {
      const webLinksAddon = new WebLinksAddon();
      const fitAddon = new FitAddon();
      const searchAddon = new SearchAddon();
      this.terminal.loadAddon(webLinksAddon);
      this.terminal.loadAddon(fitAddon);
      fitAddon.fit()
      this.terminal.loadAddon(searchAddon);

    },
    loadTerminal() {
      this.terminal.open(this.$refs.terminal)
    },
    keyboardInput() {
      this.terminal.onData(e => {
        switch (e) {
          case '\r': // Enter
          case '\u0003': // Ctrl+C
            this.terminal.write('\r\n$');
            break;
          case '\u007F': // Backspace (DEL)
            if (this.terminal._core.buffer.x > 2) {
              this.terminal.write('\b \b');
            }
            break;
        }
      })
    },
    connectWebSocket(){
      this.websocket = new WebSocket('wss://10.0.0.250:9000')
      const attachAddon = new AttachAddon(this.websocket);
      this.terminal.loadAddon(attachAddon);
      this.websocket.onopen = (conn) => {
        this.terminal.writeln("Connected");
      }
      this.websocket.onmessage = (msg) => {
        console.log(msg)
        this.terminal.writeln(msg);
      }
    },
    connectTerminal(username, password) {
      this.loadTerminal()
      this.loadAddon()
      this.keyboardInput()
      this.connectWebSocket()
      this.terminal.focus()
    },
  }
}
</script>

<style lang="scss">
  #terminal-content {
    width:960px;
    height: 600px;
    display: inline-block;
    padding: 2px;


    .terminal {
      background-color: #111;
      color:#fafafa;
      padding: 2px;
    }
  }
</style>