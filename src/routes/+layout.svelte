<script>
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import "../app.css";

  let { children } = $props();
  let isDragging = $state(false);

  onMount(() => {
    const appWindow = getCurrentWindow();

    // Detectar cuando se empieza a hacer drag
    document.addEventListener("mousedown", (e) => {
      const target = /** @type {HTMLElement} */ (e.target);
      if (
        target.hasAttribute("data-tauri-drag-region") ||
        target.closest("[data-tauri-drag-region]")
      ) {
        isDragging = true;
      }
    });

    // Detectar cuando termina el drag
    document.addEventListener("mouseup", () => {
      // Pequeño delay para asegurar que el drag terminó
      setTimeout(() => {
        isDragging = false;
      }, 100);
    });

    // Click fuera de la ventana (blur)
    window.addEventListener("blur", async () => {
      // Pequeño delay para verificar si realmente perdimos el foco
      setTimeout(async () => {
        // Solo ocultar si NO estamos haciendo drag
        if (!isDragging) {
          try {
            await appWindow.hide();
          } catch (err) {
            console.error("Failed to hide window:", err);
          }
        }
      }, 150);
    });

    // También ocultar con tecla Escape (opcional)
    window.addEventListener("keydown", async (e) => {
      if (e.key === "Escape") {
        try {
          await appWindow.hide();
        } catch (err) {
          console.error("Failed to hide window:", err);
        }
      }
    });
  });
</script>

{@render children()}
