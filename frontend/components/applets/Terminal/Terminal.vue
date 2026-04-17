<template>
  <div class="terminal-applet">
    <div class="connect-bar">
      <div class="fields">
        <label>
          Host
          <input v-model="host" type="text" autocomplete="off" placeholder="192.168.1.10" />
        </label>
        <label>
          Port
          <input v-model.number="port" type="number" min="1" max="65535" />
        </label>
        <label>
          User
          <input v-model="sshUser" type="text" autocomplete="username" />
        </label>
        <label>
          Password
          <input v-model="sshPassword" type="password" autocomplete="current-password" />
        </label>
      </div>
      <div class="actions">
        <button type="button" class="btn-connect" :disabled="!canConnect" @click="connect">
          Connect
        </button>
        <button type="button" class="btn-disconnect" :disabled="!connected" @click="disconnect">
          Disconnect
        </button>
      </div>
      <p class="hint">
        Uses a WebSocket to the app (<code>ws</code>/<code>wss</code> same as this page). Server-side SSH is
        <strong>Admin only</strong>. Prefer <code>wss</code> in production. Without
        <code>TERMINAL_SSH_ALLOWLIST</code>, only private/loopback addresses are allowed.
      </p>
    </div>
    <div ref="terminalEl" class="terminal-shell" />
  </div>
</template>

<script>
import { Terminal } from "@xterm/xterm";
import { AttachAddon } from "@xterm/addon-attach";
import { FitAddon } from "@xterm/addon-fit";
import "@xterm/xterm/css/xterm.css";

import { useUserStore } from "../../../stores/user";
import { createNotification } from "../../../utils";

export default {
  name: "Terminal",
  props: {
    username: { type: String, default: "" },
    password: { type: String, default: "" },
  },
  data() {
    return {
      host: "",
      port: 22,
      sshUser: this.username,
      sshPassword: this.password,
      term: null,
      fitAddon: null,
      wsAttach: null,
      websocket: null,
      connected: false,
      onResizeBound: null,
    };
  },
  watch: {
    username(v) {
      this.sshUser = v;
    },
    password(v) {
      this.sshPassword = v;
    },
  },
  computed: {
    canConnect() {
      return (
        !this.connected &&
        this.host.trim().length > 0 &&
        this.sshUser.length > 0 &&
        this.sshPassword.length > 0 &&
        this.port >= 1 &&
        this.port <= 65535
      );
    },
  },
  mounted() {
    this.initTerminal();
  },
  beforeUnmount() {
    if (this.onResizeBound) {
      window.removeEventListener("resize", this.onResizeBound);
    }
    this.disconnect();
    this.term?.dispose();
  },
  methods: {
    initTerminal() {
      this.term = new Terminal({ cursorBlink: true, fontFamily: "Consolas, monospace" });
      this.fitAddon = new FitAddon();
      this.term.loadAddon(this.fitAddon);
      this.term.open(this.$refs.terminalEl);
      this.onResizeBound = () => {
        this.fitAddon?.fit();
      };
      window.addEventListener("resize", this.onResizeBound);
      this.$nextTick(() => this.fitAddon?.fit());
    },
    disconnect() {
      if (this.wsAttach && this.term) {
        try {
          this.term.unloadAddon(this.wsAttach);
        } catch {
          /* ignore */
        }
      }
      this.wsAttach = null;
      try {
        this.websocket?.close();
      } catch {
        /* ignore */
      }
      this.websocket = null;
      this.connected = false;
    },
    connect() {
      const user = useUserStore();
      const token = user.token;
      if (!token) {
        createNotification(null, "Terminal", "Sign in required", "danger");
        return;
      }

      this.disconnect();

      const proto = window.location.protocol === "https:" ? "wss:" : "ws:";
      const url = `${proto}//${window.location.host}/terminal/ws?token=${encodeURIComponent(token)}`;
      const ws = new WebSocket(url);
      ws.binaryType = "arraybuffer";
      this.websocket = ws;

      ws.onopen = () => {
        const payload = {
          host: this.host.trim(),
          port: Number(this.port) || 22,
          username: this.sshUser,
          password: this.sshPassword,
          cols: this.term.cols,
          rows: this.term.rows,
        };
        ws.send(JSON.stringify(payload));
        this.wsAttach = new AttachAddon(ws);
        this.term.loadAddon(this.wsAttach);
        this.connected = true;
        this.fitAddon?.fit();
        this.term.focus();
      };

      ws.onerror = () => {
        createNotification(null, "Terminal", "WebSocket error", "danger");
      };

      ws.onclose = () => {
        this.connected = false;
        if (this.wsAttach && this.term) {
          try {
            this.term.unloadAddon(this.wsAttach);
          } catch {
            /* ignore */
          }
        }
        this.wsAttach = null;
        this.websocket = null;
      };
    },
  },
};
</script>

<style lang="scss" scoped>
.terminal-applet {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 280px;
  background: var(--bg, #1e1e1e);
  color: var(--fg, #eee);
}

.connect-bar {
  flex: 0 0 auto;
  padding: 8px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  font-size: 12px;
}

.fields {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 8px;

  label {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 120px;
  }

  input {
    padding: 4px 6px;
    border-radius: 4px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: rgba(0, 0, 0, 0.25);
    color: inherit;
  }
}

.actions {
  display: flex;
  gap: 8px;
  margin-bottom: 6px;

  button {
    padding: 4px 12px;
    border-radius: 4px;
    border: none;
    cursor: pointer;
    font-size: 12px;
  }

  .btn-connect {
    background: #3c8dbc;
    color: #fff;
  }

  .btn-connect:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-disconnect {
    background: rgba(255, 255, 255, 0.12);
    color: inherit;
  }
}

.hint {
  margin: 0;
  opacity: 0.75;
  line-height: 1.35;
  max-width: 720px;
}

.terminal-shell {
  flex: 1 1 auto;
  min-height: 200px;
  padding: 4px;
  overflow: hidden;
}

:deep(.terminal) {
  height: 100%;
  background-color: #111;
  color: #fafafa;
}
</style>
