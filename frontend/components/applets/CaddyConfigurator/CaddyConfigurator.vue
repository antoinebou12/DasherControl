<template>
  <div class="caddy-configurator">
    <n-grid :cols="2" :x-gap="12" :y-gap="12">
      <n-gi>
        <n-form-item label="Hostname">
          <n-input v-model:value="hostname" placeholder="app.example.com" />
        </n-form-item>
      </n-gi>
      <n-gi>
        <n-form-item label="Upstream">
          <n-input v-model:value="upstream" placeholder="127.0.0.1:8080" />
        </n-form-item>
      </n-gi>
      <n-gi>
        <n-form-item label="TLS email (optional)">
          <n-input v-model:value="tlsEmail" placeholder="you@example.com" />
        </n-form-item>
      </n-gi>
      <n-gi>
        <n-form-item label="Path prefix (optional)">
          <n-input v-model:value="pathPrefix" placeholder="api" />
        </n-form-item>
      </n-gi>
    </n-grid>
    <n-space align="center" class="row-actions">
      <n-checkbox v-model:checked="encode">encode zstd gzip</n-checkbox>
      <n-button type="primary" :loading="loading" @click="generate">Generate</n-button>
      <n-button quaternary :disabled="!output" @click="copyOutput">Copy</n-button>
    </n-space>
    <textarea v-model="output" class="caddy-output" rows="12" placeholder="Caddyfile output…" readonly />
    <p v-if="error" class="error">{{ error }}</p>
  </div>
</template>

<script>
import axios from "axios";
import {
  NButton,
  NCheckbox,
  NFormItem,
  NGi,
  NGrid,
  NInput,
  NSpace,
} from "naive-ui";
import { useUserStore } from "../../../stores/user";
import { createNotification } from "../../../utils";

export default {
  name: "CaddyConfigurator",
  components: { NButton, NCheckbox, NFormItem, NGi, NGrid, NInput, NSpace },
  data() {
    return {
      hostname: "",
      upstream: "",
      tlsEmail: "",
      pathPrefix: "",
      encode: true,
      output: "",
      loading: false,
      error: "",
    };
  },
  methods: {
    authHeaders() {
      const t = useUserStore().token;
      return t ? { Authorization: `Bearer ${t}` } : {};
    },
    generate() {
      this.error = "";
      this.loading = true;
      const site = {
        hostname: this.hostname.trim(),
        upstream: this.upstream.trim(),
        tls_email: this.tlsEmail.trim() || null,
        path_prefix: this.pathPrefix.trim() || null,
        encode: this.encode,
      };
      axios({
        method: "post",
        url: "/caddy/api/generate",
        headers: {
          "Content-Type": "application/json",
          ...this.authHeaders(),
        },
        data: { sites: [site] },
      })
        .then((res) => {
          this.output = res.data.caddyfile || "";
          this.loading = false;
        })
        .catch((err) => {
          this.loading = false;
          const msg =
            (err.response && err.response.status === 401 && "Sign in required") ||
            (err.response && err.response.data) ||
            err.message ||
            "Request failed";
          this.error = typeof msg === "string" ? msg : JSON.stringify(msg);
          createNotification(null, "Caddy generator", this.error, "danger");
        });
    },
    copyOutput() {
      if (!this.output) return;
      navigator.clipboard.writeText(this.output).then(() => {
        createNotification(null, "Copied", "Caddyfile copied to clipboard", "success");
      });
    },
  },
};
</script>

<style lang="scss" scoped>
.caddy-configurator {
  background: var(--darcule-bg, var(--bg));
  color: var(--fg);
  padding: 8px;
  .row-actions {
    margin: 12px 0;
  }
  .error {
    color: #f87171;
    font-size: 0.85rem;
    margin-top: 8px;
  }
  .caddy-output {
    width: 100%;
    margin-top: 8px;
    background: var(--bg);
    color: var(--fg);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    padding: 8px;
    font-family: ui-monospace, monospace;
    font-size: 0.85rem;
    resize: vertical;
    box-sizing: border-box;
  }
}
</style>
