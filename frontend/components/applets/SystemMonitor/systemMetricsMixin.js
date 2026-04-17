import axios from "axios";

/** Shared poll + helpers for `/system/api/metrics` applets. */
export default {
  data() {
    return {
      metrics: null,
      pollTimer: null,
      speedTestLoading: false,
      speedTestResult: null,
      speedTestError: null,
    };
  },
  computed: {
    memoryPercent() {
      const m = this.metrics;
      if (!m || !m.memory_total_bytes) return 0;
      const p = (100 * m.memory_used_bytes) / m.memory_total_bytes;
      return Math.max(0, Math.min(100, p));
    },
  },
  created() {
    this.fetchMetrics();
    this.pollTimer = setInterval(() => this.fetchMetrics(), 1500);
  },
  beforeUnmount() {
    if (this.pollTimer) clearInterval(this.pollTimer);
  },
  methods: {
    fetchMetrics() {
      axios
        .get("/system/api/metrics")
        .then((res) => {
          this.metrics = res.data;
        })
        .catch(() => {
          /* keep last snapshot */
        });
    },
    runSpeedTest() {
      this.speedTestResult = null;
      this.speedTestError = null;
      this.speedTestLoading = true;
      axios
        .post("/system/api/network_speed_test")
        .then((res) => {
          this.speedTestResult = res.data;
        })
        .catch((err) => {
          const d = err.response && err.response.data;
          this.speedTestError = (d && d.error) || err.message || "Speed test failed";
        })
        .finally(() => {
          this.speedTestLoading = false;
        });
    },
    formatBytes(n) {
      if (n == null || Number.isNaN(n)) return "—";
      const u = ["B", "KB", "MB", "GB", "TB"];
      let v = Number(n);
      let i = 0;
      while (v >= 1024 && i < u.length - 1) {
        v /= 1024;
        i++;
      }
      return `${v.toFixed(i > 0 ? 1 : 0)} ${u[i]}`;
    },
    formatPercent(x) {
      if (x == null || Number.isNaN(x)) return "—";
      return `${Number(x).toFixed(1)}%`;
    },
    formatMbps(m) {
      if (m == null || Number.isNaN(m)) return "—";
      return `${Number(m).toFixed(2)} Mb/s`;
    },
    clampPercent(x) {
      if (x == null || Number.isNaN(x)) return 0;
      return Math.max(0, Math.min(100, Number(x)));
    },
  },
};
