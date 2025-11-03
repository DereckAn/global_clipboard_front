# 🗺️ Global Clipboard Manager - Roadmap

Este documento contiene el plan de desarrollo futuro del proyecto, organizado por dificultad y prioridad.

## 🐛 Bugs Actuales

### 🔴 Crítico - Arreglar Primero

- No hay bugs todavia

---

## 📋 Próximas Mejoras

### 🟢 Fase 1: Mejoras Básicas de UI/UX (Fácil)


#### 1.2 Interpretar y Mostrar SVG
- [ ] Detectar cuando el clipboard contiene SVG
- [ ] Crear componente para preview de SVG
- [ ] Añadir tipo de contenido "svg" a la base de datos
- [ ] Guardar SVG como texto en `content_text`
- [ ] Renderizar SVG en el ContentViewer
- **Dificultad**: 🟢 Fácil
- **Tiempo estimado**: 1-2 días
- **Archivos**:
  - `src-tauri/src/clipboard/types.rs` (detección)
  - `src/lib/components/content/ContentViewer.svelte` (renderizado)

#### 1.3 Settings Mejorados
- [ ] **Límite de items guardados**
  - Toggle para activar/desactivar límite
  - Slider para seleccionar cantidad (100, 500, 1000, ilimitado)
  - Auto-eliminar items más antiguos cuando se alcanza el límite

- [ ] **Tiempo de retención**
  - Opciones: 7 días, 30 días, 90 días, 6 meses, 1 año, nunca
  - Tarea programada para limpiar items antiguos

- [ ] **Mostrar uso de memoria**
  - Calcular tamaño de la base de datos
  - Mostrar número de items
  - Mostrar espacio usado en disco
  - Botón para limpiar y optimizar base de datos

- [ ] **Otras configuraciones**
  - Auto-inicio al arrancar el sistema
  - Sonidos de notificación
  - Intervalo de monitoreo del clipboard (actualmente 500ms)

- **Dificultad**: 🟡 Media
- **Tiempo estimado**: 3-4 días
- **Archivos**:
  - `src/routes/settings/+page.svelte`
  - `src-tauri/src/commands/settings.rs`
  - Nueva tarea programada en Rust para limpieza

---

### 🟡 Fase 2: Funcionalidades Medias (Medio)

#### 2.1 Soporte para Imágenes Copiadas
- [ ] Detectar cuando se copia una imagen
- [ ] **Opción A (Local)**: Guardar imagen en disco local
  - Crear carpeta `~/Library/Application Support/clip/images/`
  - Guardar como PNG o formato original
  - Guardar path en `file_url`

- [ ] **Opción B (Cloud - futuro)**: Subir a storage
  - Por ahora usar local, preparar estructura para cloud

- [ ] Crear thumbnail para preview
- [ ] Mostrar preview en sidebar y ContentViewer
- [ ] Permitir copiar imagen de vuelta al clipboard

- **Dificultad**: 🟡 Media
- **Tiempo estimado**: 4-5 días
- **Archivos**:
  - `src-tauri/src/clipboard/monitor.rs`
  - `src-tauri/src/clipboard/types.rs`
  - Nuevo módulo: `src-tauri/src/storage/images.rs`

#### 2.2 UI Responsive (Mobile & Tablet)
- [ ] Crear breakpoints en Tailwind
- [ ] Diseñar layout mobile-first
- [ ] Adaptar sidebar para móvil (drawer o tabs)
- [ ] Adaptar header para móvil (menú hamburguesa)
- [ ] Gestos táctiles (swipe, long-press)
- [ ] Probar en diferentes tamaños de pantalla

- **Dificultad**: 🟡 Media
- **Tiempo estimado**: 5-7 días
- **Archivos**: Todos los componentes en `src/lib/components/`

#### 2.3 Testing Básico
- [ ] **Unit Tests (Frontend)**
  - Configurar Vitest
  - Tests para stores
  - Tests para utilidades

- [ ] **Unit Tests (Backend)**
  - Tests para repository
  - Tests para clipboard detection
  - Tests para color conversion

- [ ] **Integration Tests**
  - Tests para Tauri commands
  - Tests para base de datos

- **Dificultad**: 🟡 Media
- **Tiempo estimado**: 5-7 días
- **Archivos**: Nuevos archivos `*.test.ts` y `*.rs` con `#[cfg(test)]`

---

### 🔴 Fase 3: Sistema de Cuentas y Autenticación (Difícil)

