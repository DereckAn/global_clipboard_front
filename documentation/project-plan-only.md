# Clipboard Manager - Plan de Desarrollo (Solo Instrucciones)

## 📋 Información del Proyecto

### Stack Tecnológico
- **Framework**: Tauri v2 (Rust + WebView)
- **UI Framework**: Svelte 5 + SvelteKit
- **Package Manager**: Bun
- **Estilos**: TailwindCSS v4
- **Estado**: Svelte 5 Runes ($state, $derived, $effect)
- **Storage Local**: SQLite (via Tauri)

### Arquitectura
**Clean Architecture** con 4 capas:
1. **Domain** (types/) - Entidades e interfaces
2. **Infrastructure** (lib/tauri/) - Implementaciones (Tauri, API)
3. **Application** (lib/stores/) - Lógica de negocio (Stores)
4. **Presentation** (routes/ + lib/components/) - UI

---

## 📁 Estructura Final del Proyecto

```
global_clipboard/
├── src/
│   ├── routes/                          # Pages (SvelteKit auto-routing)
│   │   ├── +layout.svelte              # Layout principal
│   │   ├── +page.svelte                # Home (/)
│   │   ├── search/+page.svelte         # Búsqueda
│   │   ├── favorites/+page.svelte      # Favoritos
│   │   └── settings/+page.svelte       # Configuración
│   │
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ui/                      # Componentes base
│   │   │   │   ├── Button.svelte
│   │   │   │   ├── Card.svelte
│   │   │   │   ├── Input.svelte
│   │   │   │   ├── Modal.svelte
│   │   │   │   └── Spinner.svelte
│   │   │   │
│   │   │   ├── clipboard/               # Componentes de negocio
│   │   │   │   ├── ClipboardList.svelte
│   │   │   │   ├── ClipboardItem.svelte
│   │   │   │   └── EmptyState.svelte
│   │   │   │
│   │   │   ├── layout/
│   │   │   │   ├── Sidebar.svelte
│   │   │   │   └── Header.svelte
│   │   │   │
│   │   │   └── SearchBar.svelte
│   │   │
│   │   ├── stores/                      # Estado (Svelte 5 Runes)
│   │   │   ├── clipboard.svelte.ts
│   │   │   ├── ui.svelte.ts
│   │   │   └── settings.svelte.ts
│   │   │
│   │   ├── tauri/                       # Integración Tauri
│   │   │   ├── commands.ts
│   │   │   ├── storage.ts
│   │   │   └── clipboard.ts
│   │   │
│   │   ├── types/                       # TypeScript types
│   │   │   ├── clipboard.ts
│   │   │   ├── settings.ts
│   │   │   └── index.ts
│   │   │
│   │   └── utils/
│   │       ├── cn.ts                    # Class merge utility
│   │       └── format.ts                # Format helpers
│   │
│   ├── app.html                         # HTML template
│   └── app.css                          # TailwindCSS v4
│
├── src-tauri/                           # Backend Rust (próxima fase)
│   └── src/
│       ├── main.rs
│       ├── commands/                    # Tauri commands
│       ├── storage/                     # SQLite
│       └── clipboard/                   # OS clipboard
│
├── static/                              # Assets estáticos
├── svelte.config.js
├── vite.config.ts
├── tsconfig.json
├── package.json
└── tailwind.config.js
```

---

## 🎯 Plan de Desarrollo por Fases

### Sistema de Checkpoints
- Después de cada fase: `git commit -m "✅ Fase X completada"`
- Si se corta la conversación: busca el último checkpoint y continúa

---

## FASE 1: Setup Inicial (30-45 min)

### 🎯 Objetivo
Configurar proyecto base con todas las dependencias.

### 📦 Paso 1.1: Instalar Dependencias
```bash
bun add clsx tailwind-merge
bun add -d tailwindcss @tailwindcss/vite
```

**Paquetes**:
- `clsx` + `tailwind-merge` - Combinar clases de Tailwind
- `tailwindcss` + `@tailwindcss/vite` - TailwindCSS v4

