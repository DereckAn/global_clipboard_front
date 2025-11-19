---
id: ai-context-short
status: in-progress
owner: dereck
last_updated: 2025-02-14
related_code:
  - src/lib/components/header/Header.svelte
  - src/lib/components/settings/SettingsContent.svelte
  - src/lib/stores/settings.svelte.ts
  - src-tauri/src/commands/settings.rs
related_docs:
  - ../40-operations/testing-strategy.md
  - ../30-implementation/modules-overview.md
  - documentation/Plan_Maestro.md
---

# AI Context – Quick Reference

## Proyecto en una frase
Global Clipboard Manager es una app de escritorio Tauri (Rust + Svelte 5) que monitorea el portapapeles, guarda items en SQLite y ofrece una UI con search, favoritos y políticas de limpieza.

## Arquitectura resumida
- **Frontend:** SvelteKit (SPA), Tailwind v4, stores con runas (`$state`). Carpetas clave: `src/lib/components`, `src/lib/stores`, `src/lib/tauri`.
- **Backend:** `src-tauri/` con comandos en Rust, repositorio SQLite (`clipboard_items`), monitor de portapapeles e imagen, limpieza (`src-tauri/src/cleanup`).
- **Comunicación:** `@tauri-apps/api/core.invoke` encapsulado en `src/lib/tauri/commands.ts`.

## Funcionalidades prioritarias (feb/2025)
1. **Settings avanzados:** tabs (Account, Clipboard, Item Management, Memory, etc.). Lógica en `settingsStore`, UI en `SettingsContent`.
2. **Límites y retención:** configurables, guardados en `settings.json` mediante `save_cleanup_settings` y consumidos por `get_cleanup_settings`.
3. **Limpieza automática:** comandos `cleanup_old_items`, `cleanup_excess_items`, test manual en `documentation/TESTING_CLEANUP.md`.

## Pruebas disponibles
- `bun run test:frontend`: Vitest + Testing Library (`settingsStore`, `TabItemsSettings`, `SettingsContent`).
- `bun run test:unit`: `cargo test --lib`.
- `bun run test:integration`: `cargo test --test cleanup_integration_tests`.
- `bun run test:all`: check + frontend + cargo.
Ver `docs/40-operations/testing-strategy.md` para detalles.

## Puntos de atención para la IA
1. **Compatibilidad Svelte 5:** no usar `new Component()`. Para pruebas, usar Testing Library.
2. **Persistencia de settings:** cualquier cambio en límites/retención debe actualizar `settingsStore` y `src-tauri/src/commands/settings.rs`.
3. **Documentación viva:** seguir la estructura de `documentation/Plan_Maestro.md`. Actualizar módulos en `docs/30-implementation` y estrategias en `docs/40-operations`.
4. **Eventos Tauri:** UI escucha `clipboard-item-added` y `clipboard-items-removed`. Asegurar sincronización al tocar stores o componentes principales.

## Comandos habituales
```bash
bun run dev            # frontend
bun run tauri dev      # app completa
bun run check          # svelte-check
bun run test:frontend  # Vitest
cd src-tauri && cargo test   # backend
```

## Próximos pasos abiertos
- Documentar pruebas para Memory/Application tabs y captura de pantalla.
- Integrar CI con `bun run test:all`.
- Preparar ADR cuando se añadan dependencias mayores (ej. Supabase sync).
