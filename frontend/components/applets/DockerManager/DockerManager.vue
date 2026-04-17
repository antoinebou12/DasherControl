<template>
  <div class="center docker-manager-containers">
    <div class="docker-toolbar">
      <v-btn size="small" variant="text" @click="triggerImportZip">
        <template #prepend>
          <i class="bx bx-import" />
        </template>
        Import zip
      </v-btn>
      <input
        ref="importZipInput"
        type="file"
        accept=".zip,application/zip"
        class="docker-import-input"
        @change="onImportZipSelected"
      />
    </div>
    <table class="metric-table docker-table">
      <thead>
        <tr>
          <th class="quick-status-col" />
          <th>ID</th>
          <th>Name</th>
          <th>Image</th>
          <th>State</th>
          <th>Status</th>
          <th>IP</th>
          <th>Port</th>
          <th class="export-col">Export</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="tr in containers" :key="tr.id" :class="{ 'tr-selected': false }">
          <td>
            <div class="quick-status">
              <i class="bx bxs-server" />
              <i :class="get_status_theme(tr.status)" class="bx bxs-circle" />
            </div>
            <div class="quick-control">
              <v-btn v-if="get_status(tr.status)" icon size="x-small" variant="text" @click="stop_container(tr.id)">
                <i class="bx bx-stop" />
              </v-btn>
              <v-btn v-if="!get_status(tr.status)" icon size="x-small" variant="text" @click="restart_container(tr.id)">
                <i class="bx bx-play" />
              </v-btn>
            </div>
          </td>
          <td>{{ tr.id }}</td>
          <td>{{ tr.name }}</td>
          <td>{{ tr.image }}</td>
          <td>{{ tr.state }}</td>
          <td>{{ tr.status }}</td>
          <td>{{ tr.ip }}</td>
          <td>
            <a class="port-link" @click.prevent="openContainer(tr)">{{ tr.public_port }}</a>
          </td>
          <td class="export-col">
            <v-btn icon size="x-small" variant="text" title="Export filesystem zip" @click="export_container_zip(tr)">
              <i class="bx bx-archive" />
            </v-btn>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import axios from "axios";
import { useUserStore } from "../../../stores/user";
import { createNotification } from "../../../utils";

export default {
  name: "DockerManager",
  data() {
    return {
      statusInterval: null,
      containers: [],
    };
  },
  created() {
    this.get_container();
    this.statusInterval = setInterval(() => {
      this.get_container();
    }, 1000);
  },
  beforeUnmount() {
    if (this.statusInterval) clearInterval(this.statusInterval);
  },
  methods: {
    authHeaders() {
      const t = useUserStore().token;
      return t ? { Authorization: `Bearer ${t}` } : {};
    },
    get_container() {
      axios({
        method: "get",
        url: "/containers/api/docker_list",
        headers: { "Content-Type": "application/json" },
      }).then((response) => {
        this.containers = [];
        const rows = response.data || [];
        for (let i = 0; i < rows.length; i++) {
          const row = rows[i];
          const name0 = (row.names && row.names[0]) || row.id || "";
          const fullId = row.id || "";
          this.containers.push({
            fullId,
            id: fullId.slice(0, 12),
            name: String(name0).replace("/", ""),
            image: row.image || "",
            state: row.state || "",
            status: row.status || "",
            ip: typeof window !== "undefined" ? window.location.hostname.replace("www.", "") : "",
            public_port: "X",
          });
        }
      });
    },
    get_status(status) {
      return String(status || "").includes("Up");
    },
    get_status_theme(status) {
      return this.get_status(status) ? "up-docker" : "down-docker";
    },
    stop_container(id) {
      axios({
        method: "post",
        headers: this.authHeaders(),
        url: "/containers/api/stop/" + encodeURIComponent(id),
      }).then(() => {
        this.get_container();
      });
    },
    restart_container(id) {
      axios({
        method: "post",
        headers: this.authHeaders(),
        url: "/containers/api/restart/" + encodeURIComponent(id),
      }).then(() => {
        this.get_container();
      });
    },
    export_container_zip(tr) {
      const id = tr.fullId;
      if (!id) return;
      axios({
        method: "get",
        url: "/containers/api/docker_export_zip/" + encodeURIComponent(id),
        headers: { ...this.authHeaders() },
        responseType: "blob",
      })
        .then((response) => {
          const blob = new Blob([response.data], { type: "application/zip" });
          const url = URL.createObjectURL(blob);
          const a = document.createElement("a");
          a.href = url;
          a.download = `dasher-docker-${id.slice(0, 12)}.zip`;
          a.click();
          URL.revokeObjectURL(url);
          createNotification(this, "Export", "Saved zip download started.", "success");
        })
        .catch((err) => {
          const msg =
            err.response?.data instanceof Blob
              ? "Export failed (check permissions / Docker)."
              : err.message || "Export failed.";
          createNotification(this, "Export failed", msg, "danger");
        });
    },
    triggerImportZip() {
      const el = this.$refs.importZipInput;
      if (el) el.click();
    },
    onImportZipSelected(ev) {
      const input = ev.target;
      const file = input.files && input.files[0];
      if (!file) return;
      const fd = new FormData();
      fd.append("file", file);
      axios
        .post("/containers/api/docker_import_zip", fd, {
          headers: { ...this.authHeaders() },
        })
        .then((res) => {
          const r = res.data || {};
          const ref = r.image_ref || "";
          createNotification(this, "Import", ref ? `Image ${ref}` : "Image imported.", "success");
          this.get_container();
        })
        .catch((err) => {
          createNotification(
            this,
            "Import failed",
            err.response?.data?.message || err.message || "Import failed.",
            "danger",
          );
        })
        .finally(() => {
          input.value = "";
        });
    },
    openContainer(containerData) {
      this.$emit("openContainer", containerData);
    },
  },
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";

.docker-manager-containers {
  background: var(--bg);
  color: var(--fg);

  .docker-toolbar {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 8px;
  }

  .docker-import-input {
    display: none;
  }

  .export-col {
    width: 48px;
    text-align: center;
  }

  .quick-status-col {
    width: 10px;
  }

  .tr-selected {
    background: rgba(var(--vs-color, 189, 147, 249), 0.1);
    color: rgba(var(--vs-color, 189, 147, 249), 0.8);
  }
  .quick-status {
    display: inline-block;
    position: relative;
    margin: 10px;
    .bxs-server {
      font-size: 30px;
    }
    .bxs-circle {
      position: absolute;
      bottom: 0;
      right: 0;
      margin: -3px -6px;
    }
  }
  .quick-control {
    display: inline-flex;
    gap: 4px;
    vertical-align: middle;
  }
  .port-link {
    cursor: pointer;
    text-decoration: underline;
  }
  .up-docker {
    color: #4ade80;
  }
  .down-docker {
    color: #f87171;
  }
}
</style>
