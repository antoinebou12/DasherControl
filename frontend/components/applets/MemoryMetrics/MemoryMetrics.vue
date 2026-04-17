<template>
  <div class="metric-applet">
    <n-h3 style="margin: 0 0 12px">Memory &amp; swap</n-h3>

    <div class="section-label">RAM</div>
    <n-progress
      type="line"
      :percentage="memoryPercent"
      :height="12"
      indicator-placement="inside"
      :rail-style="{ background: 'rgba(128,128,128,0.2)' }"
    />
    <div class="muted">
      {{ formatBytes(metrics?.memory_used_bytes) }} / {{ formatBytes(metrics?.memory_total_bytes) }}
    </div>

    <div class="section-label" style="margin-top: 16px">Swap</div>
    <div class="muted">
      {{ formatBytes(metrics?.swap_used_bytes) }} / {{ formatBytes(metrics?.swap_total_bytes) }}
    </div>
  </div>
</template>

<script>
import { NH3, NProgress } from "naive-ui";
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "MemoryMetrics",
  components: { NH3, NProgress },
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";
</style>
