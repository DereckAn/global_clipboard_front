import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [tailwindcss(), sveltekit()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  // ============================================
  // MOBILE DEVELOPMENT (uncomment when needed)
  // ============================================
  // To develop for iOS/Android or access dev server from other devices:
  // 1. Set TAURI_DEV_HOST env variable to your local IP (e.g., 192.168.1.100)
  // 2. Uncomment the code below and comment out the server config above
  //
  // const host = process.env.TAURI_DEV_HOST;
  // server: {
  //   port: 1420,
  //   strictPort: true,
  //   host: host || false,
  //   hmr: host
  //     ? {
  //         protocol: "ws",
  //         host,
  //         port: 1421,
  //       }
  //     : undefined,
  //   watch: {
  //     ignored: ["**/src-tauri/**"],
  //   },
  // },
}));
