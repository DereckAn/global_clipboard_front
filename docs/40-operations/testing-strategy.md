---
id: ops-testing-strategy
status: active
owner: dereck
last_updated: 2025-12-06
related_code:
  - src/lib/stores/__tests__/settingsStore.test.ts
  - src/lib/components/settings/__tests__/SettingsContent.test.ts
  - src/lib/components/settings/tabs/__tests__/TabItemsSettings.test.ts
  - src/lib/utils/__tests__/highlight.test.ts
  - src-tauri/src/cleanup/mod.rs
  - src-tauri/tests/cleanup_integration_tests.rs
  - vitest.config.ts
  - vitest.setup.ts
related_docs:
  - ../30-implementation/modules-overview.md
---

# Testing Strategy

## 1. Propósito

Garantizar que Global Clipboard mantenga calidad y regresiones controladas a medida que escala el código en Svelte 5 + Tauri. Este documento define los tipos de pruebas, herramientas, comandos y gatillos recomendados.

## 2. Pirámide de pruebas

| Capa                    | Alcance principal                                   | Herramientas                                   | Comando                                           |
| ----------------------- | --------------------------------------------------- | ---------------------------------------------- | ------------------------------------------------- |
| Unitarias (UI + lógica) | Stores Svelte, componentes aislados, helpers Rust   | Vitest + @testing-library/svelte, `cargo test` | `bun run test:frontend`, `bun run test:unit`      |
| Integración             | Repositorio SQLite, comandos Tauri, limpieza        | `cargo test --test cleanup_integration_tests`  | `bun run test:integration`                        |
| End-to-End / Manual     | Flujo Tauri completo (copiar → persistir → limpiar) | `bun run tauri dev` + guía manual              | Documentado en `documentation/TESTING_CLEANUP.md` |

## 3. Frontend (Svelte 5)

### 3.1 Configuración

El proyecto usa **Svelte 5 con runes** (`$state`, `$derived`, `$effect`), lo cual requiere configuración especial de Vitest.

#### Archivos de configuración clave:

**`vitest.config.ts`**

```typescript
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [
    svelte({
      compilerOptions: {
        runes: true, // ← Necesario para que $state y otros runes funcionen
      },
    }),
  ],
  resolve: {
    conditions: ["browser"],
    alias: {
      $lib: resolve(__dirname, "./src/lib"),
    },
  },
  test: {
    environment: "jsdom",
    include: ["src/**/*.{test,spec}.{ts,js}"],
    globals: true,
    setupFiles: ["./vitest.setup.ts"],
  },
});
```

**`vitest.setup.ts`**

```typescript
import { vi } from "vitest";

// Mock de Tauri API - necesario porque Tauri no existe en el entorno de test
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
  emit: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));
```

### 3.2 Dependencias requeridas

| Librería                       | Propósito                                                      |
| ------------------------------ | -------------------------------------------------------------- |
| `vitest`                       | Test runner compatible con Vite                                |
| `jsdom`                        | Simula el DOM en Node.js                                       |
| `@testing-library/svelte`      | Helpers para testear componentes Svelte                        |
| `@sveltejs/vite-plugin-svelte` | Compila archivos `.svelte` y `.svelte.ts` con soporte de runes |

### 3.3 Consideraciones especiales para Svelte 5 Runes

Los archivos `.svelte.ts` que usan runes (`$state`, `$derived`, etc.) **deben ser compilados** por el plugin de Svelte. Por eso:

1. El `vitest.config.ts` usa `@sveltejs/vite-plugin-svelte` en lugar de `sveltekit()`
2. Se habilita `compilerOptions.runes: true`
3. Los mocks de Tauri se configuran en `vitest.setup.ts` globalmente

### 3.4 Suites existentes

| Suite              | Ubicación                                                             | Propósito                                      |
| ------------------ | --------------------------------------------------------------------- | ---------------------------------------------- |
| Settings Store     | `src/lib/stores/__tests__/settingsStore.test.ts`                      | Sincronización de cleanup settings con backend |
| Tab Items Settings | `src/lib/components/settings/tabs/__tests__/TabItemsSettings.test.ts` | Callbacks del slider, toggle y retención       |
| Settings Content   | `src/lib/components/settings/__tests__/SettingsContent.test.ts`       | Integración con `settingsStore`                |
| Highlight Utils    | `src/lib/utils/__tests__/highlight.test.ts`                           | Función de highlighting para búsqueda          |

### 3.5 Reglas para tests de frontend

1. **Ubicación:** Nuevos tests deben ir en carpeta `__tests__` junto al código que prueban.
2. **Naming:** Usar `*.test.ts` para archivos de test.
3. **Mocks locales:** Preferir `vi.mock()` al inicio del archivo sobre mocks globales.
4. **Limpiar estado:** Usar `beforeEach` con `vi.clearAllMocks()` y `localStorage.clear()`.
5. **Tauri commands:** Mockear en cada archivo de test los comandos específicos que se usan.

### 3.6 Ejemplo de test para stores con runes

```typescript
import { beforeEach, describe, expect, it, vi } from "vitest";

// Mock de comandos ANTES de importar el store
vi.mock("$lib/tauri/commands", () => ({
  tauriGetCleanupSettings: vi.fn().mockResolvedValue({
    maxItemsEnabled: true,
    maxLocalItems: 500,
  }),
  tauriSaveCleanupSettings: vi.fn().mockResolvedValue(undefined),
}));

// Mock de eventos de Tauri
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

import { SettingsStore } from "$lib/stores/settings.svelte";

describe("SettingsStore", () => {
  let store: SettingsStore;

  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
    store = new SettingsStore();
  });

  it("loads settings from backend", async () => {
    await store.loadSettings();
    expect(store.maxItemsEnabled).toBe(true);
  });
});
```

