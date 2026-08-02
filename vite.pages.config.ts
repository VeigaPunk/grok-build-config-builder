import { defineConfig } from "vite";
import viteReact from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "node:path";

// Static SPA build for GitHub Pages (base path under /repo-name/).
export default defineConfig({
  base: "/grok-build-config-builder/",
  plugins: [tailwindcss(), viteReact()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  build: {
    outDir: "dist-pages",
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(__dirname, "pages.html"),
    },
  },
});
