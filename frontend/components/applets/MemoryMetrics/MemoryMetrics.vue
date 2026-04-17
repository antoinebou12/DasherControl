<template>
  <div class="metric-applet">
    <h3 class="text-h6 ma-0 mb-3">Memory &amp; swap</h3>

    <div class="section-label">RAM</div>
    <v-progress-linear :model-value="memoryPercent" color="primary" height="12" rounded class="mb-1" />
    <div class="muted">
      {{ formatBytes(metrics?.memory_used_bytes) }} / {{ formatBytes(metrics?.memory_total_bytes) }}
    </div>

    <div class="section-label mt-4">Swap</div>
    <div class="muted">
      {{ formatBytes(metrics?.swap_used_bytes) }} / {{ formatBytes(metrics?.swap_total_bytes) }}
    </div>
  </div>
</template>

<script>
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "MemoryMetrics",
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";
</style>