**Verificar**: `bun run dev` debe funcionar sin errores

**✅ Checkpoint**: Dependencias instaladas

---

### 🎨 Paso 1.2: Configurar TailwindCSS v4

**Archivos a crear/modificar**:

1. **`src/app.css`**
   - Agregar `@import "tailwindcss"`
   - Agregar tema con `@theme { }` usando CSS variables
   - Dark mode always
   - Estilos base para body y elementos

2. **`vite.config.ts`**
   - Importar `tailwindcss from '@tailwindcss/vite'`
   - Agregar al array de plugins: `tailwindcss()`
   - Configurar servidor en puerto 1420
   - Ignorar watch de `src-tauri/`

3. **`src/app.html`**
   - Verificar que existe tag `%sveltekit.head%`
   - Verificar que existe tag `%sveltekit.body%`


**Verificar**: Fondo debe cambiar según dark/light mode

**✅ Checkpoint**: TailwindCSS v4 configurado

---

### ⚙️ Paso 1.3: Configurar TypeScript

**Archivo**: `tsconfig.json`

**Verificar que tenga**:
- `"extends": "./.svelte-kit/tsconfig.json"`
- `"strict": true`
- Path aliases configurados (`$lib`, `$lib/*`)

**✅ Checkpoint**: TypeScript configurado

---

### 📦 Paso 1.4: Configurar SvelteKit

**Archivo**: `svelte.config.js`

**Configurar**:
- `adapter-static` (ya instalado)
- Pages: `'build'`
- Fallback: `'index.html'` (importante para Tauri)
- `precompress: false`

**¿Por qué static?**: Tauri necesita archivos estáticos, no servidor Node.

**✅ Checkpoint**: SvelteKit configurado para Tauri

---

## FASE 2: Domain Layer - Types (20 min)

### 🎯 Objetivo
Definir entidades y contratos del dominio.

### 📄 Paso 2.1: Types de Clipboard

**Archivo**: `src/lib/types/clipboard.ts`

**Definir**:
- `type ContentType` - Tipos de contenido ('text', 'code', 'link', 'image', etc.)
- `interface ClipboardItem` - Entidad principal
  - id, contentType, contentText, contentMetadata
  - fileUrl, fileName, fileSizeBytes, fileMimeType
  - isFavorite, isSnippet, snippetName
  - createdAt, updatedAt
  - synced, serverId
- `interface CreateClipboardItemDto` - Para crear items
- `interface UpdateClipboardItemDto` - Para actualizar
- `interface ClipboardRepository` - Contrato (interface)
  - Métodos: getItems, getItem, searchItems, getFavorites, getSnippets
  - Métodos: createItem, updateItem, deleteItem, clearAll
- `interface GetItemsOptions` - Filtros y paginación

**✅ Checkpoint**: Clipboard types definidos

---

### 📄 Paso 2.2: Types de Settings

**Archivo**: `src/lib/types/settings.ts`

**Definir**:
- `interface AppSettings`
  - maxLocalItems
  - showHotkey, enableAnalytics
- `const DEFAULT_SETTINGS` con valores por defecto

**✅ Checkpoint**: Settings types definidos

---

### 📄 Paso 2.3: Index de Types

**Archivo**: `src/lib/types/index.ts`

**Hacer**:
- `export * from './clipboard'`
- `export * from './settings'`

**✅ Checkpoint**: Types exportados centralizadamente

---

## FASE 3: Infrastructure Layer - Tauri (30 min)

### 🎯 Objetivo
Implementar Repository con Tauri commands.

### 🛠️ Paso 3.1: Utility - cn

**Archivo**: `src/lib/utils/cn.ts`

**Crear función**:
- Importar `clsx` y `twMerge`
- Función `cn()` que combina ambos
- Retorna clases mergeadas sin conflictos

**Uso**: `cn('px-4', isActive && 'bg-primary', className)`

**✅ Checkpoint**: Utility cn creado

---

### 🛠️ Paso 3.2: Utility - format

