# macOS Preview Pipeline (v1.2)

## Resumen

- Los archivos e imágenes que provienen del portapapeles **ya no se copian** a `~/Library/Application Support/clip/files`.
- Persistimos solamente:
  - `file_url`: ruta original en el filesystem.
  - `content_metadata.external_path`: copia de esa ruta, con `external_missing` para reflejar si dejó de existir.
  - Caché de miniaturas (`file_thumbnails/<hash>.png`) producidas por **Quick Look / QLThumbnailImageCreate**.
- Los formatos textuales (.log, .md, .rs, .ts, etc.) almacenan `text_preview` (primeros ~32 KB) y `preview_language`. En el frontend se muestran en un bloque monoespaciado, similar a Raycast.
- Eliminamos los archivos del usuario solamente cuando el mismo usuario los borra en Finder; desde la app se elimina únicamente el registro/caché.

## Flujo macOS

1. El monitor detecta `ClipboardContent::File` o `ClipboardContent::ImageFile`.
2. Se genera metadata (`hash`, `mime`, `original_name`, `external_path`).
3. Si el MIME/ext es textual → se guarda `preview_type="text"` + `text_preview`.
4. Si no → se genera thumbnail via Quick Look y se almacena en caché (`preview_type="image"`, `thumbnail_path`).
5. En `get_items_paginated`, antes de retornar cada item:
   - Se comprueba `external_path`/`file_url`.
   - Si no existe → `external_missing = true` en metadata.
6. `ContentViewer`:
   - Muestra “File not found” + acciones deshabilitadas cuando `external_missing` es true.
   - Para previews de texto, muestra el snippet; para el resto, la miniatura Quick Look.

## Pendientes / Roadmap

- Marcar automáticamente `external_missing` cuando, al abrir un item, falle la apertura del archivo.
- Replicar el enfoque “sin copia” en Windows (IShellItemImageFactory) y Linux (cuando sea viable).
- Considerar un comando de limpieza que quite todas las miniaturas huérfanas (`file_thumbnails/*.png`) cuyo `hash` ya no esté asociado a un item.
