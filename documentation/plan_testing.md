Plan Próximo (Testing)
      1. Desbloquear Vitest: Reintentar bun run test:frontend en un entorno donde esbuild
         pueda crear su proceso helper (el error actual es The service was stopped: write
         EPIPE). Una vez pase, activar bun run test:frontend:watch para desarrollo continuo.
      2. Ampliar cobertura frontend:
          - Añadir Vitest + @testing-library/svelte con jsdom en vitest.config.ts para probar
            componentes (SidebarItem, ContentViewer) y stores (mockeando tauri invocations).
          - Cubrir utilidades críticas (clipboard.svelte.ts load/search, settings store, cn,
            highlight) creando fixtures en src/lib/**/__tests__.
      3. Backend adicional:
          - Tests para commands/settings.rs y commands/clipboard.rs, aislando dependencias
            con tauri::test::mock_builder o wrappers para AppHandle.
          - Casos del monitor/thumbnail pipeline usando TempDir y archivos dummy para
            verificar external_missing, Quick Look cache y deduplicación de hashes.
          - Confirmar que cleanup_integration_tests.rs cubre las nuevas rutas (text/image/
            file cleanup) o agregar otro _integration_tests.rs para thumbnails.
      4. CI integración:
          - Una vez Vitest funcione, actualizar workflows para ejecutar bun run test:frontend
            además de bun run check y los tests Rust, garantizando paridad local/CI.
          - Documentar en PR el output de bun run test:all.
  - Documentación: AGENTS.md ahora refleja que existen suites unitarias en image_handler.rs,
    db/repository.rs, commands/images.rs, y que Vitest está disponible con los nuevos scripts
    (bun run test:frontend, bun run test:frontend:watch, bun run test:all). También se anotó
    la recomendación de reportar cuando Vitest falle por restricciones de entorno.
  - Estado de pruebas:
      - ✅ cargo test image_handler -- --nocapture
      - ✅ cargo test repository -- --nocapture
      - ✅ cargo test ensure_thumbnail -- --nocapture
      - ⚠️ bun run test:frontend (falla aquí por esbuild → write EPIPE; repetir localmente
        donde el binario pueda iniciarse)



1. Medir en ejecución

  - macOS: usa Activity Monitor, sample u htop para ver CPU/RAM en tiempo real. También
    puedes lanzar la app desde terminal con time o usar ps -p <pid> -o %cpu,%mem,rss.
  - macOS Instruments: abre Xcode > Open Developer Tool > Instruments. El template “Time
    Profiler” y “Allocations” te dejan identificar qué funciones de Rust o Svelte están
    consumiendo más CPU o memoria. Ejecuta el binario Tauri (target/debug/global_clipboard)
    bajo Instruments y registra escenarios (copiar imágenes grandes, búsquedas, etc.).
  - Windows/Linux: Process Explorer / perf top / valgrind --tool=massif para medir uso de
    memoria y CPU.

  2. Perfilar Rust

  - Usa cargo bench o cargo flamegraph (con perf/dtrace) sobre módulos intensivos
    (clipboard::monitor, db::repository). Esto te dirá qué funciones bloquean más tiempo.
  - Para memoria, heaptrack o massif con el binario release (cargo build --release).
    Ejecuta acciones concretas (copiar 100 imágenes, búsquedas) y revisa dónde crecen los
    allocations.

  3. Perfilar frontend (Svelte/Tauri)

  - Arranca con bun run tauri dev y abre las DevTools (ya se abren por defecto en debug). Usa
    la pestaña Performance (Chrome) para ver render loops, watchers, stores. Revisa si hay
    efectos ($effect) que se disparan más de lo necesario.
  - Si la WebView consume, revisa timeline y memory para encontrar componentes que se
    renderizan en exceso (por ejemplo, SidebarItem cuando hay cientos de items).

  4. Telemetría ligera (local)

  - Agrega logs temporales para medir tiempos de consulta en ClipboardRepository (start =
    Instant::now(); ... println!("get_items_paginated took {:?}", start.elapsed());).
  - Usa tracing con niveles INFO/DEBUG para saber cuánto tarda cada paso y correlacionar con
    picos de CPU.

  5. Optimización general

  - Revisa queries de SQLite (EXPLAIN QUERY PLAN) para asegurarte de que usan índices.
  - Limita la cantidad de items renderizados en Svelte (virtual scroll). Ya tienes
    paginación, pero podrías “window” el DOM para evitar que el WebView pinte cientos de
    nodos.
  - Para imágenes, evita cargar thumbnails repetidamente: usa caché (convertFileSrc +
    resolveThumbnail).
  - Minimiza watchers globales: clipboard-items-removed ya actualiza store incrementalmente;
    evita recargar toda la lista salvo cuando sea necesario.

  6. Ruta de degradación

  - Define presupuestos (“la app no debe superar 150 MB RSS con 1k items, ni 10% CPU en
    idle”). Mide contra esos objetivos después de cada cambio grande.
  - Considera publicar un modo “release profiling”: cargo build --release --features tracing
    y usa tauri-plugin-log para ver eventos en la consola.