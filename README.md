# Global Clipboard Manager

<div align="center">
  <img src=".github/app-icon.png" alt="Global Clipboard Icon" width="128" height="128">

  <p><strong>A powerful, modern clipboard manager for macOS</strong></p>

  [![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-FFC131?logo=tauri&logoColor=white)](https://tauri.app)
  [![Svelte 5](https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte&logoColor=white)](https://svelte.dev)
  [![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
  [![TypeScript](https://img.shields.io/badge/TypeScript-5.0-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org)
</div>

---

## ✨ Features

### 🚀 Core Features
- **⚡ Lightning Fast** - Built with Rust and native macOS APIs for optimal performance
- **🎯 Smart Detection** - Automatically detects and categorizes clipboard content:
  - 📝 Plain text
  - 💻 Code snippets (with syntax detection for 20+ languages)
  - 🎨 Colors (HEX, RGB, HSL, OKLCH formats)
  - 🔗 URLs (with metadata preview)
  - 📁 Files and images
- **🔍 Powerful Search** - Find anything in your clipboard history instantly
- **⭐ Favorites & Snippets** - Save frequently used items for quick access
- **🎨 Beautiful UI** - Modern, dark-themed interface built with Svelte 5 and TailwindCSS v4
- **📦 Infinite Scroll** - Efficient pagination loads 50-100 items at a time
- **🔐 Privacy First** - All data stored locally in SQLite database

### ⌨️ Productivity Features
- **⌨️ Global Hotkeys** - Quick access from anywhere (customizable)
- **🎯 Auto-Hide** - Window hides automatically when losing focus
- **🔄 Duplicate Detection** - Prevents saving the same content multiple times
- **📅 Date Grouping** - Items organized by Today, Yesterday, This Week, This Month, Older
- **🎨 Syntax Highlighting** - Beautiful code preview with language detection
- **🌈 Color Preview** - Visual color swatches with format conversion

### 🛠️ Advanced Features
- **🔗 Link Metadata** - Fetches titles, descriptions, and images from URLs
- **🌐 Domain Extraction** - Automatic domain detection for links
- **🎨 Color Conversion** - Convert between HEX, RGB, HSL, HSV, OKLCH formats
- **🔧 Clean Architecture** - Modular design following best practices

---

## 📸 Screenshots

<div align="center">
  <img src=".github/app-icon.png" alt="App in macOS Dock" width="500">
  <p><em>Global Clipboard running in the macOS dock</em></p>
</div>

---

## 🚀 Getting Started

### Prerequisites

- **macOS** 10.15+ (Catalina or later)
- **Rust** 1.70 or later ([Install](https://rustup.rs))
- **Bun** latest version ([Install](https://bun.sh))
- **Xcode Command Line Tools** (`xcode-select --install`)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/global_clipboard.git
   cd global_clipboard
   ```

2. **Install dependencies**
   ```bash
   bun install
   ```

3. **Run in development mode**
   ```bash
   bun run tauri dev
   ```

4. **Build for production**
   ```bash
   bun run tauri build
   ```

   The app bundle will be created in `src-tauri/target/release/bundle/`

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `⌘⇧V` | Toggle clipboard window (customizable in Settings) |
| `⌘K` | Focus search bar |
| `⌘W` | Close window |
| `⌘Q` | Quit application |
| `↑/↓` | Navigate clipboard items |
| `Enter` | Copy selected item to clipboard |
| `⌘⭐` | Toggle favorite |

---

## 🏗️ Architecture

### Technology Stack

**Frontend:**
- [Svelte 5](https://svelte.dev) - Reactive UI framework with Runes
- [SvelteKit](https://kit.svelte.dev) - Application framework (SPA mode)
- [TailwindCSS v4](https://tailwindcss.com) - Utility-first CSS with `@theme`
- [TypeScript](https://www.typescriptlang.org) - Type-safe JavaScript
- [Vite](https://vitejs.dev) - Fast build tool and dev server

**Backend:**
- [Tauri v2](https://tauri.app) - Lightweight desktop application framework
- [Rust](https://www.rust-lang.org) - Systems programming language
- [SQLite](https://www.sqlite.org) - Local database via `rusqlite`
- [Tauri Plugins](https://tauri.app/plugin/):
  - `tauri-plugin-global-shortcut` - System-wide keyboard shortcuts
  - `tauri-plugin-notification` - Native notifications
  - `tauri-plugin-opener` - Open files and URLs

### Project Structure

```
global_clipboard/
├── src/                          # Frontend (SvelteKit + Svelte 5)
│   ├── lib/
│   │   ├── components/          # Reusable UI components
│   │   │   ├── sidebar/         # Sidebar with items list
│   │   │   ├── content/         # Content viewer panel
│   │   │   ├── header/          # Search and filters
│   │   │   ├── settings/        # Settings components
│   │   │   └── ui/              # Base UI components
│   │   ├── stores/              # Svelte 5 runes-based stores
│   │   ├── tauri/               # Tauri command wrappers
│   │   ├── types/               # TypeScript type definitions
│   │   └── utils/               # Utility functions
│   ├── routes/                  # SvelteKit routes
│   └── app.css                  # TailwindCSS v4 with @theme
│
├── src-tauri/                   # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── clipboard/           # Clipboard monitoring
│   │   │   ├── monitor.rs       # Background clipboard watcher
│   │   │   ├── operations.rs   # Read/write operations
│   │   │   └── types.rs         # Content type detection
│   │   ├── commands/            # Tauri commands (API)
│   │   │   ├── clipboard.rs    # CRUD operations
│   │   │   ├── settings.rs     # Settings & hotkeys
│   │   │   ├── colors.rs       # Color conversion
│   │   │   └── links.rs         # URL metadata
│   │   ├── db/                  # Database layer
│   │   │   ├── repository.rs   # Data access with pagination
│   │   │   ├── models.rs       # Data models
│   │   │   └── schema.rs       # SQLite schema
│   │   ├── shortcuts.rs         # Global hotkey management
│   │   └── lib.rs               # App initialization
│   └── capabilities/            # Tauri permissions
│
└── README.md                    # This file
```

### Clean Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│                    PRESENTATION                          │
│        (routes/ + lib/components/)                       │
│  • Svelte 5 components with Runes                       │
│  • No business logic                                    │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                  APPLICATION                             │
│              (lib/stores/)                               │
│  • State management with $state, $derived               │
│  • Business logic orchestration                         │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                 INFRASTRUCTURE                           │
│           (lib/tauri/ + Rust backend)                    │
│  • Tauri command wrappers (TypeScript)                  │
│  • Repository implementations (Rust)                     │
│  • Database access (SQLite)                             │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│                    DOMAIN                                │
│        (lib/types/ + Rust models)                        │
│  • Type definitions & interfaces                        │
│  • Business rules & validation                          │
└─────────────────────────────────────────────────────────┘
```

---

## 🗄️ Database Schema

```sql
CREATE TABLE clipboard_items (
    id TEXT PRIMARY KEY,
    content_type TEXT NOT NULL,      -- 'text', 'code', 'color', 'link', 'file', 'image'
    content_text TEXT,
    content_metadata TEXT,            -- JSON: { language, domain, formats, etc. }
    source_app TEXT,
    code_language TEXT,
    file_url TEXT,
    file_name TEXT,
    file_size_bytes INTEGER,
    file_mime_type TEXT,
    is_favorite INTEGER DEFAULT 0,
    is_snippet INTEGER DEFAULT 0,
    snippet_name TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced INTEGER DEFAULT 0,
    server_id TEXT
);

CREATE TABLE user_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

---

## 🎨 Key Design Decisions

### Why Svelte 5?
- **Runes system** provides clearer reactivity than stores
- **Smaller bundle size** compared to React/Vue
- **Better performance** with compile-time optimization
- **Cleaner syntax** reduces boilerplate

### Why Tauri?
- **Lightweight** - 600KB bundle vs 100MB+ Electron
- **Fast startup** - Native webview, no bundled Chromium
- **Secure** - Rust backend with explicit permissions
- **Cross-platform** - Single codebase for macOS, Windows, Linux

### Why Rust?
- **Memory safety** without garbage collection
- **Performance** - Near C/C++ speed
- **Reliability** - Prevents crashes and data races
- **Modern tooling** - Cargo, clippy, rustfmt

### Pagination Strategy
- **Server-side pagination** (50-100 items per page)
- **Infinite scroll** for smooth UX
- **Optimized for mobile** and large datasets
- **Memory efficient** - Only loads visible items

---

## 🔧 Development

### Run Tests
```bash
# Rust tests
cargo test

# TypeScript checks
bun run check

# Linting
cargo clippy
```

### Format Code
```bash
# Rust
cargo fmt

# TypeScript/Svelte
bunx prettier --write src
```

### Build Commands
```bash
# Development
bun run dev              # Frontend only
bun run tauri dev        # Full app with hot reload

# Production
bun run build            # Build frontend
bun run tauri build      # Build app bundle

# Type checking
bun run check            # Check types
bun run check:watch      # Watch mode
```

---

## 🐛 Troubleshooting

### Port 1420 already in use
```bash
lsof -ti:1420 | xargs kill -9
```

### Hotkey conflicts
Go to Settings and change the global hotkey to avoid conflicts with other apps.

### App won't start
```bash
# Kill existing processes
pkill -9 global_clipboard

# Clear app data (WARNING: deletes all clipboard history)
rm -rf ~/Library/Application\ Support/clip/
```

### Build fails on macOS
Ensure Xcode Command Line Tools are installed:
```bash
xcode-select --install
```

---

## 🔒 Privacy & Security

- **100% Local** - All data stored on your device
- **No telemetry** - We don't track or collect any data
- **No internet required** - Works completely offline (except URL metadata fetching)
- **Sandboxed** - Tauri security model with explicit permissions
- **Open source** - Audit the code yourself

**Database location:** `~/Library/Application Support/clip/clipboard.db`

---

## 🚧 Roadmap

### v1.1 (Next Release)
- [ ] Windows and Linux support
- [ ] Custom themes
- [ ] Export/import clipboard history
- [ ] Sync between devices (optional, cloud-based)

### v1.2
- [ ] Plugin system
- [ ] Advanced search filters
- [ ] OCR for images
- [ ] Password protection

### v2.0
- [ ] Mobile apps (iOS/Android)
- [ ] End-to-end encrypted sync
- [ ] Team collaboration features
- [ ] API for integrations

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Follow the existing code style
- Run tests before submitting PR
- Update documentation as needed
- Use conventional commits

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Tauri Team](https://tauri.app) - For the amazing framework
- [Svelte Team](https://svelte.dev) - For Svelte 5 and the new Runes system
- [Rust Community](https://www.rust-lang.org) - For the incredible language and ecosystem
- [TailwindCSS](https://tailwindcss.com) - For the utility-first CSS framework

---

## 📬 Contact

**Created by:** [@laruina](https://github.com/laruina)

**Issues:** [GitHub Issues](https://github.com/yourusername/global_clipboard/issues)

---

<div align="center">
  <strong>⭐ Star this repo if you find it useful! ⭐</strong>

  <br/>

  Made with ❤️ using Tauri, Svelte 5, and Rust
</div>
