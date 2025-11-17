---
id: impl-modules-overview
status: in-progress
owner: dereck
last_updated: 2025-02-14
related_code:
  - src-tauri/src/db/repository.rs
  - src-tauri/src/lib.rs
  - src/lib/stores/clipboard.svelte.ts
related_docs:
  - tauri-commands.md
  - ../10-features/feat-missing-file-cleanup.md
---

# Modules Overview

## 1. Backend (Rust)

| Módulo | Ubicación | Responsabilidad | Cambios recientes |
| --- | --- | --- | --- |
| `db::repository` | `src-tauri/src/db/repository.rs` | CRUD sobre SQLite, paginación, FTS5, limpieza de assets. | `cleanup_missing_file_records()` y `prune_missing_file_items()` eliminan registros cuyos archivos ya no existen y devuelven los IDs para la UI. |
| `commands::clipboard` | `src-tauri/src/commands/clipboard.rs` | Comandos CRUD + utilidades de portapapeles. | Nuevo comando `cleanup_missing_clipboard_files` expone la limpieza de archivos huérfanos. |
| `lib.rs` (builder) | `src-tauri/src/lib.rs` | Configura ventanas, tray, eventos y listeners. | - Limpieza en `WindowEvent::Focused(false)` → `clipboard-items-removed`. <br> - En macOS el tray carga `tray_icon_template.png` y lo marca como template (con fallback). |

## 2. Frontend (Svelte)

| Módulo | Ubicación | Responsabilidad | Cambios recientes |
| --- | --- | --- | --- |
| `clipboardStore` | `src/lib/stores/clipboard.svelte.ts` | Estado global de items, paginación, búsqueda. | Ya no ejecuta limpieza por intervalos; se limita a reflejar `clipboard-item-added` y, desde `+page.svelte`, recibe `clipboard-items-removed`. |
| `+page.svelte` | `src/routes/+page.svelte` | Vista principal (Sidebar + Content). | Añadió listener para `clipboard-items-removed` que depura el store y recalcula `totalItems`. |

## 3. Integraciones clave
- **Eventos Tauri → WebView**: `clipboard-item-added`, `clipboard-items-removed`. Mantener listeners centralizados para evitar estados inconsistentes.
- **Asset cleanup**: `clipboard::asset_cleanup` se reutiliza en `prune_missing_file_items` para borrar thumbnails y archivos asociados.

## 4. Futuras actualizaciones
- Cuando se agreguen nuevas ventanas o procesos en background, asegurar que también se suscriban/emitan los mismos eventos de limpieza.
- Documentar en `svelte-components.md` los componentes específicos que reaccionen a `clipboard-items-removed` (ej. Sidebar) una vez se detalle ese archivo.
