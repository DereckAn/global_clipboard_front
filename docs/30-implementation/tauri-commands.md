---
id: impl-tauri-commands
status: in-progress
owner: dereck
last_updated: 2025-02-14
related_code:
  - src-tauri/src/commands/clipboard.rs
  - src-tauri/src/lib.rs
related_docs:
  - ../10-features/feat-missing-file-cleanup.md
  - modules-overview.md
---

# Tauri Commands Reference

## 1. Propósito
Documento de referencia para los comandos expuestos a la capa Svelte. Cada fila describe objetivo, parámetros y notas de integración. Mantener este archivo sincronizado con `src-tauri/src/commands`.

## 2. Comandos disponibles

| Comando | Fuente | Descripción | Parámetros | Respuesta | Notas |
| --- | --- | --- | --- | --- | --- |
| `get_clipboard_items_paginated` | `commands::clipboard` | Obtiene items paginados ordenados por `updated_at DESC`. | `limit: i64`, `offset: i64` | `Vec<ClipboardItem>` | Prunea ítems con `file_url` inválido antes de responder. |
| `search_clipboard_items_fts` | `commands::clipboard` | Búsqueda FTS5 con ranking BM25. | `query: String`, `limit`, `offset` | `Vec<ClipboardItem>` | También elimina registros sin archivo. |
| `cleanup_missing_clipboard_files` | `commands::clipboard` (nuevo v1.1) | Recorre ítems con `file_url` y elimina los que referencian rutas inexistentes; además borra assets asociados. | — | `Vec<String>` (IDs eliminados) | Invocado desde backend al evento `Focused(false)` y disponible para llamadas manuales desde frontend. Emite `clipboard-items-removed`. |
| `count_clipboard_items` | `commands::clipboard` | Total de registros en DB. | — | `i64` | Útil para calcular `hasMore`. |
| `count_search_results_fts` | `commands::clipboard` | Total de resultados de búsqueda. | `query: String` | `i64` | Debe llamarse tras `search_clipboard_items_fts`. |
| `get_cleanup_settings` | `commands::settings` | Lee desde `settings.json` los flags y valores de limpieza (límite de items y retención). | — | `{ max_items_enabled, max_local_items, retention_enabled, retention_days }` | `settingsStore.loadSettings()` lo consume para alinear la UI con lo que usará el backend en la tarea automática. |
| `ensure_thumbnail` | `commands::images` | Garantiza miniatura junto al archivo. | `file_path: String` | `String` (ruta miniatura) | Usa `image_handler::ensure_thumbnail`. |
| `write_image_to_clipboard` | `commands::clipboard` | Copia una imagen guardada en sandbox al clipboard del sistema. | `image_path: String` | `()` | Usa `write_clipboard_image`. |
| `write_file_to_clipboard` | `commands::clipboard` | Copia un archivo externo (macOS). | `path: String` | `()` | Disponible solo en macOS. |
| `remove_duplicate_items` | `commands::clipboard` | Elimina duplicados basados en `content_text`. | — | `usize` (borrados) | Borrado lógico + limpieza de assets. |
| `cleanup_old_items` / `cleanup_excess_items` | `commands::settings` | Herramientas de limpieza según políticas configuradas. | Depende de settings | `usize` | Consumidas desde pantalla de Settings. |

## 3. Eventos emitidos

| Evento | Emisor | Payload | Descripción |
| --- | --- | --- | --- |
| `clipboard-item-added` | `ClipboardMonitor` (`src-tauri/src/clipboard/monitor.rs`) | `ClipboardItem` (serialized) | Se dispara al registrar un nuevo ítem o “bump”. |
| `clipboard-items-removed` | `src-tauri/src/lib.rs` (evento `WindowEvent::Focused(false)`) | `Vec<String>` | Notifica a la UI los IDs eliminados por `cleanup_missing_clipboard_files`. |

## 4. Consideraciones de implementación
- Cada nuevo comando debe añadirse a esta tabla, actualizar `modules-overview.md` y registrar su evento en `src-tauri/src/lib.rs`.  
- `cleanup_missing_clipboard_files` debe mantenerse ligero (solo revisa filas con `file_url`). En caso de mover la limpieza a otra tarea, actualizar este doc y el feature correspondiente.  
- Los eventos deben tener listeners explícitos en la UI (`+page.svelte` u otros lugares). Documentar dichos listeners en `svelte-components.md` si impactan componentes.
