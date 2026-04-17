<template>
  <div class="metric-applet">
    <n-h3 style="margin: 0 0 12px">CPU &amp; load</n-h3>

    <n-grid :cols="2" :x-gap="12" :y-gap="12" responsive="screen">
      <n-gi>
        <n-statistic label="CPU usage" :value="formatPercent(metrics?.cpu_usage_percent)" />
      </n-gi>
      <n-gi>
        <n-statistic label="Processes" :value="metrics?.process_count ?? '—'" />
      </n-gi>
    </n-grid>

    <div v-if="metrics?.cpus?.length" class="cpu-cores" style="margin-top: 12px">
      <div v-for="(c, i) in metrics.cpus" :key="i" class="cpu-row">
        <span class="cpu-name">{{ c.name }}</span>
        <n-progress type="line" :percentage="clampPercent(c.usage_percent)" :height="8" :show-indicator="false" />
        <span class="cpu-meta">{{ formatPercent(c.usage_percent) }} @ {{ c.frequency_mhz }} MHz</span>
      </div>
    </div>

    <div class="section-label" style="margin-top: 14px">Load average</div>
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
import { NGi, NGrid, NH3, NProgress, NStatistic } from "naive-ui";
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "CpuMetrics",
  components: { NGi, NGrid, NH3, NProgress, NStatistic },
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";
</style>