#### 3.1 Integración con Supabase
- [ ] Crear proyecto en Supabase
- [ ] Diseñar schema en Supabase:
  ```sql
  -- users (ya viene con Supabase Auth)

  -- user_profiles
  CREATE TABLE user_profiles (
    id UUID PRIMARY KEY REFERENCES auth.users(id),
    email TEXT,
    subscription_tier TEXT, -- 'free', 'pro'
    subscription_status TEXT, -- 'active', 'cancelled', 'expired'
    subscription_expires_at TIMESTAMP,
    storage_used_bytes BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
  );

  -- clipboard_items_cloud
  CREATE TABLE clipboard_items_cloud (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES auth.users(id),
    content_type TEXT,
    content_text TEXT,
    content_metadata JSONB,
    image_url TEXT, -- URL a Supabase Storage
    file_url TEXT,
    is_favorite BOOLEAN DEFAULT FALSE,
    is_snippet BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    synced BOOLEAN DEFAULT TRUE
  );

  -- Row Level Security (RLS)
  ALTER TABLE clipboard_items_cloud ENABLE ROW LEVEL SECURITY;

  CREATE POLICY "Users can only access their own items"
    ON clipboard_items_cloud
    FOR ALL
    USING (auth.uid() = user_id);
  ```

- [ ] Configurar Supabase Storage para imágenes
- [ ] Instalar Supabase SDK en el frontend
- [ ] Configurar variables de entorno

- **Dificultad**: 🔴 Difícil
- **Tiempo estimado**: 3-4 días
- **Archivos**:
  - Nueva carpeta: `src/lib/supabase/`
  - `.env` para configuración

#### 3.2 OAuth con Supabase
- [ ] Configurar Supabase Auth
- [ ] Implementar login con Google
- [ ] Implementar login con GitHub
- [ ] Implementar login con email/password
- [ ] Crear pantalla de login/registro
- [ ] Crear pantalla de perfil de usuario
- [ ] Implementar logout
- [ ] Manejar tokens y refresh tokens
- [ ] Guardar sesión en localStorage

- **Dificultad**: 🔴 Difícil
- **Tiempo estimado**: 5-7 días
- **Archivos**:
  - `src/routes/auth/+page.svelte` (nueva ruta)
  - `src/routes/profile/+page.svelte` (nueva ruta)
  - `src/lib/stores/auth.svelte.ts` (nuevo store)
  - `src/lib/supabase/auth.ts`

#### 3.3 Sincronización Local ↔ Cloud
- [ ] Decidir estrategia de sync:
  - **Opción A**: Siempre guardar en ambos (local + cloud)
  - **Opción B**: Solo cloud para usuarios Pro
  - **Opción C**: Híbrido (local primero, sync en background)

- [ ] Implementar sync bidireccional
- [ ] Manejar conflictos (last-write-wins o custom)
- [ ] Indicador visual de estado de sync
- [ ] Offline-first: trabajar sin internet
- [ ] Queue de operaciones pendientes
- [ ] Retry automático cuando vuelve la conexión

- **Dificultad**: 🔴 Muy Difícil
- **Tiempo estimado**: 7-10 días
- **Archivos**:
  - `src/lib/stores/sync.svelte.ts` (nuevo)
  - `src/lib/supabase/sync.ts` (nuevo)
  - Actualizar `clipboard.svelte.ts`

---

### 🟣 Fase 4: Monetización (Difícil)

#### 4.1 Sistema de Planes
- [ ] **Plan Free**:
  - Límite de 1000 items
  - Solo almacenamiento local
  - No sync entre dispositivos
  - Anuncios (opcional)

- [ ] **Plan Pro** ($4.99/mes o $49/año):
  - Items ilimitados
  - Sync entre dispositivos
  - 5GB de storage en cloud
  - Sin anuncios
  - Soporte prioritario
  - Búsqueda avanzada
  - Organizacion con carpetas/tags

- [ ] Crear página de precios
- [ ] UI para comparar planes
- [ ] Lógica para verificar límites del plan

- **Dificultad**: 🟡 Media
- **Tiempo estimado**: 3-4 días
- **Archivos**:
  - `src/routes/pricing/+page.svelte` (nuevo)
  - `src/lib/stores/subscription.svelte.ts` (nuevo)

#### 4.2 Integración de Pagos
- [ ] Elegir procesador de pagos:
  - **Stripe** (recomendado - más completo)
  - Paddle
  - LemonSqueezy

