---
id: ops-testing-strategy
status: in-progress
owner: dereck
last_updated: 2025-02-14
related_code:
  - src/lib/stores/__tests__/settingsStore.test.ts
  - src/lib/components/settings/__tests__/SettingsContent.test.ts
  - src/lib/components/settings/tabs/__tests__/TabItemsSettings.test.ts
  - src-tauri/src/cleanup/mod.rs
  - src-tauri/tests/cleanup_integration_tests.rs
related_docs:
  - ../30-implementation/modules-overview.md
---

# Testing Strategy

## 1. Propósito
Garantizar que Global Clipboard mantenga calidad y regresiones controladas a medida que escala el código en Svelte 5 + Tauri. Este documento define los tipos de pruebas, herramientas, comandos y gatillos recomendados.

## 2. Pirámide de pruebas

| Capa | Alcance principal | Herramientas | Comando |
| --- | --- | --- | --- |
| Unitarias (UI + lógica) | Stores Svelte, componentes aislados, helpers Rust | Vitest + @testing-library/svelte, `cargo test` | `bun run test:frontend`, `bun run test:unit` |
| Integración | Repositorio SQLite, comandos Tauri, limpieza | `cargo test --test cleanup_integration_tests` | `bun run test:integration` |
| End-to-End / Manual | Flujo Tauri completo (copiar → persistir → limpiar) | `bun run tauri dev` + guía manual | Documentado en `documentation/TESTING_CLEANUP.md` |

## 3. Frontend (Svelte 5)

- **Objetivo:** validar stores y componentes críticos (Settings, Sidebar, futuros tabs) contra regresiones de wiring.
- **Herramientas:** Vitest v4, jsdom, `@testing-library/svelte`. Configuradas en `vitest.config.ts` con plugin SvelteKit y `resolve.conditions=["browser"]`.
- **Suites existentes:**
  - `src/lib/stores/__tests__/settingsStore.test.ts` – sincronización de `maxItemsEnabled`, `maxLocalItems`, retención y persistencia a `settings.json`.
  - `src/lib/components/settings/tabs/__tests__/TabItemsSettings.test.ts` – callbacks del slider, toggle y botones de retención.
  - `src/lib/components/settings/__tests__/SettingsContent.test.ts` – integración con `settingsStore`.
- **Reglas:**
  1. Nuevos componentes deben acompañarse de pruebas en `src/lib/**/__tests__`.
  2. Evitar mocks globales; usar `vi.mock` local y limpiar con `afterEach`.
  3. Ejecutar `bun run test:frontend` antes de cada PR que toque Settings, Sidebar o stores.

## 4. Backend (Rust/Tauri)

- **Unitarias (`bun run test:unit`):** enfocadas en módulos de `src-tauri/src/clipboard`, `db`, `commands`. Usar `#[cfg(test)]`, `TempDir` para filesystem y `insta` opcional para snapshots de SQL si se requiere en el futuro.
- **Integración (`bun run test:integration`):**
  - `cleanup_integration_tests.rs` valida que políticas de retención/exceso borran lo esperado sin afectar favoritos/snippets.
  - Debe generar y limpiar bases temporales; no tocar `~/Library/Application Support/clip`.
- **Cobertura futura:** añadir suites para `clipboard/listener.rs`, `commands/images.rs` y sincronización cuando se implemente Supabase.

## 5. Flujo manual / E2E

- Usar `bun run tauri dev` para validar UI real cuando se agregan features grandes (clipboard capture, settings tabs, shortcuts).
- Guías manuales existentes: `documentation/TESTING_CLEANUP.md` detalla pasos para retención/exceso.
- Registrar pasos en PRs (qué se probó, en qué SO) para trazabilidad.

## 6. Automatización y pipeline

- **Comando estándar:** `bun run test:all` → `bun run check` + Vitest + `cargo test`.
- **Antes de mergear:** ejecutar `bun run test:frontend` y `bun run test:integration` cuando se toquen Settings o limpieza.
- **CI recomendado:** GitHub Actions con tres jobs en paralelo (check + frontend + cargo) para mantener tiempos bajos.

## 7. Mantenimiento y escalabilidad

1. **Naming:** usar `*.test.ts` (frontend) y `*_tests.rs` (Rust). Colocar tests junto al código (`__tests__`).
2. **Datos temporales:** siempre usar `TempDir`/`tempfile` en Rust y `mockLocalStorage` en Svelte; nunca mutar archivos reales.
3. **Documentación:** cuando se agregue una suite importante, actualizar este archivo y `docs/30-implementation/modules-overview.md`.
4. **Gate de calidad:** ningún PR que toque lógica de limpieza/paginación debe fusionarse con tests rojos o sin cobertura correspondiente.

## 8. Próximos pasos

- Añadir pruebas de Tab Memory/Application (auto-start, tray) simulando `tauriEnableAutoStart`.
- Diseñar smoke tests E2E (Bun script) que invoquen comandos Tauri (`test_cleanup_preview`, `test_force_cleanup`) y validen DB stats.
- Integrar cobertura (Vitest `--coverage`, `cargo tarpaulin`) para monitorear tendencias.
