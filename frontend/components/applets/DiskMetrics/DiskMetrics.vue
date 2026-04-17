<template>
  <div class="metric-applet">
    <n-h3 style="margin: 0 0 12px">Disks</n-h3>

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
</template>

<script>
import { NH3, NTable } from "naive-ui";
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "DiskMetrics",
  components: { NH3, NTable },
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";
</style>
