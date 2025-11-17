---
id: feat-missing-file-cleanup
status: done
owner: dereck
last_updated: 2025-02-14
related_code:
  - src-tauri/src/db/repository.rs
  - src-tauri/src/lib.rs
  - src-tauri/src/commands/clipboard.rs
  - src/routes/+page.svelte
related_docs:
  - ../30-implementation/tauri-commands.md
  - ../30-implementation/modules-overview.md
---

# Feature: Limpieza automática de archivos/imágenes huérfanos

1. **Resumen**  
   - Evita que ítems cuyos archivos/imágenes originales fueron eliminados sigan apareciendo en el historial del portapapeles.  
   - La verificación corre al ocultar la ventana (hotkey o pérdida de foco) y durante cualquier consulta a la base de datos (paginación o búsqueda).  
   - Los registros eliminados disparan un evento a la UI, que remueve los elementos inmediatamente sin requerir reinicio.

2. **Historias de usuario**  
   - *Como usuario*, cuando borro una captura en Finder, el clipboard debería dejar de mostrarla en cuanto cierre la ventana.  
   - *Como usuario*, no quiero ver errores de “File not found” cuando la app intenta abrir una ruta inexistente.  
   - *Como usuario*, espero que los thumbnails y metadatos se limpien junto con el ítem para no desperdiciar espacio.

3. **Comportamiento funcional**  
   - Backend detecta rutas inválidas para cada `file_url` o `content_metadata.external_path`.  
   - Si no existen, elimina el registro, borra sus assets cacheados y acumula los IDs removidos.  
   - Emite `clipboard-items-removed` al WebView principal; la UI elimina esos ítems del store.  
   - En peticiones `get_items_paginated` y `search_items_fts`, los elementos inexistentes se filtran automáticamente antes de responder.

4. **UI/UX**  
   - No añade botones ni paneles nuevos; el proceso es transparente.  
   - La lista se actualiza de forma silenciosa cuando se recibe el evento `clipboard-items-removed`.  
   - Se evita mostrar “File not found” en `ContentViewer.svelte` porque el ítem desaparece antes de seleccionarse.

5. **Diseño técnico**  
   - `ClipboardRepository::prune_missing_file_items` verifica cada fila con `file_url` y, si el path no existe, invoca `asset_cleanup::delete_file_url` y borra la fila.  
   - `cleanup_missing_file_records()` expone este barrido para `cleanup_missing_clipboard_files` (comando Tauri).  
   - `tauri::WindowEvent::Focused(false)` (en `src-tauri/src/lib.rs`) lanza una tarea async que ejecuta dicha limpieza y emite `clipboard-items-removed`.  
   - La UI escucha ese evento en `src/routes/+page.svelte` y actualiza `clipboardStore.items`.  
   - Las rutas inexistentes se detectan también en las consultas (`get_items_paginated` / `search_items_fts`) para evitar resultados obsoletos en cualquier escenario.

6. **Flujo**  
   1. Usuario oculta ventana.  
   2. Evento `Focused(false)` → `cleanup_missing_file_records()` → elimina filas con `file_url` inválido.  
   3. IDs eliminados se envían al WebView.  
   4. `clipboardStore` remueve los elementos y ajusta `totalItems/hasMore`.  
   5. Siguiente apertura ya muestra solo registros válidos.  
   *(Alternativamente: cuando se llama a `get_items_paginated` o `search_items_fts`, se aplican los mismos pasos sin requerir evento de UI.)*

7. **APIs/Comandos**  
   - `cleanup_missing_clipboard_files` (nuevo comando Tauri).  
   - `clipboard-items-removed` (evento emitido desde Rust).  
   - `ClipboardRepository::cleanup_missing_file_records`, `::prune_missing_file_items`.  
   - `appWindow` ya no escucha eventos en frontend; todo se maneja en backend.

8. **Archivos involucrados**  
   - `src-tauri/src/db/repository.rs` (lógica de limpieza + unit tests).  
   - `src-tauri/src/commands/clipboard.rs` (exposición del comando).  
   - `src-tauri/src/lib.rs` (hook en evento de ventana).  
   - `src/routes/+page.svelte` (listener del evento y sincronización del store).  
   - `src/lib/stores/clipboard.svelte.ts` (ya no contiene timers ni listeners; solo mantiene estado).

9. **Edge cases**  
   - Favoritos con archivos borrados: también se eliminan (policy definida, no hay excepción).  
   - Ítems duplicados por hash: se evita antes de llegar a este flujo (detect duplicates).  
   - Apps con múltiples ventanas: solo la principal emite/escucha actualmente; si agregamos más vistas, deberán suscribirse al mismo evento.

10. **Testing**  
   - Unit tests en `db/repository.rs` (`pagination_marks_missing_external_files`, `fts_search_prunes_missing_files`).  
   - Recomendado: QA manual copiando archivos/imágenes, luego borrándolos y ocultando la ventana para verificar que desaparecen.  
   - Verificar que `clipboard-items-removed` no se emite vacío y que `ContentViewer` no muestra errores.

11. **Estado actual**  
   - Implementado y probado (Rust unit tests + validación manual).  
   - Vitest pendiente para listeners/eventos en frontend cuando la suite se habilite.

12. **Notas para IA**  
   - Si se introduce un modo headless o background-only, este feature debería dispararse también en intervalos o hooks equivalentes.  
   - Mantener sincronizado `docs/30-implementation/tauri-commands.md` y `modules-overview.md` con este comando/evento.  
   - En futuras migraciones, si el almacenamiento cambia de ruta, actualizar `resolve_external_path`.
