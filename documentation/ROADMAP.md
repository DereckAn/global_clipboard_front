# 🗺️ Global Clipboard Manager – Roadmap

Guía priorizada del desarrollo. Cada fase agrupa iniciativas por impacto y esfuerzo. Los checkboxes indican el estado actual y sirven como checklist cuando se implementen los cambios.

---

## 🐛 Estado de Bugs

- 🔴 **Críticos**: ninguno reportado.
- 🟠 **Pendientes**: documentar aquí cualquier regresión detectada en testing manual.

---

## 🟢 Fase 1 · Experiencia Básica
Enfoque en consolidar la base de la aplicación (UI consistente y configuración clara).  
**Estimado total:** ~4 días · **Dificultad:** 🟡 media.

### 1.1 Settings consolidados ✅
- Límite de items, retención temporal y métricas de uso ya operativos.
- Próximo mantenimiento: refinar tooltips y documentación inline.

### 1.2 Monitoreo y feedback ✅
- Migrado a eventos nativos (`clipboard-master`) para evitar polling.
- Notificaciones Tray y logging depurado.

### 1.3 Roadmap de UI inmediata
- [ ] **Settings en pestañas**  
  - Distribuir opciones en categorías (“General”, “Limpieza”, “Integraciones”).  
  - `src/routes/settings/+page.svelte`.
- [ ] **Panel de extensiones**  
  - Nueva pestaña “Laboratorio” con acciones como “Descargar OCR”, packs de emojis/ASCII, etc.  
  - Preparar hook para habilitar funcionalidades bajo feature flags.
- [ ] **Gestión visual de imágenes**  
  - En el sidebar, eliminar assets asociados al borrar un item (`src-tauri/src/clipboard/image_handler.rs`).  
  - Mostrar confirmación ligera cuando se borra el archivo del disco.

### 1.4 Improve the icon. ✅
- [ ] ** Change the icon color when the background change. 
  - Cambiar de negro a blanco el fill porque a veces no se nota. 
  - El tray icon. A veces es blanco y con el fondo blanco es muy poco notorio. 

### 1.5 files como .log extraer el texto. ✅

### 1.6 POner linter y formatear el codigo en ci/cd
### 1.7 preview en otros OS (windows u linux)
### 1.8 en el drop down filter que buscque en toda la base de datos y luego ahga pagination. 
### 1.9 Integracion con red lan para mandar files entre diferentes dispositivos y conectarse por medio de codigo qr 
### 1.10 Clipboard para terminal. 
---

## 🟡 Fase 2 · Funcionalidades Medias
Amplía capacidades clave antes de sincronización en la nube.  
**Estimado total:** ~8‑10 días · **Dificultad:** 🟡 media.

- ### 2.1 Ecosistema de imágenes
  - ✅ Copia desde Finder y navegadores con hash + thumbnails.
  - ✅ Rebote de imágenes (copy back) sin duplicados.
  - ✅ Validar formatos TIFF/HEIC de macOS (`Cmd+Shift+4/5`) y convertir a PNG para vista previa.
  - ✅ Metadata enriquecida (`is_screenshot`, `original_extension`, rutas locales normalizadas).
- [ ] **Soporte de capturas de pantalla instantáneas ampliado**
  - Identificar capturas en Windows/Linux y etiquetar metadata igualmente.
  - Añadir configuración para personalizar formato de salida (PNG/JPEG).
- [ ] **Sincronizar miniaturas y datos**  
  - ✅ Regenerar miniaturas locales cuando falten (`ensure_thumbnail` + fallback en sidebar).  
  - ✅ Eliminar archivos/miniaturas asociados al borrar items o por limpieza automática.
  - [ ] Resolver carga de `thumbnail_path` para elementos sincronizados entre dispositivos.  
  - [ ] Gestionar subida/descarga de thumbnails en el flujo cloud.  
  - `src/lib/components/sidebar/SidebarItem.svelte`, `src/lib/tauri/commands.ts`.

### 2.2 Refinamiento visual
- [ ] Layout responsivo (breakpoints Tailwind, sidebar colapsable, menú móvil).  
  - Revisar todos los componentes en `src/lib/components/**`.
- [ ] Gestos táctiles básicos (swipe para marcar favorito, long-press para borrar).  
  - Se puede prototipar con `pointer events` + stores.

### 2.3 Automatización y tests
- [ ] **Frontend**: configurar Vitest + pruebas de stores/utilidades (`src/lib/stores`, `src/lib/utils`).  
- [ ] **Backend**: ampliar integración Rust (clipboard repo, comandos).  
- [ ] **End-to-end ligero**: script Bun que use Tauri API para validar flujo copia → render.

---

## 🔴 Fase 3 · Cuentas y Cloud
Habilita sincronización entre dispositivos y modelos de negocio.  
**Estimado total:** 15‑20 días · **Dificultad:** 🔴 alta.

### 3.1 Supabase – infraestructura
- [ ] Aprovisionar proyecto Supabase y documentar `.env` requerido.
- [ ] Implementar tablas (`user_profiles`, `clipboard_items_cloud`) y políticas RLS para seguridad.  
  - Crear módulo `src/lib/supabase/schema.sql` como referencia.
- [ ] Configurar Supabase Storage (bucket para imágenes) y SDK en frontend (`src/lib/supabase/client.ts`).

### 3.2 Autenticación
- [ ] Flujo OAuth (Google, GitHub) y email/password mediante Supabase Auth.  
  - Rutas nuevas: `src/routes/auth/+page.svelte`, `src/routes/profile/+page.svelte`.  
  - Stores: `src/lib/stores/auth.svelte.ts`.
- [ ] Persistencia de sesión, refresco de tokens y cierre seguro.

### 3.3 Sincronización local ↔ cloud
- [ ] Definir estrategia (local-first con sync en background recomendado).  
- [ ] Resolver conflictos (timestamp/last-write) y estados offline.  
- [ ] Diseñar colas de operaciones + reintentos (`src/lib/stores/sync.svelte.ts`, `src/lib/supabase/sync.ts`).  
- [ ] Indicadores UI: iconos de estado en sidebar, mensajes de “sin conexión”.

---

## 🔵 Ideas Futuras / Backlog
- Motor de OCR opcional (descarga bajo demanda desde Settings → Laboratorio).  
- Búsqueda semántica (vector DB) para mejorar resultados.  
- Integración con servicios de traducción.  
- Packs de emojis y arte ASCII para pegado rápido.  
- Atajos específicos por plataforma (ej. hooks Win+V en Windows aún por investigar limitaciones OS).

---

## 📌 Cómo usar este roadmap
- Cada ítem debe generar tickets/tareas una vez que se inicie su desarrollo.  
- Actualiza el estado (✅ / ☐) tras completar pruebas y revisión.  
- Si se detectan bugs o bloqueos, agrega notas en la sección correspondiente con fecha y responsable.

Mantén este documento alineado con la realidad del repositorio; sirve como referencia viva para colaboradores y para priorizar las próximas iteraciones.

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


en el cd/ci 
Testear que funciona el auto start en cada os