- [ ] Configurar Stripe:
  - Crear cuenta de Stripe
  - Configurar productos y precios
  - Instalar Stripe SDK
  - Configurar webhooks

- [ ] Crear flujo de pago:
  - Botón "Upgrade to Pro"
  - Checkout page (Stripe Checkout o custom)
  - Confirmación de pago
  - Activación automática del plan Pro

- [ ] Manejar suscripciones:
  - Activación
  - Renovación automática
  - Cancelación
  - Reembolsos
  - Actualización de método de pago

- [ ] Webhooks de Stripe:
  - `payment_succeeded`
  - `subscription_updated`
  - `subscription_cancelled`

- [ ] Edge Functions en Supabase para webhooks

- **Dificultad**: 🔴 Muy Difícil
- **Tiempo estimado**: 7-10 días
- **Archivos**:
  - `src/lib/stripe/` (nueva carpeta)
  - Edge Functions en Supabase
  - `src/routes/checkout/+page.svelte` (nuevo)

---

### 🟣 Fase 5: Funcionalidades Avanzadas (Muy Difícil)

#### 5.1 Búsqueda Avanzada (Plan Pro)
- [ ] Filtros combinados
- [ ] Búsqueda por fecha exacta
- [ ] Búsqueda por rango de fechas
- [ ] Búsqueda por tipo de contenido múltiple
- [ ] Búsqueda con operadores (AND, OR, NOT)
- [ ] Búsqueda en metadata
- [ ] Guardar búsquedas frecuentes
- [ ] Full-text search con ranking

- **Dificultad**: 🔴 Difícil
- **Tiempo estimado**: 4-5 días

#### 5.2 Organización con Carpetas/Tags
- [ ] Crear sistema de carpetas
- [ ] Crear sistema de tags
- [ ] Drag & drop para mover items
- [ ] Multi-select para operaciones en batch
- [ ] Colores personalizados para carpetas
- [ ] Smart folders (filtros automáticos)

- **Dificultad**: 🔴 Difícil
- **Tiempo estimado**: 7-10 días

#### 5.3 Colaboración (Futuro lejano)
- [ ] Compartir items con otros usuarios
- [ ] Espacios de trabajo en equipo
- [ ] Permisos (view, edit, admin)
- [ ] Comentarios en items
- [ ] Historial de cambios

- **Dificultad**: 🔴 Muy Difícil
- **Tiempo estimado**: 2-3 semanas

#### 5.4 API Pública
- [ ] Diseñar REST API
- [ ] Autenticación con API keys
- [ ] Rate limiting
- [ ] Documentación con OpenAPI
- [ ] SDKs para lenguajes populares

- **Dificultad**: 🔴 Muy Difícil
- **Tiempo estimado**: 2-3 semanas

#### 5.5 Extensiones del Navegador
- [ ] Chrome/Edge extension
- [ ] Firefox extension
- [ ] Safari extension
- [ ] Sync con la app de escritorio

- **Dificultad**: 🔴 Muy Difícil
- **Tiempo estimado**: 3-4 semanas

#### 5.6 Apps Móviles Nativas
- [ ] iOS app con Tauri Mobile
- [ ] Android app con Tauri Mobile
- [ ] Compartir código con desktop
- [ ] Notificaciones push
- [ ] Widget de home screen

- **Dificultad**: 🔴 Muy Difícil
- **Tiempo estimado**: 4-6 semanas

---

## 📊 Resumen por Prioridad

### 🔥 Prioridad Alta (Hacer primero)
1. 🐛 Arreglar search bar
2. 🎨 Mejorar diseño visual
3. 📸 Soporte para imágenes
4. ⚙️ Settings mejorados
5. 📱 UI responsive

### ⭐ Prioridad Media (Hacer después)
6. 🖼️ Soporte para SVG
7. ✅ Testing básico
8. 🔐 Sistema de cuentas
9. 💰 Monetización

### 🌟 Prioridad Baja (Futuro)
10. 🔍 Búsqueda avanzada
11. 📁 Carpetas y tags
12. 👥 Colaboración
13. 🔌 API pública
14. 🌐 Extensiones del navegador
15. 📱 Apps móviles

---

## 🎯 Roadmap Visual