**Archivo**: `src/lib/utils/format.ts`

**Crear funciones**:
- `formatDate(date)` - "Just now", "5m ago", "2h ago", etc.
- `truncateText(text, maxLength)` - Truncar con "..."
- `formatFileSize(bytes)` - "1.5 MB", "245 KB"
- `getContentTypeIcon(type)` - Emoji según tipo

**✅ Checkpoint**: Format utilities creados

---

### 🔌 Paso 3.3: Tauri Commands

**Archivo**: `src/lib/tauri/commands.ts`

**Definir funciones async**:
- `tauriGetItems()` - Obtener todos los items
- `tauriGetItem(id)` - Obtener item por ID
- `tauriCreateItem(data)` - Crear item
- `tauriUpdateItem(id, data)` - Actualizar item
- `tauriDeleteItem(id)` - Eliminar item
- `tauriSearchItems(query)` - Buscar items
- `tauriClearAllItems()` - Limpiar todo
- `tauriWriteToClipboard(text)` - Escribir al clipboard OS
- `tauriReadFromClipboard()` - Leer del clipboard OS

**Helper**:
- `deserializeClipboardItem(item)` - Convertir strings de fecha a Date

**Nota**: Estos commands todavía no existen en Rust, son el contrato

**✅ Checkpoint**: Tauri commands tipados

---

### 📦 Paso 3.4: Repository Implementation

**Archivo**: `src/lib/tauri/storage.ts`

**Crear clase**:
- `class TauriClipboardRepository implements ClipboardRepository`
- Implementar todos los métodos de la interface
- Usar los commands de `commands.ts`
- Aplicar filtros y paginación en `getItems()`
- Crear singleton: `export const clipboardRepository`

**✅ Checkpoint**: Repository implementado

---

## FASE 4: Application Layer - Stores (45 min)

### 🎯 Objetivo
Crear stores con Svelte 5 runes.

### 📦 Paso 4.1: Clipboard Store

**Archivo**: `src/lib/stores/clipboard.svelte.ts`

**Crear clase**:
- `class ClipboardStore`
- Properties con `$state()`:
  - `items = $state<ClipboardItem[]>([])`
  - `isLoading = $state(false)`
  - `error = $state<string | null>(null)`
- Métodos async:
  - `loadItems(options?)` - Cargar items del repository
  - `createItem(data)` - Crear y agregar a items
  - `updateItem(id, data)` - Actualizar en items
  - `deleteItem(id)` - Eliminar de items
  - `toggleFavorite(id)` - Toggle favorite
  - `clearAll()` - Limpiar todos
  - `search(query)` - Buscar items
- Singleton: `export const clipboardStore`

**Patrón**: Métodos modifican el estado reactivo directamente

**✅ Checkpoint**: Clipboard store creado

---

### 🎨 Paso 4.2: UI Store

**Archivo**: `src/lib/stores/ui.svelte.ts`

**Crear clase**:
- `class UiStore`
- Properties con `$state()`:
  - `theme = $state<'light' | 'dark' | 'system'>('system')`
  - `isSidebarCollapsed = $state(false)`
  - `isSettingsOpen = $state(false)`
- Constructor:
  - Cargar de localStorage
  - Usar `$effect()` para guardar cambios automáticamente
  - Aplicar theme al DOM
- Métodos:
  - `setTheme(theme)`
  - `toggleSidebar()`
  - `applyTheme()` - Aplica clase dark/light al html
- Singleton: `export const uiStore`

**✅ Checkpoint**: UI store creado

---

### ⚙️ Paso 4.3: Settings Store

**Archivo**: `src/lib/stores/settings.svelte.ts`

**Crear clase**:
- `class SettingsStore`
- Properties con `$state()`:
  - `maxLocalItems`
  - `showHotkey`
  - `enableAnalytics`
- Constructor:
  - Cargar de localStorage
  - Usar `$effect()` para guardar automáticamente
