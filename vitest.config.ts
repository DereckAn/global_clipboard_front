import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";
import { fileURLToPath } from "url";
import { defineConfig } from "vitest/config";

const __dirname = fileURLToPath(new URL(".", import.meta.url));

export default defineConfig({
  plugins: [
    svelte({
      // Habilitar el modo de compilación para tests
      compilerOptions: {
        // Necesario para que $state y otros runes funcionen
        runes: true,
      },
    }),
  ],
  resolve: {
    conditions: ["browser"],
    alias: {
      $lib: resolve(__dirname, "./src/lib"),
    },
  },
  test: {
    environment: "jsdom",
    include: ["src/**/*.{test,spec}.{ts,js}"],
    globals: true,
    setupFiles: ["./vitest.setup.ts"],
  },
});
