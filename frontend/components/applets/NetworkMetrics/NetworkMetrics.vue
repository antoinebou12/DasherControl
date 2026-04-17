<template>
  <div class="metric-applet">
    <n-space vertical size="medium">
      <n-space justify="space-between" align="center" wrap>
        <n-h3 style="margin: 0">Network</n-h3>
        <n-button :loading="speedTestLoading" type="primary" size="small" @click="runSpeedTest">Run speed test</n-button>
      </n-space>

      <n-alert v-if="speedTestResult != null" type="info" closable @close="speedTestResult = null">
        {{ formatMbps(speedTestResult.megabits_per_second) }} · {{ formatBytes(speedTestResult.bytes_downloaded) }} in
        {{ speedTestResult.duration_ms }} ms
      </n-alert>
      <n-alert v-if="speedTestError != null" type="error" closable @close="speedTestError = null">
        {{ speedTestError }}
      </n-alert>

      <div class="section-label">Interfaces (since last poll)</div>
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
    </n-space>
  </div>
</template>

<script>
import { NAlert, NButton, NH3, NSpace, NTable } from "naive-ui";
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "NetworkMetrics",
  components: { NAlert, NButton, NH3, NSpace, NTable },
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";
</style>
