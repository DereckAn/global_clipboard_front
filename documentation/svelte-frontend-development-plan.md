# Clipboard Manager - Plan de Desarrollo Frontend (Svelte 5)

## Información del Proyecto

### Stack Tecnológico
- **Framework**: Tauri v2 (Rust + WebView)
- **UI Framework**: Svelte 5 + SvelteKit
- **Package Manager**: Bun
- **Estilos**: TailwindCSS v4
- **Estado**: Svelte 5 Runes ($state, $derived, $effect)
- **Storage Local**: SQLite (via Tauri)
- **Build Tool**: Vite

### Diferencias Clave con React

**Svelte 5 Runes** (nuevo sistema de reactividad):
- `$state()` - Variables reactivas
- `$derived()` - Valores computados
- `$effect()` - Side effects
- No más `let` reactivo, no más stores tradicionales

**TailwindCSS v4**:
- Nueva sintaxis con `@theme`
- CSS nativo en vez de PostCSS
- Configuración en CSS, no JS

### Arquitectura Frontend

Usamos **Clean Architecture** adaptada para Svelte:

```
┌─────────────────────────────────────────────────────────┐
│                    PRESENTATION                          │
│        (routes/ + lib/components/)                       │
│  • Solo renderizado y eventos UI                        │
│  • Svelte components                                    │
└──────────────────────┬──────────────────────────────────┘
                       │ usa
┌──────────────────────▼──────────────────────────────────┐
│                  APPLICATION                             │
│              (lib/stores/)                               │
│  • Lógica de presentación                               │
│  • Svelte 5 runes ($state, $derived)                    │
│  • Estado global                                        │
└──────────────────────┬──────────────────────────────────┘
                       │ usa
┌──────────────────────▼──────────────────────────────────┐
│                 INFRASTRUCTURE                           │
│           (lib/tauri/ + lib/api/)                        │
│  • Implementación de Repositories                       │
│  • Acceso a Tauri commands                              │
│  • Acceso a APIs HTTP (futuro)                          │
└──────────────────────┬──────────────────────────────────┘
                       │ implementa contratos de
┌──────────────────────▼──────────────────────────────────┐
│                    DOMAIN                                │
│                (lib/types/)                              │
│  • Entidades (ClipboardItem, User)                      │
│  • Interfaces (Repositories)                            │
│  • Enums, Types                                         │
│  • No depende de nada                                   │
└─────────────────────────────────────────────────────────┘
```

---

## Estructura Final del Proyecto

```
global_clipboard/
├── src/
│   ├── routes/                          # Pages (SvelteKit routing)
│   │   ├── +layout.svelte              # Root layout
│   │   ├── +page.svelte                # Home (/)
│   │   ├── search/
│   │   │   └── +page.svelte            # Search page
│   │   ├── favorites/
│   │   │   └── +page.svelte            # Favorites
│   │   └── settings/
│   │       └── +page.svelte            # Settings
│   │
│   ├── lib/                             # Código reutilizable
│   │   ├── components/                  # PRESENTATION LAYER
│   │   │   ├── ui/                      # Base components
│   │   │   │   ├── Button.svelte
│   │   │   │   ├── Card.svelte
│   │   │   │   ├── Input.svelte
│   │   │   │   ├── Modal.svelte
│   │   │   │   └── Spinner.svelte
│   │   │   │
│   │   │   ├── clipboard/               # Clipboard components
│   │   │   │   ├── ClipboardList.svelte
│   │   │   │   ├── ClipboardItem.svelte
│   │   │   │   └── EmptyState.svelte
│   │   │   │
│   │   │   ├── layout/                  # Layout components
│   │   │   │   ├── Sidebar.svelte
│   │   │   │   └── Header.svelte
│   │   │   │
│   │   │   └── SearchBar.svelte
│   │   │
│   │   ├── stores/                      # APPLICATION LAYER (State)
│   │   │   ├── clipboard.svelte.ts      # Clipboard state (Svelte 5 runes)
│   │   │   ├── ui.svelte.ts             # UI state
│   │   │   └── settings.svelte.ts       # Settings state
│   │   │
│   │   ├── tauri/                       # INFRASTRUCTURE LAYER
│   │   │   ├── commands.ts              # Typed Tauri commands
│   │   │   ├── storage.ts               # Repository implementation
│   │   │   └── clipboard.ts             # Clipboard operations
│   │   │
│   │   ├── types/                       # DOMAIN LAYER
│   │   │   ├── clipboard.ts
│   │   │   ├── settings.ts
│   │   │   └── index.ts
│   │   │
│   │   └── utils/                       # Utilities
│   │       ├── cn.ts                    # Class merge
│   │       └── format.ts                # Formatters
│   │
│   ├── app.html                         # HTML template
│   └── app.css                          # Global CSS + Tailwind
│
├── src-tauri/                           # Rust backend
│   └── ...
│
├── static/                              # Static assets
│
├── svelte.config.js                     # SvelteKit config
├── vite.config.ts                       # Vite config
├── tsconfig.json                        # TypeScript config
├── package.json                         # Dependencies
└── tailwind.config.js                   # Tailwind v4 config
```

---

## Plan de Desarrollo - Fases

### ✅ CHECKPOINT SYSTEM

Después de cada fase:
1. Verificar que todo funciona
2. Git commit: `git commit -m "✅ Fase X completada"`
3. Continuar con siguiente fase

Si se corta la conversación:
- Ve a la última fase completada
- Continúa desde ahí

---

## FASE 1: Setup Inicial y Configuración Base

**Duración estimada**: 30-45 minutos

**Objetivo**: Configurar el proyecto con todas las dependencias necesarias.

---

### Paso 1.1: Instalar Dependencias

**Comando**:
```bash
bun add clsx tailwind-merge
bun add -d tailwindcss @tailwindcss/vite
```

**Explicación**:
- `clsx` + `tailwind-merge`: Para combinar clases de Tailwind
- `tailwindcss` + `@tailwindcss/vite`: TailwindCSS v4

**Verificación**:
```bash
bun run dev
```
Debe abrir la app sin errores.

**Checkpoint**: ✅ Dependencias instaladas

---

### Paso 1.2: Configurar TailwindCSS v4

**Archivo a crear**: `src/app.css`