- Métodos:
  - `updateMaxLocalItems(value)`
  - `updateShowHotkey(key)`
  - `toggleAnalytics()`
  - `reset()` - Restaurar valores por defecto
- Singleton: `export const settingsStore`

**✅ Checkpoint**: Settings store creado

---

## FASE 5: UI Components Base (60 min)

### 🎯 Objetivo
Crear componentes base reutilizables.

### Componentes a Crear

**Cada componente debe**:
- Usar TypeScript con `interface Props`
- Usar `$props()` para recibir props
- Usar función `cn()` para clases
- Soportar prop `class` para customización
- Usar snippets: `{@render children?.()}`

---

#### 🔘 Paso 5.1: Button

**Archivo**: `src/lib/components/ui/Button.svelte`

**Props**:
- `variant`: 'default' | 'destructive' | 'outline' | 'ghost'
- `size`: 'default' | 'sm' | 'lg' | 'icon'
- `class`, `onclick`, `disabled`, `type`

**Estilos**:
- Base: rounded-md, font-medium, transition
- Variantes según props
- Sizes según props

**✅ Checkpoint**: Button creado

---

#### 🃏 Paso 5.2: Card Components

**Archivos**:
- `src/lib/components/ui/Card.svelte`
- `src/lib/components/ui/CardHeader.svelte`
- `src/lib/components/ui/CardTitle.svelte`
- `src/lib/components/ui/CardContent.svelte`

**Cada uno**:
- Props: `class`, `children`
- Estilos específicos según componente

**✅ Checkpoint**: Card components creados

---

#### 📝 Paso 5.3: Input

**Archivo**: `src/lib/components/ui/Input.svelte`

**Props**:
- `type`, `placeholder`, `class`, `disabled`
- `value = $bindable('')` - Para two-way binding

**Estilos**: Estilos de input con focus, disabled, etc.

**✅ Checkpoint**: Input creado

---

#### ⏳ Paso 5.4: Spinner

**Archivo**: `src/lib/components/ui/Spinner.svelte`

**Props**:
- `size`: 'sm' | 'md' | 'lg'
- `class`

**Estilos**: Animación spin, border circular

**✅ Checkpoint**: Spinner creado

---

#### 🪟 Paso 5.5: Modal

**Archivo**: `src/lib/components/ui/Modal.svelte`

**Props**:
- `isOpen`, `onClose`, `title`, `class`, `children`

**Funcionalidad**:
- Cerrar con ESC
- Backdrop clickeable
- Portal fixed inset-0

**✅ Checkpoint**: Modal creado

---

## FASE 6: Business Components (60 min)

### 🎯 Objetivo
Componentes de negocio del clipboard.

---

#### 📋 Paso 6.1: ClipboardItem

**Archivo**: `src/lib/components/clipboard/ClipboardItem.svelte`

**Props**: `item: ClipboardItem`

**Funcionalidad**:
- Mostrar contenido del item
- Botones: favorite, copy, delete
- Usar `clipboardStore` para acciones
- Estado local `isCopied` con `$state()`
- Mostrar icon según contentType
- Mostrar fecha formateada

**Estilos**: Card con hover effects

**✅ Checkpoint**: ClipboardItem creado

---

#### 📜 Paso 6.2: ClipboardList

**Archivo**: `src/lib/components/clipboard/ClipboardList.svelte`

**Props**:
- `items: ClipboardItem[]`
- `isLoading?: boolean`
- `emptyMessage?: string`

**Renderizado**:
- Si loading → Spinner
- Si empty → EmptyState
- Si items → Loop con ClipboardItem

**✅ Checkpoint**: ClipboardList creado

---

#### 🗂️ Paso 6.3: EmptyState

**Archivo**: `src/lib/components/clipboard/EmptyState.svelte`

**Props**: `message: string`, `icon?: string`

**Renderizado**: Icon grande + mensaje centrado

**✅ Checkpoint**: EmptyState creado

---

#### 🔍 Paso 6.4: SearchBar

**Archivo**: `src/lib/components/SearchBar.svelte`

