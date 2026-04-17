<template>
  <div class="metric-applet">
    <n-space vertical size="large">
      <n-space justify="space-between" align="center" wrap>
        <n-h3 style="margin: 0">Host metrics (all)</n-h3>
        <n-button :loading="speedTestLoading" type="primary" size="small" @click="runSpeedTest">
          Run network speed test
        </n-button>
      </n-space>

      <n-alert v-if="speedTestResult != null" type="info" closable @close="speedTestResult = null">
        Download: {{ formatMbps(speedTestResult.megabits_per_second) }} · {{ formatBytes(speedTestResult.bytes_downloaded) }} in
        {{ speedTestResult.duration_ms }} ms
      </n-alert>
      <n-alert v-if="speedTestError != null" type="error" closable @close="speedTestError = null">
        {{ speedTestError }}
      </n-alert>

      <n-grid :cols="2" :x-gap="12" :y-gap="12" responsive="screen">
        <n-gi>
          <n-statistic label="CPU usage" :value="formatPercent(metrics?.cpu_usage_percent)" />
        </n-gi>
        <n-gi>
          <n-statistic label="Processes" :value="metrics?.process_count ?? '—'" />
        </n-gi>
      </n-grid>

      <div v-if="metrics?.cpus?.length" class="cpu-cores">
        <div v-for="(c, i) in metrics.cpus" :key="i" class="cpu-row">
          <span class="cpu-name">{{ c.name }}</span>
          <n-progress type="line" :percentage="clampPercent(c.usage_percent)" :height="8" :show-indicator="false" />
          <span class="cpu-meta">{{ formatPercent(c.usage_percent) }} @ {{ c.frequency_mhz }} MHz</span>
        </div>
      </div>

      <div>
        <div class="section-label">Memory</div>
        <n-progress
          type="line"
          :percentage="memoryPercent"
          :height="12"
          :indicator-placement="'inside'"
          :rail-style="{ background: 'rgba(128,128,128,0.2)' }"
        />
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
        <n-space vertical size="small">
          <div>{{ metrics.gpu.name }}</div>
          <div class="muted">
            Utilization {{ metrics.gpu.gpu_utilization_percent }}% · VRAM util {{ metrics.gpu.memory_utilization_percent }}% ·
            {{ formatBytes(metrics.gpu.memory_used_bytes) }} / {{ formatBytes(metrics.gpu.memory_total_bytes) }}
          </div>
          <div class="muted">
            Clocks MHz — graphics {{ metrics.gpu.graphics_clock_mhz ?? "—" }}, SM {{ metrics.gpu.sm_clock_mhz ?? "—" }}, mem
            {{ metrics.gpu.memory_clock_mhz ?? "—" }}
          </div>
        </n-space>
      </div>
      <div v-else class="muted">No NVIDIA GPU data (NVML unavailable).</div>

      <div>
        <div class="section-label">Disks</div>
        <n-table v-if="metrics?.disks?.length" :single-line="false" size="small">
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
        </n-table>
        <div v-else class="muted">No disk data.</div>
      </div>

      <div>
        <div class="section-label">Network (since last poll)</div>
        <n-table v-if="metrics?.network_interfaces?.length" :single-line="false" size="small">
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
        </n-table>
        <div v-else class="muted">No interfaces.</div>
      </div>
    </n-space>
  </div>
</template>

<script>
import { NAlert, NButton, NGi, NGrid, NH3, NProgress, NSpace, NStatistic, NTable } from "naive-ui";
import systemMetricsMixin from "./systemMetricsMixin.js";

export default {
  name: "SystemMonitor",
  components: { NAlert, NButton, NGi, NGrid, NH3, NProgress, NSpace, NStatistic, NTable },
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "./systemMetricsApplet.scss";
</style>