```css
@import "tailwindcss";

/* Theme variables (Tailwind v4 syntax) */
@theme {
  /* Colors */
  --color-background: oklch(1 0 0);
  --color-foreground: oklch(0.145 0 0);
  --color-card: oklch(1 0 0);
  --color-card-foreground: oklch(0.145 0 0);
  --color-primary: oklch(0.145 0 0);
  --color-primary-foreground: oklch(0.985 0 0);
  --color-secondary: oklch(0.961 0 0);
  --color-secondary-foreground: oklch(0.145 0 0);
  --color-muted: oklch(0.961 0 0);
  --color-muted-foreground: oklch(0.454 0.004 286);
  --color-accent: oklch(0.961 0 0);
  --color-accent-foreground: oklch(0.145 0 0);
  --color-destructive: oklch(0.577 0.245 27.325);
  --color-destructive-foreground: oklch(0.985 0 0);
  --color-border: oklch(0.898 0 0);
  --color-input: oklch(0.898 0 0);
  --color-ring: oklch(0.145 0 0);
  
  /* Dark mode */
  @media (prefers-color-scheme: dark) {
    --color-background: oklch(0.145 0 0);
    --color-foreground: oklch(0.985 0 0);
    --color-card: oklch(0.145 0 0);
    --color-card-foreground: oklch(0.985 0 0);
    --color-primary: oklch(0.985 0 0);
    --color-primary-foreground: oklch(0.145 0 0);
    --color-secondary: oklch(0.221 0.013 286);
    --color-secondary-foreground: oklch(0.985 0 0);
    --color-muted: oklch(0.221 0.013 286);
    --color-muted-foreground: oklch(0.635 0.017 286);
    --color-accent: oklch(0.221 0.013 286);
    --color-accent-foreground: oklch(0.985 0 0);
    --color-destructive: oklch(0.406 0.139 27.325);
    --color-destructive-foreground: oklch(0.985 0 0);
    --color-border: oklch(0.221 0.013 286);
    --color-input: oklch(0.221 0.013 286);
    --color-ring: oklch(0.832 0.013 286);
  }
  
  /* Radius */
  --radius-sm: 0.25rem;
  --radius: 0.5rem;
  --radius-md: 0.375rem;
  --radius-lg: 0.5rem;
}

/* Base styles */
* {
  @apply border-border;
}

body {
  @apply bg-background text-foreground;
  font-family: system-ui, -apple-system, sans-serif;
}
```

**Archivo a crear**: `vite.config.ts` (actualizar)

```typescript
import { defineConfig } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite'
import tailwindcss from '@tailwindcss/vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    sveltekit(),
    tailwindcss(),
  ],
  
  // Prevent vite from obscuring rust errors
  clearScreen: false,
  
  // Tauri expects a fixed port
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
})
```

**Archivo a crear**: `src/app.html` (actualizar si existe)

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Clipboard Manager</title>
    %sveltekit.head%
  </head>
  <body data-sveltekit-preload-data="hover">
    <div style="display: contents">%sveltekit.body%</div>
  </body>
</html>
```

**Verificación**:
```bash
bun run dev
```
El fondo debe ser blanco/negro según theme del sistema.

**Checkpoint**: ✅ TailwindCSS v4 configurado

---

### Paso 1.3: Configurar TypeScript

**Archivo a actualizar**: `tsconfig.json`

```json
{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    "allowJs": true,
    "checkJs": true,
    "esModuleInterop": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "sourceMap": true,
    "strict": true,
    "moduleResolution": "bundler",
    "paths": {
      "$lib": ["./src/lib"],
      "$lib/*": ["./src/lib/*"]
    }
  }
}
```

**¿Por qué?**: SvelteKit ya viene con path aliases (`$lib`), solo confirmamos la configuración.

**Checkpoint**: ✅ TypeScript configurado

---

### Paso 1.4: Configurar SvelteKit (Adapter Static)

**Archivo a actualizar**: `svelte.config.js`

```javascript
import adapter from '@sveltejs/adapter-static'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  
  kit: {
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: 'index.html',
      precompress: false,
      strict: true,
    }),
  },
}

export default config
```

**¿Por qué adapter-static?**: Tauri necesita archivos estáticos, no un servidor Node.

**Checkpoint**: ✅ SvelteKit configurado

---

## FASE 2: Domain Layer - Types e Interfaces

**Duración estimada**: 20 minutos

**Objetivo**: Definir las entidades y contratos del dominio.

---

### Paso 2.1: Types de Clipboard

**Archivo**: `src/lib/types/clipboard.ts`

```typescript
export type ContentType = 'text' | 'rich_text' | 'code' | 'link' | 'color' | 'image' | 'file'

export interface ClipboardItem {
  id: string
  contentType: ContentType
  contentText: string | null
  contentMetadata: Record<string, any>
  
  // Files (futuro)
  fileUrl: string | null
  fileName: string | null
  fileSizeBytes: number | null
  fileMimeType: string | null
  
  // Organization
  isFavorite: boolean
  isSnippet: boolean
  snippetName: string | null
  
  // Timestamps
  createdAt: Date
  updatedAt: Date
  
  // Sync (futuro - Pro)
  synced: boolean
  serverId: string | null
}

export interface CreateClipboardItemDto {
  contentType: ContentType
  contentText: string
  contentMetadata?: Record<string, any>
}

export interface UpdateClipboardItemDto {
  contentText?: string
  isFavorite?: boolean
  isSnippet?: boolean
  snippetName?: string
}

// Repository interface (contrato)
export interface ClipboardRepository {
  // Queries
  getItems(options?: GetItemsOptions): Promise<ClipboardItem[]>
  getItem(id: string): Promise<ClipboardItem | null>
  searchItems(query: string): Promise<ClipboardItem[]>
  getFavorites(): Promise<ClipboardItem[]>
  getSnippets(): Promise<ClipboardItem[]>
  
  // Mutations
  createItem(data: CreateClipboardItemDto): Promise<ClipboardItem>
  updateItem(id: string, data: UpdateClipboardItemDto): Promise<ClipboardItem>
  deleteItem(id: string): Promise<void>
  clearAll(): Promise<void>
}

export interface GetItemsOptions {
  limit?: number
  offset?: number
  contentType?: ContentType
  isFavorite?: boolean
  isSnippet?: boolean
}
```

**Checkpoint**: ✅ Clipboard types definidos

---

### Paso 2.2: Types de Settings

**Archivo**: `src/lib/types/settings.ts`

```typescript
export interface AppSettings {
  // General
  maxLocalItems: number
  autoSaveClipboard: boolean
  
