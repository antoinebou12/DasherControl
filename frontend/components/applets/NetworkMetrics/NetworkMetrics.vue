<template>
  <div class="metric-applet">
    <div class="d-flex flex-column ga-3">
      <div class="d-flex flex-wrap align-center justify-space-between ga-2">
        <h3 class="text-h6 ma-0">Network</h3>
        <v-btn color="primary" size="small" :loading="speedTestLoading" @click="runSpeedTest">Run speed test</v-btn>
      </div>

      <v-alert
        v-if="speedTestResult != null"
        type="info"
        density="compact"
        variant="tonal"
        closable
        @click:close="speedTestResult = null"
      >
        {{ formatMbps(speedTestResult.megabits_per_second) }} · {{ formatBytes(speedTestResult.bytes_downloaded) }} in
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

      <div class="section-label">Interfaces (since last poll)</div>
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
</template>

<script>
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "NetworkMetrics",
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";
</style>
