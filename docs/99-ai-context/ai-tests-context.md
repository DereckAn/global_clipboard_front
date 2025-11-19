---
id: ai-tests-context
status: in-progress
owner: dereck
last_updated: 2025-02-14
related_code:
  - src/lib/stores/__tests__/settingsStore.test.ts
  - src/lib/components/settings/__tests__/SettingsContent.test.ts
  - src/lib/components/settings/tabs/__tests__/TabItemsSettings.test.ts
  - src-tauri/tests/cleanup_integration_tests.rs
related_docs:
  - ai-context-short.md
  - ../40-operations/testing-strategy.md
---

# AI Tests Context

## Suite Overview

| Área | Ficheros | Objetivo |
| --- | --- | --- |
| **Frontend – Settings** | `src/lib/stores/__tests__/settingsStore.test.ts` | Garantiza que `settingsStore` sincroniza `maxItemsEnabled`, `maxLocalItems`, retención y llama a `tauriSaveCleanupSettings`. |
| | `src/lib/components/settings/tabs/__tests__/TabItemsSettings.test.ts` | Verifica que el toggle, slider y botones invocan callbacks correctos. |
| | `src/lib/components/settings/__tests__/SettingsContent.test.ts` | Integra el tab “Item Management” con `settingsStore`. |
| **Backend – Limpieza** | `src-tauri/tests/cleanup_integration_tests.rs` | Comprueba retención/exceso sobre SQLite real, respetando favoritos/snippets. |

## Comandos clave
```bash
bun run test:frontend      # Vitest + @testing-library/svelte (jsdom)
bun run test:unit          # cargo test --lib
bun run test:integration   # cargo test --test cleanup_integration_tests
bun run test:all           # check + frontend + cargo test
```

## Configuración Vitest
- `vitest.config.ts` usa plugin SvelteKit, `environment: "jsdom"` y `resolve.conditions = ["browser"]`.
- Testing Library se importa desde `@testing-library/svelte`; limpiar con `cleanup()` en `afterEach`.

## Reglas al escribir pruebas
1. **Colocar en `__tests__`** junto al código (`src/lib/.../__tests__`).
2. **Usar mocks locales** (`vi.mock`) para stores y comandos Tauri; resetear en `afterEach`.
3. **Evitar `new Component()`**. Montar con Testing Library.
4. **Rust:** usar `TempDir`/`tempfile` para DBs temporales, no tocar `~/Library/Application Support/clip`.
5. **Actualizar documentación** (`docs/40-operations/testing-strategy.md`) cuando se agregue una suite relevante.

## Qué debe validar la IA antes de un PR
- `bun run check`
- `bun run test:frontend`
- `bun run test:unit`
- `bun run test:integration` (si cambia limpieza o backend)

## Próximas suites sugeridas
- Pruebas para Tab Memory/Application (auto-start, tray toggles) simulando `tauriEnableAutoStart`.
- Tests para captura de pantalla una vez se implemente la feature (comando + botón en Header).
- Smoke tests E2E mediante scripts Bun que invoquen comandos `test_cleanup_preview` y verifiquen stats.
