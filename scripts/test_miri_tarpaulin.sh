#!/bin/bash
# 🛡️ RyDit - Script de Testing con MIRI y TARPAULIN
# 
# Uso: ./scripts/test_miri_tarpaulin.sh [miri|tarpaulin|all] [crate_name]
#
# Ejemplos:
#   ./scripts/test_miri_tarpaulin.sh miri rydit-lexer
#   ./scripts/test_miri_tarpaulin.sh tarpaulin rydit-parser
#   ./scripts/test_miri_tarpaulin.sh all

set -e

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Directorio del proyecto
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

# Crates seguros
SAFE_CRATES=("rydit-lexer" "rydit-parser" "rydit-vm" "rydit-stream")

# Función de ayuda
usage() {
    echo -e "${BLUE}🛡️ RyDit - Testing con MIRI y TARPAULIN${NC}"
    echo ""
    echo "Uso: $0 [comando] [crate_name]"
    echo ""
    echo "Comandos:"
    echo "  miri       - Ejecutar tests con MIRI (detecta undefined behavior)"
    echo "  tarpaulin  - Ejecutar coverage con TARPAULIN (genera reporte HTML)"
    echo "  all        - Ejecutar ambos (miri + tarpaulin)"
    echo "  baseline   - Ejecutar tests normales (baseline)"
    echo ""
    echo "Crates disponibles:"
    echo "  rydit-lexer    (21 tests)"
    echo "  rydit-parser   (24 tests)"
    echo "  rydit-vm       (19 tests)"
    echo "  rydit-stream   (17 tests)"
    echo "  all            - Todos los crates seguros"
    echo ""
    echo "Ejemplos:"
    echo "  $0 miri rydit-lexer"
    echo "  $0 tarpaulin rydit-parser"
    echo "  $0 all"
    echo "  $0 baseline all"
    exit 1
}

# Verificar dependencias
check_dependencies() {
    local tool=$1
    
    case $tool in
        "miri")
            if ! rustup component list --installed | grep -q "miri"; then
                echo -e "${YELLOW}Instalando MIRI...${NC}"
                rustup component add miri
            fi
            ;;
        "tarpaulin")
            if ! command -v cargo-tarpaulin &> /dev/null; then
                echo -e "${YELLOW}Instalando cargo-tarpaulin...${NC}"
                cargo install cargo-tarpaulin
            fi
            ;;
    esac
}

# Ejecutar tests baseline
run_baseline() {
    local crate=$1
    
    echo -e "${BLUE}==================================${NC}"
    echo -e "${BLUE}🧪 BASELINE TESTS: $crate${NC}"
    echo -e "${BLUE}==================================${NC}"
    
    if [ "$crate" == "all" ]; then
        cargo test -p rydit-lexer -p rydit-parser -p rydit-vm -p rydit-stream --lib
    else
        cargo test -p "$crate" --lib
    fi
    
    echo -e "${GREEN}✅ Baseline tests completados${NC}"
}

# Ejecutar MIRI
run_miri() {
    local crate=$1
    
    echo -e "${BLUE}==================================${NC}"
    echo -e "${BLUE}🔍 MIRI TESTING: $crate${NC}"
    echo -e "${BLUE}==================================${NC}"
    
    check_dependencies "miri"
    
    if [ "$crate" == "all" ]; then
        for c in "${SAFE_CRATES[@]}"; do
            echo -e "${YELLOW}Testing $c con MIRI...${NC}"
            cargo miri test -p "$c" --lib -- --nocapture
        done
    else
        cargo miri test -p "$crate" --lib -- --nocapture
    fi
    
    echo -e "${GREEN}✅ MIRI testing completado${NC}"
    echo -e "${YELLOW}Nota: Si MIRI detecta undefined behavior, verás un error detallado arriba${NC}"
}

# Ejecutar TARPAULIN
run_tarpaulin() {
    local crate=$1
    
    echo -e "${BLUE}==================================${NC}"
    echo -e "${BLUE}📊 TARPAULIN COVERAGE: $crate${NC}"
    echo -e "${BLUE}==================================${NC}"
    
    check_dependencies "tarpaulin"
    
    # Crear directorio de coverage
    mkdir -p "$PROJECT_DIR/coverage"
    
    if [ "$crate" == "all" ]; then
        echo -e "${YELLOW}Generando coverage combinado...${NC}"
        cargo tarpaulin \
            -p rydit-lexer \
            -p rydit-parser \
            -p rydit-vm \
            -p rydit-stream \
            --out Html \
            --output-dir "$PROJECT_DIR/coverage/all" \
            --follow-exec \
            --verbose
    else
        echo -e "${YELLOW}Generando coverage para $crate...${NC}"
        cargo tarpaulin \
            -p "$crate" \
            --out Html \
            --output-dir "$PROJECT_DIR/coverage/$crate" \
            --follow-exec \
            --verbose
    fi
    
    echo -e "${GREEN}✅ Coverage generado en $PROJECT_DIR/coverage/${NC}"
    echo -e "${YELLOW}Abre el archivo index.html en tu navegador para ver el reporte${NC}"
}

# Ejecutar todo
run_all() {
    echo -e "${BLUE}==================================${NC}"
    echo -e "${BLUE}🚀 TESTING COMPLETO (MIRI + TARPAULIN)${NC}"
    echo -e "${BLUE}==================================${NC}"
    
    # Baseline primero
    run_baseline "all"
    
    # MIRI
    run_miri "all"
    
    # TARPAULIN
    run_tarpaulin "all"
    
    echo -e "${GREEN}==================================${NC}"
    echo -e "${GREEN}✅ TESTING COMPLETO FINALIZADO${NC}"
    echo -e "${GREEN}==================================${NC}"
}

# Main
if [ $# -lt 1 ]; then
    usage
fi

COMMAND=$1
CRATE=${2:-all}

case $COMMAND in
    "miri")
        run_miri "$CRATE"
        ;;
    "tarpaulin")
        run_tarpaulin "$CRATE"
        ;;
    "all")
        run_all
        ;;
    "baseline")
        run_baseline "$CRATE"
        ;;
    *)
        echo -e "${RED}❌ Comando desconocido: $COMMAND${NC}"
        usage
        ;;
esac