**Props**:
- `value = $bindable('')`
- `placeholder?`
- `onClear?`

**Renderizado**: Input con icon de búsqueda + botón clear

**✅ Checkpoint**: SearchBar creado

---

#### 🧭 Paso 6.5: Sidebar

**Archivo**: `src/lib/components/layout/Sidebar.svelte`

**Funcionalidad**:
- Array de navItems (label, icon, path)
- Usar `$page.url.pathname` para active
- Usar `goto()` para navegación
- Usar `uiStore` para collapsed state
- Toggle collapse button
- Footer con info de usuario

**✅ Checkpoint**: Sidebar creado

---

#### 🎯 Paso 6.6: Header

**Archivo**: `src/lib/components/layout/Header.svelte`

**Funcionalidad**:
- Título de la app
- Botón toggle theme
- Usar `uiStore`

**✅ Checkpoint**: Header creado

---

## FASE 7: Pages (SvelteKit Routes) (45 min)

### 🎯 Objetivo
Crear todas las páginas del MVP.

---

#### 🏠 Paso 7.1: Root Layout

**Archivo**: `src/routes/+layout.svelte`

**Estructura**:
- Importar `../app.css`
- Sidebar + contenido
- Usar `uiStore.isSidebarCollapsed` para margin
- Header
- `<slot />` para contenido de páginas

**✅ Checkpoint**: Root layout creado

---

#### 📋 Paso 7.2: Home Page

**Archivo**: `src/routes/+page.svelte`

**Funcionalidad**:
- `onMount()` → `clipboardStore.loadItems()`
- Botón "Clear All" con confirmación
- Renderizar `<ClipboardList items={clipboardStore.items} />`

**✅ Checkpoint**: Home page creada

---

#### 🔍 Paso 7.3: Search Page

**Archivo**: `src/routes/search/+page.svelte`

**Funcionalidad**:
- `query = $state('')`
- `searchResults = $state<ClipboardItem[]>([])`
- Usar `$effect()` para debounced search (300ms)
- SearchBar con bind:value
- Renderizar resultados

**✅ Checkpoint**: Search page creada

---

#### ⭐ Paso 7.4: Favorites Page

**Archivo**: `src/routes/favorites/+page.svelte`

**Funcionalidad**:
- `onMount()` → cargar items
- Usar `$derived()` para filtrar favoritos
- Renderizar ClipboardList con filtrados

**✅ Checkpoint**: Favorites page creada

---

#### ⚙️ Paso 7.5: Settings Page

**Archivo**: `src/routes/settings/+page.svelte`

**Secciones**:
1. **Appearance**: Theme selector (light/dark/system)
2. **Clipboard**: Toggle auto-save, mostrar max items
3. **About**: Version, subscription tier

**Usar**: `uiStore` y `settingsStore`

**✅ Checkpoint**: Settings page creada

---

## FASE 8: Testing y Verificación (15 min)

### 🎯 Objetivo
Verificar que todo funciona.

### 📋 Checklist

```bash
# Type check
bun run check

# Desarrollo
bun run dev
```

**Verificar**:
- [ ] App carga sin errores
- [ ] Sidebar funciona
- [ ] Navegación entre páginas funciona
- [ ] Theme toggle funciona (dark/light)
- [ ] Sidebar collapse funciona
- [ ] UI components se ven bien
- [ ] Lista de items vacía (normal, falta backend)

**✅ Checkpoint**: Frontend completo y verificado

---

## 🚧 PRÓXIMAS FASES (Después de Frontend)

### FASE 9: Backend Rust (3-4 horas)

**Objetivo**: Implementar los Tauri commands que el frontend espera.

**Tareas**:
1. Setup SQLite con `rusqlite` o `sqlx`
2. Crear schema y migrations
3. Implementar todos los commands:
   - `get_clipboard_items`
   - `create_clipboard_item`
   - `update_clipboard_item`
   - `delete_clipboard_item`
   - `search_clipboard_items`
   - `clear_all_clipboard_items`
   - `write_to_clipboard`
   - `read_from_clipboard`
