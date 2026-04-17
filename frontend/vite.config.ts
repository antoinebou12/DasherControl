import fs from "node:fs";
import path from "node:path";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

function copyTeraTemplates() {
  return {
    name: "copy-tera-templates",
    closeBundle() {
      const srcDir = path.resolve(__dirname, "templates");
      const destDir = path.resolve(__dirname, "dist/templates");
      fs.mkdirSync(destDir, { recursive: true });
      fs.cpSync(srcDir, destDir, { recursive: true });
    },
  };
}

export default defineConfig({
  plugins: [vue(), copyTeraTemplates()],
  root: __dirname,
  publicDir: "public",
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "."),
    },
  },
  server: {
    port: 9999,
    https: {},
    proxy: {
      "/tenants": "http://127.0.0.1:8080",
      "/workspaces": "http://127.0.0.1:8080",
      "/containers": "http://127.0.0.1:8080",
      "/caddy": "http://127.0.0.1:8080",
      "/system": "http://127.0.0.1:8080",
      "/assistant": "http://127.0.0.1:8080",
    },
  },
  build: {
    outDir: "dist/public",
    emptyOutDir: true,
    sourcemap: true,
    rollupOptions: {
      input: path.resolve(__dirname, "index.html"),
    },
  },
});