## 4. Backend (Rust/Tauri)

### 4.1 Comandos de testing

| Comando                    | Descripción                                |
| -------------------------- | ------------------------------------------ |
| `bun run test:unit`        | Tests unitarios de Rust (módulos internos) |
| `bun run test:integration` | Tests de integración (cleanup, DB)         |
| `bun run test`             | Todos los tests de Rust (`cargo test`)     |

### 4.2 Tests unitarios

Enfocados en módulos de `src-tauri/src/`:

- `clipboard/` - Image handler, file handler, types
- `db/` - Repository, schema, FTS migration
- `commands/` - Clipboard, settings, images

**Convenciones:**

- Usar `#[cfg(test)]` para módulos de test
- Usar `TempDir` para tests que requieren filesystem
- Usar `insta` opcional para snapshots de SQL si se requiere

### 4.3 Tests de integración

Ubicados en `src-tauri/tests/`:

- `cleanup_integration_tests.rs` - Valida políticas de retención y exceso

**Requisitos:**

- Generar y limpiar bases de datos temporales
- Nunca tocar `~/Library/Application Support/clip` (producción)
- Validar que favoritos/snippets no son afectados por cleanup

### 4.4 Cobertura futura

Añadir suites para:

- `clipboard/listener.rs` - Event handling
- `commands/images.rs` - Thumbnail generation
- Sincronización cuando se implemente Supabase

## 5. Flujo manual / E2E

- Usar `bun run tauri dev` para validar UI real cuando se agregan features grandes (clipboard capture, settings tabs, shortcuts).
- Guías manuales existentes: `documentation/TESTING_CLEANUP.md` detalla pasos para retención/exceso.
- Registrar pasos en PRs (qué se probó, en qué SO) para trazabilidad.

## 6. Automatización y pipeline

### 6.1 Comandos disponibles

```bash
# Frontend
bun run test:frontend        # Vitest una vez
bun run test:frontend:watch  # Vitest en modo watch

# Backend
bun run test:unit            # Cargo tests unitarios
bun run test:integration     # Cleanup integration tests
bun run test                 # Todos los tests de Rust

# Completo
bun run test:all             # check + frontend + Rust tests
bun run check                # Type checking con svelte-check
```

### 6.2 Pre-merge checklist

Antes de mergear un PR:

1. `bun run check` - Sin errores de TypeScript
2. `bun run test:frontend` - Todos los tests pasan
3. `bun run test` - Cargo tests pasan
4. `cargo fmt && cargo clippy --fix --allow-dirty` - Código formateado

### 6.3 CI recomendado

GitHub Actions con tres jobs en paralelo:

1. **check** - `bun run check`
2. **frontend** - `bun run test:frontend`
3. **backend** - `bun run test`

## 7. Mantenimiento y escalabilidad

1. **Naming:** usar `*.test.ts` (frontend) y `*_tests.rs` (Rust). Colocar tests junto al código (`__tests__`).
2. **Datos temporales:** siempre usar `TempDir`/`tempfile` en Rust y `mockLocalStorage` en Svelte; nunca mutar archivos reales.
3. **Documentación:** cuando se agregue una suite importante, actualizar este archivo y `docs/30-implementation/modules-overview.md`.
4. **Gate de calidad:** ningún PR que toque lógica de limpieza/paginación debe fusionarse con tests rojos o sin cobertura correspondiente.

## 8. Troubleshooting

### Error: `$state is not defined`

**Causa:** El plugin de Svelte no está compilando los runes correctamente.

**Solución:** Verificar que `vitest.config.ts` tenga:

```typescript
plugins: [
  svelte({
    compilerOptions: {
      runes: true,
    },
  }),
];
```

### Error: `document is not defined`

**Causa:** El entorno de test no es jsdom.

**Solución:** Verificar en `vitest.config.ts`:

```typescript
test: {
  environment: "jsdom",
}
```

### Error: `Failed to invoke 'command_name'`

**Causa:** Tauri API no está mockeada.

**Solución:** Agregar mock en el test o en `vitest.setup.ts`:

```typescript
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));
```

### Tests fallan al importar módulos de Tauri

**Causa:** Alias `$lib` no configurado o condiciones de resolución incorrectas.

**Solución:** Verificar en `vitest.config.ts`:

```typescript
resolve: {
  conditions: ["browser"],
  alias: {
    $lib: resolve(__dirname, "./src/lib"),
  },
},
```

### Tests pasan localmente pero fallan en CI

**Posibles causas:**

1. Diferencias de versión de Node/Bun
2. Mocks que dependen del sistema de archivos
3. Tests que dependen del orden de ejecución

**Solución:**

- Usar `beforeEach` para resetear estado
- Evitar dependencias de archivos reales
- Agregar `vi.clearAllMocks()` en cada test

## 8. Próximos pasos

- Añadir pruebas de Tab Memory/Application (auto-start, tray) simulando `tauriEnableAutoStart`.
- Diseñar smoke tests E2E (Bun script) que invoquen comandos Tauri (`test_cleanup_preview`, `test_force_cleanup`) y validen DB stats.
- Integrar cobertura (Vitest `--coverage`, `cargo tarpaulin`) para monitorear tendencias.
