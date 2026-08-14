import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { defineConfig } from "vite";

// Clean, production-grade config. The previous template shipped Manus/Builder.io
// preview plugins that injected a large debug runtime into index.html; those are
// removed so the public site stays small, fast, and free of any injected
// telemetry — matching the product's no-telemetry promise.
export default defineConfig({
  // Served from the custom apex domain turkmenai.tech, not a github.io/<repo> subpath.
  base: "/",
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(import.meta.dirname, "client", "src"),
      "@shared": path.resolve(import.meta.dirname, "shared"),
    },
  },
  envDir: path.resolve(import.meta.dirname),
  root: path.resolve(import.meta.dirname, "client"),
  build: {
    outDir: path.resolve(import.meta.dirname, "dist/public"),
    emptyOutDir: true,
  },
  server: {
    port: 3000,
    strictPort: false,
    host: true,
    fs: { strict: true, deny: ["**/.*"] },
  },
});