  // UI
  theme: 'light' | 'dark' | 'system'
  
  // Hotkeys
  showHotkey: string
  
  // Advanced
  enableAnalytics: boolean
}

export const DEFAULT_SETTINGS: AppSettings = {
  maxLocalItems: 1000,
  autoSaveClipboard: true,
  theme: 'system',
  showHotkey: 'CommandOrControl+Shift+V',
  enableAnalytics: false,
}
```

**Checkpoint**: ✅ Settings types definidos

---

### Paso 2.3: Exports

**Archivo**: `src/lib/types/index.ts`

```typescript
export * from './clipboard'
export * from './settings'
```

**Checkpoint**: ✅ Types exportados

---

## FASE 3: Infrastructure Layer - Tauri Integration

**Duración estimada**: 30 minutos

**Objetivo**: Implementar el Repository usando Tauri commands.

---

### Paso 3.1: Utility - cn

**Archivo**: `src/lib/utils/cn.ts`

```typescript
import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
```

**Checkpoint**: ✅ Utility cn creado

---

### Paso 3.2: Utility - format

**Archivo**: `src/lib/utils/format.ts`

```typescript
export function formatDate(date: Date | string): string {
  const d = typeof date === 'string' ? new Date(date) : date
  
  const now = new Date()
  const diffInMs = now.getTime() - d.getTime()
  const diffInMinutes = Math.floor(diffInMs / 60000)
  const diffInHours = Math.floor(diffInMs / 3600000)
  const diffInDays = Math.floor(diffInMs / 86400000)
  
  if (diffInMinutes < 1) return 'Just now'
  if (diffInMinutes < 60) return `${diffInMinutes}m ago`
  if (diffInHours < 24) return `${diffInHours}h ago`
  if (diffInDays < 7) return `${diffInDays}d ago`
  
  return d.toLocaleDateString('en-US', { 
    month: 'short', 
    day: 'numeric',
    year: d.getFullYear() !== now.getFullYear() ? 'numeric' : undefined
  })
}

export function truncateText(text: string, maxLength: number = 100): string {
  if (text.length <= maxLength) return text
  return text.slice(0, maxLength) + '...'
}

export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

export function getContentTypeIcon(type: string): string {
  const icons: Record<string, string> = {
    text: '📝',
    code: '💻',
    link: '🔗',
    color: '🎨',
    image: '🖼️',
    file: '📄',
    rich_text: '📋',
  }
  return icons[type] || '📄'
}
```

**Checkpoint**: ✅ Format utilities creados

---

### Paso 3.3: Tauri Commands

**Archivo**: `src/lib/tauri/commands.ts`

```typescript
import { invoke } from '@tauri-apps/api/core'
import type { ClipboardItem, CreateClipboardItemDto, UpdateClipboardItemDto } from '$lib/types'

// ============================================
// CLIPBOARD COMMANDS
// ============================================

export async function tauriGetItems(): Promise<ClipboardItem[]> {
  const items = await invoke<any[]>('get_clipboard_items')
  return items.map(deserializeClipboardItem)
}

export async function tauriGetItem(id: string): Promise<ClipboardItem | null> {
  try {
    const item = await invoke<any>('get_clipboard_item', { id })
    return deserializeClipboardItem(item)
  } catch (error) {
    console.error('Error getting item:', error)
    return null
  }
}

export async function tauriCreateItem(data: CreateClipboardItemDto): Promise<ClipboardItem> {
  const item = await invoke<any>('create_clipboard_item', { data })
  return deserializeClipboardItem(item)
}

export async function tauriUpdateItem(id: string, data: UpdateClipboardItemDto): Promise<ClipboardItem> {
  const item = await invoke<any>('update_clipboard_item', { id, data })
  return deserializeClipboardItem(item)
}

export async function tauriDeleteItem(id: string): Promise<void> {
  await invoke('delete_clipboard_item', { id })
}

export async function tauriSearchItems(query: string): Promise<ClipboardItem[]> {
  const items = await invoke<any[]>('search_clipboard_items', { query })
  return items.map(deserializeClipboardItem)
}

export async function tauriClearAllItems(): Promise<void> {
  await invoke('clear_all_clipboard_items')
}

// ============================================
// CLIPBOARD OPERATIONS (OS)
// ============================================

export async function tauriWriteToClipboard(text: string): Promise<void> {
  await invoke('write_to_clipboard', { text })
}

export async function tauriReadFromClipboard(): Promise<string> {
  return await invoke<string>('read_from_clipboard')
}

// ============================================
// HELPERS
// ============================================

function deserializeClipboardItem(item: any): ClipboardItem {
  return {
    ...item,
    createdAt: new Date(item.createdAt),
    updatedAt: new Date(item.updatedAt),
  }
}
```

**Checkpoint**: ✅ Tauri commands tipados

---

### Paso 3.4: Repository Implementation

**Archivo**: `src/lib/tauri/storage.ts`

```typescript
import type { 
  ClipboardRepository, 
  ClipboardItem, 
  CreateClipboardItemDto,
  UpdateClipboardItemDto,
  GetItemsOptions
} from '$lib/types'
import * as commands from './commands'

export class TauriClipboardRepository implements ClipboardRepository {
  async getItems(options?: GetItemsOptions): Promise<ClipboardItem[]> {
    let items = await commands.tauriGetItems()
    
    // Apply filters
    if (options?.contentType) {
      items = items.filter(item => item.contentType === options.contentType)
    }
    
    if (options?.isFavorite !== undefined) {
      items = items.filter(item => item.isFavorite === options.isFavorite)
    }
    
    if (options?.isSnippet !== undefined) {
      items = items.filter(item => item.isSnippet === options.isSnippet)
    }
    
    // Apply pagination
    if (options?.offset !== undefined) {
      items = items.slice(options.offset)
    }
    
    if (options?.limit !== undefined) {
      items = items.slice(0, options.limit)
    }
    
    return items
  }
  
  async getItem(id: string): Promise<ClipboardItem | null> {
    return await commands.tauriGetItem(id)
  }
  
  async searchItems(query: string): Promise<ClipboardItem[]> {
    return await commands.tauriSearchItems(query)
  }
  
