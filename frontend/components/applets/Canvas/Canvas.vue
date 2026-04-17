<template>
  <div ref="root" class="canvas-applet-root">
    <canvas
      v-show="resolvedMode === 'canvas2d'"
      ref="canvas2d"
      class="canvas-surface"
    />
    <canvas
      v-show="resolvedMode === 'three'"
      ref="canvasThree"
      class="canvas-surface"
    />
  </div>
</template>

<script>
import * as THREE from "three";

export default {
  name: "Canvas",
  props: {
    canvasMode: {
      type: String,
      default: "three",
    },
  },
  data() {
    return {
      _resizeObserver: null,
      _canvas2dRaf: null,
      _threeRaf: null,
      _renderer: null,
      _scene: null,
      _camera: null,
      _mesh: null,
      _2dT: 0,
    };
  },
  computed: {
    resolvedMode() {
      return this.canvasMode === "canvas2d" ? "canvas2d" : "three";
    },
  },
  watch: {
    resolvedMode() {
      this.teardownGraphics();
      this.$nextTick(() => {
        this.syncCanvasLayout();
        this.startGraphics();
      });
    },
  },
  mounted() {
    this._resizeObserver = new ResizeObserver(() => {
      this.syncCanvasLayout();
      this.onResizeThree();
    });
    this._resizeObserver.observe(this.$refs.root);
    this.$nextTick(() => {
      this.syncCanvasLayout();
      this.startGraphics();
    });
  },
  beforeUnmount() {
    this.teardownGraphics();
    if (this._resizeObserver) {
      this._resizeObserver.disconnect();
      this._resizeObserver = null;
    }
  },
  methods: {
    syncCanvasLayout() {
      const root = this.$refs.root;
      if (!root) return;
      const w = Math.max(1, Math.floor(root.clientWidth));
      const h = Math.max(1, Math.floor(root.clientHeight));
      const dpr = typeof window !== "undefined" ? window.devicePixelRatio || 1 : 1;

      const c2d = this.$refs.canvas2d;
      if (c2d) {
        c2d.width = Math.floor(w * dpr);
        c2d.height = Math.floor(h * dpr);
        c2d.style.width = `${w}px`;
        c2d.style.height = `${h}px`;
        const ctx = c2d.getContext("2d");
        if (ctx) {
          ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
        }
      }

      const c3 = this.$refs.canvasThree;
      if (c3) {
        c3.style.width = `${w}px`;
        c3.style.height = `${h}px`;
      }
    },

    teardownGraphics() {
      if (this._canvas2dRaf != null) {
        cancelAnimationFrame(this._canvas2dRaf);
        this._canvas2dRaf = null;
      }
      if (this._threeRaf != null) {
        cancelAnimationFrame(this._threeRaf);
        this._threeRaf = null;
      }
      if (this._mesh) {
        this._mesh.geometry?.dispose();
        if (Array.isArray(this._mesh.material)) {
          this._mesh.material.forEach((m) => m.dispose());
        } else {
          this._mesh.material?.dispose();
        }
        this._mesh = null;
      }
      if (this._renderer) {
        this._renderer.dispose();
        this._renderer = null;
      }
      this._scene = null;
      this._camera = null;
    },

    startGraphics() {
      if (this.resolvedMode === "canvas2d") {
        this.startCanvas2d();
      } else {
        this.startThree();
      }
    },

    startCanvas2d() {
      const canvas = this.$refs.canvas2d;
      if (!canvas) return;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      const draw = () => {
        this._canvas2dRaf = requestAnimationFrame(draw);
        const w = canvas.width / (window.devicePixelRatio || 1);
        const h = canvas.height / (window.devicePixelRatio || 1);
        const t = this._2dT++;

        ctx.fillStyle = `hsl(${t % 360}, 35%, 18%)`;
        ctx.fillRect(0, 0, w, h);

        ctx.strokeStyle = "rgba(120, 220, 200, 0.85)";
        ctx.lineWidth = 2;
        ctx.beginPath();
        const cx = w * 0.5 + Math.cos(t * 0.04) * w * 0.22;
        const cy = h * 0.5 + Math.sin(t * 0.035) * h * 0.18;
        ctx.arc(cx, cy, Math.min(w, h) * 0.08, 0, Math.PI * 2);
        ctx.stroke();

        ctx.fillStyle = "rgba(200, 230, 255, 0.9)";
        ctx.font = `${Math.max(12, Math.min(w, h) * 0.06)}px sans-serif`;
        ctx.fillText("Canvas 2D", 12, 24);
      };
      draw();
    },

    onResizeThree() {
      if (!this._renderer || !this._camera || !this.$refs.canvasThree) return;
      const canvas = this.$refs.canvasThree;
      const w = Math.max(1, canvas.clientWidth);
      const h = Math.max(1, canvas.clientHeight);
      this._renderer.setPixelRatio(window.devicePixelRatio || 1);
      this._renderer.setSize(w, h, false);
      this._camera.aspect = w / h;
      this._camera.updateProjectionMatrix();
    },

    startThree() {
      const canvas = this.$refs.canvasThree;
      if (!canvas) return;

      const renderer = new THREE.WebGLRenderer({
        canvas,
        antialias: true,
        alpha: true,
      });
      renderer.setPixelRatio(window.devicePixelRatio || 1);

      const scene = new THREE.Scene();
      const camera = new THREE.PerspectiveCamera(55, 1, 0.1, 100);
      camera.position.set(0, 0, 3.2);

      const geo = new THREE.BoxGeometry(1, 1, 1);
      const mat = new THREE.MeshNormalMaterial();
      const mesh = new THREE.Mesh(geo, mat);
      scene.add(mesh);

      const light = new THREE.HemisphereLight(0xffffff, 0x222233, 0.9);
      scene.add(light);

      this._renderer = renderer;
      this._scene = scene;
      this._camera = camera;
      this._mesh = mesh;

      this.onResizeThree();

      const loop = () => {
        this._threeRaf = requestAnimationFrame(loop);
        mesh.rotation.x += 0.012;
        mesh.rotation.y += 0.018;
        renderer.render(scene, camera);
      };
      loop();
    },
  },
};
</script>

<style scoped>
.canvas-applet-root {
  width: 100%;
  height: 100%;
  min-height: 0;
  min-width: 0;
  position: relative;
  background: #12141a;
}

.canvas-surface {
  display: block;
  width: 100%;
  height: 100%;
  position: absolute;
  left: 0;
  top: 0;
}
</style>
