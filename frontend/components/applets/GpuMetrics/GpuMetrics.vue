<template>
  <div class="metric-applet">
    <h3 class="text-h6 ma-0 mb-3">GPU (NVIDIA)</h3>

    <template v-if="metrics?.gpu">
      <div>{{ metrics.gpu.name }}</div>
      <div class="muted">
        Utilization {{ metrics.gpu.gpu_utilization_percent }}% · VRAM util {{ metrics.gpu.memory_utilization_percent }}% ·
        {{ formatBytes(metrics.gpu.memory_used_bytes) }} / {{ formatBytes(metrics.gpu.memory_total_bytes) }}
      </div>
      <div class="muted">
        Clocks MHz — graphics {{ metrics.gpu.graphics_clock_mhz ?? "—" }}, SM {{ metrics.gpu.sm_clock_mhz ?? "—" }}, mem
        {{ metrics.gpu.memory_clock_mhz ?? "—" }}
      </div>
    </template>
    <div v-else class="muted">No NVIDIA GPU data (NVML unavailable).</div>
  </div>
</template>

<script>
import systemMetricsMixin from "../SystemMonitor/systemMetricsMixin.js";

export default {
  name: "GpuMetrics",
  mixins: [systemMetricsMixin],
};
</script>

<style lang="scss" scoped>
@import "../SystemMonitor/systemMetricsApplet.scss";
</style>
