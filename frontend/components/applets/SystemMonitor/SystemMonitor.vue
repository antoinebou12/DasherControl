<template>
  <div class="metric-applet">
    <div class="d-flex flex-column ga-4">
      <div class="d-flex flex-wrap align-center justify-space-between ga-2">
        <h3 class="text-h6 ma-0">Host metrics (all)</h3>
        <v-btn color="primary" size="small" :loading="speedTestLoading" @click="runSpeedTest">
          Run network speed test
        </v-btn>
      </div>

      <v-alert
        v-if="speedTestResult != null"
        type="info"
        density="compact"
        variant="tonal"
        closable
        @click:close="speedTestResult = null"
      >
        Download: {{ formatMbps(speedTestResult.megabits_per_second) }} ·
        {{ formatBytes(speedTestResult.bytes_downloaded) }} in
        {{ speedTestResult.duration_ms }} ms
      </v-alert>
      <v-alert
        v-if="speedTestError != null"
        type="error"
        density="compact"
        variant="tonal"
        closable
        @click:close="speedTestError = null"
      >
        {{ speedTestError }}
      </v-alert>

      <v-row dense>
        <v-col cols="12" sm="6">
          <div class="text-caption text-medium-emphasis">CPU usage</div>
          <div class="text-h6">{{ formatPercent(metrics?.cpu_usage_percent) }}</div>
        </v-col>
        <v-col cols="12" sm="6">
          <div class="text-caption text-medium-emphasis">Processes</div>
          <div class="text-h6">{{ metrics?.process_count ?? "—" }}</div>
        </v-col>
      </v-row>

      <div v-if="metrics?.cpus?.length" class="cpu-cores">
        <div v-for="(c, i) in metrics.cpus" :key="i" class="cpu-row">
          <span class="cpu-name">{{ c.name }}</span>
          <v-progress-linear
            :model-value="clampPercent(c.usage_percent)"
            color="primary"
            height="8"
            rounded
            class="cpu-bar"
          />
          <span class="cpu-meta">{{ formatPercent(c.usage_percent) }} @ {{ c.frequency_mhz }} MHz</span>
        </div>
      </div>

      <div>
        <div class="section-label">Memory</div>
        <v-progress-linear :model-value="memoryPercent" color="primary" height="12" rounded class="mb-1" />
        <div class="muted">
          {{ formatBytes(metrics?.memory_used_bytes) }} / {{ formatBytes(metrics?.memory_total_bytes) }} · swap
          {{ formatBytes(metrics?.swap_used_bytes) }} / {{ formatBytes(metrics?.swap_total_bytes) }}
        </div>
      </div>

      <div>
        <div class="section-label">Load average</div>
        <div class="muted">
          {{ metrics?.load_average_one?.toFixed?.(2) ?? "—" }} /
          {{ metrics?.load_average_five?.toFixed?.(2) ?? "—" }} /
          {{ metrics?.load_average_fifteen?.toFixed?.(2) ?? "—" }}
          <span v-if="metrics?.host_name"> · {{ metrics.host_name }}</span>
        </div>
        <div v-if="metrics?.kernel_version" class="muted small">{{ metrics.os_name }} · {{ metrics.kernel_version }}</div>
      </div>

      <div v-if="metrics?.gpu">
        <div class="section-label">GPU (NVIDIA)</div>
        <div class="d-flex flex-column ga-1">
          <div>{{ metrics.gpu.name }}</div>
          <div class="muted">
            Utilization {{ metrics.gpu.gpu_utilization_percent }}% · VRAM util {{ metrics.gpu.memory_utilization_percent }}% ·
            {{ formatBytes(metrics.gpu.memory_used_bytes) }} / {{ formatBytes(metrics.gpu.memory_total_bytes) }}
          </div>
          <div class="muted">
            Clocks MHz — graphics {{ metrics.gpu.graphics_clock_mhz ?? "—" }}, SM {{ metrics.gpu.sm_clock_mhz ?? "—" }}, mem
            {{ metrics.gpu.memory_clock_mhz ?? "—" }}
          </div>
        </div>
      </div>
      <div v-else class="muted">No NVIDIA GPU data (NVML unavailable).</div>

      <div>
        <div class="section-label">Disks</div>
        <table v-if="metrics?.disks?.length" class="metric-table">
          <thead>
            <tr>
              <th>Mount</th>
              <th>FS</th>
              <th>Used</th>
              <th>Total</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(d, i) in metrics.disks" :key="i">
              <td>{{ d.mount_point }}</td>
              <td>{{ d.file_system }}</td>
              <td>{{ formatBytes(d.used_bytes) }}</td>
              <td>{{ formatBytes(d.total_bytes) }}</td>
            </tr>
          </tbody>
        </table>
        <div v-else class="muted">No disk data.</div>
      </div>

      <div>
        <div class="section-label">Network (since last poll)</div>
        <table v-if="metrics?.network_interfaces?.length" class="metric-table">
          <thead>
            <tr>
              <th>Interface</th>
              <th>Down B/s</th>
              <th>Up B/s</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="n in metrics.network_interfaces" :key="n.name">
              <td>{{ n.name }}</td>
              <td>{{ n.received_bytes_per_sec.toFixed(0) }}</td>
              <td>{{ n.transmitted_bytes_per_sec.toFixed(0) }}</td>
            </tr>
          </tbody>
        </table>
        <div v-else class="muted">No interfaces.</div>
      </div>
    </div>
  </div>
</template>

<script>
import systemMetricsMixin from "./systemMetricsMixin.js";

export default {
  name: "SystemMonitor",
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "./systemMetricsApplet.scss";

.cpu-bar {
  min-width: 0;
}
</style>
