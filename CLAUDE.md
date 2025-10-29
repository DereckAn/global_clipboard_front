# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Global Clipboard Manager is a cross-platform clipboard synchronization application built with Tauri v2 (Rust backend + WebView frontend). It allows users to sync clipboard content across devices with support for text, code, links, colors, images, and files.

**Current Status**: Early development - basic Tauri scaffold with Svelte 5 frontend structure planned but not yet implemented.

## Technology Stack

### Frontend
- **Framework**: Tauri v2 with SvelteKit + Svelte 5
- **Package Manager**: Bun
- **Language**: TypeScript
- **Styling**: TailwindCSS v4
- **Reactivity**: Svelte 5 Runes (`$state`, `$derived`, `$effect`)
- **Adapter**: `@sveltejs/adapter-static` (SPA mode for Tauri)
- **Local Storage**: SQLite (via Tauri commands)

### Backend (Rust/Tauri)
- **Framework**: Tauri v2
- **Language**: Rust
- **Database**: SQLite (local)
- **Plugins**: `tauri-plugin-opener`

### Build Tools
- **Frontend Build**: Vite 6 with SvelteKit
- **Backend Build**: Cargo
- **Dev Server**: Port 1420 (fixed for Tauri)
- **HMR**: Port 1421

## Architecture

The frontend follows **Clean Architecture** with clear layer separation:

```
┌─────────────────────────────────────────────────────────┐
│                    PRESENTATION                          │
│        (routes/ + lib/components/)                       │
│  • Svelte components only                               │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                  APPLICATION                             │
│              (lib/stores/)                               │
│  • State management with Svelte 5 runes                 │
│  • Business logic                                       │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                 INFRASTRUCTURE                           │
│           (lib/tauri/ + lib/api/)                        │
│  • Tauri command wrappers                               │
│  • Repository implementations                            │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                    DOMAIN                                │
│                (lib/types/)                              │
│  • Types, interfaces, enums                             │
│  • Repository contracts                                 │
└─────────────────────────────────────────────────────────┘
```

### Key Directories

- **`src/routes/`**: SvelteKit pages using file-based routing
- **`src/lib/components/`**: Reusable Svelte components
  - `ui/`: Base UI components (Button, Card, Input, etc.)
  - `clipboard/`: Business-specific components
  - `layout/`: Layout components (Sidebar, Header)
- **`src/lib/stores/`**: Svelte 5 runes-based stores for state management
- **`src/lib/tauri/`**: Typed wrappers for Tauri commands
- **`src/lib/types/`**: TypeScript type definitions and interfaces
- **`src/lib/utils/`**: Utility functions (cn, formatters)
- **`src-tauri/`**: Rust backend code

## Development Commands

### Frontend Development
```bash
# Start frontend dev server only
bun run dev

# Type checking
bun run check

# Type checking with watch mode
bun run check:watch

# Build frontend for production
bun run build

# Preview production build
bun run preview
```

### Full Application Development
```bash
# Start Tauri app with frontend (recommended for development)
bun run tauri dev

# Build production app
bun run tauri build
```

### Backend (Rust) Development
```bash
# From src-tauri directory
cargo build          # Build
cargo test          # Run tests
cargo clippy        # Linting
```

## Important Implementation Notes

### Svelte 5 Runes (New Reactivity System)

**DO NOT use legacy Svelte patterns**. This project uses Svelte 5 with the new runes system:

- `$state()` - reactive state variables
- `$derived()` - computed values
- `$effect()` - side effects
- `$bindable()` - bindable props

**Example of correct Svelte 5 store pattern:**
```typescript
// lib/stores/clipboard.svelte.ts
class ClipboardStore {
  items = $state<ClipboardItem[]>([])
  isLoading = $state(false)

  async loadItems() {
    this.isLoading = true
    this.items = await repository.getItems()
    this.isLoading = false
  }
}

export const clipboardStore = new ClipboardStore()
```

### TailwindCSS v4

