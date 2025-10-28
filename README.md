# Global Clipboard Manager - Arquitectura Completa v2

## Tabla de Contenidos
1. [Visión General del Proyecto](#visión-general-del-proyecto)
2. [Stack Tecnológico](#stack-tecnológico)
3. [Estructura de Repositorios](#estructura-de-repositorios)
4. [Arquitectura del Sistema](#arquitectura-del-sistema)
5. [Base de Datos](#base-de-datos)
6. [Patrones de Diseño](#patrones-de-diseño)
7. [API Endpoints](#api-endpoints)
8. [Sistema de Tiempo Real](#sistema-de-tiempo-real)
9. [Estrategia de Caché](#estrategia-de-caché)
10. [Almacenamiento de Archivos](#almacenamiento-de-archivos)
11. [Autenticación y Autorización](#autenticación-y-autorización)
12. [Testing](#testing)
13. [Monitoreo y Observabilidad](#monitoreo-y-observabilidad)
14. [Seguridad](#seguridad)
15. [Plan de Implementación](#plan-de-implementación)
16. [Infraestructura y DevOps](#infraestructura-y-devops)
17. [Escalabilidad](#escalabilidad)
18. [Modelo de Negocio](#modelo-de-negocio)

---

## Visión General del Proyecto

### Descripción
Aplicación de gestión de clipboard multiplataforma que permite a los usuarios copiar contenido en un dispositivo y pegarlo en otros dispositivos, independientemente del sistema operativo.

### Características Principales
- **Sincronización en tiempo real** entre dispositivos
- **Soporte multiplataforma**: Windows, macOS, Linux, iOS, Android, tablets
- **Tipos de contenido**: texto, rich text, código, colores, links, imágenes, archivos
- **Capacidad**: hasta 5,000 items por usuario
- **Retención**: 1 año de historial automático
- **Búsqueda full-text** avanzada
- **Favoritos y snippets**
- **Tamaño máximo**: 10 MB por item
- **Modelo freemium**: versión gratuita (local) y premium (sincronización en la nube)

### Métricas Objetivo
- **Usuarios iniciales**: 1,000
- **Usuarios escalados**: 1M+
- **Items por usuario**: hasta 5,000
- **Latencia**: <100ms para sincronización en tiempo real
- **Disponibilidad**: 99.9% uptime

---

## Stack Tecnológico

### Frontend (Cliente)
- **Framework**: Tauri v2 (Rust + WebView nativo)
- **UI Framework**: React / Svelte / Vue (a elegir)
- **Lenguaje**: TypeScript
- **Gestión de Estado**: 
  - TanStack Query (React Query) - servidor state
  - Zustand / Jotai - estado local
- **Estilos**: TailwindCSS
- **Almacenamiento Local**: SQLite (vía Tauri)
- **Comunicación**: WebSocket + REST API

### Backend
- **Framework**: Rust + Axum
- **Lenguaje**: Rust (estabilidad, performance, type-safety)
- **Alternativa**: Bun + Hono/Elysia (si prefieres JavaScript/TypeScript)

### Base de Datos
- **Principal**: PostgreSQL 16+
- **Caché & Pub/Sub**: Redis 7+
- **Local (Cliente)**: SQLite

### Almacenamiento de Archivos
- **Desarrollo**: MinIO (S3-compatible)
- **Producción**: AWS S3
- **CDN**: CloudFront (AWS) o CloudFlare

### Infraestructura Inicial (Económica)
- **Deployment**: Railway / Render / Fly.io
- **Database**: Supabase (PostgreSQL + Auth + Storage integrados)
- **Redis**: Upstash (serverless, tier gratuito generoso)
- **Object Storage**: Supabase Storage / MinIO

### Infraestructura Escalada (AWS)
- **Compute**: ECS Fargate / EKS (Kubernetes)
- **Database**: RDS PostgreSQL con read replicas
- **Cache**: ElastiCache Redis (cluster mode)
- **Storage**: S3 + CloudFront
- **Load Balancer**: Application Load Balancer (ALB)
- **DNS**: Route 53
- **Secrets**: AWS Secrets Manager

### Herramientas de Desarrollo
- **Package Manager**: Bun (frontend), Cargo (Rust)
- **CI/CD**: GitHub Actions
- **IaC**: Terraform / Pulumi
- **Containers**: Docker + Docker Compose
- **Orquestación**: Kubernetes (para escalar)

---

## Estructura de Repositorios

### Arquitectura Multirepo (2 Repositorios Principales)

**Razón**: Simplicidad, independencia de deployment, CI/CD más rápido, menor complejidad.

```
Organización GitHub: clipboard-manager

Repositorios:

1. clipboard-app          (Frontend - Desktop & Mobile)
2. clipboard-backend      (Backend - API + Workers + Infrastructure)
```

### Ventajas de esta Estructura

1. **Simplicidad**: Cada repo es fácil de entender y mantener
2. **Independencia**: Deploy del backend sin afectar apps cliente
3. **CI/CD rápido**: Solo testea/compila lo que cambió
4. **Permisos granulares**: Fácil dar acceso específico
5. **No tooling extra**: No necesitas Turborepo, Nx, etc.
6. **Escalabilidad**: Fácil agregar más repos (web dashboard, admin panel)

---

## Repositorio 1: clipboard-app

### Estructura Detallada

```
clipboard-app/
├── src-tauri/                        # Rust backend de Tauri
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   │
│   │   ├── commands/                 # Tauri commands (API para frontend)
│   │   │   ├── mod.rs
│   │   │   ├── clipboard.rs          # Clipboard operations
│   │   │   ├── sync.rs               # Sync operations
│   │   │   ├── auth.rs               # Auth operations
│   │   │   ├── storage.rs            # Local storage operations
│   │   │   └── settings.rs           # Settings operations
│   │   │
│   │   ├── storage/                  # SQLite local storage
│   │   │   ├── mod.rs
│   │   │   ├── database.rs           # Database connection
│   │   │   ├── schema.rs             # SQLite schema
│   │   │   ├── migrations/           # SQLite migrations
│   │   │   │   ├── mod.rs
│   │   │   │   ├── m001_initial.rs
│   │   │   │   └── m002_add_indexes.rs
│   │   │   └── models.rs             # Local models
│   │   │
│   │   ├── sync/                     # Sync engine
│   │   │   ├── mod.rs
│   │   │   ├── engine.rs             # Main sync logic
│   │   │   ├── websocket.rs          # WebSocket client
│   │   │   ├── conflict_resolution.rs # Conflict handling
│   │   │   ├── queue.rs              # Offline queue
│   │   │   └── state.rs              # Sync state management
│   │   │
│   │   ├── clipboard/                # OS clipboard integration
│   │   │   ├── mod.rs
│   │   │   ├── listener.rs           # Clipboard change listener
│   │   │   ├── reader.rs             # Read from clipboard
│   │   │   ├── writer.rs             # Write to clipboard
│   │   │   ├── desktop.rs            # Desktop-specific (Win/Mac/Linux)
│   │   │   └── mobile.rs             # Mobile-specific (iOS/Android)
│   │   │
│   │   ├── platform/                 # Platform-specific code
│   │   │   ├── mod.rs
│   │   │   ├── windows.rs            # Windows APIs
│   │   │   ├── macos.rs              # macOS APIs
│   │   │   ├── linux.rs              # Linux APIs
│   │   │   ├── ios.rs                # iOS APIs
│   │   │   └── android.rs            # Android APIs
│   │   │
│   │   ├── api/                      # HTTP client for backend
│   │   │   ├── mod.rs
│   │   │   ├── client.rs             # HTTP client setup
│   │   │   ├── auth.rs               # Auth endpoints
│   │   │   ├── clipboard.rs          # Clipboard endpoints
│   │   │   ├── files.rs              # File upload/download
│   │   │   └── types.rs              # API types (copied from backend)
│   │   │
│   │   ├── utils/
│   │   │   ├── mod.rs
│   │   │   ├── crypto.rs             # Encryption utils
│   │   │   ├── logger.rs             # Logging setup
│   │   │   ├── notifications.rs      # System notifications
│   │   │   └── shortcuts.rs          # Keyboard shortcuts
│   │   │
│   │   └── error.rs                  # Error types
│   │
│   ├── icons/                        # App icons (all platforms)
│   │   ├── icon.png
│   │   ├── icon.icns                 # macOS
│   │   ├── icon.ico                  # Windows
│   │   └── ...
│   │
│   ├── gen/                          # Generated platform code
│   │   ├── android/                  # Android project
│   │   │   ├── app/
│   │   │   └── build.gradle
│   │   └── apple/                    # iOS/macOS project
│   │       └── ...
│   │
│   ├── capabilities/                 # Platform-specific permissions
│   │   ├── desktop.json
│   │   ├── mobile.json
│   │   └── default.json
│   │
│   ├── Cargo.toml
│   ├── tauri.conf.json               # Tauri configuration
│   └── build.rs
│
├── src/                              # Frontend (TypeScript/React/Svelte/Vue)
│   ├── main.tsx                      # Entry point
│   ├── App.tsx
│   │
│   ├── components/
│   │   ├── shared/                   # Shared components (desktop & mobile)
│   │   │   ├── ClipboardList/
│   │   │   │   ├── ClipboardList.tsx
│   │   │   │   ├── ClipboardItem.tsx
│   │   │   │   └── ClipboardItem.module.css
│   │   │   ├── SearchBar/
│   │   │   │   ├── SearchBar.tsx
│   │   │   │   └── SearchBar.module.css
│   │   │   ├── EmptyState/
│   │   │   ├── LoadingSpinner/
│   │   │   └── ErrorBoundary/
│   │   │
│   │   ├── desktop/                  # Desktop-specific components
│   │   │   ├── Sidebar/
│   │   │   ├── TitleBar/
│   │   │   ├── ContextMenu/
│   │   │   ├── KeyboardShortcuts/
│   │   │   └── SystemTray/
│   │   │
│   │   └── mobile/                   # Mobile-specific components
│   │       ├── BottomNav/
│   │       ├── SwipeActions/
│   │       ├── MobileHeader/
│   │       └── PullToRefresh/
│   │
│   ├── pages/                        # Pages/Screens
│   │   ├── Home/
│   │   │   ├── Home.tsx
│   │   │   └── Home.module.css
│   │   ├── Search/
│   │   ├── Favorites/
│   │   ├── Snippets/
│   │   ├── Settings/
│   │   └── Auth/
│   │       ├── Login.tsx
│   │       └── Register.tsx
│   │
│   ├── hooks/                        # Custom React hooks
│   │   ├── useClipboard.ts           # Clipboard operations
│   │   ├── useSync.ts                # Sync state
│   │   ├── useWebSocket.ts           # WebSocket connection
│   │   ├── usePlatform.ts            # Platform detection
│   │   ├── useAuth.ts                # Authentication
│   │   ├── useSearch.ts              # Search functionality
│   │   ├── useKeyboardShortcuts.ts   # Keyboard shortcuts
│   │   └── useOfflineQueue.ts        # Offline queue management
│   │
│   ├── stores/                       # State management (Zustand/Jotai)
│   │   ├── authStore.ts
│   │   ├── clipboardStore.ts
│   │   ├── syncStore.ts
│   │   ├── settingsStore.ts
│   │   └── uiStore.ts
│   │
│   ├── lib/                          # Libraries and utilities
│   │   ├── api/                      # API client
│   │   │   ├── client.ts             # Axios/fetch wrapper
│   │   │   ├── auth.ts               # Auth endpoints
│   │   │   ├── clipboard.ts          # Clipboard endpoints
│   │   │   ├── files.ts              # File endpoints
│   │   │   └── websocket.ts          # WebSocket client
│   │   │
│   │   ├── tauri/                    # Tauri commands wrapper
│   │   │   ├── commands.ts           # Typed Tauri commands
│   │   │   ├── events.ts             # Tauri events
│   │   │   └── storage.ts            # Local storage
│   │   │
│   │   ├── utils/
│   │   │   ├── platform.ts           # Platform detection
│   │   │   ├── format.ts             # Formatting utils
│   │   │   ├── validation.ts         # Validation
│   │   │   └── clipboard.ts          # Clipboard helpers
│   │   │
│   │   └── constants.ts              # Constants
│   │
│   ├── types/                        # TypeScript types
│   │   ├── clipboard.ts              # Copied from backend
│   │   ├── user.ts                   # Copied from backend
│   │   ├── api.ts                    # Copied from backend
│   │   ├── tauri.ts                  # Tauri-specific types
│   │   └── index.ts
│   │
│   ├── styles/                       # Global styles
│   │   ├── globals.css
│   │   ├── variables.css
│   │   ├── desktop.css               # Desktop-specific styles
│   │   ├── mobile.css                # Mobile-specific styles
│   │   └── themes/
│   │       ├── light.css
│   │       └── dark.css
│   │
│   ├── assets/                       # Static assets
│   │   ├── images/
│   │   ├── fonts/
│   │   └── icons/
│   │
│   └── config/
│       ├── constants.ts
│       └── environment.ts
│
├── public/                           # Public assets
│   └── favicon.ico
│
├── tests/                            # Tests
│   ├── unit/
│   ├── integration/
│   └── e2e/
│
├── scripts/                          # Build and utility scripts
│   ├── sync-types.sh                 # Sync types from backend
│   └── build-all.sh
│
├── .github/
│   └── workflows/
│       ├── ci.yml                    # CI for all platforms
│       ├── build-desktop.yml         # Build desktop apps
│       ├── build-mobile.yml          # Build mobile apps
│       └── release.yml               # Release workflow
│
├── package.json
├── bun.lockb
├── tsconfig.json
├── tailwind.config.js
├── vite.config.ts
├── .gitignore
├── .env.example
└── README.md
```

### Scripts Importantes

**package.json**:
```json
{
  "scripts": {
    "dev": "tauri dev",
    "dev:mobile": "tauri android dev",
    "build": "tauri build",
    "build:desktop": "tauri build --target desktop",
    "build:ios": "tauri ios build",
    "build:android": "tauri android build",
    "test": "vitest",
    "test:e2e": "playwright test",
    "lint": "eslint src",
    "sync-types": "./scripts/sync-types.sh"
  }
}
```

---

## Repositorio 2: clipboard-backend

### Estructura Detallada

```
clipboard-backend/
├── api-server/                       # Main API Server (Rust + Axum)
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   │
│   │   ├── api/                      # API Layer (Presentation)
│   │   │   ├── mod.rs
│   │   │   │
│   │   │   ├── routes/               # HTTP routes
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth.rs           # POST /auth/login, /register, etc.
│   │   │   │   ├── clipboard.rs      # CRUD /clipboard/items
│   │   │   │   ├── users.rs          # GET /users/me, PATCH /users/me
│   │   │   │   ├── devices.rs        # GET /devices, POST /devices/register
│   │   │   │   ├── snippets.rs       # CRUD /snippets
│   │   │   │   ├── search.rs         # GET /clipboard/search
│   │   │   │   ├── files.rs          # POST /clipboard/upload, GET /files/:id
│   │   │   │   ├── health.rs         # GET /health, /health/ready
│   │   │   │   └── websocket.rs      # WS /ws
│   │   │   │
│   │   │   ├── middleware/           # HTTP middleware
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth.rs           # JWT validation
│   │   │   │   ├── rate_limit.rs     # Rate limiting
│   │   │   │   ├── cors.rs           # CORS configuration
│   │   │   │   ├── logging.rs        # Request logging
│   │   │   │   └── error_handler.rs  # Global error handler
│   │   │   │
│   │   │   ├── dto/                  # Data Transfer Objects
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth_dto.rs       # LoginDto, RegisterDto, etc.
│   │   │   │   ├── clipboard_dto.rs  # CreateItemDto, UpdateItemDto
│   │   │   │   ├── user_dto.rs       # UpdateUserDto, etc.
│   │   │   │   └── common.rs         # PaginationDto, etc.
│   │   │   │
│   │   │   └── response.rs           # Standard API responses
│   │   │
│   │   ├── domain/                   # Domain Layer (Business Logic Core)
│   │   │   ├── mod.rs
│   │   │   │
│   │   │   ├── entities/             # Domain entities
│   │   │   │   ├── mod.rs
│   │   │   │   ├── user.rs           # User entity
│   │   │   │   ├── clipboard_item.rs # ClipboardItem entity
│   │   │   │   ├── device.rs         # Device entity
│   │   │   │   ├── session.rs        # Session entity
│   │   │   │   ├── tag.rs            # Tag entity
│   │   │   │   └── snippet.rs        # Snippet entity
│   │   │   │
│   │   │   ├── repositories/         # Repository traits (interfaces)
│   │   │   │   ├── mod.rs
│   │   │   │   ├── user_repository.rs
│   │   │   │   ├── clipboard_repository.rs
│   │   │   │   ├── device_repository.rs
│   │   │   │   ├── session_repository.rs
│   │   │   │   └── tag_repository.rs
│   │   │   │
│   │   │   ├── value_objects/        # Value objects
│   │   │   │   ├── mod.rs
│   │   │   │   ├── email.rs          # Email validation
│   │   │   │   ├── content_type.rs   # ContentType enum
│   │   │   │   └── subscription_tier.rs
│   │   │   │
│   │   │   └── events/               # Domain events
│   │   │       ├── mod.rs
│   │   │       ├── clipboard_events.rs
│   │   │       └── user_events.rs
│   │   │
│   │   ├── application/              # Application Layer (Use Cases)
│   │   │   ├── mod.rs
│   │   │   │
│   │   │   ├── services/             # Application services
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth_service.rs   # Login, register, refresh
│   │   │   │   ├── clipboard_service.rs # CRUD items, validation
│   │   │   │   ├── search_service.rs # Full-text search
│   │   │   │   ├── file_service.rs   # Upload, download, thumbnails
│   │   │   │   ├── sync_service.rs   # Sync logic, conflicts
│   │   │   │   └── notification_service.rs # WebSocket broadcasting
│   │   │   │
│   │   │   └── use_cases/            # Specific use cases
│   │   │       ├── mod.rs
│   │   │       ├── create_clipboard_item.rs
│   │   │       ├── sync_items.rs
│   │   │       ├── search_items.rs
│   │   │       └── share_item.rs
│   │   │
│   │   ├── infrastructure/           # Infrastructure Layer (External)
│   │   │   ├── mod.rs
│   │   │   │
│   │   │   ├── database/             # Database connection
│   │   │   │   ├── mod.rs
│   │   │   │   ├── connection.rs     # Connection pool
│   │   │   │   └── models.rs         # SQLx models
│   │   │   │
│   │   │   ├── repositories/         # Repository implementations
│   │   │   │   ├── mod.rs
│   │   │   │   ├── postgres_user_repository.rs
│   │   │   │   ├── postgres_clipboard_repository.rs
│   │   │   │   ├── postgres_device_repository.rs
│   │   │   │   ├── postgres_session_repository.rs
│   │   │   │   └── postgres_tag_repository.rs
│   │   │   │
│   │   │   ├── cache/                # Redis cache
│   │   │   │   ├── mod.rs
│   │   │   │   ├── redis_client.rs   # Redis connection
│   │   │   │   └── cache_service.rs  # Cache operations
│   │   │   │
│   │   │   ├── storage/              # File storage
│   │   │   │   ├── mod.rs
│   │   │   │   ├── s3_storage.rs     # S3 implementation
│   │   │   │   └── storage_trait.rs  # Storage interface
│   │   │   │
│   │   │   ├── realtime/             # WebSocket & Pub/Sub
│   │   │   │   ├── mod.rs
│   │   │   │   ├── websocket_manager.rs # WebSocket connections
│   │   │   │   ├── redis_pubsub.rs   # Redis Pub/Sub
│   │   │   │   └── message_broker.rs # Message routing
│   │   │   │
│   │   │   └── external/             # External services
│   │   │       ├── mod.rs
│   │   │       ├── email_service.rs  # Email sending
│   │   │       └── payment_service.rs # Stripe/Paddle
│   │   │
│   │   ├── config/                   # Configuration
│   │   │   ├── mod.rs
│   │   │   ├── settings.rs           # App settings
│   │   │   └── environment.rs        # Environment variables
│   │   │
│   │   └── utils/                    # Utilities
│   │       ├── mod.rs
│   │       ├── jwt.rs                # JWT generation/validation
│   │       ├── password.rs           # Password hashing (bcrypt)
│   │       ├── validation.rs         # Input validation
│   │       └── error.rs              # Error types
│   │
│   ├── migrations/                   # Database migrations (sqlx)
│   │   ├── 20250101000001_create_users.sql
│   │   ├── 20250101000002_create_devices.sql
│   │   ├── 20250101000003_create_clipboard_items.sql
│   │   ├── 20250101000004_create_sessions.sql
│   │   ├── 20250101000005_create_tags.sql
│   │   ├── 20250101000006_create_audit_logs.sql
│   │   └── ...
│   │
│   ├── tests/
│   │   ├── unit/                     # Unit tests
│   │   ├── integration/              # Integration tests
│   │   └── common/                   # Test utilities
│   │
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── README.md
│
├── workers/                          # Background workers
│   ├── cleanup-worker/               # Limpia items expirados
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   └── cleanup.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── analytics-worker/             # Procesa analytics
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   └── analytics.rs
│   │   └── Cargo.toml
│   │
│   └── thumbnail-worker/             # Genera thumbnails
│       ├── src/
│       │   ├── main.rs
│       │   └── thumbnail.rs
│       └── Cargo.toml
│
├── infrastructure/                   # IaC y deployment
│   ├── docker/
│   │   ├── Dockerfile.api
│   │   ├── Dockerfile.worker
│   │   ├── docker-compose.yml        # Development
│   │   └── docker-compose.prod.yml   # Production
│   │
│   ├── kubernetes/                   # K8s manifests (para escalar)
│   │   ├── base/
│   │   │   ├── namespace.yaml
│   │   │   ├── configmap.yaml
│   │   │   ├── api-deployment.yaml
│   │   │   ├── api-service.yaml
│   │   │   ├── api-hpa.yaml          # Horizontal Pod Autoscaler
│   │   │   ├── redis-deployment.yaml
│   │   │   ├── redis-service.yaml
│   │   │   ├── workers-deployment.yaml
│   │   │   └── ingress.yaml
│   │   │
│   │   ├── overlays/
│   │   │   ├── development/
│   │   │   ├── staging/
│   │   │   └── production/
│   │   │
│   │   └── kustomization.yaml
│   │
│   ├── terraform/                    # AWS Infrastructure as Code
│   │   ├── modules/
│   │   │   ├── vpc/
│   │   │   │   ├── main.tf
│   │   │   │   ├── variables.tf
│   │   │   │   └── outputs.tf
│   │   │   ├── rds/
│   │   │   ├── elasticache/
│   │   │   ├── ecs/
│   │   │   ├── eks/
│   │   │   ├── s3/
│   │   │   ├── cloudfront/
│   │   │   └── alb/
│   │   │
│   │   ├── environments/
│   │   │   ├── dev/
│   │   │   │   ├── main.tf
│   │   │   │   ├── variables.tf
│   │   │   │   └── terraform.tfvars
│   │   │   ├── staging/
│   │   │   └── production/
│   │   │
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   ├── outputs.tf
│   │   └── README.md
│   │
│   └── scripts/
│       ├── setup-dev.sh              # Setup local development
│       ├── deploy-staging.sh
│       ├── deploy-production.sh
│       ├── backup-db.sh
│       └── restore-db.sh
│
├── docs/                             # Documentación
│   ├── api/
│   │   ├── openapi.yaml              # OpenAPI spec
│   │   └── postman-collection.json
│   ├── architecture/
│   │   ├── system-design.md
│   │   ├── database-schema.md
│   │   └── deployment-architecture.md
│   ├── development/
│   │   ├── setup-guide.md
│   │   ├── coding-standards.md
│   │   ├── testing-guide.md
│   │   └── contributing.md
│   └── operations/
│       ├── deployment-guide.md
│       ├── monitoring-guide.md
│       └── disaster-recovery.md
│
├── scripts/                          # Utility scripts
│   ├── seed-db.sql                   # Seed data for development
│   ├── benchmark.sh                  # Performance benchmarking
│   └── export-types.sh               # Export types for frontend
│
├── .github/
│   └── workflows/
│       ├── ci.yml                    # Continuous Integration
│       ├── cd-staging.yml            # Deploy to staging
│       ├── cd-production.yml         # Deploy to production
│       ├── security-scan.yml         # Security scanning
│       └── dependency-update.yml     # Automated dependency updates
│
├── .gitignore
├── .env.example
├── Cargo.toml                        # Workspace Cargo.toml
├── Cargo.lock
└── README.md
```

### Workspace Cargo.toml

```toml
[workspace]
members = [
    "api-server",
    "workers/cleanup-worker",
    "workers/analytics-worker",
    "workers/thumbnail-worker",
]

[workspace.dependencies]
# Shared dependencies across all workspace members
tokio = { version = "1.35", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "uuid", "chrono"] }
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

---

## Arquitectura del Sistema

### Diagrama de Arquitectura de Alto Nivel

```
┌─────────────────────────────────────────────────────────────────┐
│                      CAPA DE CLIENTES                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │  Tauri   │  │  Tauri   │  │  Tauri   │  │  Tauri   │        │
│  │ Windows  │  │   macOS  │  │   iOS    │  │ Android  │        │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘        │
│       │             │             │             │                │
│       │      SQLite local + Sync Engine         │                │
│       └─────────────┴─────────────┴─────────────┘                │
│                         │                                        │
│                         │ HTTPS/WSS                              │
└─────────────────────────┼────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                   API GATEWAY / LOAD BALANCER                    │
│              (Nginx/Traefik o AWS ALB + CloudFront)             │
└─────────────────────────┬───────────────────────────────────────┘
                          │
        ┌─────────────────┴─────────────────┐
        │                                   │
        ▼                                   ▼
┌──────────────────┐              ┌──────────────────────┐
│   Auth Service   │              │  Clipboard Service   │
│   (REST API)     │              │  (REST + WebSocket)  │
│                  │              │                      │
│ - Register       │              │ - CRUD Items         │
│ - Login          │              │ - Search             │
│ - Refresh Token  │              │ - Real-time Sync     │
│ - Logout         │              │ - File Upload        │
└────────┬─────────┘              └──────────┬───────────┘
         │                                   │
         │        ┌──────────────────────────┴──────────────┐
         │        │                                          │
         ▼        ▼                                          ▼
┌────────────────────┐     ┌──────────────┐     ┌──────────────────┐
│   PostgreSQL       │     │    Redis     │     │  S3 / MinIO      │
│   (Primary DB)     │     │              │     │  (Object Storage)│
│                    │     │ - Cache      │     │                  │
│ - Users            │     │ - Sessions   │     │ - Images         │
│ - Devices          │     │ - Pub/Sub    │     │ - Files          │
│ - Clipboard Items  │     │ - Rate Limit │     │ - Thumbnails     │
│ - Tags             │     └──────────────┘     └──────────────────┘
│ - Sessions         │
│ - Audit Logs       │
└────────────────────┘

         ┌──────────────────────────────────┐
         │      Background Workers          │
         ├──────────────────────────────────┤
         │ - Cleanup Worker (items >1 año)  │
         │ - Analytics Worker               │
         │ - Thumbnail Generator            │
         │ - Email Worker (futuro)          │
         └──────────────────────────────────┘
```

### Flujo de Sincronización en Tiempo Real

```
Dispositivo A (Windows)           Backend (Rust)              Dispositivo B (iOS)
      │                                │                              │
      │  1. Copy texto                 │                              │
      │     ↓ Clipboard Listener       │                              │
      │  2. Save local (SQLite)        │                              │
      │                                │                              │
      │  3. POST /clipboard/items      │                              │
      ├───────────────────────────────>│                              │
      │                                │                              │
      │                           4. Guardar en                       │
      │                              PostgreSQL                       │
      │                                │                              │
      │                           5. Publish a                        │
      │                           Redis channel                       │
      │                          "clipboard:{user_id}"                │
      │                                │                              │
      │                                │─────────────────────────────>│
      │                                │  6. WS: NewItem event        │
      │                                │                              │
      │  7. Response: Item created     │                              │
      │<───────────────────────────────┤                              │
      │                                │       7. Save local (SQLite) │
      │                                │       8. Update UI           │
      │                                │       9. Show notification   │
      │                                │<─────────────────────────────│
```

### Arquitectura en Capas (Clean Architecture)

```
┌─────────────────────────────────────────────────────┐
│              PRESENTATION LAYER                      │
│  (API Routes, WebSocket Handlers, DTOs)             │
│                                                      │
│  • HTTP Routes (Axum)                               │
│  • Middleware (Auth, Rate Limit, CORS)              │
│  • Request/Response DTOs                            │
│  • WebSocket handlers                               │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│             APPLICATION LAYER                        │
│  (Services, Use Cases, Business Logic)              │
│                                                      │
│  • AuthService                                      │
│  • ClipboardService                                 │
│  • SearchService                                    │
│  • FileService                                      │
│  • SyncService                                      │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│               DOMAIN LAYER                           │
│  (Entities, Domain Models, Repository Interfaces)   │
│                                                      │
│  • User, ClipboardItem, Device (Entities)           │
│  • Email, ContentType (Value Objects)               │
│  • Repository Traits (interfaces)                   │
│  • Domain Events                                    │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│           INFRASTRUCTURE LAYER                       │
│  (DB Implementations, External Services, Storage)   │
│                                                      │
│  • PostgreSQL Repository Implementations            │
│  • Redis Cache                                      │
│  • S3 Storage                                       │
│  • WebSocket Manager                                │
│  • Email Service                                    │
└─────────────────────────────────────────────────────┘
```

---

## Base de Datos

### Esquema PostgreSQL

#### Tabla: users
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    
    -- Suscripción
    subscription_tier VARCHAR(50) DEFAULT 'free' CHECK (subscription_tier IN ('free', 'pro', 'enterprise')),
    subscription_starts_at TIMESTAMPTZ,
    subscription_expires_at TIMESTAMPTZ,
    subscription_auto_renew BOOLEAN DEFAULT true,
    
    -- Límites por tier
    max_items INTEGER DEFAULT 0, -- 0 para free (solo local), 5000 para pro
    max_file_size_mb INTEGER DEFAULT 0,
    
    -- Metadata
    email_verified BOOLEAN DEFAULT false,
    email_verification_token VARCHAR(255),
    password_reset_token VARCHAR(255),
    password_reset_expires_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ -- Soft delete
);

CREATE INDEX idx_users_email ON users(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_subscription ON users(subscription_tier, subscription_expires_at);
```

#### Tabla: devices
```sql
CREATE TABLE devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Información del dispositivo
    device_name VARCHAR(255) NOT NULL,
    device_type VARCHAR(50) NOT NULL CHECK (device_type IN ('windows', 'macos', 'linux', 'ios', 'android', 'tablet')),
    device_os_version VARCHAR(100),
    app_version VARCHAR(50),
    
    -- Identificación única
    device_fingerprint VARCHAR(255) UNIQUE NOT NULL,
    
    -- Estado
    is_active BOOLEAN DEFAULT true,
    last_active_at TIMESTAMPTZ DEFAULT NOW(),
    last_ip_address INET,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, device_fingerprint)
);

CREATE INDEX idx_devices_user ON devices(user_id, is_active);
CREATE INDEX idx_devices_last_active ON devices(last_active_at);
```

#### Tabla: clipboard_items
```sql
CREATE TABLE clipboard_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id UUID REFERENCES devices(id) ON DELETE SET NULL,
    
    -- Tipo de contenido
    content_type VARCHAR(50) NOT NULL CHECK (content_type IN (
        'text',        -- Texto plano
        'rich_text',   -- HTML/Markdown
        'code',        -- Código con syntax
        'color',       -- Color hex/rgb
        'link',        -- URL
        'image',       -- Imagen
        'file'         -- Archivo genérico
    )),
    
    -- Contenido textual (para text, rich_text, code, color, link)
    content_text TEXT,
    
    -- Metadata adicional (JSON)
    content_metadata JSONB DEFAULT '{}',
    -- Ejemplos:
    -- Para code: {"language": "javascript", "fileName": "example.js"}
    -- Para color: {"hex": "#FF5733", "rgb": "255,87,51", "name": "Coral"}
    -- Para link: {"title": "GitHub", "favicon": "url"}
    
    -- Archivos e imágenes
    file_url VARCHAR(500),              -- URL en S3
    file_name VARCHAR(500),
    file_size_bytes BIGINT,
    file_mime_type VARCHAR(100),
    thumbnail_url VARCHAR(500),          -- Para imágenes
    
    -- Organización
    is_favorite BOOLEAN DEFAULT false,
    is_snippet BOOLEAN DEFAULT false,
    snippet_name VARCHAR(255),           -- Solo si is_snippet = true
    snippet_description TEXT,
    
    -- Búsqueda full-text
    search_vector tsvector GENERATED ALWAYS AS (
        setweight(to_tsvector('english', coalesce(content_text, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(snippet_name, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(file_name, '')), 'C')
    ) STORED,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ DEFAULT NOW() + INTERVAL '1 year',
    deleted_at TIMESTAMPTZ,              -- Soft delete
    
    -- Constraints
    CONSTRAINT check_content CHECK (
        (content_text IS NOT NULL) OR (file_url IS NOT NULL)
    ),
    CONSTRAINT check_file_size CHECK (
        file_size_bytes IS NULL OR file_size_bytes <= 10485760 -- 10MB
    ),
    CONSTRAINT check_snippet_name CHECK (
        (is_snippet = false) OR (is_snippet = true AND snippet_name IS NOT NULL)
    )
);

-- Índices para performance
CREATE INDEX idx_clipboard_user_created ON clipboard_items(user_id, created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_clipboard_user_type ON clipboard_items(user_id, content_type) WHERE deleted_at IS NULL;
CREATE INDEX idx_clipboard_user_favorite ON clipboard_items(user_id, is_favorite) WHERE is_favorite = true AND deleted_at IS NULL;
CREATE INDEX idx_clipboard_user_snippet ON clipboard_items(user_id, is_snippet) WHERE is_snippet = true AND deleted_at IS NULL;
CREATE INDEX idx_clipboard_expires ON clipboard_items(expires_at) WHERE expires_at IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX idx_clipboard_search ON clipboard_items USING gin(search_vector);
CREATE INDEX idx_clipboard_metadata ON clipboard_items USING gin(content_metadata);
```

#### Tabla: tags
```sql
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    color VARCHAR(7),                    -- Hex color (#FF5733)
    icon VARCHAR(50),                    -- Icon name (opcional)
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, name)
);

CREATE INDEX idx_tags_user ON tags(user_id);
```

#### Tabla: clipboard_item_tags (Many-to-Many)
```sql
CREATE TABLE clipboard_item_tags (
    clipboard_item_id UUID NOT NULL REFERENCES clipboard_items(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    PRIMARY KEY (clipboard_item_id, tag_id)
);

CREATE INDEX idx_item_tags_item ON clipboard_item_tags(clipboard_item_id);
CREATE INDEX idx_item_tags_tag ON clipboard_item_tags(tag_id);
```

#### Tabla: sessions
```sql
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id UUID REFERENCES devices(id) ON DELETE CASCADE,
    
    -- Token
    token_hash VARCHAR(255) UNIQUE NOT NULL, -- SHA256 del JWT
    refresh_token_hash VARCHAR(255) UNIQUE,
    
    -- Metadata
    ip_address INET,
    user_agent TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_used_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    
    -- Estado
    is_active BOOLEAN DEFAULT true,
    revoked_at TIMESTAMPTZ
);

CREATE INDEX idx_sessions_token ON sessions(token_hash) WHERE is_active = true;
CREATE INDEX idx_sessions_user ON sessions(user_id, is_active);
CREATE INDEX idx_sessions_expires ON sessions(expires_at) WHERE is_active = true;
```

#### Tabla: audit_logs
```sql
CREATE TABLE audit_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    device_id UUID REFERENCES devices(id) ON DELETE SET NULL,
    
    -- Acción
    action VARCHAR(100) NOT NULL, -- 'CREATE', 'UPDATE', 'DELETE', 'LOGIN', etc.
    resource_type VARCHAR(100),   -- 'clipboard_item', 'user', 'device'
    resource_id UUID,
    
    -- Metadata
    metadata JSONB DEFAULT '{}',
    changes JSONB,                -- Cambios realizados (before/after)
    
    -- Contexto
    ip_address INET,
    user_agent TEXT,
    
    -- Timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_audit_user_created ON audit_logs(user_id, created_at DESC);
CREATE INDEX idx_audit_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_action ON audit_logs(action, created_at DESC);
```

#### Tabla: usage_metrics (para billing y analytics)
```sql
CREATE TABLE usage_metrics (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Métricas
    metric_type VARCHAR(50) NOT NULL, -- 'api_calls', 'storage_used', 'items_created'
    metric_value BIGINT NOT NULL,
    
    -- Periodo
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    
    -- Metadata
    metadata JSONB DEFAULT '{}',
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_usage_user_period ON usage_metrics(user_id, period_start, period_end);
CREATE INDEX idx_usage_type ON usage_metrics(metric_type, period_start);
```

### Funciones y Triggers

#### Trigger: Actualizar updated_at automáticamente
```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_devices_updated_at BEFORE UPDATE ON devices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_clipboard_items_updated_at BEFORE UPDATE ON clipboard_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

#### Función: Limpiar items expirados
```sql
CREATE OR REPLACE FUNCTION cleanup_expired_items()
RETURNS void AS $$
BEGIN
    UPDATE clipboard_items
    SET deleted_at = NOW()
    WHERE expires_at < NOW()
      AND deleted_at IS NULL;
END;
$$ LANGUAGE plpgsql;
```

### Vistas útiles

#### Vista: Estadísticas por usuario
```sql
CREATE VIEW user_statistics AS
SELECT 
    u.id as user_id,
    u.email,
    u.subscription_tier,
    COUNT(ci.id) as total_items,
    COUNT(ci.id) FILTER (WHERE ci.is_favorite) as favorite_items,
    COUNT(ci.id) FILTER (WHERE ci.is_snippet) as snippet_items,
    SUM(ci.file_size_bytes) as total_storage_bytes,
    MAX(ci.created_at) as last_clipboard_activity,
    COUNT(DISTINCT d.id) as active_devices
FROM users u
LEFT JOIN clipboard_items ci ON u.id = ci.user_id AND ci.deleted_at IS NULL
LEFT JOIN devices d ON u.id = d.user_id AND d.is_active = true
WHERE u.deleted_at IS NULL
GROUP BY u.id, u.email, u.subscription_tier;
```

### SQLite Local (Cliente)

**Estructura para almacenamiento offline en Tauri**:

```sql
-- Local users table (simplified)
CREATE TABLE local_user (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL,
    full_name TEXT,
    subscription_tier TEXT
);

-- Local clipboard items (full offline support)
CREATE TABLE local_clipboard_items (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    content_type TEXT NOT NULL,
    content_text TEXT,
    content_metadata TEXT, -- JSON string
    file_url TEXT,
    file_name TEXT,
    file_size_bytes INTEGER,
    file_mime_type TEXT,
    is_favorite INTEGER DEFAULT 0,
    is_snippet INTEGER DEFAULT 0,
    snippet_name TEXT,
    
    -- Sync metadata
    synced INTEGER DEFAULT 0,
    sync_version INTEGER DEFAULT 1,
    server_updated_at TEXT,
    
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at TEXT
);

CREATE INDEX idx_local_clipboard_created ON local_clipboard_items(created_at DESC);
CREATE INDEX idx_local_clipboard_synced ON local_clipboard_items(synced);

-- Sync queue (para cuando hay conflictos o está offline)
CREATE TABLE sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- 'CREATE', 'UPDATE', 'DELETE'
    payload TEXT, -- JSON
    retries INTEGER DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

---

## Patrones de Diseño

### 1. Clean Architecture / Hexagonal Architecture

**Objetivo**: Separar la lógica de negocio de las dependencias externas (DB, APIs, frameworks).

**Capas**:
- **Domain Layer**: Entidades, Value Objects, Repository Interfaces (core puro)
- **Application Layer**: Casos de uso, Services, lógica de negocio
- **Infrastructure Layer**: Implementaciones concretas (PostgreSQL, Redis, S3)
- **Presentation Layer**: API routes, DTOs, validaciones

**Beneficios**:
- Testeable independientemente
- Cambio de tecnologías sin afectar lógica de negocio
- Código mantenible y escalable
- Dependencias apuntan hacia adentro (domain no depende de nada)

### 2. Repository Pattern

**Objetivo**: Abstraer el acceso a datos.

**Implementación**:
- Definir traits/interfaces en Domain Layer
- Implementaciones concretas en Infrastructure Layer
- Services usan las interfaces, no las implementaciones

**Estructura**:
```
domain/repositories/clipboard_repository.rs → trait ClipboardRepository
infrastructure/repositories/postgres_clipboard_repository.rs → impl ClipboardRepository
```

**Beneficio**: Cambiar DB (PostgreSQL → MongoDB) sin tocar lógica de negocio.

### 3. Service Layer Pattern

**Objetivo**: Encapsular lógica de negocio compleja.

**Servicios principales**:
- **AuthService**: Login, registro, refresh tokens, validaciones
- **ClipboardService**: CRUD de items, validaciones de límites, permisos
- **SearchService**: Búsqueda full-text, filtros, ranking
- **SyncService**: Sincronización entre dispositivos, conflict resolution
- **FileService**: Upload, download, thumbnail generation, validación
- **NotificationService**: Envío de notificaciones push/WebSocket

**Beneficio**: Lógica de negocio centralizada, reutilizable, testeable.

### 4. Factory Pattern

**Objetivo**: Crear instancias complejas de objetos.

**Uso en el proyecto**:
- Factory para crear ClipboardItem según content_type
- Factory para crear diferentes tipos de notificaciones
- Factory para crear clientes de storage (S3, MinIO, local)
- Factory para crear diferentes respuestas de error

### 5. Strategy Pattern

**Objetivo**: Algoritmos intercambiables en runtime.

**Uso**:
- **Storage Strategy**: S3, MinIO, filesystem local
- **Cache Strategy**: Redis, Memcached, in-memory
- **Authentication Strategy**: JWT, OAuth, API keys
- **Conflict Resolution Strategy**: Last-write-wins, merge, user-prompt

### 6. Observer Pattern / Pub-Sub

**Objetivo**: Notificar a múltiples observadores sobre eventos.

**Implementación**:
- Redis Pub/Sub para broadcasting de eventos
- WebSocket connections como observers
- Eventos domain: ItemCreated, ItemUpdated, ItemDeleted
- Todos los dispositivos del usuario reciben notificaciones

### 7. Unit of Work Pattern

**Objetivo**: Agrupar operaciones de base de datos en transacciones atómicas.

**Uso**:
- Crear item + asociar tags + actualizar métricas → 1 transacción
- Garantizar consistencia de datos
- Rollback automático si algo falla

### 8. CQRS (Command Query Responsibility Segregation) - Ligero

**Objetivo**: Separar operaciones de lectura (queries) y escritura (commands).

**Implementación**:
- **Commands**: CreateItem, UpdateItem, DeleteItem (mutan estado)
- **Queries**: GetItems, SearchItems, GetStatistics (solo lectura)
- Permite optimizar reads vs writes independientemente
- Diferentes modelos de datos para read vs write si es necesario

**Beneficio**: Escalabilidad (read replicas para queries), performance.

### 9. Middleware/Chain of Responsibility

**Objetivo**: Procesar requests a través de una cadena de handlers.

**Middlewares implementados**:
- **Authentication**: Validar JWT, extraer user_id
- **Rate Limiting**: Prevenir abuse
- **Logging**: Log de requests/responses
- **Error Handling**: Convertir errores a respuestas HTTP
- **Request Validation**: Validar DTOs
- **CORS**: Configurar CORS headers

**Orden típico**: CORS → Logging → Rate Limit → Auth → Validation → Handler

### 10. DTO (Data Transfer Object) Pattern

**Objetivo**: Transferir datos entre capas con validación explícita.

**DTOs principales**:
- **CreateClipboardItemDto**: Validar input para crear item
- **UpdateClipboardItemDto**: Validar update parcial
- **LoginDto**: Email + password con validación
- **RegisterDto**: Email, password, full_name
- **SearchQueryDto**: Query, filters, pagination

**Beneficio**: Validación centralizada, separación de concerns, type safety.

---

## API Endpoints

### Base URL
```
Desarrollo: http://localhost:8000/api/v1
Producción: https://api.clipboard-manager.com/api/v1
```

### Autenticación

#### POST /api/v1/auth/register
Registrar nuevo usuario.

**Request Body**:
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "full_name": "John Doe"
}
```

**Response** (201):
```json
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "full_name": "John Doe",
    "subscription_tier": "free"
  },
  "access_token": "jwt_token",
  "refresh_token": "refresh_token",
  "expires_in": 3600
}
```

#### POST /api/v1/auth/login
Iniciar sesión.

**Request Body**:
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "device_fingerprint": "unique_device_id"
}
```

**Response** (200): (mismo que register)

#### POST /api/v1/auth/refresh
Renovar access token.

**Request Body**:
```json
{
  "refresh_token": "refresh_token"
}
```

**Response** (200):
```json
{
  "access_token": "new_jwt_token",
  "expires_in": 3600
}
```

#### POST /api/v1/auth/logout
Cerrar sesión (revocar token).

**Headers**: `Authorization: Bearer <token>`

**Response** (204): No content

#### GET /api/v1/auth/me
Obtener información del usuario actual.

**Headers**: `Authorization: Bearer <token>`

**Response** (200):
```json
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "full_name": "John Doe",
    "subscription_tier": "pro",
    "subscription_expires_at": "2025-12-31T23:59:59Z",
    "created_at": "2025-01-01T00:00:00Z"
  }
}
```

### Clipboard Items

#### GET /api/v1/clipboard/items
Listar items del usuario con paginación.

**Query Parameters**:
- `limit` (default: 50, max: 100)
- `offset` (default: 0)
- `content_type` (opcional): text, image, file, code, etc.
- `is_favorite` (opcional): true/false
- `is_snippet` (opcional): true/false
- `created_after` (opcional): ISO 8601 timestamp
- `created_before` (opcional): ISO 8601 timestamp

**Headers**: `Authorization: Bearer <token>`

**Response** (200):
```json
{
  "items": [
    {
      "id": "uuid",
      "content_type": "text",
      "content_text": "Hello World",
      "is_favorite": false,
      "is_snippet": false,
      "created_at": "2025-01-01T12:00:00Z",
      "updated_at": "2025-01-01T12:00:00Z"
    }
  ],
  "total": 1234,
  "limit": 50,
  "offset": 0,
  "has_more": true
}
```

#### POST /api/v1/clipboard/items
Crear nuevo item.

**Headers**: 
- `Authorization: Bearer <token>`
- `Content-Type: application/json`

**Request Body**:
```json
{
  "content_type": "text",
  "content_text": "Hello World",
  "content_metadata": {},
  "device_id": "uuid"
}
```

**Response** (201):
```json
{
  "item": {
    "id": "uuid",
    "content_type": "text",
    "content_text": "Hello World",
    "created_at": "2025-01-01T12:00:00Z",
    "expires_at": "2026-01-01T12:00:00Z"
  }
}
```

#### GET /api/v1/clipboard/items/:id
Obtener item específico.

**Headers**: `Authorization: Bearer <token>`

**Response** (200): (objeto item completo)

#### PATCH /api/v1/clipboard/items/:id
Actualizar item (parcial).

**Headers**: `Authorization: Bearer <token>`

**Request Body** (todos los campos opcionales):
```json
{
  "content_text": "Updated text",
  "is_favorite": true,
  "content_metadata": {"updated": true}
}
```

**Response** (200): (item actualizado)

#### DELETE /api/v1/clipboard/items/:id
Eliminar item (soft delete).

**Headers**: `Authorization: Bearer <token>`

**Response** (204): No content

#### POST /api/v1/clipboard/items/:id/favorite
Marcar item como favorito.

**Headers**: `Authorization: Bearer <token>`

**Response** (200): (item actualizado con is_favorite: true)

#### DELETE /api/v1/clipboard/items/:id/favorite
Quitar de favoritos.

**Headers**: `Authorization: Bearer <token>`

**Response** (200): (item actualizado con is_favorite: false)

### Búsqueda

#### GET /api/v1/clipboard/search
Búsqueda full-text en clipboard items.

**Query Parameters**:
- `q` (required): texto de búsqueda
- `content_type` (opcional): filtrar por tipo
- `limit` (default: 20, max: 100)
- `offset` (default: 0)

**Headers**: `Authorization: Bearer <token>`

**Response** (200):
```json
{
  "results": [
    {
      "id": "uuid",
      "content_type": "code",
      "content_text": "const greeting = 'Hello TypeScript';",
      "content_metadata": {"language": "typescript"},
      "created_at": "2025-01-01T12:00:00Z",
      "relevance": 0.95
    }
  ],
  "total": 45,
  "query": "typescript",
  "limit": 20,
  "offset": 0
}
```

### Snippets

#### GET /api/v1/snippets
Listar snippets del usuario.

**Query Parameters**: (similar a /clipboard/items)

**Headers**: `Authorization: Bearer <token>`

#### POST /api/v1/snippets
Crear snippet (básicamente un clipboard item con is_snippet=true).

**Request Body**:
```json
{
  "content_type": "code",
  "content_text": "console.log('Hello');",
  "content_metadata": {"language": "javascript"},
  "snippet_name": "Console Log",
  "snippet_description": "Simple console.log"
}
```

**Response** (201): (snippet creado)

#### PATCH /api/v1/snippets/:id
Actualizar snippet.

#### DELETE /api/v1/snippets/:id
Eliminar snippet.

### Archivos

#### POST /api/v1/clipboard/upload
Subir archivo o imagen.

**Headers**: 
- `Authorization: Bearer <token>`
- `Content-Type: multipart/form-data`

**Form Data**:
- `file`: archivo (max 10MB)
- `device_id`: uuid
- `content_metadata` (opcional): JSON string

**Response** (201):
```json
{
  "item": {
    "id": "uuid",
    "content_type": "image",
    "file_url": "https://cdn.clipboard.com/files/uuid.jpg",
    "thumbnail_url": "https://cdn.clipboard.com/thumbs/uuid.jpg",
    "file_name": "screenshot.jpg",
    "file_size_bytes": 123456,
    "file_mime_type": "image/jpeg",
    "created_at": "2025-01-01T12:00:00Z"
  }
}
```

#### GET /api/v1/clipboard/files/:id/download
Descargar archivo.

**Headers**: `Authorization: Bearer <token>`

**Response**: Redirect 302 a S3 signed URL o stream directo del archivo.

### Dispositivos

#### GET /api/v1/devices
Listar dispositivos del usuario.

**Headers**: `Authorization: Bearer <token>`

**Response** (200):
```json
{
  "devices": [
    {
      "id": "uuid",
      "device_name": "MacBook Pro",
      "device_type": "macos",
      "is_active": true,
      "last_active_at": "2025-01-01T12:00:00Z",
      "created_at": "2024-12-01T10:00:00Z"
    }
  ]
}
```

#### POST /api/v1/devices/register
Registrar nuevo dispositivo.

**Headers**: `Authorization: Bearer <token>`

**Request Body**:
```json
{
  "device_name": "iPhone 15",
  "device_type": "ios",
  "device_fingerprint": "unique_device_id",
  "device_os_version": "17.0",
  "app_version": "1.0.0"
}
```

**Response** (201): (device creado)

#### DELETE /api/v1/devices/:id
Desactivar dispositivo.

**Headers**: `Authorization: Bearer <token>`

**Response** (204): No content

### WebSocket

#### WS /api/v1/ws
Conexión WebSocket para sincronización en tiempo real.

**Connection**: `wss://api.clipboard-manager.com/api/v1/ws?token=<jwt_token>`

**Mensajes Client → Server**:
```json
{
  "type": "subscribe",
  "device_id": "uuid"
}

{
  "type": "ping"
}
```

**Mensajes Server → Client**:
```json
{
  "type": "new_item",
  "item": { /* item completo */ }
}

{
  "type": "item_updated",
  "item": { /* item actualizado */ }
}

{
  "type": "item_deleted",
  "item_id": "uuid"
}

{
  "type": "pong"
}
```

### Health Checks

#### GET /health
Health check básico.

**Response** (200):
```json
{
  "status": "healthy",
  "timestamp": "2025-01-01T12:00:00Z",
  "version": "1.0.0"
}
```

#### GET /health/ready
Readiness check (verifica dependencias).

**Response** (200):
```json
{
  "status": "ready",
  "checks": {
    "database": "ok",
    "redis": "ok",
    "s3": "ok"
  }
}
```

#### GET /health/live
Liveness check (solo verifica que el proceso está vivo).

**Response** (200):
```json
{
  "status": "alive"
}
```

### Códigos de Error Estándar

Todos los errores siguen este formato:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "details": {
      "field": "email",
      "reason": "must be a valid email address"
    }
  }
}
```

**Códigos comunes**:
- `VALIDATION_ERROR` (400) - Input inválido
- `UNAUTHORIZED` (401) - Token inválido o expirado
- `FORBIDDEN` (403) - No tiene permisos
- `NOT_FOUND` (404) - Recurso no encontrado
- `CONFLICT` (409) - Conflicto (ej: email ya existe)
- `RATE_LIMIT_EXCEEDED` (429) - Demasiados requests
- `INTERNAL_SERVER_ERROR` (500) - Error del servidor
- `SERVICE_UNAVAILABLE` (503) - Servicio no disponible

---

## Sistema de Tiempo Real

### Arquitectura WebSocket + Redis Pub/Sub

**Componentes**:
1. **WebSocket Manager**: Gestiona todas las conexiones WebSocket activas
2. **Redis Pub/Sub**: Canal de comunicación entre múltiples instancias del servidor
3. **Message Broker**: Rutea mensajes a los dispositivos correctos del usuario

### Flujo Detallado de Sincronización

**1. Usuario copia algo en Dispositivo A**:
- Clipboard listener detecta cambio
- Guarda en SQLite local inmediatamente
- POST /clipboard/items al backend
- Marca como "pendiente sync" en SQLite

**2. Backend procesa**:
- Valida item y límites del usuario
- Guarda en PostgreSQL
- Invalida caché Redis relacionada
- Publica evento a Redis channel: `clipboard:{user_id}`

**3. Redis Pub/Sub broadcast**:
- Todas las instancias del API server están suscritas
- Cada instancia recibe el mensaje
- Cada instancia verifica si tiene WebSocket connections del user_id

**4. Dispositivos B, C, D reciben**:
- WebSocket Manager envía mensaje `new_item` a todos los dispositivos conectados
- Dispositivos guardan en SQLite local
- Actualizan UI
- Muestran notificación (opcional)

### Canales Redis

```
clipboard:{user_id}              - Eventos de clipboard del usuario
clipboard:{user_id}:devices      - Cambios en dispositivos (registro, desactivación)
notifications:{user_id}          - Notificaciones generales
system:broadcast                 - Mensajes broadcast a todos los usuarios
```

### Reconnection Strategy (Cliente)

**Exponential Backoff**:
- Intento 1: 1 segundo
- Intento 2: 2 segundos
- Intento 3: 4 segundos
- Intento 4: 8 segundos
- Intento 5: 16 segundos
- Máximo: 60 segundos

**Al reconectar**:
- Enviar último `sync_version` conocido
- Servidor responde con todos los items creados/actualizados desde ese version
- Cliente actualiza SQLite local

### Heartbeat (Keepalive)

**Cliente**:
- Envía `ping` cada 30 segundos
- Si no recibe `pong` en 60 segundos, asume desconexión
- Inicia proceso de reconnection

**Servidor**:
- Responde con `pong` a cada `ping`
- Si no recibe `ping` en 90 segundos, cierra conexión
- Limpia recursos del WebSocket

### Manejo de Conflictos

**Estrategia: Last-Write-Wins** (por ahora):
- Item con `updated_at` más reciente gana
- Simple pero efectivo para clipboard (no colaborativo)

**Futuro** (si hay edición de snippets):
- Operational Transformation (OT)
- CRDTs (Conflict-free Replicated Data Types)
- User prompt para resolver manualmente

### Escalabilidad WebSocket

**Problema**: Múltiples instancias del servidor, ¿cómo saber en cuál está conectado el usuario?

**Solución**: Redis Pub/Sub
- Todas las instancias suscritas a los canales
- Cuando se publica un evento, TODAS las instancias lo reciben
- Cada instancia verifica si tiene al usuario conectado
- Solo la instancia correcta envía el mensaje WebSocket

**No necesita**:
- Sticky sessions
- Session affinity
- Conocer en qué servidor está el usuario

### Message Format

**Estructura estándar**:
```json
{
  "type": "new_item|item_updated|item_deleted|pong|error",
  "timestamp": "2025-01-01T12:00:00Z",
  "data": { /* payload específico */ }
}
```

---

## Estrategia de Caché

### Redis Cache Strategy

#### 1. Cache de Items Recientes
```
Key: clipboard:user:{user_id}:items:recent
Type: List (LPUSH/LRANGE)
TTL: 5 minutos
Content: IDs de los últimos 50 items
```

**Uso**: Primera carga de la app, lista principal.

**Invalidación**: Al crear/eliminar item.

#### 2. Cache de Items Individuales
```
Key: clipboard:item:{item_id}
Type: Hash (HSET/HGET)
TTL: 1 hora
Content: Todos los campos del item serializado
```

**Uso**: Detalle de un item, evitar query a PostgreSQL.

**Invalidación**: Al actualizar/eliminar item.

#### 3. Cache de Favoritos
```
Key: clipboard:user:{user_id}:favorites
Type: List
TTL: 15 minutos
Content: IDs de items favoritos
```

**Invalidación**: Al marcar/desmarcar favorito.

#### 4. Cache de Snippets
```
Key: clipboard:user:{user_id}:snippets
Type: List
TTL: 30 minutos
Content: IDs de snippets
```

**Invalidación**: Al crear/actualizar/eliminar snippet.

#### 5. Cache de Búsquedas
```
Key: clipboard:search:{user_id}:{query_hash}
Type: String (JSON serializado)
TTL: 10 minutos
Content: Resultados de búsqueda
```

**Uso**: Cachear búsquedas frecuentes (ej: usuario busca "typescript" varias veces).

**query_hash**: SHA256 de (query + filters + sort)

#### 6. Session Cache
```
Key: session:{token_hash}
Type: Hash
TTL: Igual al expires_at del token
Fields: user_id, device_id, expires_at, subscription_tier
```

**Uso**: Validación rápida de tokens sin consultar PostgreSQL.

**Invalidación**: Al logout, o expiración natural.

#### 7. Rate Limiting
```
Key: rate_limit:{user_id}:{endpoint}
Type: String (contador)
TTL: 1 minuto o 1 hora (según endpoint)
Content: Número de requests
```

**Comandos**:
- `INCR rate_limit:{user_id}:clipboard_items`
- `EXPIRE rate_limit:{user_id}:clipboard_items 60`
- Si count > límite → HTTP 429

#### 8. User Statistics
```
Key: stats:user:{user_id}
Type: Hash
TTL: 1 hora
Fields: total_items, storage_used, favorite_count, snippet_count
```

**Uso**: Dashboard, mostrar stats sin query pesada.

### Cache Invalidation Patterns

**Write-Through**:
1. Escribir a PostgreSQL primero
2. Actualizar/invalidar caché después
3. Garantiza consistencia (DB es source of truth)

**Invalidación específica**:

**Al crear item**:
- Invalidar `clipboard:user:{user_id}:items:recent`
- Invalidar `stats:user:{user_id}`

**Al actualizar item**:
- Invalidar `clipboard:item:{item_id}`
- Si cambió favorito: invalidar `clipboard:user:{user_id}:favorites`
- Invalidar `clipboard:user:{user_id}:items:recent` (por si el orden cambió)

**Al eliminar item**:
- Eliminar `clipboard:item:{item_id}`
- Invalidar todas las listas relacionadas

**Al buscar**:
- Verificar si existe `clipboard:search:{user_id}:{query_hash}`
- Si existe: retornar cached
- Si no: query PostgreSQL, guardar en caché

### Cache Warming

**Al inicio del servidor**:
- Pre-cachear usuarios activos (últimas 24h)
- Pre-cachear items recientes de usuarios premium

**Background job** (cada 5 minutos):
- Refrescar caché de usuarios más activos
- Limpiar cachés expiradas manualmente (SCAN + DEL)

### Monitoreo de Cache

**Métricas importantes**:
- Cache hit rate (objetivo: >80%)
- Cache miss rate
- Eviction rate (si Redis se queda sin memoria)
- Latency de comandos Redis

**Alertas**:
- Cache hit rate < 70% por 10 minutos
- Redis memory usage > 80%
- Redis latency > 10ms p99

---

## Almacenamiento de Archivos

### Estrategia de Storage

#### Desarrollo Local
- **MinIO**: S3-compatible, corre en Docker
- Bucket: `clipboard-dev`
- Endpoint: `http://localhost:9000`

#### Producción
- **AWS S3**: Escalable, confiable, económico
- **CloudFront**: CDN para distribución rápida global
- Buckets separados: `clipboard-prod-files`, `clipboard-prod-thumbnails`

### Estructura de Buckets

```
clipboard-{env}/
├── images/
│   └── {user_id}/
│       └── {year}/
│           └── {month}/
│               ├── {item_id}.jpg
│               └── {item_id}_thumb.jpg
│
├── files/
│   └── {user_id}/
│       └── {year}/
│           └── {month}/
│               └── {item_id}-{sanitized_filename}.pdf
│
└── temp/
    └── uploads/
        └── {upload_id} (se elimina tras 24h)
```

**Ejemplo**:
```
clipboard-prod/images/550e8400-e29b-41d4-a716-446655440000/2025/01/abc123.jpg
clipboard-prod/images/550e8400-e29b-41d4-a716-446655440000/2025/01/abc123_thumb.jpg
```

### Proceso de Upload

**1. Cliente solicita upload**:
- POST /clipboard/upload con `multipart/form-data`
- Form data: `file`, `device_id`, `content_metadata` (opcional)

**2. Servidor valida**:
- Tamaño < 10MB
- MIME type permitido (imágenes: jpg, png, gif, webp; archivos: pdf, zip, etc.)
- Usuario no excede límite de storage (plan pro: 5GB)
- Verificar magic numbers (no solo extensión)

**3. Servidor procesa**:
- Generar UUID para item_id
- Sanitizar filename
- Subir a S3: `images/{user_id}/{year}/{month}/{item_id}.jpg`
- Si es imagen: generar thumbnail (200x200px, JPEG quality 80)
- Subir thumbnail: `images/{user_id}/{year}/{month}/{item_id}_thumb.jpg`
- Guardar metadata en PostgreSQL (file_url, thumbnail_url, file_size_bytes)
- Broadcast via WebSocket

**4. Servidor responde**:
- Retornar item con URLs públicas (via CloudFront)
- Cliente guarda en SQLite local

### Signed URLs vs Public URLs

**Desarrollo** (MinIO):
- URLs públicas temporales OK

**Producción** (S3 + CloudFront):
- **Archivos privados**: Generar signed URLs (expiración 1 hora)
- **Thumbnails**: Públicos con CloudFront caching (más rápido)

**Generar Signed URL**:
```rust
use aws_sdk_s3::presigning::PresigningConfig;

let expires_in = Duration::from_secs(3600); // 1 hora
let presigned_request = s3_client
    .get_object()
    .bucket(&bucket)
    .key(&key)
    .presigned(PresigningConfig::expires_in(expires_in)?)
    .await?;

let signed_url = presigned_request.uri();
```

### Optimización de Imágenes

**Al subir**:
- **Original**: Guardar as-is (hasta 10MB)
- **Thumbnail**: 200x200px, JPEG quality 80, ~10-20KB
- **WebP version** (futuro): Mejor compresión, soporte moderno

**Herramientas**:
- Rust: `image` crate para resize/convert
- Worker separado (thumbnail-worker) para no bloquear upload

### Limpieza de Archivos Huérfanos

**Background Job** (cleanup-worker, cada 24 horas):
1. Buscar archivos en S3 sin registro en PostgreSQL
2. Mover a bucket de cuarentena: `clipboard-quarantine`
3. Después de 7 días en cuarentena: eliminar permanentemente
4. Log de todo para auditoría

### Lifecycle Policies (S3)

**Configurar en Terraform**:
```hcl
resource "aws_s3_bucket_lifecycle_configuration" "clipboard" {
  bucket = aws_s3_bucket.clipboard.id

  rule {
    id     = "delete-temp-uploads"
    status = "Enabled"

    filter {
      prefix = "temp/uploads/"
    }

    expiration {
      days = 1
    }
  }

  rule {
    id     = "archive-old-items"
    status = "Enabled"

    filter {
      prefix = "files/"
    }

    transition {
      days          = 90
      storage_class = "INTELLIGENT_TIERING"
    }
  }
}
```

### CDN (CloudFront)

**Configuración**:
- Origin: S3 bucket
- Cache behavior: Public para thumbnails, Signed URLs para archivos
- Cache-Control: `public, max-age=31536000` (1 año para inmutables)
- Compression: Brotli + Gzip habilitado
- Edge locations: Global

**Invalidación**:
- Solo necesaria si se actualiza un archivo con el mismo nombre (poco común)
- Mejor práctica: usar UUIDs en nombres → nunca invalidar

---

## Autenticación y Autorización

### Estrategia de Autenticación

**JWT (JSON Web Tokens)** con dos tipos de tokens:
- **Access Token**: Corta duración (1 hora), usado en cada request
- **Refresh Token**: Larga duración (30 días), usado para renovar access token

### Estructura del JWT

**Access Token**:
```json
{
  "sub": "user_id",              // Subject (user)
  "email": "user@example.com",
  "device_id": "device_uuid",
  "subscription_tier": "pro",
  "iat": 1704067200,             // Issued at
  "exp": 1704070800,             // Expires at (1 hora después)
  "type": "access"
}
```

**Refresh Token**:
```json
{
  "sub": "user_id",
  "session_id": "session_uuid",  // Referencia a sessions table
  "iat": 1704067200,
  "exp": 1706659200,             // Expires at (30 días después)
  "type": "refresh"
}
```

**Firma**: HS256 (HMAC with SHA-256) con secret de 256+ bits

### Flujo de Autenticación Completo

**1. Register/Login**:
- Usuario envía email + password
- Servidor valida credenciales (bcrypt hash verification)
- Crear registro en `sessions` table
- Generar access token + refresh token
- Cachear session en Redis: `session:{token_hash}`
- Retornar ambos tokens al cliente

**2. Request con Access Token**:
- Cliente incluye: `Authorization: Bearer <access_token>`
- Middleware valida:
  - Verificar firma JWT
  - Verificar expiración
  - Extraer `user_id` y `device_id`
- Opcional: verificar en Redis cache si session activa (más rápido)
- Si válido: procesar request
- Si inválido/expirado: HTTP 401

**3. Refresh Token Flow**:
- Access token expiró
- Cliente envía: `POST /auth/refresh` con `refresh_token`
- Servidor valida:
  - Verificar firma JWT
  - Verificar expiración
  - Verificar session en DB no está revocada
- Generar nuevo access token (mismo user_id, nuevo exp)
- Opcional: rotar refresh token (generar nuevo refresh token)
- Retornar nuevo access token

**4. Logout**:
- Cliente envía: `POST /auth/logout` con access token
- Servidor:
  - Extraer `session_id` del token
  - Marcar session como revoked en DB: `UPDATE sessions SET revoked_at = NOW()`
  - Eliminar de caché Redis
- Cliente elimina tokens localmente

**5. Logout de todos los dispositivos**:
- Revocar todas las sessions del user_id
- Útil si usuario pierde un dispositivo

### Autorización (Verificación de Permisos)

**Ownership Verification**:
- En cada endpoint que accede a recursos (clipboard items, devices):
  - Extraer `user_id` del JWT
  - Verificar que el recurso pertenece al `user_id`
  - Si no: HTTP 403 Forbidden

**Subscription Tier Verification**:
- Verificar límites según tier:
  - Free: no puede subir archivos, no sincronización
  - Pro: 5000 items, 5GB storage, sincronización ilimitada

### Seguridad Adicional

#### 1. Rate Limiting por Endpoint

**Login**:
- 5 intentos por 15 minutos por IP
- 10 intentos por 1 hora por email

**Refresh Token**:
- 10 requests por minuto por usuario

**API Endpoints**:
- 100 requests por minuto por usuario
- 1000 requests por hora por usuario

**Implementación**: Redis con `INCR` + `EXPIRE`

#### 2. Dispositivos de Confianza

**Al login desde nuevo dispositivo**:
- Registrar device_fingerprint
- Enviar email de notificación: "Nuevo login desde iPhone 15 en San Francisco"
- Permitir revocar sesión desde email

**Dashboard de Dispositivos**:
- Listar todos los dispositivos activos
- Permitir desactivar dispositivos remotamente

#### 3. Password Security

**Hashing**: bcrypt con cost factor 12
- Cost 12 = ~250ms de hashing (balance seguridad/performance)
- Rainbow tables inútiles
- Resistente a brute force

**Validación de Complejidad**:
- Mínimo 8 caracteres
- Al menos 1 mayúscula
- Al menos 1 minúscula
- Al menos 1 número
- Opcional: 1 símbolo especial

**Prevención de Passwords Comunes**:
- Verificar contra lista de passwords comprometidos (ej: Have I Been Pwned API)
- Rechazar "password123", "123456", "qwerty", etc.

#### 4. 2FA (Futuro - Fase 4)

**TOTP** (Time-based One-Time Password):
- Google Authenticator, Authy compatible
- Código de 6 dígitos que cambia cada 30 segundos
- Guardar secret en DB encriptado

**Backup Codes**:
- 10 códigos de un solo uso
- Por si pierde acceso al authenticator

**Flujo**:
- Login normal → si tiene 2FA habilitado → solicitar código → validar → generar tokens

---

## Testing

### Estrategia de Testing

#### 1. Unit Tests

**Objetivo**: Probar funciones y módulos aislados sin dependencias externas.

**Coverage Target**: >80% (crítico: >95%)

**Herramientas**:
- **Rust**: `cargo test` + `cargo-tarpaulin` (coverage)
- **Frontend**: Vitest / Jest

**Áreas a testear**:
- Funciones de utilidad (validación, parsing, formateo)
- Value objects (Email validation, ContentType parsing)
- Lógica de negocio en Services (sin DB, usando mocks)
- DTOs y validaciones
- Helpers y conversiones

**Ejemplo**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(Email::new("user@example.com").is_ok());
        assert!(Email::new("invalid").is_err());
    }

    #[test]
    fn test_content_type_parsing() {
        assert_eq!(ContentType::from_str("text").unwrap(), ContentType::Text);
    }
}
```

#### 2. Integration Tests

**Objetivo**: Probar interacción entre componentes (Service + Repository + DB).

**Herramientas**:
- **Rust**: `#[tokio::test]` con bases de datos reales
- **Testcontainers**: PostgreSQL y Redis efímeros en Docker
- **sqlx::test**: Pool de base de datos de prueba

**Áreas a testear**:
- Repository implementations (CRUD completo)
- API endpoints (sin red real, test de Axum router)
- WebSocket connections
- Background jobs
- Cache invalidation
- File upload/download

**Ejemplo**:
```rust
#[sqlx::test]
async fn test_create_clipboard_item(pool: PgPool) {
    let repo = PostgresClipboardRepository::new(pool);
    let item = ClipboardItem {
        content_type: ContentType::Text,
        content_text: Some("Hello".to_string()),
        ..Default::default()
    };
    
    let result = repo.create(item).await;
    assert!(result.is_ok());
}
```

#### 3. E2E Tests (End-to-End)

**Objetivo**: Probar flujos completos desde perspectiva del usuario.

**Herramientas**:
- **Frontend**: Playwright / Cypress
- **Backend**: Tests de API completa con servidor real
- **Docker Compose**: Ambiente completo (API + PostgreSQL + Redis + MinIO)

**Flujos a testear**:
- Registro → Login → Crear item → Ver en otro dispositivo (simular)
- Upload de imagen → Generar thumbnail → Ver thumbnail
- Búsqueda → Ver resultados correctos
- Marcar favorito → Listar favoritos
- Eliminar item → Verificar soft delete y sincronización

**Ejemplo Playwright**:
```typescript
test('create and sync clipboard item', async ({ page }) => {
  await page.goto('http://localhost:3000');
  await page.fill('[data-testid="email"]', 'test@example.com');
  await page.fill('[data-testid="password"]', 'password123');
  await page.click('[data-testid="login"]');
  
  // Esperar a que cargue la lista
  await page.waitForSelector('[data-testid="clipboard-list"]');
  
  // Crear item
  await page.fill('[data-testid="new-item-input"]', 'Test clipboard item');
  await page.press('[data-testid="new-item-input"]', 'Enter');
  
  // Verificar que aparece en la lista
  await expect(page.locator('text=Test clipboard item')).toBeVisible();
});
```

#### 4. Load/Performance Tests

**Objetivo**: Validar performance bajo carga, encontrar bottlenecks.

**Herramientas**:
- **k6** (preferido, scripts en JavaScript)
- **Apache JMeter** (alternativa, GUI)
- **Grafana** para visualización de métricas

**Escenarios**:

**Scenario 1: Normal Load**
- 100 usuarios virtuales (VUs)
- Duración: 10 minutos
- Mix de operaciones: 60% reads, 30% writes, 10% search

**Scenario 2: Peak Load**
- 1000 usuarios simultáneos
- Duración: 5 minutos
- Verificar que no hay degradación

**Scenario 3: WebSocket Stress**
- 500 WebSocket connections simultáneas
- 10 mensajes/segundo broadcast
- Verificar latencia < 100ms p95

**Scenario 4: File Upload**
- 50 usuarios subiendo archivos (5MB avg)
- Verificar throughput y success rate

**Métricas objetivo**:
- **P50 latency**: <50ms (API REST)
- **P95 latency**: <200ms
- **P99 latency**: <500ms
- **WebSocket delivery**: <100ms
- **Error rate**: <0.1%
- **Throughput**: >1000 req/s

**Script k6 ejemplo**:
```javascript
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '2m', target: 100 },
    { duration: '5m', target: 100 },
    { duration: '2m', target: 0 },
  ],
};

export default function () {
  let res = http.get('http://api.localhost/api/v1/clipboard/items', {
    headers: { 'Authorization': 'Bearer TOKEN' },
  });
  
  check(res, {
    'status is 200': (r) => r.status === 200,
    'response time < 200ms': (r) => r.timings.duration < 200,
  });
  
  sleep(1);
}
```

#### 5. Security Tests

**Objetivo**: Encontrar vulnerabilidades de seguridad.

**Herramientas**:
- **OWASP ZAP** (automated security scanning)
- **sqlmap** (SQL injection testing)
- **cargo audit** (Rust dependency vulnerabilities)
- **npm audit** (frontend dependencies)

**Tests**:
- **SQL Injection**: Intentar inyectar SQL en búsqueda, filters
- **XSS**: Intentar inyectar scripts en content_text
- **CSRF**: Verificar que tokens CSRF funcionan
- **JWT Manipulation**: Intentar modificar JWT, cambiar user_id
- **Rate Limiting**: Verificar que rate limits funcionan
- **Authorization**: Intentar acceder a recursos de otros usuarios
- **File Upload**: Intentar subir archivos maliciosos (.exe, scripts)

### Test Data Management

**Fixtures**:
- Usuarios de prueba con diferentes tiers (free, pro)
- Items de ejemplo de cada tipo (text, image, code, etc.)
- Dispositivos simulados (Windows, macOS, iOS, Android)

**Database Seeding**:
- Script SQL: `scripts/seed-db.sql`
- Crea usuarios de prueba, items de ejemplo
- Ejecutar con: `psql -U postgres -d clipboard_test < scripts/seed-db.sql`

**Factories** (para generar datos random en tests):
```rust
impl ClipboardItem {
    pub fn factory() -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            content_type: ContentType::Text,
            content_text: Some(faker::lorem::sentence()),
            ..Default::default()
        }
    }
}
```

### CI/CD Testing Pipeline

**En cada Pull Request** (`.github/workflows/ci.yml`):
1. **Lint**: `cargo clippy`, `eslint`
2. **Format**: `cargo fmt --check`, `prettier --check`
3. **Unit Tests**: `cargo test`, `bun test`
4. **Integration Tests**: con Testcontainers
5. **Security Scan**: `cargo audit`, `npm audit`
6. **Build**: Verificar que compila

**En merge a main** (`.github/workflows/cd-staging.yml`):
1. Todos los tests de PR
2. **E2E Tests**: Playwright en staging
3. **Build Docker images**
4. **Deploy to staging**
5. **Smoke tests** en staging

**Antes de release** (manual):
1. **Load Tests**: k6 en staging
2. **Security Audit**: OWASP ZAP
3. **Manual QA**: Probar flujos principales
4. **Tag version**: `git tag v1.0.0`

**Nightly** (`.github/workflows/nightly.yml`):
1. **Dependency Updates**: Dependabot o Renovate
2. **Extended Tests**: Tests que toman mucho tiempo
3. **Performance Regression**: Comparar con baseline
4. **Backup Verification**: Verificar que backups funcionan

### Code Coverage

**Herramientas**:
- **Rust**: `cargo-tarpaulin` o `cargo-llvm-cov`
- **Frontend**: Istanbul (incluido en Jest/Vitest)
- **Reportes**: Codecov.io o Coveralls.io

**Targets**:
- **Overall**: >80%
- **Critical paths** (auth, sync, clipboard CRUD): >95%
- **Utilities y helpers**: >90%

**Comando**:
```bash
cargo tarpaulin --out Html --output-dir coverage
```

**Integración con CI**:
- Upload coverage a Codecov en cada PR
- Bloquear merge si coverage baja significativamente

---

## Monitoreo y Observabilidad

### Tres Pilares de Observabilidad

#### 1. Logs (Logging)

**Objetivo**: Registrar eventos y errores para debugging y auditoría.

**Herramientas**:
- **Rust**: `tracing` crate (structured logging)
- **Agregación**: 
  - Desarrollo: stdout con `tracing-subscriber`
  - Staging/Producción: Grafana Loki o AWS CloudWatch Logs

**Niveles de Log**:
- **ERROR**: Errores críticos que requieren atención inmediata
- **WARN**: Situaciones anómalas que no detienen el servicio
- **INFO**: Eventos importantes (login, crear item, delete)
- **DEBUG**: Información detallada para debugging (no en producción)
- **TRACE**: Muy verboso, solo en desarrollo local

**Formato Estructurado** (JSON):
```json
{
  "timestamp": "2025-01-01T12:00:00.123Z",
  "level": "INFO",
  "target": "clipboard_api::routes::clipboard",
  "message": "Item created successfully",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "device_id": "abc123",
  "item_id": "def456",
  "request_id": "req_xyz789",
  "latency_ms": 45
}
```

**Request ID**:
- Generar UUID único por request
- Propagar a través de toda la cadena (services, repositories)
- Incluir en respuesta HTTP: header `X-Request-ID`
- Permite rastrear un request completo en logs

**Implementación**:
```rust
use tracing::{info, error, instrument};

#[instrument(skip(repo), fields(request_id = %request_id))]
async fn create_item(repo: &impl ClipboardRepository, item: ClipboardItem) -> Result<ClipboardItem> {
    info!(user_id = %item.user_id, "Creating clipboard item");
    
    match repo.create(item).await {
        Ok(created) => {
            info!(item_id = %created.id, "Item created successfully");
            Ok(created)
        }
        Err(e) => {
            error!(error = %e, "Failed to create item");
            Err(e)
        }
    }
}
```

#### 2. Metrics (Métricas)

**Objetivo**: Medir performance y behavior del sistema cuantitativamente.

**Herramientas**:
- **Rust**: `metrics` crate
- **Exportación**: Prometheus endpoint `/metrics`
- **Visualización**: Grafana dashboards
- **Alerting**: Prometheus Alertmanager

**Tipos de Métricas**:

**Counters** (solo incrementan):
- `http_requests_total{method="POST", endpoint="/clipboard/items", status="200"}`
- `clipboard_items_created_total`
- `websocket_messages_sent_total`
- `db_errors_total`

**Gauges** (pueden subir/bajar):
- `http_requests_in_flight`
- `websocket_connections_active`
- `db_connections_active`
- `clipboard_items_total_count`
- `storage_bytes_used`

**Histograms** (distribución de valores):
- `http_request_duration_seconds{endpoint="/clipboard/items", method="GET"}`
- `db_query_duration_seconds{operation="SELECT"}`
- `websocket_message_delivery_seconds`

**Métricas Clave del Sistema**:

**API**:
- `http_requests_total`: Total requests (por endpoint, method, status)
- `http_request_duration_seconds`: Latencia (P50, P95, P99)
- `http_requests_in_flight`: Requests concurrentes

**Base de Datos**:
- `db_connections_active`: Conexiones activas al pool
- `db_connections_idle`: Conexiones idle
- `db_query_duration_seconds`: Tiempo de queries
- `db_errors_total`: Errores de DB

**Redis**:
- `redis_commands_total`: Comandos ejecutados
- `redis_command_duration_seconds`: Latencia de comandos
- `cache_hits_total`: Cache hits
- `cache_misses_total`: Cache misses

**WebSocket**:
- `websocket_connections_active`: Conexiones activas
- `websocket_messages_sent_total`: Mensajes enviados
- `websocket_messages_received_total`: Mensajes recibidos
- `websocket_connection_duration_seconds`: Duración de conexiones

**Negocio**:
- `clipboard_items_created_total`: Items creados
- `clipboard_items_active`: Items activos totales
- `users_registered_total`: Usuarios registrados
- `users_active_daily`: DAU (Daily Active Users)
- `storage_bytes_used`: Storage usado total

**Sistema**:
- `process_cpu_usage_percent`: Uso de CPU
- `process_memory_bytes`: Memoria usada
- `process_open_file_descriptors`: File descriptors abiertos

**Implementación**:
```rust
use metrics::{counter, histogram, gauge};

// En handler
counter!("http_requests_total", "endpoint" => "/clipboard/items", "method" => "POST", "status" => "201").increment(1);

let start = Instant::now();
let result = service.create_item(item).await;
let duration = start.elapsed();
histogram!("http_request_duration_seconds", "endpoint" => "/clipboard/items").record(duration.as_secs_f64());

// En WebSocket manager
gauge!("websocket_connections_active").set(active_connections as f64);
```

#### 3. Traces (Tracing Distribuido)

**Objetivo**: Rastrear requests a través de múltiples servicios y capas.

**Herramientas**:
- **Rust**: `tracing` + `tracing-opentelemetry`
- **Backend**: Grafana Tempo o Jaeger
- **Estándar**: OpenTelemetry (OTEL)

**Beneficios**:
- Ver exactamente dónde se gasta el tiempo en un request
- Identificar bottlenecks (ej: query lenta en DB)
- Debugging de issues en producción

**Spans Importantes**:
- HTTP request completo (parent span)
- Service method calls
- Database queries
- Redis operations
- S3 uploads/downloads
- External API calls (email, payment)

**Ejemplo de Trace**:
```
[POST /clipboard/items] 245ms
├─ [AuthMiddleware::validate_token] 5ms
├─ [ClipboardService::create_item] 230ms
│  ├─ [validate_limits] 2ms
│  ├─ [PostgresRepo::create] 180ms
│  │  └─ [sqlx::query INSERT] 175ms
│  ├─ [Redis::invalidate_cache] 8ms
│  └─ [Redis::publish] 10ms
└─ [serialize_response] 10ms
```

**Implementación**:
```rust
use tracing::instrument;

#[instrument(skip(repo))]
async fn create_item(repo: &impl ClipboardRepository, item: ClipboardItem) -> Result<ClipboardItem> {
    // Automatically creates a span "create_item"
    repo.create(item).await
}
```

### Health Checks

**Endpoints**:

**GET /health** (básico):
- Solo verifica que el proceso está vivo
- Response: `{"status": "healthy"}`
- Uso: Smoke tests, uptime monitors

**GET /health/ready** (readiness):
- Verifica dependencias (DB, Redis, S3)
- Si alguna falla → HTTP 503
- Response: `{"status": "ready", "checks": {"database": "ok", "redis": "ok", "s3": "ok"}}`
- Uso: Kubernetes readiness probes, load balancer

**GET /health/live** (liveness):
- Solo verifica que el proceso responde
- Response: `{"status": "alive"}`
- Uso: Kubernetes liveness probes (restart si falla)

**Implementación**:
```rust
async fn health_ready(pool: PgPool, redis: RedisClient) -> Result<Json<HealthCheck>> {
    let db_ok = sqlx::query("SELECT 1").fetch_one(&pool).await.is_ok();
    let redis_ok = redis.ping().await.is_ok();
    
    if db_ok && redis_ok {
        Ok(Json(HealthCheck {
            status: "ready",
            checks: hashmap! {
                "database" => "ok",
                "redis" => "ok",
            }
        }))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
```

### Alerting (Alertas)

**Herramientas**:
- **Prometheus Alertmanager**: Gestiona y rutea alertas
- **PagerDuty / Opsgenie**: On-call, escalación
- **Slack**: Notificaciones a canal de equipo
- **Email**: Alertas críticas

**Alertas Críticas** (PagerDuty + Slack):
- API error rate > 1% por 5 minutos
- P99 latency > 1 segundo por 5 minutos
- Database connection pool exhausted
- Redis unavailable por 2 minutos
- S3 unavailable
- Disk usage > 90%
- Memory usage > 95%
- CPU usage > 90% sostenido por 10 minutos

**Alertas de Warning** (solo Slack):
- Cache hit rate < 70% por 15 minutos
- Slow queries detectadas (>1s)
- WebSocket reconnections frecuentes (>10/min)
- Background job failures (>5 en 1 hora)
- Disk usage > 80%

**Configuración Prometheus Alert**:
```yaml
groups:
  - name: api_alerts
    rules:
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.01
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }}% over the last 5 minutes"
      
      - alert: HighLatency
        expr: histogram_quantile(0.99, rate(http_request_duration_seconds_bucket[5m])) > 1
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High latency detected"
          description: "P99 latency is {{ $value }}s"
```

### Dashboards (Grafana)

**Dashboard 1: API Overview**
- Requests/segundo (línea temporal)
- Error rate (línea temporal + gauge)
- P50/P95/P99 latency (línea temporal)
- Endpoints más lentos (tabla)
- Status codes distribution (pie chart)
- Active users (gauge)
- Active WebSocket connections (gauge)

**Dashboard 2: Database Performance**
- Connection pool usage (gauge + línea)
- Query duration por tipo (histogram)
- Slow queries (tabla con query text)
- Table sizes (bar chart)
- Index usage statistics
- Locks y deadlocks

**Dashboard 3: Business Metrics**
- Items creados (hoy, esta semana, este mes)
- Active users (DAU, WAU, MAU)
- Storage usado (gauge + tendencia)
- User growth (línea temporal)
- Premium conversion rate (gauge)
- Top content types (pie chart)

**Dashboard 4: Infrastructure**
- CPU usage por servicio (línea temporal)
- Memory usage (gauge + línea)
- Disk I/O (línea temporal)
- Network I/O (línea temporal)
- Container restarts (contador)
- Pod status (si K8s)

### APM (Application Performance Monitoring)

**Opciones**:
- **Self-hosted**: Grafana stack (Loki + Tempo + Mimir + Grafana)
- **Managed**: DataDog APM (completo pero costoso)
- **Alternativa**: New Relic, Elastic APM

**Beneficios**:
- Ver flujo completo de requests end-to-end
- Identificar N+1 queries automáticamente
- Profiling de CPU y memoria
- Error tracking con stack traces completos
- Correlación de logs, metrics, traces

**Recomendación**: Empezar con Grafana stack (gratis, self-hosted), migrar a DataDog si el presupuesto lo permite.

### Error Tracking

**Herramientas**:
- **Sentry** (recomendado, gratis para startups)
- **Rollbar** (alternativa)
- **Bugsnag** (alternativa)

**Features**:
- Capturar panics en Rust automáticamente
- Capturar excepciones en frontend
- Agrupar errores similares
- Notificaciones de nuevos errores
- Release tracking (saber en qué versión apareció el error)

**Context en Errores**:
- `user_id`
- `device_id`
- `request_id`
- URL y método HTTP
- Headers relevantes (no sensibles)
- Stack trace completo
- Breadcrumbs (últimas acciones del usuario)

**Integración Rust**:
```rust
sentry::init(("DSN", sentry::ClientOptions {
    release: Some(env!("CARGO_PKG_VERSION").into()),
    environment: Some(std::env::var("ENVIRONMENT").unwrap().into()),
    ..Default::default()
}));

// Captura automática de panics
sentry::integrations::panic::register_panic_handler();
```

---

## Seguridad

### Principios de Seguridad

1. **Defense in Depth**: Múltiples capas de seguridad
2. **Least Privilege**: Permisos mínimos necesarios
3. **Secure by Default**: Configuración segura por defecto
4. **Fail Securely**: Si algo falla, fallar de manera segura
5. **Zero Trust**: No confiar en nada por defecto

### Seguridad en la Aplicación

#### 1. Autenticación y Sesiones

**Passwords**:
- **Hashing**: Bcrypt con cost factor 12+ (no MD5, no SHA1)
- **Validación**: Min 8 caracteres, mayúsculas, minúsculas, números
- **Prevención de passwords comunes**: Verificar contra lista (HaveIBeenPwned API)
- **No almacenar en texto plano**: NUNCA

**JWT**:
- **Algoritmo**: HS256 (HMAC-SHA256) o RS256 (RSA)
- **Secret**: 256+ bits, aleatorio, en Secrets Manager
- **Access token**: Corto (1 hora)
- **Refresh token**: Largo (30 días) pero rotable
- **No incluir info sensible**: Solo IDs, no passwords

**Session Management**:
- Revocar sesiones al cambiar password
- Límite de sesiones activas por usuario (ej: 10)
- Logout en todos los dispositivos (si es necesario)

#### 2. Autorización

**Verificar Ownership**:
- En CADA request que accede a recursos
- No confiar en `user_id` del cliente
- Extraer `user_id` del JWT validado
- Verificar: `item.user_id == authenticated_user_id`

**Validar Permisos por Tier**:
- Free: no upload archivos, no sincronización
- Pro: límites de storage, items, etc.
- Rechazar operaciones si excede límites

#### 3. Input Validation

**Validar TODO input del usuario**:
- Email: formato válido, longitud máxima
- Password: complejidad mínima
- content_text: longitud máxima (ej: 1MB)
- file_size: máximo 10MB
- content_type: whitelist de valores permitidos

**Sanitización**:
- HTML/Markdown: usar librería confiable (ej: `ammonia` en Rust)
- SQL: SIEMPRE usar prepared statements (nunca concatenar)
- Filesystem paths: sanitizar, prevenir path traversal

**Whitelist > Blacklist**:
- MIME types: whitelist de tipos permitidos
- Content types: enum cerrado
- Extensiones de archivos: whitelist

#### 4. SQL Injection

**Prevención**:
- **SIEMPRE** usar prepared statements / parameterized queries
- **NUNCA** concatenar strings en SQL
- sqlx en Rust ayuda (queries verificadas en compile time)

**Ejemplo SEGURO**:
```rust
sqlx::query_as!(
    ClipboardItem,
    r#"SELECT * FROM clipboard_items WHERE user_id = $1 AND content_text ILIKE $2"#,
    user_id,
    format!("%{}%", search_query)
)
.fetch_all(&pool)
.await
```

**Ejemplo INSEGURO** (NUNCA hacer):
```rust
// ❌ PELIGROSO
let query = format!("SELECT * FROM clipboard_items WHERE user_id = '{}'", user_id);
sqlx::query(&query).fetch_all(&pool).await
```

#### 5. XSS (Cross-Site Scripting)

**Prevención en Frontend**:
- **Escapar HTML** al renderizar `content_text`
- React/Vue/Svelte escapan por defecto (no usar `dangerouslySetInnerHTML` sin sanitizar)
- Para rich text: usar librería sanitizer (`DOMPurify` en JS)

**Content-Security-Policy headers**:
```
Content-Security-Policy: 
  default-src 'self'; 
  script-src 'self' 'unsafe-inline' https://cdn.example.com;
  style-src 'self' 'unsafe-inline';
  img-src 'self' https://cdn.clipboard.com data:;
  connect-src 'self' wss://api.clipboard.com;
```

#### 6. CSRF (Cross-Site Request Forgery)

**Prevención**:
- **CSRF tokens** en formularios (si hay)
- **SameSite cookies**: `SameSite=Lax` o `Strict`
- **Validar Origin/Referer headers** en requests mutables
- Para API con JWT: CSRF menos relevante (JWT en header, no cookie)

#### 7. Rate Limiting

**Por IP**:
- 100 requests/minuto para endpoints públicos
- 1000 requests/hora para requests no autenticados

**Por Usuario**:
- 100 requests/minuto para endpoints normales
- 10 requests/minuto para endpoints caros (search, upload)
- 5 login attempts cada 15 minutos

**Progresivo**:
- Aumentar delay con cada intento fallido (exponential backoff)
- Lockout temporal tras 5 intentos fallidos

**Implementación**: Redis `INCR` + `EXPIRE`

#### 8. CORS (Cross-Origin Resource Sharing)

**Configuración**:
- Whitelist de orígenes permitidos (no `*` en producción)
- `Access-Control-Allow-Origin: https://app.clipboard.com`
- `Access-Control-Allow-Credentials: true` (solo para orígenes confiables)
- `Access-Control-Allow-Methods: GET, POST, PATCH, DELETE`
- `Access-Control-Allow-Headers: Authorization, Content-Type`

#### 9. File Upload Security

**Validación estricta**:
- Verificar **magic numbers** (primeros bytes del archivo), no solo extensión
- Escanear con antivirus (ClamAV en worker)
- Límite estricto de tamaño (10MB)
- MIME type whitelist

**Almacenamiento seguro**:
- Almacenar en S3, no en filesystem del servidor
- Servir con `Content-Disposition: attachment` (forzar download, no ejecutar)
- Nombres de archivo únicos (UUID), no usar filename del usuario

**Prevenir**:
- Ejecutables (.exe, .sh, .bat)
- Scripts (.js, .php, .py)
- Archivos embeddables maliciosos (.svg con scripts)

#### 10. Secrets Management

**NO commitear secrets en git**:
- Usar `.env` files (gitignored)
- `.env.example` con valores de ejemplo (committed)
- GitHub: verificar que no hay secrets expuestos (git-secrets)

**Producción**:
- **AWS Secrets Manager** o **HashiCorp Vault**
- No hardcodear en código
- Rotar secrets regularmente (cada 90 días)
- Auditar acceso a secrets

### Seguridad en la Infraestructura

#### 1. Network Security

**VPC Privada**:
- Database y Redis en subnets privadas (no acceso público)
- Solo API servers en subnet pública
- Security groups restrictivos (mínimo necesario)

**HTTPS Obligatorio**:
- TLS 1.2+ (no TLS 1.0/1.1)
- Certificado válido (Let's Encrypt gratis)
- HSTS header: `Strict-Transport-Security: max-age=31536000; includeSubDomains`

**WAF (Web Application Firewall)**:
- AWS WAF o CloudFlare
- Protección contra ataques comunes (SQL injection, XSS)
- Rate limiting a nivel de IP

#### 2. Database Security

**No exponer públicamente**:
- DB solo accesible desde VPC privada
- No public IP

**Usuario con permisos mínimos**:
- No usar usuario `root`/`postgres`
- Crear usuario específico con solo permisos necesarios

**Backup encriptado**:
- Automated backups diarios
- Encriptación at-rest habilitada (RDS encryption)
- Retención 30 días

**Connection Security**:
- TLS/SSL para conexiones
- Rotate credentials regularmente

#### 3. Redis Security

**Password fuerte**:
- `requirepass` con password complejo
- No default password

**No exponer públicamente**:
- Solo accesible desde VPC privada

**TLS**:
- Habilitar TLS para conexiones (ElastiCache soporta)

#### 4. S3 Security

**Buckets privados**:
- Block public access habilitado por defecto
- Solo acceso via signed URLs o IAM roles

**Versioning**:
- Habilitado (permite recuperar archivos borrados accidentalmente)

**Encryption**:
- Encryption at-rest (SSE-S3 o SSE-KMS)

**Access Logging**:
- Logs de todos los accesos a bucket
- Auditar accesos sospechosos

#### 5. Container Security

**Imágenes base**:
- Usar imágenes oficiales (ej: `rust:1.75-slim`)
- Escanear vulnerabilidades (`trivy`, `docker scan`)

**No correr como root**:
```dockerfile
USER appuser
```

**Read-only filesystem** (cuando posible):
```yaml
securityContext:
  readOnlyRootFilesystem: true
```

**Minimal images**:
- Multi-stage builds para reducir tamaño
- Solo binario necesario en imagen final

### Compliance y Privacidad

#### GDPR (General Data Protection Regulation)

**Si tienes usuarios en EU**:
- **Right to Access**: Permitir exportar datos del usuario
- **Right to Erasure**: Permitir eliminar cuenta y todos los datos
- **Consent**: Consentimiento explícito para procesar datos
- **Data Minimization**: Solo recolectar datos necesarios
- **Privacy Policy**: Clara y accesible

**Implementación**:
- Endpoint: `GET /users/me/export` (retorna JSON con todos los datos)
- Endpoint: `DELETE /users/me` (soft delete inicial, hard delete después de 30 días)

#### Encriptación

**En tránsito**:
- TLS 1.2+ para todas las conexiones
- HTTPS mandatory (redirect HTTP → HTTPS)
- WebSocket Secure (WSS)

**En reposo** (at-rest):
- RDS: encryption habilitada
- S3: encryption habilitada (SSE-S3)
- Backups: encriptados

**End-to-End** (futuro - Fase 4):
- Cliente encripta `content_text` antes de enviar
- Servidor solo almacena datos encriptados (no puede leer)
- Usuario maneja clave de encriptación (password-derived)
- Más seguro pero más complejo

### Security Audits

**Regular** (automático):
- **Dependency updates**: Semanal (Dependabot / Renovate)
- **Vulnerability scanning**: `cargo audit`, `npm audit` en cada commit
- **OWASP ZAP scans**: Mensual

**Antes de releases** (manual):
- **Penetration testing**: Contratar profesional o usar Bug Bounty
- **Security code review**: Revisar cambios sensibles
- **Secrets scanning**: Verificar que no hay secrets en repo

### Incident Response Plan

**Proceso**:

**1. Detección**:
- Monitoring alerts
- User reports
- Security scanners

**2. Contención**:
- Aislar sistema afectado
- Revocar credenciales comprometidas
- Bloquear IPs maliciosas

**3. Erradicación**:
- Identificar vulnerabilidad
- Aplicar fix
- Deploy fix inmediatamente

**4. Recuperación**:
- Restaurar servicio normal
- Verificar que fix funciona
- Monitorear de cerca

**5. Lecciones aprendidas**:
- Postmortem document
- ¿Qué salió mal?
- ¿Cómo prevenir en el futuro?
- Actualizar runbooks

**Comunicación**:
- Notificar a usuarios si hay data breach (GDPR requirement)
- Transparencia en status page

---

## Plan de Implementación

### Fase 1: MVP (2-3 meses)

**Objetivo**: Producto mínimo viable funcional para early adopters (solo desktop, texto).

#### Sprint 1-2: Setup e Infraestructura Base (2 semanas)
**clipboard-app**:
- [ ] Setup proyecto Tauri v2
- [ ] Configurar Rust backend (src-tauri)
- [ ] Configurar frontend (React/Svelte/Vue con TypeScript)
- [ ] Setup SQLite local storage
- [ ] Configurar TailwindCSS

**clipboard-backend**:
- [ ] Setup proyecto Rust + Axum
- [ ] Estructura de carpetas (Clean Architecture)
- [ ] Configurar PostgreSQL local (Docker Compose)
- [ ] Configurar Redis local (Docker Compose)
- [ ] Primera migration: create users table
- [ ] Docker Compose para desarrollo completo

**CI/CD**:
- [ ] GitHub Actions: CI básico (lint, test)

#### Sprint 3-4: Autenticación (2 semanas)
**Backend**:
- [ ] Schema: users, sessions tables
- [ ] JWT generation y validation
- [ ] Password hashing (bcrypt)
- [ ] Endpoints:
  - [ ] POST /auth/register
  - [ ] POST /auth/login
  - [ ] POST /auth/refresh
  - [ ] POST /auth/logout
  - [ ] GET /auth/me
- [ ] Middleware de autenticación
- [ ] Rate limiting en login
- [ ] Unit tests de auth

**Frontend**:
- [ ] UI: Login screen
- [ ] UI: Register screen
- [ ] Integración con backend (API client)
- [ ] Store de autenticación (Zustand/Jotai)
- [ ] Persistir tokens localmente

#### Sprint 5-6: Clipboard Items - Solo Texto (2 semanas)
**Backend**:
- [ ] Schema: clipboard_items, devices tables
- [ ] Repositories: ClipboardRepository, DeviceRepository
- [ ] Services: ClipboardService
- [ ] Endpoints:
  - [ ] GET /clipboard/items (con paginación)
  - [ ] POST /clipboard/items
  - [ ] GET /clipboard/items/:id
  - [ ] PATCH /clipboard/items/:id
  - [ ] DELETE /clipboard/items/:id
- [ ] Validación de límites por tier
- [ ] Unit + integration tests

**Frontend (Tauri)**:
- [ ] Clipboard listener (detectar copy)
- [ ] Guardar en SQLite local
- [ ] UI: Lista de clipboard items
- [ ] UI: Detalle de item
- [ ] UI: Copiar item al clipboard OS
- [ ] Tauri commands para clipboard operations

#### Sprint 7-8: Sincronización Básica (2 semanas)
**Backend**:
- [ ] API: Paginación, filtros, ordering

**Frontend**:
- [ ] Botón "Sync" manual
- [ ] Fetch items desde backend
- [ ] Merge con items locales (conflict resolution básico: last-write-wins)
- [ ] Indicadores de sync en UI (syncing, synced, error)
- [ ] Offline queue (si no hay internet, quedar pendiente)

#### Sprint 9-10: Devices Management (2 semanas)
**Backend**:
- [ ] Endpoint: POST /devices/register
- [ ] Endpoint: GET /devices
- [ ] Endpoint: DELETE /devices/:id (desactivar)

**Frontend**:
- [ ] Device fingerprinting (ID único del dispositivo)
- [ ] Auto-register device al login
- [ ] UI: Settings → Devices list
- [ ] UI: Desactivar dispositivo remoto

#### Sprint 11-12: Deploy y Testing (2 semanas)
**Backend**:
- [ ] Deploy a Railway/Render
- [ ] Setup PostgreSQL en Supabase
- [ ] Setup Redis en Upstash
- [ ] Variables de entorno en prod
- [ ] Health checks

**Frontend**:
- [ ] Build para Windows
- [ ] Build para macOS
- [ ] Code signing (futuro, skip por ahora)
- [ ] Testing en ambas plataformas

**Testing**:
- [ ] E2E tests básicos
- [ ] Load testing inicial
- [ ] Security scan básico

**Documentación**:
- [ ] README con instrucciones de setup
- [ ] API documentation (básico)

**Entregable Fase 1**:
- ✅ App Tauri funcional en Windows/Mac
- ✅ Usuarios pueden registrarse/login
- ✅ Clipboard text-only con sync manual
- ✅ Backend deployado y estable
- ✅ SQLite local funcional

---

### Fase 2: Core Features (2-3 meses)

**Objetivo**: Funcionalidad completa del producto, tiempo real, más tipos de contenido.

#### Sprint 13-14: WebSocket + Real-Time Sync (2 semanas)
**Backend**:
- [ ] WebSocket endpoint: WS /ws
- [ ] Redis Pub/Sub implementation
- [ ] WebSocket manager (track connections)
- [ ] Message broker (routing)
- [ ] Broadcast events: new_item, item_updated, item_deleted

**Frontend**:
- [ ] WebSocket client
- [ ] Auto-connect on login
- [ ] Handle messages (update UI en tiempo real)
- [ ] Reconnection logic (exponential backoff)
- [ ] Heartbeat (ping/pong cada 30s)

#### Sprint 15-16: File Upload (Imágenes) (2 semanas)
**Backend**:
- [ ] MinIO setup local (Docker)
- [ ] S3 client implementation (aws-sdk-s3)
- [ ] FileService: upload, download
- [ ] Thumbnail generation (image crate)
- [ ] Endpoint: POST /clipboard/upload (multipart)
- [ ] Endpoint: GET /clipboard/files/:id/download
- [ ] Schema update: file_url, thumbnail_url, etc.

**Frontend**:
- [ ] UI: Drag & drop para upload
- [ ] UI: File picker
- [ ] Upload con progress bar
- [ ] Preview de imágenes
- [ ] Thumbnail display

#### Sprint 17-18: Más Tipos de Contenido (2 semanas)
**Backend**:
- [ ] Support: rich_text (HTML/Markdown)
- [ ] Support: code (con metadata: language)
- [ ] Support: color (hex parser)
- [ ] Support: link (con metadata: title, favicon)
- [ ] Support: file (archivos genéricos, no imágenes)
- [ ] Validación por tipo

**Frontend**:
- [ ] Renderizado: rich_text (sanitizado)
- [ ] Renderizado: code (syntax highlighting)
- [ ] Renderizado: color (preview visual)
- [ ] Renderizado: link (clickable, con favicon)
- [ ] Renderizado: file (icon + download button)

#### Sprint 19-20: Búsqueda Full-Text (2 semanas)
**Backend**:
- [ ] PostgreSQL full-text search setup
- [ ] Índice GIN en search_vector
- [ ] Endpoint: GET /clipboard/search
- [ ] SearchService con ranking
- [ ] Cache de búsquedas frecuentes (Redis)

**Frontend**:
- [ ] UI: Barra de búsqueda global
- [ ] UI: Resultados con highlighting
- [ ] Filtros: por tipo, fecha
- [ ] Debounce en búsqueda (no buscar en cada keystroke)

#### Sprint 21-22: Favoritos y Snippets (2 semanas)
**Backend**:
- [ ] Endpoint: POST /clipboard/items/:id/favorite
- [ ] Endpoint: DELETE /clipboard/items/:id/favorite
- [ ] Endpoints: CRUD /snippets
- [ ] Validación: snippet_name required si is_snippet

**Frontend**:
- [ ] UI: Botón "Favorite" en cada item
- [ ] UI: Sección "Favorites" (lista filtrada)
- [ ] UI: Sección "Snippets"
- [ ] UI: Crear snippet (form)
- [ ] UI: Editar snippet

#### Sprint 23-24: Cache y Optimizaciones (2 semanas)
**Backend**:
- [ ] Redis caching layer completo
- [ ] Cache strategy implementation (todos los keys)
- [ ] Cache invalidation en todas las operaciones
- [ ] Rate limiting completo
- [ ] Database query optimizations (indexes, explain analyze)
- [ ] Connection pooling tuning

**Testing**:
- [ ] Load testing con k6
- [ ] Benchmark performance
- [ ] Optimizar queries lentas

**Entregable Fase 2**:
- ✅ Sincronización en tiempo real (WebSocket)
- ✅ Soporte completo de tipos de contenido (texto, imágenes, código, etc.)
- ✅ Búsqueda full-text funcional
- ✅ Favoritos y snippets
- ✅ Performance optimizada (caching, indices)

---

### Fase 3: Premium Features & Mobile (2-3 meses)

**Objetivo**: Modelo de negocio, apps mobile, características premium, polish UX.

#### Sprint 25-26: Sistema de Suscripciones (2 semanas)
**Backend**:
- [ ] Schema: subscription_tier, limits
- [ ] Lógica de validación de límites por tier
- [ ] Endpoint: GET /subscription/plans
- [ ] Endpoint: POST /subscription/upgrade (webhook placeholder)
- [ ] Integración con Stripe/Paddle (webhooks)
- [ ] Handle payment events

**Frontend**:
- [ ] UI: Paywall (cuando alcanza límite free)
- [ ] UI: Upgrade flow (modal con planes)
- [ ] UI: Settings → Subscription management
- [ ] Integración con Stripe Checkout

#### Sprint 27-28: Tauri Mobile - iOS (2 semanas)
**Frontend**:
- [ ] Setup Tauri iOS (`tauri ios init`)
- [ ] Adaptar UI para mobile (responsive)
- [ ] Touch gestures (swipe, long-press)
- [ ] iOS-specific: clipboard integration
- [ ] iOS permissions (clipboard, storage, notifications)
- [ ] Testing en simulador iOS
- [ ] TestFlight setup (para beta testers)

#### Sprint 29-30: Tauri Mobile - Android (2 semanas)
**Frontend**:
- [ ] Setup Tauri Android (`tauri android init`)
- [ ] Adaptar UI (ya está responsive de iOS)
- [ ] Android-specific: clipboard integration
- [ ] Android permissions
- [ ] Testing en emulador Android
- [ ] Google Play internal testing

#### Sprint 31-32: Advanced Search & Filters (2 semanas)
**Backend**:
- [ ] Filtros avanzados (múltiples tipos, fecha range, favoritos)
- [ ] Sorting customizable (date, relevance)
- [ ] Saved searches (opcional)

**Frontend**:
- [ ] UI: Advanced filters panel
- [ ] UI: Sort options
- [ ] UI: Date range picker

#### Sprint 33-34: Historial y Cleanup (2 semanas)
**Backend**:
- [ ] Worker: cleanup-worker (elimina items > 1 año)
- [ ] Cron job config
- [ ] Endpoint: GET /clipboard/history (timeline view)
- [ ] Soft delete → hard delete después de 30 días

**Frontend**:
- [ ] UI: Timeline view (agrupado por fecha)
- [ ] UI: Restore deleted items (undo delete)

**Testing**:
- [ ] Automated testing de cleanup worker

#### Sprint 35-36: Polish & UX (2 semanas)
**Frontend**:
- [ ] Keyboard shortcuts (Ctrl+C, Ctrl+V, Ctrl+F, etc.)
- [ ] Context menus (right-click)
- [ ] System tray icon (desktop)
- [ ] Notifications (local y push)
- [ ] Onboarding flow (tutorial first-time)
- [ ] Settings panel completo
- [ ] Dark mode / themes
- [ ] Animations y transitions

**Testing**:
- [ ] User testing con beta testers
- [ ] Collect feedback y fix bugs

**Entregable Fase 3**:
- ✅ Modelo freemium funcional con Stripe
- ✅ Apps mobile (iOS/Android) en beta testing
- ✅ Historial completo (5000 items, 1 año)
- ✅ UX pulida y profesional
- ✅ Onboarding y tutoriales

---

### Fase 4: Scale & Advanced (3+ meses)

**Objetivo**: Preparar para 1M+ usuarios, features avanzadas, enterprise features.

#### Sprint 37-40: Migración a AWS (4 semanas)
**Infraestructura**:
- [ ] Terraform para toda la infra AWS
- [ ] VPC setup (public/private subnets)
- [ ] RDS PostgreSQL con Multi-AZ
- [ ] Read replicas para DB
- [ ] ElastiCache Redis cluster mode
- [ ] ECS Fargate o EKS setup
- [ ] ALB (Application Load Balancer)
- [ ] S3 buckets + CloudFront
- [ ] Route 53 (DNS)
- [ ] ACM (SSL certificates)
- [ ] CloudWatch logs y métricas
- [ ] Secrets Manager

**Migration**:
- [ ] Plan de migración detallado
- [ ] Testing en staging (AWS)
- [ ] Migración de datos (PostgreSQL dump/restore)
- [ ] Cutover a producción (downtime mínimo)
- [ ] Rollback plan

#### Sprint 41-42: Compartir Clips (2 semanas)
**Backend**:
- [ ] Schema: shared_clips table
- [ ] Endpoint: POST /clipboard/items/:id/share
- [ ] Generar link público único
- [ ] Permisos: view-only, expiración
- [ ] Endpoint: GET /shared/:share_id (público, no auth)

**Frontend**:
- [ ] UI: Botón "Share" en item
- [ ] UI: Modal de share (opciones, copy link)
- [ ] UI: View shared clip (página pública)

#### Sprint 43-44: Tags y Organización (2 semanas)
**Backend**:
- [ ] Schema: tags, clipboard_item_tags (many-to-many)
- [ ] Endpoints: CRUD /tags
- [ ] Endpoint: POST /clipboard/items/:id/tags
- [ ] Filtros por tags

**Frontend**:
- [ ] UI: Tag manager
- [ ] UI: Asignar tags a items
- [ ] UI: Filter by tags
- [ ] UI: Tag colors

#### Sprint 45-46: Analytics Dashboard (2 semanas)
**Backend**:
- [ ] Worker: analytics-worker
- [ ] Schema: usage_metrics
- [ ] Endpoint: GET /analytics/stats
- [ ] Métricas: items created, storage used, activity heatmap

**Frontend**:
- [ ] UI: Dashboard para usuarios (mis stats)
- [ ] UI: Admin dashboard (opcional, para ver todos los usuarios)

#### Sprint 47-48: Performance at Scale (2 semanas)
**Backend**:
- [ ] Database partitioning (clipboard_items por mes)
- [ ] Connection pooling optimization (PgBouncer)
- [ ] Horizontal scaling tests (múltiples instancias)
- [ ] CDN optimization (CloudFront)
- [ ] Caching improvements (más aggressive)

**Testing**:
- [ ] Load testing con 10K+ usuarios simultáneos
- [ ] Stress testing
- [ ] Chaos engineering (kill instances, verificar recovery)

#### Sprint 49-50: Advanced Security (2 semanas)
**Backend**:
- [ ] End-to-end encryption (opcional, complejo)
- [ ] 2FA implementation (TOTP)
- [ ] Security audit completo
- [ ] Penetration testing (contratar profesional)
- [ ] GDPR compliance review

**Frontend**:
- [ ] UI: 2FA setup
- [ ] UI: Backup codes

**Entregable Fase 4**:
- ✅ Sistema escalado a 1M+ usuarios (AWS)
- ✅ Features avanzadas (compartir, tags, analytics)
- ✅ Seguridad enterprise-grade (2FA, audits)
- ✅ Alta disponibilidad (Multi-AZ, load balancing)
- ✅ Performance optimizada

---

### Ongoing (Durante todo el desarrollo)

**Cada Sprint**:
- [ ] Testing continuo (unit, integration, e2e)
- [ ] Code reviews
- [ ] Documentation updates
- [ ] Security patches
- [ ] Dependency updates
- [ ] Performance monitoring
- [ ] User feedback incorporation
- [ ] Bug fixes

**Mensual**:
- [ ] Retrospective
- [ ] Planning siguiente mes
- [ ] Review de métricas (DAU, MAU, conversión)

---

## Infraestructura y DevOps

### Ambientes

#### Development (Local)
**clipboard-app**:
- Local machine (Windows/Mac)
- SQLite local

**clipboard-backend**:
- Docker Compose:
  - PostgreSQL 16
  - Redis 7
  - MinIO (S3-compatible)
  - API server (hot reload con `cargo watch`)

**Acceso**:
- API: `http://localhost:8000`
- MinIO UI: `http://localhost:9001`
- PostgreSQL: `localhost:5432`
- Redis: `localhost:6379`

#### Staging
**Frontend**:
- Builds de testing (no distribuidos públicamente)

**Backend**:
- Cloud: Railway / Render
- Database: Supabase PostgreSQL
- Cache: Upstash Redis (serverless, tier gratuito)
- Storage: Supabase Storage o MinIO en Railway

**Acceso**:
- API: `https://api-staging.clipboard-manager.com`

**Propósito**:
- Testing de features antes de producción
- E2E tests
- Beta testing

#### Production
**Frontend**:
- Distribuido via instaladores (Windows: .msi, Mac: .dmg)
- Mobile: App Store, Google Play

**Backend**:
- Cloud: AWS
- Compute: ECS Fargate (containers) o EKS (Kubernetes)
- Database: RDS PostgreSQL Multi-AZ + read replicas
- Cache: ElastiCache Redis cluster mode
- Storage: S3 + CloudFront (CDN)
- Load Balancer: ALB (Application Load Balancer)
- DNS: Route 53
- SSL: ACM (AWS Certificate Manager)

**Acceso**:
- API: `https://api.clipboard-manager.com`
- CDN: `https://cdn.clipboard-manager.com`

---

### CI/CD Pipeline

#### GitHub Actions Workflows

**Repositorio: clipboard-app**

**.github/workflows/ci.yml** (on PR):
```yaml
name: CI - Frontend
on: [push, pull_request]
jobs:
  lint-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v1
      - run: bun install
      - run: bun run lint
      - run: bun run test
      
  rust-check:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo clippy -- -D warnings
      - run: cargo test
```

**.github/workflows/build-desktop.yml** (on tag):
```yaml
name: Build Desktop Apps
on:
  push:
    tags: ['v*']
jobs:
  build:
    strategy:
      matrix:
        platform: [windows-latest, macos-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v1
      - uses: dtolnay/rust-toolchain@stable
      - run: bun install
      - run: bun run tauri build
      - uses: actions/upload-artifact@v4
        with:
          name: app-${{ matrix.platform }}
          path: src-tauri/target/release/bundle/
```

**.github/workflows/build-mobile.yml** (on tag):
```yaml
name: Build Mobile Apps
on:
  push:
    tags: ['v*']
jobs:
  build-ios:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v1
      - run: bun install
      - run: bun run tauri ios build
      
  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v1
      - run: bun install
      - run: bun run tauri android build
```

**Repositorio: clipboard-backend**

**.github/workflows/ci.yml** (on PR):
```yaml
name: CI - Backend
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo audit
```

**.github/workflows/cd-staging.yml** (on push to main):
```yaml
name: Deploy to Staging
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker image
        run: docker build -t clipboard-api:staging -f infrastructure/docker/Dockerfile.api .
      - name: Push to registry
        run: docker push registry.example.com/clipboard-api:staging
      - name: Deploy to Railway/Render
        run: ./infrastructure/scripts/deploy-staging.sh
      - name: Run E2E tests
        run: bun run test:e2e
```

**.github/workflows/cd-production.yml** (on tag push):
```yaml
name: Deploy to Production
on:
  push:
    tags: ['v*']
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run all tests
        run: cargo test --all
      - name: Build Docker image
        run: docker build -t clipboard-api:${{ github.ref_name }} -f infrastructure/docker/Dockerfile.api .
      - name: Security scan
        run: trivy image clipboard-api:${{ github.ref_name }}
      - name: Push to ECR
        run: |
          aws ecr get-login-password | docker login ...
          docker push ...
      - name: Deploy to ECS
        run: aws ecs update-service ...
      - name: Health check
        run: ./infrastructure/scripts/health-check.sh
      - name: Rollback on failure
        if: failure()
        run: ./infrastructure/scripts/rollback.sh
```

---

### Docker

#### Dockerfile.api (Multi-stage build)

```dockerfile
# Stage 1: Builder
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY api-server/Cargo.toml api-server/Cargo.lock ./

# Build dependencies (cached layer)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY api-server/src ./src
COPY api-server/migrations ./migrations

# Build application
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser

# Copy binary from builder
COPY --from=builder /app/target/release/api-server /usr/local/bin/api-server

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Switch to non-root user
USER appuser

EXPOSE 8000

CMD ["api-server"]
```

#### docker-compose.yml (Development)

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:16-alpine
    container_name: clipboard-postgres
    environment:
      POSTGRES_DB: clipboard_dev
      POSTGRES_USER: dev
      POSTGRES_PASSWORD: dev
    ports:
      - "5432:5432"
    volumes:
      - postgres-data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U dev"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    container_name: clipboard-redis
    ports:
      - "6379:6379"
    command: redis-server --appendonly yes
    volumes:
      - redis-data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  minio:
    image: minio/minio:latest
    container_name: clipboard-minio
    command: server /data --console-address ":9001"
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - minio-data:/data
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 30s
      timeout: 20s
      retries: 3

  api:
    build:
      context: .
      dockerfile: infrastructure/docker/Dockerfile.api
    container_name: clipboard-api
    environment:
      DATABASE_URL: postgres://dev:dev@postgres:5432/clipboard_dev
      REDIS_URL: redis://redis:6379
      S3_ENDPOINT: http://minio:9000
      S3_ACCESS_KEY: minioadmin
      S3_SECRET_KEY: minioadmin
      S3_BUCKET: clipboard-dev
      JWT_SECRET: dev-secret-key-change-in-production
      RUST_LOG: info
    ports:
      - "8000:8000"
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
      minio:
        condition: service_healthy
    volumes:
      - ./api-server:/app
    command: cargo watch -x run

volumes:
  postgres-data:
  redis-data:
  minio-data:
```

---

### Kubernetes (Producción - AWS EKS)

**Estructura**:
```
infrastructure/kubernetes/
├── base/
│   ├── namespace.yaml
│   ├── configmap.yaml
│   ├── api-deployment.yaml
│   ├── api-service.yaml
│   ├── api-hpa.yaml
│   ├── ingress.yaml
│   └── ...
├── overlays/
│   ├── production/
│   │   ├── kustomization.yaml
│   │   └── patches/
│   └── staging/
│       ├── kustomization.yaml
│       └── patches/
└── kustomization.yaml
```

**api-deployment.yaml**:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: clipboard-api
  labels:
    app: clipboard-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: clipboard-api
  template:
    metadata:
      labels:
        app: clipboard-api
    spec:
      containers:
      - name: api
        image: <ECR_REPO>/clipboard-api:latest
        ports:
        - containerPort: 8000
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: clipboard-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: clipboard-secrets
              key: redis-url
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: clipboard-secrets
              key: jwt-secret
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health/live
            port: 8000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health/ready
            port: 8000
          initialDelaySeconds: 10
          periodSeconds: 5
```

**api-hpa.yaml** (Horizontal Pod Autoscaler):
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: clipboard-api-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: clipboard-api
  minReplicas: 3
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

---

### Terraform (AWS Infrastructure as Code)

**Estructura**:
```
infrastructure/terraform/
├── modules/
│   ├── vpc/
│   ├── rds/
│   ├── elasticache/
│   ├── ecs/
│   ├── s3/
│   └── ...
├── environments/
│   ├── production/
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   └── terraform.tfvars
│   └── staging/
└── ...
```

**Ejemplo: main.tf (production)**:
```hcl
terraform {
  backend "s3" {
    bucket = "clipboard-terraform-state"
    key    = "production/terraform.tfstate"
    region = "us-east-1"
  }
}

module "vpc" {
  source = "../../modules/vpc"
  environment = "production"
}

module "rds" {
  source = "../../modules/rds"
  vpc_id = module.vpc.vpc_id
  private_subnet_ids = module.vpc.private_subnet_ids
  instance_class = "db.r6g.xlarge"
  multi_az = true
}

module "elasticache" {
  source = "../../modules/elasticache"
  vpc_id = module.vpc.vpc_id
  private_subnet_ids = module.vpc.private_subnet_ids
  node_type = "cache.r6g.large"
  num_cache_nodes = 3
}

module "ecs" {
  source = "../../modules/ecs"
  vpc_id = module.vpc.vpc_id
  public_subnet_ids = module.vpc.public_subnet_ids
  desired_count = 3
}

# ... más módulos
```

---

### Backup Strategy

#### Database (RDS PostgreSQL)
- **Automated daily backups**: 11:00 PM UTC
- **Retention**: 30 días
- **Point-in-time recovery**: Habilitado
- **Cross-region replication**: Opcional (para DR)
- **Backup testing**: Restore mensual a staging

**Script de backup manual**:
```bash
#!/bin/bash
# infrastructure/scripts/backup-db.sh

pg_dump -h $DB_HOST -U $DB_USER -d clipboard_prod \
  --format=custom --file=backup_$(date +%Y%m%d_%H%M%S).dump
  
aws s3 cp backup_*.dump s3://clipboard-backups/database/
```

#### S3 (Archivos)
- **Versioning**: Habilitado (permite recuperar versiones anteriores)
- **Replication**: A otro bucket en diferente región
- **Lifecycle policies**: Archivos viejos a Glacier después de 90 días

#### Redis
- **Snapshots**: Diarios (RDB)
- **AOF** (Append-Only File): Habilitado para durabilidad
- **Replication**: Cluster mode con réplicas

---

### Disaster Recovery (DR)

**RTO** (Recovery Time Objective): 1 hora
**RPO** (Recovery Point Objective): 5 minutos

**Estrategia**:
1. **Multi-AZ deployment**: DB y Redis en múltiples availability zones
2. **Read replicas**: Para failover rápido
3. **Load balancer health checks**: Auto-failover si instancia falla
4. **Automated backups**: Restore rápido si necesario
5. **DR drills**: Cada 3 meses, simular disaster y practicar recovery

**Runbook**:
1. Detectar outage (monitoring)
2. Verificar causa (logs, métricas)
3. Si DB: failover a replica
4. Si app: scale up instancias
5. Si total: restore from backup
6. Comunicar a usuarios (status page)
7. Postmortem después

---

### Secrets Management

#### Development
**Local**:
- `.env` files (gitignored)
- `.env.example` committed (con valores de ejemplo)

**Ejemplo .env**:
```
DATABASE_URL=postgres://dev:dev@localhost:5432/clipboard_dev
REDIS_URL=redis://localhost:6379
S3_ENDPOINT=http://localhost:9000
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
JWT_SECRET=dev-secret-key-change-in-production
```

#### Staging/Production
**AWS Secrets Manager**:
- Store all secrets: DB password, JWT secret, API keys
- Rotation automation (cada 90 días)
- Audit logging de acceso
- IAM policies para acceso granular

**Kubernetes Secrets** (sealed):
- Use `sealed-secrets` controller
- Encrypt secrets en git
- Auto-decrypt en cluster

**Acceso desde app**:
```rust
use aws_sdk_secretsmanager::Client;

let client = Client::new(&config);
let secret = client
    .get_secret_value()
    .secret_id("clipboard/production/database")
    .send()
    .await?;
    
let db_password = secret.secret_string().unwrap();
```

---

## Escalabilidad

### Arquitectura Escalable

#### Horizontal Scaling (Scale Out)

**API Servers**:
- **Stateless design**: No guardar estado en memoria (usar Redis)
- **Load balancer**: ALB distribuye tráfico entre múltiples instancias
- **Auto-scaling**: ECS/K8s scale up/down automáticamente
- **Target**: Manejar 10,000 requests/segundo con 10-20 instancias

**Workers**:
- **Queue-based**: Usar SQS o Redis queues
- **Múltiples instancias**: Procesamiento paralelo
- **Idempotent operations**: Safe to retry

**Database**:
- **Read replicas**: Para queries read-heavy (GET /clipboard/items)
- **Connection pooling**: PgBouncer (1000+ connections → 100 al DB)
- **Sharding** (si necesario en el futuro): Por user_id

**Redis**:
- **Cluster mode**: Múltiples shards para distribuir carga
- **Replication**: Cada shard tiene réplicas para alta disponibilidad

#### Vertical Scaling (Scale Up)

**API Server**: 
- Empezar: 1 CPU, 2GB RAM
- Medio: 2 CPU, 4GB RAM
- Grande: 4 CPU, 8GB RAM

**Database**:
- Empezar: db.t3.medium (2 vCPU, 4GB RAM)
- Medio: db.r6g.large (2 vCPU, 16GB RAM)
- Grande: db.r6g.xlarge (4 vCPU, 32GB RAM)

**Redis**:
- Empezar: cache.t3.micro (2 cores, 0.5GB)
- Medio: cache.r6g.large (2 cores, 13GB)
- Grande: cache.r6g.xlarge (4 cores, 26GB)

#### Caching Layers (Reducir carga en DB)

**Niveles**:
1. **Browser cache**: Assets estáticos (CSS, JS, imágenes)
2. **CDN cache** (CloudFront): Archivos, thumbnails
3. **Application cache** (Redis): Items, búsquedas, stats
4. **Database cache**: PostgreSQL `shared_buffers`
5. **Client cache** (SQLite): En Tauri, offline-first

**Benefit**: Reducir 80% de queries a DB con caching efectivo.

### Database Optimization

#### Índices

**Índices compuestos** para queries comunes:
```sql
CREATE INDEX idx_clipboard_user_created ON clipboard_items(user_id, created_at DESC);
CREATE INDEX idx_clipboard_user_type ON clipboard_items(user_id, content_type);
```

**Partial indexes** para filtros frecuentes:
```sql
CREATE INDEX idx_clipboard_favorites ON clipboard_items(user_id, is_favorite) 
  WHERE is_favorite = true AND deleted_at IS NULL;
```

**Monitoring**:
- Ver índices no usados: `pg_stat_user_indexes`
- Queries lentas: `pg_stat_statements`

#### Query Optimization

**EXPLAIN ANALYZE** para queries lentas:
```sql
EXPLAIN ANALYZE
SELECT * FROM clipboard_items 
WHERE user_id = 'uuid' 
ORDER BY created_at DESC 
LIMIT 50;
```

**Evitar N+1 queries**:
- Usar JOINs en vez de múltiples queries
- Eager loading de relaciones

**Batch operations**:
- Insertar múltiples rows en una query
- `INSERT INTO ... VALUES (...), (...), (...)`

#### Partitioning (Para escalar a millones)

**Particionar clipboard_items por mes**:
```sql
CREATE TABLE clipboard_items_2025_01 PARTITION OF clipboard_items
    FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');
```

**Beneficios**:
- Queries más rápidas (solo escanean partición relevante)
- Eliminar particiones viejas es instantáneo
- Índices más pequeños por partición

#### Connection Pooling

**PgBouncer**:
- Reusa conexiones a PostgreSQL
- Soporta 1000+ cliente connections → 100 DB connections
- Reduce overhead de crear/destruir connections

**Configuración**:
```ini
[databases]
clipboard_prod = host=rds-endpoint.amazonaws.com dbname=clipboard_prod

[pgbouncer]
pool_mode = transaction
max_client_conn = 1000
default_pool_size = 100
```

### Rate Limiting Strategies

#### Por Usuario (Prevenir abuse)

**Endpoints normales**:
- 100 requests/minuto
- 1000 requests/hora

**Endpoints caros** (search, upload):
- 10 requests/minuto

**Login**:
- 5 intentos fallidos cada 15 minutos
- Lockout temporal

**Implementación** (Redis):
```rust
let key = format!("rate_limit:{}:{}", user_id, endpoint);
let count: i32 = redis.incr(&key).await?;

if count == 1 {
    redis.expire(&key, 60).await?; // TTL 1 minuto
}

if count > 100 {
    return Err(Error::RateLimitExceeded);
}
```

#### Por IP (Protección DDoS)

**Requests no autenticados**:
- 1000 requests/minuto por IP

**Usar AWS WAF o CloudFlare** para rate limiting a nivel de red.

#### Throttling Gradual

**80% del límite**: Enviar header `X-RateLimit-Warning: approaching limit`
**100% del límite**: HTTP 429 con `Retry-After: 60`
**Backoff exponencial**: Sugerir al cliente esperar 1s, 2s, 4s, 8s...

### CDN Strategy (CloudFront)

**Beneficios**:
- Edge locations globales (baja latencia)
- Cache de assets (reduce carga en servidor)
- DDoS protection

**Configuración**:
- **Origin**: S3 bucket
- **Cache behavior**:
  - Thumbnails: cache 1 año (inmutables)
  - Archivos: cache 1 día (puede cambiar)
  - API: no cache (dinámico)
- **Compression**: Brotli + Gzip
- **HTTP/2**: Habilitado

**Invalidación**:
- Solo si archivo cambia con mismo nombre (raro)
- Mejor: usar hash en nombre de archivo (cache busting)

### Monitoring for Scale

#### Métricas Clave

**Throughput**:
- Requests/segundo por endpoint
- Target: >1000 req/s con 10 instancias

**Latency**:
- P50, P95, P99 por endpoint
- Target: P95 < 200ms, P99 < 500ms

**Error Rate**:
- % de requests con 5xx errors
- Target: <0.1%

**Resource Usage**:
- CPU usage por instancia (target <70%)
- Memory usage (target <80%)
- Database connection pool (target <80% de max)
- Redis memory usage

#### Auto-scaling Triggers

**Scale UP cuando**:
- CPU > 70% por 5 minutos
- Memory > 80%
- Request queue depth > 100
- P95 latency > 500ms

**Scale DOWN cuando**:
- CPU < 30% por 10 minutos
- Memory < 40%
- Mínimo 2 instancias siempre

**Configuración ECS**:
```hcl
resource "aws_appautoscaling_policy" "api_cpu" {
  name               = "clipboard-api-cpu-scaling"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.api.resource_id
  scalable_dimension = aws_appautoscaling_target.api.scalable_dimension
  service_namespace  = aws_appautoscaling_target.api.service_namespace

  target_tracking_scaling_policy_configuration {
    target_value       = 70.0
    predefined_metric_specification {
      predefined_metric_type = "ECSServiceAverageCPUUtilization"
    }
  }
}
```

### Cost Optimization

#### Compute

**Spot Instances** para workers (no críticos):
- 70% más baratos que On-Demand
- OK si el worker puede reiniciarse sin problema

**Reserved Instances** para baseline:
- Comprar capacidad mínima (ej: 2 instancias 24/7)
- Hasta 60% descuento vs On-Demand

**Auto-scaling down**:
- Durante madrugada (low traffic): reduce a mínimo

#### Storage

**S3 Intelligent-Tiering**:
- Mueve automáticamente archivos no accedidos a tier más barato
- Frecuent Access → Infrequent Access → Archive

**Lifecycle policies**:
- Archivos viejos (>90 días) → Glacier (mucho más barato)

**Compression**:
- Comprimir archivos antes de subir (si son texto, logs, etc.)

#### Database

**Right-sizing**:
- Empezar pequeño (db.t3.medium)
- Monitor usage y scale solo cuando es necesario
- No sobre-provisionar

**Read replicas**:
- Solo crear si realmente hay mucho read traffic
- Costo adicional pero reduce carga en primary

**Slow query optimization**:
- Identificar queries lentas y optimizarlas
- Mejor optimización > más hardware

---

## Modelo de Negocio

### Tiers de Suscripción

#### Free Tier
**Precio**: $0/mes (Gratis)

**Características**:
- Almacenamiento: **Solo local** (SQLite en dispositivo)
- Dispositivos: **1** (dispositivo actual)
- Sincronización: **No** (sin cloud sync)
- Items: Ilimitado localmente
- File upload: No
- Búsqueda: Local (SQLite FTS)
- Snippets: Sí
- Support: Community (Discord/GitHub)

**Limitaciones**:
- No puede copiar en un dispositivo y pegar en otro
- Si pierde el dispositivo, pierde todos los datos

**Objetivo**: Adquirir usuarios, probar el producto, validar PMF.

#### Pro Tier
**Precio**: $6/mes o $60/año (ahorro de $12)

**Características**:
- Almacenamiento: **5GB en la nube**
- Dispositivos: **Ilimitados**
- Sincronización: **En tiempo real** (WebSocket)
- Items: **5,000** sincronizados
- File upload: **10MB por item**
- Historial: **1 año** automático
- Búsqueda: Full-text en la nube
- Favoritos: Ilimitado
- Snippets: Ilimitado
- Support: **Email** (respuesta en 48h)

**Valor agregado**:
- Copia en laptop, pega en teléfono
- Backup automático en la nube
- Acceso desde cualquier dispositivo

**Objetivo**: Target a power users, freelancers, desarrolladores.

#### Enterprise Tier (Futuro - Fase 4)
**Precio**: Custom (negociado, ~$15-30/usuario/mes)

**Características**:
- Todo de Pro +
- Almacenamiento: **Ilimitado** (o límite alto, ej: 100GB)
- Items: **Ilimitados**
- File size: **50MB** por item
- Historial: **Ilimitado**
- **Compartir con equipo**: Clips compartidos internamente
- **SSO** (Single Sign-On): SAML, OAuth
- **Admin panel**: Gestión de usuarios, permisos
- **Audit logs**: Compliance
- **SLA**: 99.9% uptime garantizado
- Support: **Priority** (respuesta en 4h, on-call)

**Objetivo**: Empresas, equipos grandes (10-1000 usuarios).

### Estrategia de Monetización

**1. Freemium**:
- Versión gratis robusta (no crippled)
- Free users sirven como marketing (word of mouth)
- Conversión target: 5-10% free → pro

**2. Value-based pricing**:
- Premium desbloquea valor REAL (sincronización entre dispositivos)
- No es feature gatekeeping artificial
- Usuarios ven beneficio claro

**3. Annual discount**:
- $6/mes = $72/año
- $60/año = ahorro de $12 (16% descuento)
- Incentiva commitment, reduce churn

**4. No credit card en free**:
- Reducir fricción en signup
- Más usuarios registrados = más conversiones potenciales

**5. Upgrade prompts**:
- Cuando intenta sync en free → "Upgrade to Pro for cloud sync"
- Cuando alcanza límite de storage → "Upgrade for 5GB storage"
- No invasivo, contextual

### Proyección de Ingresos

#### Escenario Conservador (Año 1)

**Usuarios**:
- 10,000 usuarios registrados
- 5% conversión a Pro → **500 Pro users**

**MRR** (Monthly Recurring Revenue):
- 500 users × $6/mes = **$3,000 MRR**

**ARR** (Annual Recurring Revenue):
- $3,000 × 12 = **$36,000 ARR**

**Costos estimados** (AWS, año 1):
- ~$500/mes = $6,000/año

**Profit**: $30,000/año (antes de salarios)

#### Escenario Optimista (Año 2)

**Usuarios**:
- 100,000 usuarios registrados
- 10% conversión → **10,000 Pro users**

**MRR**:
- 10,000 users × $6 = **$60,000 MRR**

**ARR**:
- $60,000 × 12 = **$720,000 ARR**

**Costos estimados**:
- ~$10,000/mes = $120,000/año

**Profit**: $600,000/año

#### Escenario Unicornio (Año 3-5)

**Usuarios**:
- 1,000,000 usuarios registrados
- 10% conversión → **100,000 Pro users**
- 50 Enterprise customers (500 seats @ $20/seat) → $10K/mes

**MRR**:
- 100,000 Pro × $6 = $600,000
- Enterprise: $10,000
- **Total MRR: $610,000**

**ARR**:
- **$7,320,000 ARR**

**Costos estimados**:
- ~$100,000/mes = $1,200,000/año

**Profit**: $6,000,000+/año

### Métricas Clave (KPIs)

**Acquisition**:
- **CAC** (Customer Acquisition Cost): Cuánto cuesta adquirir 1 usuario pagado
- Target: <$20 (marketing eficiente)

**Retention**:
- **Churn Rate**: % usuarios que cancelan mensualmente
- Target: <5% mensual (muy bueno)
- **LTV** (Lifetime Value): CAC × (1 / Churn Rate)
- Target: LTV > 3× CAC

**Conversion**:
- **Free → Pro**: % usuarios free que upgraden
- Target: 5-10%
- **Trial → Paid** (si hay trial): %
- Target: 40-60%

**Engagement**:
- **DAU** (Daily Active Users): Usuarios activos diariamente
- **MAU** (Monthly Active Users): Usuarios activos mensualmente
- **DAU/MAU ratio**: Stickiness (target: >20%)

**Revenue**:
- **MRR** (Monthly Recurring Revenue): Ingresos recurrentes mensuales
- **MRR Growth Rate**: % crecimiento mes a mes (target: >10%)
- **ARPU** (Average Revenue Per User): MRR / usuarios pagados

**Product**:
- **Items created per user**: Engagement
- **Devices per user**: Cross-platform usage
- **Storage used**: Cuánto usan los usuarios

### Estrategia de Growth

**Phase 1: Product-Led Growth**
- Producto excelente → word of mouth
- Free tier generoso → usuarios prueban sin fricción
- Viralidad natural (cuando user A copia algo y user B lo ve)

**Phase 2: Content Marketing**
- Blog posts: "10 productivity tips with clipboard manager"
- YouTube tutorials
- Twitter presence

**Phase 3: Partnerships**
- Integración con apps populares (Notion, Slack, etc.)
- Listados en directorios (Product Hunt, AlternativeTo)

**Phase 4: Paid Advertising** (cuando hay PMF comprobado)
- Google Ads
- Twitter Ads
- Reddit Ads

---

## Notas Finales

### Priorización de Features

**Must-Have** (Fase 1-2):
- ✅ Auth y usuarios
- ✅ Clipboard sync (texto, imágenes)
- ✅ Tiempo real (WebSocket)
- ✅ Multi-dispositivo (desktop, mobile)
- ✅ Búsqueda básica
- ✅ Offline support (SQLite local)

**Should-Have** (Fase 3):
- ✅ Suscripciones y payments
- ✅ Mobile apps completas (iOS, Android)
- ✅ Tipos de contenido avanzados (code, color, etc.)
- ✅ Favoritos y snippets
- ✅ Búsqueda avanzada

**Nice-to-Have** (Fase 4+):
- ⭐ Compartir clips con otros usuarios
- ⭐ Tags y organización avanzada
- ⭐ End-to-end encryption
- ⭐ Admin dashboard
- ⭐ Team features (enterprise)

### Riesgos y Mitigaciones

**Riesgo**: Costo de infraestructura alto con muchos usuarios
**Mitigación**: 
- Empezar económico (Railway/Supabase)
- Migrar a AWS cuando haya ingresos ($10K+ MRR)
- Optimizaciones de performance agresivas

**Riesgo**: Performance pobre en escala
**Mitigación**:
- Load testing desde el inicio
- Arquitectura escalable (stateless, caching, etc.)
- Monitoring proactivo

**Riesgo**: Sincronización inconsistente (conflictos)
**Mitigación**:
- Conflict resolution bien pensado (last-write-wins inicialmente)
- Testing exhaustivo de edge cases
- Logs detallados para debugging

**Riesgo**: Seguridad (datos sensibles en clipboard)
**Mitigación**:
- Mejores prácticas desde día 1
- Security audits regulares
- End-to-end encryption en Fase 4

**Riesgo**: Competencia (otros clipboard managers)
**Mitigación**:
- Diferenciación: tiempo real, multi-plataforma nativo (Tauri)
- UX superior
- Pricing competitivo

### Próximos Pasos Inmediatos

**1. Validación**:
- [ ] Revisar y aprobar arquitectura completa
- [ ] Feedback sobre decisiones técnicas
- [ ] Priorizar features (confirmar roadmap)

**2. Setup Inicial** (Semana 1):
- [ ] Crear repositorios en GitHub
  - `clipboard-app`
  - `clipboard-backend`
- [ ] Setup estructura de carpetas
- [ ] Configurar `.gitignore`, `.editorconfig`
- [ ] Docker Compose para desarrollo local

**3. Primera Iteración** (Semana 1-2):
- [ ] Backend: Setup Axum + PostgreSQL + primera migration
- [ ] Frontend: Setup Tauri v2 + frontend framework
- [ ] CI básico (GitHub Actions)
- [ ] Hello World funcionando end-to-end

**4. Sprint 1** (Semana 3-4):
- [ ] Autenticación completa (backend + frontend)
- [ ] Primera versión deployada en staging

**5. Continuar según roadmap** (Fase 1 → Fase 2 → ...)

---

## Referencias y Recursos

### Documentación Oficial
- **Tauri v2**: https://v2.tauri.app
- **Axum**: https://docs.rs/axum
- **PostgreSQL**: https://www.postgresql.org/docs
- **Redis**: https://redis.io/docs
- **Bun**: https://bun.sh/docs
- **AWS**: https://docs.aws.amazon.com

### Librerías Rust Recomendadas
- `axum` - Web framework
- `sqlx` - PostgreSQL async driver (compile-time verified queries)
- `redis` - Redis async client
- `tokio` - Async runtime
- `serde` + `serde_json` - Serialization
- `tracing` - Structured logging
- `jsonwebtoken` - JWT
- `bcrypt` - Password hashing
- `uuid` - UUID generation
- `validator` - Input validation
- `aws-sdk-s3` - S3 client
- `image` - Image processing (thumbnails)

### Librerías Frontend (TypeScript) Recomendadas
- `@tanstack/react-query` - Server state management
- `zustand` o `jotai` - Client state
- `axios` - HTTP client
- `zod` - Schema validation
- `date-fns` - Date utilities
- `tailwindcss` - Styling
- `@tauri-apps/api` - Tauri commands

### Herramientas de Desarrollo
- **Rust**: `cargo-watch` (auto-recompilación), `cargo-tarpaulin` (coverage)
- **Database**: `sqlx-cli` (migrations), `pgAdmin` / `Postico` (GUI)
- **API Testing**: Postman, Bruno, Insomnia
- **Load Testing**: k6, Apache JMeter
- **Security**: `cargo-audit`, `trivy`, OWASP ZAP

### Inspiración y Referencias
- **1Password**: Security approach, multi-platform
- **Notion**: Real-time sync, offline support
- **Linear**: Real-time updates, performance
- **Figma**: Collaborative features, WebSocket
- **Raycast**: UX, keyboard shortcuts (desktop app similar)

---

**Documento creado**: Enero 2025  
**Última actualización**: Enero 2025  
**Versión**: 2.0 (Multirepo optimizado)  
**Autor**: Arquitectura para Clipboard Manager Global

---

## Cambios vs v1

**Principales cambios en v2**:
1. ✅ **Estructura Multirepo** (2 repos en vez de monorepo)
2. ✅ **Desktop + Mobile en mismo repo** (clipboard-app)
3. ✅ **Backend + Infrastructure en mismo repo** (clipboard-backend)
4. ✅ **No clipboard-shared repo** (copy-paste de types)
5. ✅ **Estructura más práctica y realista**
6. ✅ **CI/CD simplificado** (un workflow por repo)