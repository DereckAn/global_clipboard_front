#!/bin/bash

# Script para crear un release automático
# Uso: ./release.sh <version>
# Ejemplo: ./release.sh 1.0.0

set -e

if [ -z "$1" ]; then
  echo "❌ Error: Debes proporcionar una versión"
  echo "Uso: ./release.sh <version>"
  echo "Ejemplo: ./release.sh 0.1.0"
  exit 1
fi

VERSION=$1
TAG="v${VERSION}"

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Iniciando proceso de release v${VERSION}${NC}"

# Verificar que estamos en main
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "app" ]; then
  echo -e "${YELLOW}⚠️  No estás en la rama main o app (estás en: ${CURRENT_BRANCH})${NC}"
  read -p "¿Continuar de todos modos? (y/n) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
  fi
fi

# Verificar que no hay cambios sin commitear
if [[ -n $(git status -s) ]]; then
  echo -e "${YELLOW}⚠️  Tienes cambios sin commitear${NC}"
  git status -s
  read -p "¿Continuar de todos modos? (y/n) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
  fi
fi

# Actualizar la versión en tauri.conf.json
echo -e "${GREEN}📝 Actualizando versión en tauri.conf.json${NC}"
if command -v jq &> /dev/null; then
  jq --arg version "$VERSION" '.version = $version' src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp
  mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json
else
  echo -e "${YELLOW}⚠️  jq no está instalado. Actualiza manualmente la versión en src-tauri/tauri.conf.json${NC}"
  read -p "Presiona Enter cuando hayas actualizado la versión..." -r
fi

# Actualizar la versión en package.json (si existe)
if [ -f "package.json" ]; then
  echo -e "${GREEN}📝 Actualizando versión en package.json${NC}"
  if command -v jq &> /dev/null; then
    jq --arg version "$VERSION" '.version = $version' package.json > package.json.tmp
    mv package.json.tmp package.json
  else
    echo -e "${YELLOW}⚠️  Actualiza manualmente la versión en package.json${NC}"
  fi
fi

# Actualizar Cargo.toml (si existe)
if [ -f "src-tauri/Cargo.toml" ]; then
  echo -e "${GREEN}📝 Actualizando versión en Cargo.toml${NC}"
  if command -v sed &> /dev/null; then
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" src-tauri/Cargo.toml
    rm -f src-tauri/Cargo.toml.bak
  else
    echo -e "${YELLOW}⚠️  Actualiza manualmente la versión en src-tauri/Cargo.toml${NC}"
  fi
fi

# Mostrar cambios
echo -e "${BLUE}📋 Cambios realizados:${NC}"
git diff src-tauri/tauri.conf.json src-tauri/Cargo.toml package.json 2>/dev/null || true

# Confirmar
read -p "¿Crear commit y tag v${VERSION}? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo "❌ Cancelado"
  exit 1
fi

# Crear commit
echo -e "${GREEN}💾 Creando commit${NC}"
git add src-tauri/tauri.conf.json src-tauri/Cargo.toml package.json 2>/dev/null || true
git commit -m "chore: bump version to ${VERSION}" || echo "No hay cambios para commitear"

# Crear tag
echo -e "${GREEN}🏷️  Creando tag ${TAG}${NC}"
git tag -a "$TAG" -m "Release ${VERSION}"

# Push
echo -e "${BLUE}📤 ¿Hacer push del commit y tag a GitHub?${NC}"
echo "Esto disparará el workflow de build y release automáticamente"
read -p "¿Continuar? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  echo -e "${GREEN}⬆️  Pushing commit...${NC}"
  git push origin "$CURRENT_BRANCH"
  
  echo -e "${GREEN}⬆️  Pushing tag...${NC}"
  git push origin "$TAG"
  
  echo -e "${GREEN}✅ ¡Release iniciado!${NC}"
  echo -e "${BLUE}🔗 Puedes seguir el progreso en:${NC}"
  REPO_URL=$(git config --get remote.origin.url | sed 's/\.git$//')
  REPO_URL=$(echo "$REPO_URL" | sed 's/git@github.com:/https:\/\/github.com\//')
  echo "${REPO_URL}/actions"
else
  echo -e "${YELLOW}⚠️  No se hizo push. Puedes hacerlo manualmente con:${NC}"
  echo "  git push origin $CURRENT_BRANCH"
  echo "  git push origin $TAG"
fi

echo -e "${GREEN}🎉 ¡Proceso completado!${NC}"
