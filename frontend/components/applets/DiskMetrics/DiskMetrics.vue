<template>
  <div class="metric-applet">
    <h3 class="text-h6 ma-0 mb-3">Disks</h3>

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
</template>

<script>
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "DiskMetrics",
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";
</style>
