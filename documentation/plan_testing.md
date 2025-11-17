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