  async getFavorites(): Promise<ClipboardItem[]> {
    return this.getItems({ isFavorite: true })
  }
  
  async getSnippets(): Promise<ClipboardItem[]> {
    return this.getItems({ isSnippet: true })
  }
  
  async createItem(data: CreateClipboardItemDto): Promise<ClipboardItem> {
    return await commands.tauriCreateItem(data)
  }
  
  async updateItem(id: string, data: UpdateClipboardItemDto): Promise<ClipboardItem> {
    return await commands.tauriUpdateItem(id, data)
  }
  
  async deleteItem(id: string): Promise<void> {
    await commands.tauriDeleteItem(id)
  }
  
  async clearAll(): Promise<void> {
    await commands.tauriClearAllItems()
  }
}

// Singleton instance
export const clipboardRepository = new TauriClipboardRepository()
```

**Checkpoint**: ✅ Repository implementado

---

## FASE 4: Application Layer - Stores (Svelte 5 Runes)

**Duración estimada**: 45 minutos

**Objetivo**: Crear stores con Svelte 5 runes.

---

### Paso 4.1: Clipboard Store

**Archivo**: `src/lib/stores/clipboard.svelte.ts`

```typescript
import { clipboardRepository } from '$lib/tauri/storage'
import type { ClipboardItem, CreateClipboardItemDto, UpdateClipboardItemDto, GetItemsOptions } from '$lib/types'

class ClipboardStore {
  items = $state<ClipboardItem[]>([])
  isLoading = $state(false)
  error = $state<string | null>(null)
  
  // Load items
  async loadItems(options?: GetItemsOptions) {
    this.isLoading = true
    this.error = null
    
    try {
      this.items = await clipboardRepository.getItems(options)
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to load items'
      console.error('Failed to load items:', err)
    } finally {
      this.isLoading = false
    }
  }
  
  // Create item
  async createItem(data: CreateClipboardItemDto) {
    try {
      const newItem = await clipboardRepository.createItem(data)
      this.items = [newItem, ...this.items]
      return newItem
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to create item'
      throw err
    }
  }
  
  // Update item
  async updateItem(id: string, data: UpdateClipboardItemDto) {
    try {
      const updatedItem = await clipboardRepository.updateItem(id, data)
      this.items = this.items.map(item => 
        item.id === id ? updatedItem : item
      )
      return updatedItem
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to update item'
      throw err
    }
  }
  
  // Delete item
  async deleteItem(id: string) {
    try {
      await clipboardRepository.deleteItem(id)
      this.items = this.items.filter(item => item.id !== id)
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to delete item'
      throw err
    }
  }
  
  // Toggle favorite
  async toggleFavorite(id: string) {
    const item = this.items.find(i => i.id === id)
    if (!item) return
    
    await this.updateItem(id, { isFavorite: !item.isFavorite })
  }
  
  // Clear all
  async clearAll() {
    try {
      await clipboardRepository.clearAll()
      this.items = []
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to clear items'
      throw err
    }
  }
  
  // Search
  async search(query: string) {
    if (!query.trim()) {
      return []
    }
    
    try {
      return await clipboardRepository.searchItems(query)
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to search'
      throw err
    }
  }
}

export const clipboardStore = new ClipboardStore()
```

**¿Cómo usar?**:
```svelte
<script>
  import { clipboardStore } from '$lib/stores/clipboard.svelte'
  import { onMount } from 'svelte'
  
  onMount(() => {
    clipboardStore.loadItems()
  })
</script>

{#each clipboardStore.items as item}
  <div>{item.contentText}</div>
{/each}
```

**Checkpoint**: ✅ Clipboard store creado

---

### Paso 4.2: UI Store

**Archivo**: `src/lib/stores/ui.svelte.ts`

```typescript
class UiStore {
  // Theme
  theme = $state<'light' | 'dark' | 'system'>('system')
  
  // Sidebar
  isSidebarCollapsed = $state(false)
  
  // Modals
  isSettingsOpen = $state(false)
  
  constructor() {
    // Load from localStorage
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('ui-store')
      if (saved) {
        const parsed = JSON.parse(saved)
        this.theme = parsed.theme ?? 'system'
        this.isSidebarCollapsed = parsed.isSidebarCollapsed ?? false
      }
      
      // Apply theme
      this.applyTheme()
      
      // Watch for changes and save
      $effect(() => {
        localStorage.setItem('ui-store', JSON.stringify({
          theme: this.theme,
          isSidebarCollapsed: this.isSidebarCollapsed,
        }))
        
        this.applyTheme()
      })
    }
  }
  
  setTheme(theme: 'light' | 'dark' | 'system') {
    this.theme = theme
  }
  
  toggleSidebar() {
    this.isSidebarCollapsed = !this.isSidebarCollapsed
  }
  
  private applyTheme() {
    if (typeof window === 'undefined') return
    
    const root = document.documentElement
    root.classList.remove('light', 'dark')
    
    if (this.theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light'
      root.classList.add(systemTheme)
    } else {
      root.classList.add(this.theme)
    }
  }
}

export const uiStore = new UiStore()
```

**Checkpoint**: ✅ UI store creado

---

### Paso 4.3: Settings Store

**Archivo**: `src/lib/stores/settings.svelte.ts`

```typescript
import { DEFAULT_SETTINGS, type AppSettings } from '$lib/types'

class SettingsStore {
  maxLocalItems = $state(DEFAULT_SETTINGS.maxLocalItems)
  autoSaveClipboard = $state(DEFAULT_SETTINGS.autoSaveClipboard)
  showHotkey = $state(DEFAULT_SETTINGS.showHotkey)
  enableAnalytics = $state(DEFAULT_SETTINGS.enableAnalytics)
  