```
v1.0 (ACTUAL) ✅
├─ Clipboard monitoring
├─ Content detection
├─ Local storage
├─ Pagination
├─ Dynamic hotkeys
└─ Search básico

v1.1 (1-2 meses) 🔄
├─ 🐛 Arreglar search bar
├─ 🎨 Diseño mejorado
├─ 📸 Soporte imágenes
├─ 🖼️ Soporte SVG
├─ ⚙️ Settings avanzados
└─ ✅ Testing básico

v1.2 (2-3 meses) 📱
├─ 📱 UI responsive
├─ 🔐 Cuentas y autenticación
├─ ☁️ Sync con Supabase
└─ 💾 Cloud storage

v2.0 (4-6 meses) 💰
├─ 💰 Sistema de planes
├─ 💳 Pagos con Stripe
├─ 🔍 Búsqueda avanzada (Pro)
├─ 📁 Carpetas y tags (Pro)
└─ 📊 Analytics de uso

v3.0 (6-12 meses) 🚀
├─ 👥 Colaboración
├─ 🔌 API pública
├─ 🌐 Extensiones del navegador
├─ 📱 Apps móviles
└─ 🤖 Features con IA
```

---

## 💡 Ideas Adicionales (Brainstorming)

### Features de IA (Futuro)
- [ ] Auto-categorización de contenido
- [ ] Sugerencias inteligentes
- [ ] Resumen automático de textos largos
- [ ] Traducción automática
- [ ] Extracción de información (emails, teléfonos, etc.)
- [ ] Generación de código a partir de snippets

### Integraciones
- [ ] Zapier/Make/n8n
- [ ] Notion
- [ ] Slack
- [ ] Discord
- [ ] VS Code extension
- [ ] Obsidian plugin

### Features de Productividad
- [ ] Templates para snippets
- [ ] Variables en snippets
- [ ] Macros personalizables
- [ ] Atajos de teclado avanzados
- [ ] Modo focus (ocultar todo excepto lo necesario)

### Seguridad
- [ ] Encriptación end-to-end
- [ ] Vault para items sensibles
- [ ] Auto-eliminar después de tiempo
- [ ] Modo incógnito (no guardar)
- [ ] Blacklist de apps (no copiar de ciertas apps)

---

## 📝 Notas de Implementación

### Stack Tecnológico Recomendado

**Backend adicional (si es necesario)**:
- Supabase (Auth + Database + Storage)
- Edge Functions para webhooks
- PostgreSQL con RLS

**Pagos**:
- Stripe (recomendado)
- Stripe Checkout para UI de pago
- Webhooks para manejar eventos

**Testing**:
- Vitest (frontend)
- Rust cargo test (backend)
- Playwright (E2E)

**CI/CD**:
- GitHub Actions
- Automated releases
- Automated testing

---

## 🤔 Decisiones Pendientes

### 1. Estrategia de Sync
- [ ] ¿Local-first o Cloud-first?
- [ ] ¿Qué hacer con conflictos?
- [ ] ¿Cómo manejar items grandes?

### 2. Límites del Plan Free
- [ ] ¿1000 items es suficiente?
- [ ] ¿Poner límite de tiempo (ej. 30 días)?
- [ ] ¿Mostrar anuncios o no?

### 3. Precios
- [ ] ¿$4.99/mes es el precio correcto?
- [ ] ¿Ofrecer plan anual con descuento?
- [ ] ¿Plan lifetime?
- [ ] ¿Plan para equipos?

### 4. Storage
- [ ] ¿Cuánto storage dar en plan Pro? (5GB, 10GB, ilimitado?)
- [ ] ¿Cobrar extra por storage adicional?
- [ ] ¿Comprimir imágenes automáticamente?

---

## 📞 Recursos y Referencias

### Supabase
- [Docs](https://supabase.com/docs)
- [Auth Docs](https://supabase.com/docs/guides/auth)
- [Storage Docs](https://supabase.com/docs/guides/storage)
- [Edge Functions](https://supabase.com/docs/guides/functions)

### Stripe
- [Docs](https://stripe.com/docs)
- [Checkout](https://stripe.com/docs/payments/checkout)
- [Webhooks](https://stripe.com/docs/webhooks)
- [Subscriptions](https://stripe.com/docs/billing/subscriptions/overview)

### Tauri
- [Mobile Docs](https://v2.tauri.app/start/migrate/from-tauri-1/)

---

**Última actualización**: 2025-01-30
**Versión actual**: v1.0
**Próxima versión**: v1.1 (focus en UX y fixes)