Uses the new `@theme` syntax in CSS (not JavaScript config):

```css
/* src/app.css */
@import "tailwindcss";

@theme {
  --color-primary: oklch(0.145 0 0);
  /* ... */
}
```

### Tauri Integration

- Frontend communicates with Rust backend via `invoke()` from `@tauri-apps/api/core`
- All Tauri commands should be wrapped in typed functions in `lib/tauri/commands.ts`
- Use Repository pattern to abstract Tauri backend calls

**Example:**
```typescript
// lib/tauri/commands.ts
import { invoke } from '@tauri-apps/api/core'

export async function tauriGetItems(): Promise<ClipboardItem[]> {
  return await invoke('get_clipboard_items')
}
```

### SvelteKit Configuration

- Uses `adapter-static` with SPA mode (`ssr = false`)
- Fallback to `index.html` for client-side routing
- Path alias: `$lib` maps to `src/lib`

## Key Patterns and Conventions

### Component Props (Svelte 5)

Use the new `$props()` rune:

```svelte
<script lang="ts">
  interface Props {
    title: string
    count?: number
  }

  let { title, count = 0 }: Props = $props()
</script>
```

### Two-Way Binding

Use `$bindable()` for two-way bound props:

```svelte
<script lang="ts">
  let { value = $bindable('') }: { value: string } = $props()
</script>

<input bind:value />
```

### Snippets (Not Slots)

Svelte 5 uses snippets instead of slots:

```svelte
{@render children?.()}
```

### Class Merging

Use the `cn()` utility for combining Tailwind classes:

```svelte
<div class={cn('base-classes', conditionalClasses, className)}>
```

## Development Workflow

### Working with Frontend Changes

1. Make changes to Svelte components or stores
2. Vite will hot-reload changes automatically
3. Check types with `bun run check` before committing

### Working with Rust Backend

1. Define Tauri commands in `src-tauri/src/lib.rs`
2. Register commands in the `.invoke_handler()` macro
3. Create typed wrappers in `src/lib/tauri/commands.ts`
4. Use commands through repository implementations

### Adding New Features

Follow the Clean Architecture layers:

1. **Domain**: Define types/interfaces in `lib/types/`
2. **Infrastructure**: Implement Tauri commands and repository in `lib/tauri/`
3. **Application**: Create/update stores in `lib/stores/`
4. **Presentation**: Build UI components and pages

## Database Schema

The app will use SQLite locally. Planned schema includes:

- **clipboard_items**: Main clipboard content storage
- **user_settings**: Application preferences
- Future tables for sync metadata when cloud features are added

## Testing Strategy

Testing infrastructure is not yet set up but should include:

- **Frontend**: Vitest for unit tests, Playwright for E2E
- **Backend**: Rust `cargo test` for unit and integration tests
- **Tauri Commands**: Test through frontend E2E tests

## Build and Distribution

### Development Build
```bash
bun run tauri dev
```

### Production Build
```bash
bun run tauri build
```

Outputs platform-specific installers in `src-tauri/target/release/bundle/`

## Known Issues and Gotchas

1. **Port 1420 must be available** - Tauri expects this fixed port
2. **SSR is disabled** - This is a SPA, don't try to use server-side features
3. **Svelte 5 is new** - Some community libraries may not be compatible yet
4. **Mobile support** - Tauri v2 supports mobile but requires additional setup

## Future Plans

See `svelte-frontend-development-plan.md` for detailed frontend implementation phases. The backend will eventually include:

- Clipboard monitoring and auto-save
- WebSocket server for real-time sync (Pro version)
- PostgreSQL backend for cloud sync
- File upload to S3/MinIO
- User authentication

## Resources

- [Tauri v2 Docs](https://v2.tauri.app/)
- [SvelteKit Docs](https://svelte.dev/docs/kit)
- [Svelte 5 Runes](https://svelte.dev/docs/svelte/what-are-runes)
- [TailwindCSS v4](https://tailwindcss.com/blog/tailwindcss-v4-beta)