  constructor() {
    // Load from localStorage
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('settings-store')
      if (saved) {
        const parsed = JSON.parse(saved)
        this.maxLocalItems = parsed.maxLocalItems ?? DEFAULT_SETTINGS.maxLocalItems
        this.autoSaveClipboard = parsed.autoSaveClipboard ?? DEFAULT_SETTINGS.autoSaveClipboard
        this.showHotkey = parsed.showHotkey ?? DEFAULT_SETTINGS.showHotkey
        this.enableAnalytics = parsed.enableAnalytics ?? DEFAULT_SETTINGS.enableAnalytics
      }
      
      // Watch for changes and save
      $effect(() => {
        localStorage.setItem('settings-store', JSON.stringify({
          maxLocalItems: this.maxLocalItems,
          autoSaveClipboard: this.autoSaveClipboard,
          showHotkey: this.showHotkey,
          enableAnalytics: this.enableAnalytics,
        }))
      })
    }
  }
  
  updateMaxLocalItems(value: number) {
    this.maxLocalItems = value
  }
  
  toggleAutoSave() {
    this.autoSaveClipboard = !this.autoSaveClipboard
  }
  
  updateShowHotkey(key: string) {
    this.showHotkey = key
  }
  
  toggleAnalytics() {
    this.enableAnalytics = !this.enableAnalytics
  }
  
  reset() {
    this.maxLocalItems = DEFAULT_SETTINGS.maxLocalItems
    this.autoSaveClipboard = DEFAULT_SETTINGS.autoSaveClipboard
    this.showHotkey = DEFAULT_SETTINGS.showHotkey
    this.enableAnalytics = DEFAULT_SETTINGS.enableAnalytics
  }
}

export const settingsStore = new SettingsStore()
```

**Checkpoint**: ✅ Settings store creado

---

## FASE 5: Presentation Layer - UI Components

**Duración estimada**: 90 minutos

**Objetivo**: Crear componentes base reutilizables.

---

### Paso 5.1: Button Component

**Archivo**: `src/lib/components/ui/Button.svelte`

```svelte
<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    variant?: 'default' | 'destructive' | 'outline' | 'ghost'
    size?: 'default' | 'sm' | 'lg' | 'icon'
    class?: string
    children?: any
    onclick?: (e: MouseEvent) => void
    disabled?: boolean
    type?: 'button' | 'submit' | 'reset'
  }
  
  let {
    variant = 'default',
    size = 'default',
    class: className,
    children,
    onclick,
    disabled = false,
    type = 'button',
    ...restProps
  }: Props = $props()
</script>

<button
  {type}
  {disabled}
  {onclick}
  class={cn(
    // Base
    'inline-flex items-center justify-center rounded-md font-medium transition-colors',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
    'disabled:pointer-events-none disabled:opacity-50',
    
    // Variants
    variant === 'default' && 'bg-primary text-primary-foreground hover:bg-primary/90',
    variant === 'destructive' && 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
    variant === 'outline' && 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
    variant === 'ghost' && 'hover:bg-accent hover:text-accent-foreground',
    
    // Sizes
    size === 'default' && 'h-10 px-4 py-2',
    size === 'sm' && 'h-9 rounded-md px-3',
    size === 'lg' && 'h-11 rounded-md px-8',
    size === 'icon' && 'h-10 w-10',
    
    className
  )}
  {...restProps}
>
  {@render children?.()}
</button>
```

**Checkpoint**: ✅ Button component creado

---

### Paso 5.2: Card Component

**Archivo**: `src/lib/components/ui/Card.svelte`

```svelte
<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    class?: string
    children?: any
  }
  
  let { class: className, children }: Props = $props()
</script>

<div
  class={cn(
    'rounded-lg border bg-card text-card-foreground shadow-sm',
    className
  )}
>
  {@render children?.()}
</div>
```

**Archivo**: `src/lib/components/ui/CardHeader.svelte`

```svelte
<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    class?: string
    children?: any
  }
  
  let { class: className, children }: Props = $props()
</script>

<div class={cn('flex flex-col space-y-1.5 p-6', className)}>
  {@render children?.()}
</div>
```

**Archivo**: `src/lib/components/ui/CardTitle.svelte`

```svelte
<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    class?: string
    children?: any
  }
  
  let { class: className, children }: Props = $props()
</script>

<h3 class={cn('text-2xl font-semibold leading-none tracking-tight', className)}>
  {@render children?.()}
</h3>
```

**Archivo**: `src/lib/components/ui/CardContent.svelte`

```svelte
<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    class?: string
    children?: any
  }
  
  let { class: className, children }: Props = $props()
</script>

<div class={cn('p-6 pt-0', className)}>
  {@render children?.()}
</div>
```

**Checkpoint**: ✅ Card components creados

---

### Paso 5.3: Input Component

**Archivo**: `src/lib/components/ui/Input.svelte`

```svelte
<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    type?: string
    value?: string
    placeholder?: string
    class?: string
    disabled?: boolean
    oninput?: (e: Event) => void
  }
  
  let {
    type = 'text',
    value = $bindable(''),
    placeholder,
    class: className,
    disabled = false,
    oninput,
    ...restProps
  }: Props = $props()
</script>

<input
  {type}
  bind:value
  {placeholder}
  {disabled}
  {oninput}
  class={cn(
    'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2',
    'text-sm ring-offset-background',
    'file:border-0 file:bg-transparent file:text-sm file:font-medium',
    'placeholder:text-muted-foreground',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2',
    'disabled:cursor-not-allowed disabled:opacity-50',
    className
  )}
  {...restProps}
/>
```

**Checkpoint**: ✅ Input component creado

---

### Paso 5.4: Spinner Component

**Archivo**: `src/lib/components/ui/Spinner.svelte`

```svelte
<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    class?: string
    size?: 'sm' | 'md' | 'lg'
  }
  
  let { class: className, size = 'md' }: Props = $props()
</script>

<div
  class={cn(
    'animate-spin rounded-full border-2 border-current border-t-transparent',
    size === 'sm' && 'h-4 w-4',
    size === 'md' && 'h-8 w-8',
    size === 'lg' && 'h-12 w-12',
    className
  )}
  role="status"
  aria-label="Loading"
>
  <span class="sr-only">Loading...</span>