4. Clipboard listener (detectar cambios en OS clipboard)
5. Testing

**Estructura Rust**:
```
src-tauri/src/
├── main.rs
├── commands/
│   ├── mod.rs
│   └── clipboard.rs
├── storage/
│   ├── mod.rs
│   ├── database.rs
│   └── models.rs
└── clipboard/
    ├── mod.rs
    └── listener.rs
```

---

### FASE 10: Integración y Testing (1-2 horas)

**Tareas**:
1. Conectar frontend con backend
2. Testing E2E completo
3. Bug fixes
4. Polish UX

---

### FASE 11: Features Adicionales (Opcional)

**Tareas**:
1. Keyboard shortcuts
2. System tray
3. Notifications
4. Animations
5. Error handling mejorado

---

## 📝 Comandos de Referencia

```bash
# Desarrollo
bun run dev              # Solo frontend (Vite)
bun run tauri dev        # Frontend + Backend (app completa)

# Build
bun run build            # Build frontend
bun run tauri build      # Build app completa (.msi, .dmg)

# Testing
bun run check            # Type check
bun run check:watch      # Type check en watch mode

# Linting
bun run lint             # ESLint (si configurado)
```

---

## 🎓 Conceptos Clave

### Svelte 5 Runes
- `$state()` - Variable reactiva
- `$derived()` - Valor computado (como computed)
- `$effect()` - Side effect (como useEffect)
- `$bindable()` - Two-way binding en props
- `$props()` - Recibir props con destructuring

### Clean Architecture
- **Domain** → Entidades puras, sin dependencias
- **Application** → Lógica de negocio, usa Domain
- **Infrastructure** → Implementaciones externas (Tauri, DB)
- **Presentation** → UI, usa todo lo demás

### Repository Pattern
- Interface en Domain
- Implementation en Infrastructure
- Stores usan la interface, no la implementation
- Fácil cambiar de Tauri a API HTTP

---

## ⚠️ Notas Importantes

### Svelte 5 vs React
- No `useState`, usar `$state()`
- No `useEffect`, usar `$effect()`
- No `useMemo`, usar `$derived()`
- No React Query necesario, stores son suficientes
- Props con `$props()`, no destructuring directo
- Children con snippets: `{@render children?.()}`

### TailwindCSS v4 vs v3
- Configuración en CSS, no JS
- `@import "tailwindcss"` en vez de directives
- `@theme { }` para variables
- Usa `oklch()` para colores
- No postcss.config.js necesario

### SvelteKit + Tauri
- Adapter static es obligatorio
- Fallback index.html necesario
- Server-side features no funcionan (SSR, +server.ts)
- Todo es client-side

---

## 🎯 Resumen Ejecutivo

### Tiempo Estimado Total
- **Fase 1-8** (Frontend): 6-7 horas
- **Fase 9** (Backend): 3-4 horas
- **Fase 10** (Integración): 1-2 horas
- **Total MVP**: 10-13 horas

### Al Completar Fase 8 tendrás:
- ✅ Frontend completo y funcional
- ✅ UI/UX lista
- ✅ Navegación funcionando
- ✅ Stores con estado reactivo
- ✅ Componentes reutilizables
- ❌ Backend (siguiente fase)

### Deliverables
1. App Tauri funcional (después de Fase 9)
2. SQLite local storage
3. Clipboard manager completo
4. Multi-página (Home, Search, Favorites, Settings)
5. Dark/Light mode
6. Responsive design

---

## 📚 Recursos Útiles

- **Svelte 5**: https://svelte-5-preview.vercel.app/
- **SvelteKit**: https://kit.svelte.dev/
- **Tauri**: https://v2.tauri.app/
- **TailwindCSS v4**: https://tailwindcss.com/docs/v4-beta
- **Arquitectura Completa**: Ver `clipboard-manager-architecture-v2.md`

---

**Última actualización**: Enero 2025
**Versión**: 1.0 (Svelte 5 + Tailwind v4)
