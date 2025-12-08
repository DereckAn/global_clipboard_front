#!/bin/bash

# Script para crear releases
# - Detecta versión actual de Cargo.toml
# - Pregunta si hay cambios sin commitear y hace commit + push
# - Pregunta si crear tag para release principal (app)
# - Pregunta si crear tag para screenshot-helper binary

set -e

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Release Script - Global Clipboard${NC}"
echo ""

# Obtener la versión actual de Cargo.toml
CURRENT_VERSION=$(grep '^version = ' src-tauri/Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}📦 Versión actual en Cargo.toml: ${GREEN}v${CURRENT_VERSION}${NC}"
echo ""

# Obtener rama actual
CURRENT_BRANCH=$(git branch --show-current)
echo -e "${BLUE}🌿 Rama actual: ${GREEN}${CURRENT_BRANCH}${NC}"
echo ""

# ============================================
# PASO 1: Verificar cambios sin commitear
# ============================================

if [[ -n $(git status -s) ]]; then
  echo -e "${YELLOW}⚠️  Hay cambios sin commitear:${NC}"
  echo ""
  git status -s
  echo ""
  
  read -p "¿Quieres hacer commit de estos cambios? (y/n) " -n 1 -r
  echo
  
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Pedir mensaje del commit
    echo ""
    read -p "Escribe el mensaje del commit: " COMMIT_MSG
    
    if [ -z "$COMMIT_MSG" ]; then
      echo -e "${RED}❌ Error: El mensaje del commit no puede estar vacío${NC}"
      exit 1
    fi
    
    # Hacer commit
    echo -e "${GREEN}💾 Haciendo commit...${NC}"
    git add .
    git commit -m "$COMMIT_MSG"
    
    # Preguntar si hacer push
    read -p "¿Hacer push a ${CURRENT_BRANCH}? (y/n) " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      echo -e "${GREEN}⬆️  Pushing...${NC}"
      git push origin "$CURRENT_BRANCH"
      echo -e "${GREEN}✅ Push completado${NC}"
    fi
    
    echo ""
  else
    echo -e "${YELLOW}⚠️  Continuando sin hacer commit de los cambios...${NC}"
    echo ""
  fi
else
  echo -e "${GREEN}✅ No hay cambios sin commitear${NC}"
  echo ""
fi

# ============================================
# PASO 2: Preguntar sobre tags
# ============================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📋 OPCIONES DE RELEASE${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "1. Tag para release principal (app) - Dispara build.yml"
echo "   Formato: v1.0.1"
echo ""
echo "2. Tag para screenshot-helper binary - Dispara helper-release.yml"
echo "   Formato: helper-v1.0.0"
echo ""

# ============================================
# PASO 2a: Tag para release principal
# ============================================

read -p "¿Crear tag para release principal de la app v${CURRENT_VERSION}? (y/n) " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
  APP_TAG="v${CURRENT_VERSION}"
  
  # Verificar si el tag ya existe
  if git tag -l "$APP_TAG" | grep -q "$APP_TAG"; then
    echo -e "${YELLOW}⚠️  El tag ${APP_TAG} ya existe${NC}"
    read -p "¿Quieres eliminarlo y recrearlo? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      git tag -d "$APP_TAG"
      git push origin --delete "$APP_TAG" 2>/dev/null || true
    else
      echo -e "${YELLOW}Saltando tag de app...${NC}"
      APP_TAG=""
    fi
  fi
  
  if [ -n "$APP_TAG" ]; then
    echo -e "${GREEN}🏷️  Creando tag ${APP_TAG}${NC}"
    git tag -a "$APP_TAG" -m "Release ${CURRENT_VERSION}"
    
    read -p "¿Hacer push del tag ${APP_TAG}? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      git push origin "$APP_TAG"
      echo -e "${GREEN}✅ Tag ${APP_TAG} pushed - workflow build.yml iniciado${NC}"
    fi
  fi
fi

echo ""

# ============================================
# PASO 2b: Tag para screenshot-helper
# ============================================

read -p "¿Crear tag para screenshot-helper binary? (y/n) " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
  # Sugerir versión
  echo ""
  read -p "Versión para helper (ej: 1.0.0): " HELPER_VERSION
  
  if [ -z "$HELPER_VERSION" ]; then
    echo -e "${RED}❌ Error: La versión no puede estar vacía${NC}"
  else
    HELPER_TAG="helper-v${HELPER_VERSION}"
    
    # Verificar si el tag ya existe
    if git tag -l "$HELPER_TAG" | grep -q "$HELPER_TAG"; then
      echo -e "${YELLOW}⚠️  El tag ${HELPER_TAG} ya existe${NC}"
      read -p "¿Quieres eliminarlo y recrearlo? (y/n) " -n 1 -r
      echo
      if [[ $REPLY =~ ^[Yy]$ ]]; then
        git tag -d "$HELPER_TAG"
        git push origin --delete "$HELPER_TAG" 2>/dev/null || true
      else
        echo -e "${YELLOW}Saltando tag de helper...${NC}"
        HELPER_TAG=""
      fi
    fi
    
    if [ -n "$HELPER_TAG" ]; then
      echo -e "${GREEN}🏷️  Creando tag ${HELPER_TAG}${NC}"
      git tag -a "$HELPER_TAG" -m "Screenshot Helper Release ${HELPER_VERSION}"
      
      read -p "¿Hacer push del tag ${HELPER_TAG}? (y/n) " -n 1 -r
      echo
      if [[ $REPLY =~ ^[Yy]$ ]]; then
        git push origin "$HELPER_TAG"
        echo -e "${GREEN}✅ Tag ${HELPER_TAG} pushed - workflow helper-release.yml iniciado${NC}"
      fi
    fi
  fi
fi

echo ""

# ============================================
# RESUMEN FINAL
# ============================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}🎉 ¡Proceso completado!${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Mostrar link a actions
REPO_URL=$(git config --get remote.origin.url | sed 's/\.git$//' | sed 's/git@github.com:/https:\/\/github.com\//')
echo -e "${BLUE}🔗 Seguir progreso en: ${NC}${REPO_URL}/actions"
echo ""

# Listar tags recientes
echo -e "${BLUE}📋 Tags recientes:${NC}"
git tag -l --sort=-creatordate | head -5
