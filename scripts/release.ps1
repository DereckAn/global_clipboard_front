# Script para crear releases (PowerShell - Windows)
# - Detecta version actual de Cargo.toml
# - Pregunta si hay cambios sin commitear y hace commit + push
# - Pregunta si crear tag para release principal (app)
# - Pregunta si crear tag para screenshot-helper binary

$ErrorActionPreference = "Stop"

# Colores para output
function Write-Blue { param($msg) Write-Host $msg -ForegroundColor Cyan }
function Write-Green { param($msg) Write-Host $msg -ForegroundColor Green }
function Write-Yellow { param($msg) Write-Host $msg -ForegroundColor Yellow }
function Write-Red { param($msg) Write-Host $msg -ForegroundColor Red }

Write-Blue "Release Script - Global Clipboard"
Write-Host ""

# Obtener la version actual de Cargo.toml
$cargoContent = Get-Content "src-tauri/Cargo.toml" -Raw
if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
    $CURRENT_VERSION = $matches[1]
} else {
    Write-Red "Error: No se pudo leer la version de Cargo.toml"
    exit 1
}

Write-Host "Version actual en Cargo.toml: " -NoNewline -ForegroundColor Cyan
Write-Green "v$CURRENT_VERSION"
Write-Host ""

# Obtener rama actual
$CURRENT_BRANCH = git branch --show-current
Write-Host "Rama actual: " -NoNewline -ForegroundColor Cyan
Write-Green $CURRENT_BRANCH
Write-Host ""

# ============================================
# PASO 1: Verificar cambios sin commitear
# ============================================

$gitStatus = git status -s
if ($gitStatus) {
    Write-Yellow "Hay cambios sin commitear:"
    Write-Host ""
    git status -s
    Write-Host ""
    
    $reply = Read-Host "Quieres hacer commit de estos cambios? (y/n)"
    
    if ($reply -match '^[Yy]$') {
        # Pedir mensaje del commit
        Write-Host ""
        $COMMIT_MSG = Read-Host "Escribe el mensaje del commit"
        
        if ([string]::IsNullOrWhiteSpace($COMMIT_MSG)) {
            Write-Red "Error: El mensaje del commit no puede estar vacio"
            exit 1
        }
        
        # Hacer commit
        Write-Green "Haciendo commit..."
        git add .
        git commit -m $COMMIT_MSG
        
        # Preguntar si hacer push
        $pushReply = Read-Host "Hacer push a $CURRENT_BRANCH ? (y/n)"
        
        if ($pushReply -match '^[Yy]$') {
            Write-Green "Pushing..."
            git push origin $CURRENT_BRANCH
            Write-Green "Push completado"
        }
        
        Write-Host ""
    } else {
        Write-Yellow "Continuando sin hacer commit de los cambios..."
        Write-Host ""
    }
} else {
    Write-Green "No hay cambios sin commitear"
    Write-Host ""
}

# ============================================
# PASO 2: Preguntar sobre tags
# ============================================

Write-Blue "========================================"
Write-Blue "OPCIONES DE RELEASE"
Write-Blue "========================================"
Write-Host ""
Write-Host "1. Tag para release principal (app) - Dispara build.yml"
Write-Host "   Formato: v1.0.1"
Write-Host ""
Write-Host "2. Tag para screenshot-helper binary - Dispara helper-release.yml"
Write-Host "   Formato: helper-v1.0.0"
Write-Host ""

# ============================================
# PASO 2a: Tag para release principal
# ============================================

$appTagReply = Read-Host "Crear tag para release principal de la app v$CURRENT_VERSION ? (y/n)"

if ($appTagReply -match '^[Yy]$') {
    $APP_TAG = "v$CURRENT_VERSION"
    
    # Verificar si el tag ya existe
    $existingTag = git tag -l $APP_TAG
    if ($existingTag) {
        Write-Yellow "El tag $APP_TAG ya existe"
        $deleteReply = Read-Host "Quieres eliminarlo y recrearlo? (y/n)"
        
        if ($deleteReply -match '^[Yy]$') {
            git tag -d $APP_TAG
            git push origin --delete $APP_TAG 2>$null
        } else {
            Write-Yellow "Saltando tag de app..."
            $APP_TAG = $null
        }
    }
    
    if ($APP_TAG) {
        Write-Green "Creando tag $APP_TAG"
        git tag -a $APP_TAG -m "Release $CURRENT_VERSION"
        
        $pushTagReply = Read-Host "Hacer push del tag $APP_TAG ? (y/n)"
        
        if ($pushTagReply -match '^[Yy]$') {
            git push origin $APP_TAG
            Write-Green "Tag $APP_TAG pushed - workflow build.yml iniciado"
        }
    }
}

Write-Host ""

# ============================================
# PASO 2b: Tag para screenshot-helper
# ============================================

$helperReply = Read-Host "Crear tag para screenshot-helper binary? (y/n)"

if ($helperReply -match '^[Yy]$') {
    # Sugerir version
    Write-Host ""
    $HELPER_VERSION = Read-Host "Version para helper (ej: 1.0.0)"
    
    if ([string]::IsNullOrWhiteSpace($HELPER_VERSION)) {
        Write-Red "Error: La version no puede estar vacia"
    } else {
        $HELPER_TAG = "helper-v$HELPER_VERSION"
        
        # Verificar si el tag ya existe
        $existingHelperTag = git tag -l $HELPER_TAG
        if ($existingHelperTag) {
            Write-Yellow "El tag $HELPER_TAG ya existe"
            $deleteHelperReply = Read-Host "Quieres eliminarlo y recrearlo? (y/n)"
            
            if ($deleteHelperReply -match '^[Yy]$') {
                git tag -d $HELPER_TAG
                git push origin --delete $HELPER_TAG 2>$null
            } else {
                Write-Yellow "Saltando tag de helper..."
                $HELPER_TAG = $null
            }
        }
        
        if ($HELPER_TAG) {
            Write-Green "Creando tag $HELPER_TAG"
            git tag -a $HELPER_TAG -m "Screenshot Helper Release $HELPER_VERSION"
            
            $pushHelperReply = Read-Host "Hacer push del tag $HELPER_TAG ? (y/n)"
            
            if ($pushHelperReply -match '^[Yy]$') {
                git push origin $HELPER_TAG
                Write-Green "Tag $HELPER_TAG pushed - workflow helper-release.yml iniciado"
            }
        }
    }
}

Write-Host ""

# ============================================
# RESUMEN FINAL
# ============================================

Write-Blue "========================================"
Write-Green "Proceso completado!"
Write-Blue "========================================"
Write-Host ""

# Mostrar link a actions
$REPO_URL = "https://github.com/DereckAn/global_clipboard_front"
Write-Host "Seguir progreso en: " -NoNewline -ForegroundColor Cyan
Write-Host "$REPO_URL/actions"
Write-Host ""

# Listar tags recientes
Write-Blue "Tags recientes:"
git tag -l --sort=-creatordate | Select-Object -First 5
