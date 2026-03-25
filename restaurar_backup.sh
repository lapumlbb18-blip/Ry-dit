#!/bin/bash
# ============================================================================
# RESTAURAR BACKUP - En caso de problemas
# ============================================================================
# Uso: ./restaurar_backup.sh [nombre_backup]
# Ejemplo: ./restaurar_backup.sh antes_lazos
# ============================================================================

set -e

# Colores
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}  🔄 RESTAURAR BACKUP - RyDit Engine${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""

# Verificar argumentos
if [ -z "$1" ]; then
    echo -e "${RED}Error: Debes especificar el nombre del backup${NC}"
    echo ""
    echo "Backups disponibles:"
    ls -1 backup_seguro/
    echo ""
    exit 1
fi

BACKUP_NAME="$1"
BACKUP_PATH="backup_seguro/$BACKUP_NAME"

# Verificar que existe el backup
if [ ! -d "$BACKUP_PATH" ]; then
    echo -e "${RED}Error: El backup '$BACKUP_NAME' no existe${NC}"
    echo ""
    echo "Backups disponibles:"
    ls -1 backup_seguro/
    exit 1
fi

echo -e "${GREEN}[1/3]${NC} Verificando backup: $BACKUP_NAME..."

# Verificar archivos críticos
if [ ! -f "$BACKUP_PATH/main.rs" ]; then
    echo -e "${RED}  ❌ Error: main.rs no existe en el backup${NC}"
    exit 1
fi
echo "  ✅ main.rs encontrado"

# Confirmación
echo ""
echo -e "${YELLOW}¿Estás seguro de restaurar este backup?${NC}"
echo "  Esto sobrescribirá los archivos actuales."
echo ""
read -p "Presiona ENTER para continuar o Ctrl+C para cancelar..."

# Restaurar archivos
echo -e "${GREEN}[2/3]${NC} Restaurando archivos..."

cp "$BACKUP_PATH/main.rs" "crates/rydit-rs/src/main.rs"
echo "  ✅ main.rs restaurado"

if [ -f "$BACKUP_PATH/Cargo.toml" ]; then
    cp "$BACKUP_PATH/Cargo.toml" "crates/rydit-rs/Cargo.toml"
    echo "  ✅ Cargo.toml restaurado"
fi

if [ -f "$BACKUP_PATH/eval/mod.rs" ]; then
    cp "$BACKUP_PATH/eval/mod.rs" "crates/rydit-rs/src/eval/mod.rs"
    echo "  ✅ eval/mod.rs restaurado"
fi

if [ -f "$BACKUP_PATH/tests/mod.rs" ]; then
    cp "$BACKUP_PATH/tests/mod.rs" "crates/rydit-rs/src/tests/mod.rs"
    echo "  ✅ tests/mod.rs restaurado"
fi

# Verificar compilación
echo -e "${GREEN}[3/3]${NC} Verificando compilación..."
if cargo check -p rydit-rs > /dev/null 2>&1; then
    echo "  ✅ Compilación exitosa"
else
    echo -e "${RED}  ⚠️  Advertencia: La compilación falló (esto es normal si hay cambios a mitad)${NC}"
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  ✅ RESTAURACIÓN COMPLETADA${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Archivos restaurados desde: $BACKUP_NAME"
echo ""
echo "Para verificar:"
echo "  cargo check -p rydit-rs"
echo ""