</div>
```

**Checkpoint**: ✅ Spinner component creado

---

### Paso 5.5: Modal Component

**Archivo**: `src/lib/components/ui/Modal.svelte`

```svelte
<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    isOpen: boolean
    onClose: () => void
    title?: string
    class?: string
    children?: any
  }
  
  let {
    isOpen,
    onClose,
    title,
    class: className,
    children
  }: Props = $props()
  
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isOpen) {
      onClose()
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <!-- Backdrop -->
    <div 
      class="fixed inset-0 bg-black/50 backdrop-blur-sm"
      onclick={onClose}
      role="button"
      tabindex="-1"
    />
    
    <!-- Modal -->
    <div 
      class={cn(
        'relative z-50 w-full max-w-lg rounded-lg bg-card p-6 shadow-lg',
        'border border-border',
        className
      )}
    >
      {#if title}
        <div class="mb-4 flex items-center justify-between">
          <h2 class="text-lg font-semibold">{title}</h2>
          <button
            onclick={onClose}
            class="rounded-sm opacity-70 hover:opacity-100"
          >
            ✕
          </button>
        </div>
      {/if}
      
      {@render children?.()}
    </div>
  </div>
{/if}
```

**Checkpoint**: ✅ Modal component creado

---

## FASE 6: Business Components

**Duración estimada**: 60 minutos

---

### Paso 6.1: ClipboardItem Component

**Archivo**: `src/lib/components/clipboard/ClipboardItem.svelte`

```svelte
<script lang="ts">
  import Card from '$lib/components/ui/Card.svelte'
  import CardContent from '$lib/components/ui/CardContent.svelte'
  import Button from '$lib/components/ui/Button.svelte'
  import { tauriWriteToClipboard } from '$lib/tauri/commands'
  import { clipboardStore } from '$lib/stores/clipboard.svelte'
  import { formatDate, truncateText, getContentTypeIcon } from '$lib/utils/format'
  import type { ClipboardItem } from '$lib/types'
  
  interface Props {
    item: ClipboardItem
  }
  
  let { item }: Props = $props()
  
  let isCopied = $state(false)
  
  async function handleCopy() {
    if (item.contentText) {
      await tauriWriteToClipboard(item.contentText)
      isCopied = true
      setTimeout(() => isCopied = false, 2000)
    }
  }
  
  async function handleToggleFavorite() {
    await clipboardStore.toggleFavorite(item.id)
  }
  
  async function handleDelete() {
    await clipboardStore.deleteItem(item.id)
  }
</script>

<Card class="group hover:border-primary/50 transition-colors">
  <CardContent class="p-4">
    <!-- Header -->
    <div class="flex items-start justify-between gap-2 mb-2">
      <div class="flex items-center gap-2">
        <span class="text-xs text-muted-foreground">
          {getContentTypeIcon(item.contentType)}
        </span>
        <span class="text-xs text-muted-foreground">
          {formatDate(item.createdAt)}
        </span>
      </div>
      
      <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
        <!-- Favorite -->
        <Button
          size="icon"
          variant="ghost"
          onclick={handleToggleFavorite}
        >
          {item.isFavorite ? '⭐' : '☆'}
        </Button>
        
        <!-- Copy -->
        <Button
          size="icon"
          variant="ghost"
          onclick={handleCopy}
        >
          {isCopied ? '✓' : '📋'}
        </Button>
        
        <!-- Delete -->
        <Button
          size="icon"
          variant="ghost"
          onclick={handleDelete}
        >
          🗑️
        </Button>
      </div>
    </div>
    
    <!-- Content -->
    <div class="space-y-2">
      {#if item.isSnippet && item.snippetName}
        <div class="text-sm font-medium text-primary">
          📌 {item.snippetName}
        </div>
      {/if}
      
      <div class="text-sm {item.contentType === 'code' ? 'font-mono bg-muted p-2 rounded' : ''}">
        {item.contentText ? truncateText(item.contentText, 150) : 'No content'}
      </div>
    </div>
  </CardContent>
</Card>
```

**Checkpoint**: ✅ ClipboardItem component creado

---

### Paso 6.2: ClipboardList Component

**Archivo**: `src/lib/components/clipboard/ClipboardList.svelte`

```svelte
<script lang="ts">
  import ClipboardItem from './ClipboardItem.svelte'
  import EmptyState from './EmptyState.svelte'
  import Spinner from '$lib/components/ui/Spinner.svelte'
  import type { ClipboardItem as ClipboardItemType } from '$lib/types'
  
  interface Props {
    items: ClipboardItemType[]
    isLoading?: boolean
    emptyMessage?: string
  }
  
  let {
    items,
    isLoading = false,
    emptyMessage = 'No clipboard items yet. Copy something to get started!'
  }: Props = $props()
</script>

{#if isLoading}
  <div class="flex items-center justify-center py-12">
    <Spinner size="lg" />
  </div>
{:else if items.length === 0}
  <EmptyState message={emptyMessage} />
{:else}
  <div class="space-y-3">
    {#each items as item (item.id)}
      <ClipboardItem {item} />
    {/each}
  </div>
{/if}
```

**Checkpoint**: ✅ ClipboardList component creado

---

### Paso 6.3: EmptyState Component

**Archivo**: `src/lib/components/clipboard/EmptyState.svelte`

```svelte
<script lang="ts">
  interface Props {
    message: string
    icon?: string
  }
  
  let { message, icon = '📋' }: Props = $props()
</script>

<div class="flex flex-col items-center justify-center py-12 text-center">
  <div class="text-6xl mb-4">{icon}</div>
  <p class="text-muted-foreground max-w-md">
    {message}
  </p>
</div>
```

**Checkpoint**: ✅ EmptyState component creado

---

### Paso 6.4: SearchBar Component

**Archivo**: `src/lib/components/SearchBar.svelte`

```svelte
<script lang="ts">
  import Input from '$lib/components/ui/Input.svelte'
  import Button from '$lib/components/ui/Button.svelte'
  
  interface Props {
    value: string
    placeholder?: string
    onClear?: () => void
  }
  
  let {
    value = $bindable(''),
    placeholder = 'Search clipboard...',
    onClear
  }: Props = $props()
</script>

<div class="relative flex items-center gap-2">
  <div class="relative flex-1">
    <span class="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground">
      🔍
    </span>
    <Input
      bind:value
      {placeholder}
      class="pl-10"
    />
  </div>
  
  {#if value && onClear}
    <Button
      variant="ghost"
      size="icon"
      onclick={onClear}
    >
      ✕
    </Button>
  {/if}
</div>
```

**Checkpoint**: ✅ SearchBar component creado

---

### Paso 6.5: Sidebar Component

**Archivo**: `src/lib/components/layout/Sidebar.svelte`

```svelte
<script lang="ts">
  import { page } from '$app/stores'
  import { goto } from '$app/navigation'
  import { uiStore } from '$lib/stores/ui.svelte'
  import { cn } from '$lib/utils/cn'
  
  interface NavItem {
    label: string
    icon: string
    path: string
  }
  
  const navItems: NavItem[] = [
    { label: 'All Items', icon: '📋', path: '/' },
    { label: 'Favorites', icon: '⭐', path: '/favorites' },
    { label: 'Search', icon: '🔍', path: '/search' },
    { label: 'Settings', icon: '⚙️', path: '/settings' },
  ]
</script>

<aside class={cn(
  'fixed left-0 top-0 z-40 h-screen border-r bg-card transition-all',
  uiStore.isSidebarCollapsed ? 'w-16' : 'w-64'
)}>
  <div class="flex h-full flex-col">
    <!-- Header -->
    <div class="flex h-16 items-center justify-between border-b px-4">
      {#if !uiStore.isSidebarCollapsed}
        <h1 class="text-lg font-semibold">Clipboard</h1>
      {/if}
      <button
        onclick={() => uiStore.toggleSidebar()}
        class="rounded-md p-2 hover:bg-accent"
      >
        {uiStore.isSidebarCollapsed ? '→' : '←'}
      </button>
    </div>
    
    <!-- Navigation -->
    <nav class="flex-1 space-y-1 p-2">
      {#each navItems as item}
        <button
          onclick={() => goto(item.path)}
          class={cn(
            'flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors',
            'hover:bg-accent hover:text-accent-foreground',
            $page.url.pathname === item.path && 'bg-accent text-accent-foreground'
          )}
        >
          <span class="text-xl">{item.icon}</span>
          {#if !uiStore.isSidebarCollapsed}
            <span class="flex-1 text-left">{item.label}</span>
          {/if}
        </button>
      {/each}
    </nav>
    
    <!-- Footer -->
    <div class="border-t p-4">
      <div class="flex items-center gap-3">
        <div class="flex h-8 w-8 items-center justify-center rounded-full bg-primary text-primary-foreground">
          U
        </div>
        {#if !uiStore.isSidebarCollapsed}
          <div class="flex-1">
            <div class="text-sm font-medium">Free User</div>
            <div class="text-xs text-muted-foreground">Local only</div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</aside>
```

**Checkpoint**: ✅ Sidebar component creado

---

### Paso 6.6: Header Component

**Archivo**: `src/lib/components/layout/Header.svelte`

```svelte
<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte'
  import { uiStore } from '$lib/stores/ui.svelte'
  
  function toggleTheme() {
    const newTheme = uiStore.theme === 'dark' ? 'light' : 'dark'
    uiStore.setTheme(newTheme)
  }
</script>

<header class="sticky top-0 z-30 flex h-16 items-center justify-between border-b bg-card px-6">
  <div>
    <h2 class="text-xl font-semibold">Clipboard Manager</h2>
  </div>
  
  <div class="flex items-center gap-2">
    <Button
      variant="ghost"
      size="icon"
      onclick={toggleTheme}
    >
      {uiStore.theme === 'dark' ? '☀️' : '🌙'}
    </Button>
  </div>
</header>
```

**Checkpoint**: ✅ Header component creado

---

## FASE 7: Pages (SvelteKit Routes)

**Duración estimada**: 45 minutos

---

### Paso 7.1: Root Layout

**Archivo**: `src/routes/+layout.svelte`

```svelte
<script>
  import '../app.css'
  import Sidebar from '$lib/components/layout/Sidebar.svelte'
  import Header from '$lib/components/layout/Header.svelte'
  import { uiStore } from '$lib/stores/ui.svelte'
  import { cn } from '$lib/utils/cn'
</script>

<div class="flex h-screen bg-background">
  <Sidebar />
  
  <div class={cn(
    'flex flex-1 flex-col transition-all',
    uiStore.isSidebarCollapsed ? 'ml-16' : 'ml-64'
  )}>
    <Header />
    
    <main class="flex-1 overflow-y-auto p-6">
      <slot />
    </main>
  </div>
</div>
```

**Checkpoint**: ✅ Root layout creado

---

### Paso 7.2: Home Page

**Archivo**: `src/routes/+page.svelte`

```svelte
<script lang="ts">
  import { onMount } from 'svelte'
  import ClipboardList from '$lib/components/clipboard/ClipboardList.svelte'
  import Button from '$lib/components/ui/Button.svelte'
  import { clipboardStore } from '$lib/stores/clipboard.svelte'
  
  onMount(() => {
    clipboardStore.loadItems()
  })
  
  async function handleClearAll() {
    if (confirm('Are you sure you want to clear all clipboard items?')) {
      await clipboardStore.clearAll()
    }
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-bold">All Items</h1>
      <p class="text-muted-foreground">
        Your clipboard history
      </p>
    </div>
    
    <Button
      variant="destructive"
      onclick={handleClearAll}
    >
      Clear All
    </Button>
  </div>
  
  <!-- List -->
  <ClipboardList
    items={clipboardStore.items}
    isLoading={clipboardStore.isLoading}
  />
</div>
```

**Checkpoint**: ✅ Home page creada

---

### Paso 7.3: Search Page

**Archivo**: `src/routes/search/+page.svelte`

```svelte
<script lang="ts">
  import SearchBar from '$lib/components/SearchBar.svelte'
  import ClipboardList from '$lib/components/clipboard/ClipboardList.svelte'
  import { clipboardStore } from '$lib/stores/clipboard.svelte'
  import type { ClipboardItem } from '$lib/types'
  
  let query = $state('')
  let searchResults = $state<ClipboardItem[]>([])
  let isSearching = $state(false)
  
  // Debounced search
  let timeoutId: number | undefined
  $effect(() => {
    if (query.trim()) {
      clearTimeout(timeoutId)
      isSearching = true
      
      timeoutId = setTimeout(async () => {
        searchResults = await clipboardStore.search(query)
        isSearching = false
      }, 300)
    } else {
      searchResults = []
      isSearching = false
    }
  })
</script>

<div class="space-y-6">
  <!-- Header -->
  <div>
    <h1 class="text-3xl font-bold mb-2">Search</h1>
    <p class="text-muted-foreground">
      Find anything in your clipboard history
    </p>
  </div>
  
  <!-- Search Bar -->
  <SearchBar
    bind:value={query}
    onClear={() => query = ''}
  />
  
  <!-- Results -->
  {#if query.length > 0}
    <div>
      <p class="text-sm text-muted-foreground mb-4">
        {isSearching ? 'Searching...' : `Found ${searchResults.length} results`}
      </p>
      <ClipboardList
        items={searchResults}
        {isSearching}
        emptyMessage="No results found. Try a different search."
      />
    </div>
  {:else}
    <div class="text-center py-12 text-muted-foreground">
      Start typing to search...
    </div>
  {/if}
</div>
```

**Checkpoint**: ✅ Search page creada

---

### Paso 7.4: Favorites Page

**Archivo**: `src/routes/favorites/+page.svelte`

```svelte
<script lang="ts">
  import { onMount } from 'svelte'
  import ClipboardList from '$lib/components/clipboard/ClipboardList.svelte'
  import { clipboardStore } from '$lib/stores/clipboard.svelte'
  
  let favorites = $derived(
    clipboardStore.items.filter(item => item.isFavorite)
  )
  
  onMount(() => {
    clipboardStore.loadItems()
  })
</script>

<div class="space-y-6">
  <!-- Header -->
  <div>
    <h1 class="text-3xl font-bold">Favorites</h1>
    <p class="text-muted-foreground">
      Your starred clipboard items
    </p>
  </div>
  
  <!-- List -->
  <ClipboardList
    items={favorites}
    isLoading={clipboardStore.isLoading}
    emptyMessage="No favorites yet. Star items to save them here!"
  />
</div>
```

**Checkpoint**: ✅ Favorites page creada

---

### Paso 7.5: Settings Page

**Archivo**: `src/routes/settings/+page.svelte`

```svelte
<script lang="ts">
  import Card from '$lib/components/ui/Card.svelte'
  import CardHeader from '$lib/components/ui/CardHeader.svelte'
  import CardTitle from '$lib/components/ui/CardTitle.svelte'
  import CardContent from '$lib/components/ui/CardContent.svelte'
  import Button from '$lib/components/ui/Button.svelte'
  import { uiStore } from '$lib/stores/ui.svelte'
  import { settingsStore } from '$lib/stores/settings.svelte'
</script>

<div class="space-y-6 max-w-2xl">
  <!-- Header -->
  <div>
    <h1 class="text-3xl font-bold">Settings</h1>
    <p class="text-muted-foreground">
      Manage your preferences
    </p>
  </div>
  
  <!-- Appearance -->
  <Card>
    <CardHeader>
      <CardTitle>Appearance</CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <div>
        <label class="text-sm font-medium mb-2 block">Theme</label>
        <div class="flex gap-2">
          <Button
            variant={uiStore.theme === 'light' ? 'default' : 'outline'}
            onclick={() => uiStore.setTheme('light')}
          >
            ☀️ Light
          </Button>
          <Button
            variant={uiStore.theme === 'dark' ? 'default' : 'outline'}
            onclick={() => uiStore.setTheme('dark')}
          >
            🌙 Dark
          </Button>
          <Button
            variant={uiStore.theme === 'system' ? 'default' : 'outline'}
            onclick={() => uiStore.setTheme('system')}
          >
            💻 System
          </Button>
        </div>
      </div>
    </CardContent>
  </Card>
  
  <!-- Clipboard Settings -->
  <Card>
    <CardHeader>
      <CardTitle>Clipboard</CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <div class="font-medium">Auto-save clipboard</div>
          <div class="text-sm text-muted-foreground">
            Automatically save copied items
          </div>
        </div>
        <Button
          variant={settingsStore.autoSaveClipboard ? 'default' : 'outline'}
          onclick={() => settingsStore.toggleAutoSave()}
        >
          {settingsStore.autoSaveClipboard ? 'On' : 'Off'}
        </Button>
      </div>
      
      <div>
        <div class="font-medium mb-2">Max local items</div>
        <div class="text-sm text-muted-foreground">
          Currently storing up to {settingsStore.maxLocalItems.toLocaleString()} items locally
        </div>
      </div>
    </CardContent>
  </Card>
  
  <!-- About -->
  <Card>
    <CardHeader>
      <CardTitle>About</CardTitle>
    </CardHeader>
    <CardContent>
      <div class="space-y-2 text-sm">
        <div class="flex justify-between">
          <span class="text-muted-foreground">Version</span>
          <span class="font-medium">1.0.0</span>
        </div>
        <div class="flex justify-between">
          <span class="text-muted-foreground">Subscription</span>
          <span class="font-medium">Free (Local only)</span>
        </div>
      </div>
    </CardContent>
  </Card>
</div>
```

**Checkpoint**: ✅ Settings page creada

---

## FASE 8: Testing y Verificación

**Duración estimada**: 15 minutos

---

### Paso 8.1: Verificación Final

**Comandos**:
```bash
# Verificar que compila sin errores
bun run check

# Correr en desarrollo
bun run dev
```

**Verificar**:
- ✅ App carga sin errores
- ✅ Sidebar funciona y navega
- ✅ Theme toggle funciona
- ✅ Todas las páginas cargan
- ✅ UI components se ven bien
- ❌ Lista vacía (normal, falta backend)

**Checkpoint**: ✅ Frontend completo y funcional

---

## RESUMEN FINAL

### ✅ Lo Completado

**Arquitectura Svelte 5**:
- Clean Architecture adaptada
- Svelte 5 runes ($state, $derived, $effect)
- TailwindCSS v4 con @theme
- Repository Pattern
- SvelteKit routing

**Código Completo**:
1. ✅ Setup (TailwindCSS v4, TypeScript, SvelteKit)
2. ✅ Domain (Types e interfaces)
3. ✅ Infrastructure (Repository, Tauri commands)
4. ✅ Application (Stores con runes)
5. ✅ Presentation (UI + Business components)
6. ✅ Pages (SvelteKit routes)

**Total**: ~1800 líneas de TypeScript/Svelte

### 🚧 Próximos Pasos

**Backend Rust**:
1. SQLite database
2. Tauri commands
3. Clipboard listener

**Tiempo estimado**: 3-4 horas

### 📝 Comandos Útiles

```bash
# Desarrollo
bun run dev              # Solo frontend
bun run tauri dev       # Frontend + Backend

# Build
bun run build           # Build frontend
bun run tauri build    # Build app

# Testing
bun run check          # Type check
bun run check:watch    # Watch mode
```

---

**Fin del Plan Svelte 5** 🎉
