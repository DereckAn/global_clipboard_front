---
id: adr-0001-macos-tray-icon-template
status: done
owner: dereck
last_updated: 2025-02-14
related_code:
  - src-tauri/src/lib.rs
  - src-tauri/icons/tray_icon_template.png
related_docs:
  - ../30-implementation/modules-overview.md
---

# ADR-0001: Tray icon template para macOS

## 1. Contexto
- El icono actual del tray era un PNG negro (generado por `bun run tauri icon`), por lo que se veía siempre oscuro aun cuando la barra de menús cambiaba al modo claro.
- macOS espera íconos “template” (monocromáticos) para recolorearlos automáticamente; Tauri permite cargar un `Image` y marcar `icon_as_template(true)` solo si se provee un recurso compatible.
- En desarrollo, el icono template no existía dentro del bundle, lo que generaba un `No such file or directory` durante `setup`.

## 2. Decisión
- Añadimos un PNG monocromático (`src-tauri/icons/tray_icon_template.png`) y, únicamente en macOS, lo cargamos a través de `tauri::image::Image::from_path`.
- El tray builder ahora invoca `icon_as_template(true)` para permitir que macOS renderice el icono en blanco/negro según corresponda.
- Si el recurso no puede cargarse (por ejemplo en dev), se hace fallback al `default_window_icon` para evitar el panic.

## 3. Consecuencias
- El tray respeta el modo claro/oscuro sin necesidad de mantener dos versiones del icono.
- En dev ya no se interrumpe la ejecución por rutas inexistentes; el fallback garantiza que el tray siempre tenga un icono válido.
- Debemos mantener el archivo template sincronizado en `src-tauri/icons` y asegurarnos de que el builder lo incluya en `resources` al crear la app.

## 4. Alternativas consideradas
- **Mantener icono a color**: se descartó porque la barra de menús sería inconsistente con otras apps.
- **Generar template desde código**: implicaba rasterizar dinámicamente el PNG; no aportaba ventajas frente a mantener un recurso dedicado.

## 5. Referencias
- `src-tauri/src/lib.rs` – lógica de carga condicional y `icon_as_template(true)`.
- `docs/30-implementation/modules-overview.md` – sección Backend menciona este cambio.
