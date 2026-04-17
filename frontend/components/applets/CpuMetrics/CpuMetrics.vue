<template>
  <div class="metric-applet">
    <h3 class="text-h6 ma-0 mb-3">CPU &amp; load</h3>

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

    <div v-if="metrics?.cpus?.length" class="cpu-cores mt-3">
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

    <div class="section-label mt-4">Load average</div>
    <div class="muted">
      {{ metrics?.load_average_one?.toFixed?.(2) ?? "—" }} /
      {{ metrics?.load_average_five?.toFixed?.(2) ?? "—" }} /
      {{ metrics?.load_average_fifteen?.toFixed?.(2) ?? "—" }}
      <span v-if="metrics?.host_name"> · {{ metrics.host_name }}</span>
    </div>
    <div v-if="metrics?.kernel_version" class="muted small">{{ metrics.os_name }} · {{ metrics.kernel_version }}</div>
  </div>
</template>

<script>
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "CpuMetrics",
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";

.cpu-bar {
  min-width: 0;
}
</style>
