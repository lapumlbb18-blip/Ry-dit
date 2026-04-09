#!/data/data/com.termux/files/usr/bin/bash
# ============================================================================
# 🛡️ RyDit - Launcher: Demo HUD + Cámara 2D
# Pipeline: Zink + DRI3 + OpenGL ES → Termux-X11 (:0)
#
# Features del demo:
# - 5 entidades con movimiento autónomo
# - Health bars ancladas (world-space) con colores dinámicos
# - Nombres/IDs sobre entidades (TTF cacheado)
# - Cámara 2D: zoom (0.2x-5x), rotación, follow suave, límites
# - Debug overlay (FPS, cámara, entidades, tiempo)
# - Stats HUD (score, tiempo MM:SS, nivel)
# - Minimap con entidades
#
# Controles:
# WASD/Flechas: Mover cámara
# Q/E: Rotar cámara
# +/-: Zoom in/out
# F: Toggle debug overlay
# R: Reset cámara
# ESC: Salir
# ============================================================================

set -e

# Colores
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}🛡️ RyDit - Demo HUD + Cámara 2D${NC}"
echo -e "${CYAN}========================================${NC}"

# Detectar DISPLAY
if [ -n "$DISPLAY" ]; then
    DETECTED_DISPLAY="$DISPLAY"
    echo -e "${GREEN}✅ DISPLAY detectada desde entorno: $DETECTED_DISPLAY${NC}"
else
    # Buscar en procesos activos (thunar, openbox)
    DETECTED_DISPLAY=$(cat /proc/*/environ 2>/dev/null | tr '\0' '\n' | grep -m1 "^DISPLAY=" | cut -d= -f2)
    if [ -n "$DETECTED_DISPLAY" ]; then
        echo -e "${GREEN}✅ DISPLAY detectada de procesos: $DETECTED_DISPLAY${NC}"
    else
        DETECTED_DISPLAY=":0"
        echo -e "${YELLOW}⚠️  DISPLAY no detectada, usando default: $DETECTED_DISPLAY${NC}"
    fi
fi

# Detectar GPU
GPU_INFO=$(cat /proc/$(pgrep -f "com.termux.x11" 2>/dev/null | head -1)/environ 2>/dev/null | tr '\0' '\n' | grep -i "ADRENO\|MALI\|GPU" | head -1)
if [ -n "$GPU_INFO" ]; then
    echo -e "${GREEN}✅ GPU detectada: $GPU_INFO${NC}"
else
    echo -e "${YELLOW}⚠️  GPU no detectada, asumiendo Adreno${NC}"
fi

echo ""
echo -e "${CYAN}Pipeline: Zink + DRI3 + OpenGL ES → Termux-X11 ($DETECTED_DISPLAY)${NC}"
echo -e "${CYAN}Binario: target/release/demo_hud_camera${NC}"
echo ""

# Variables de entorno para Zink/DRI3
export DISPLAY="$DETECTED_DISPLAY"
export MESA_LOADER_DRIVER_OVERRIDE=zink
export GALLIUM_DRIVER=zink
export __GL_SYNC_TO_VBLANK=0
export MESA_GL_VERSION_OVERRIDE=3.3
export ZINK_DEBUG=compact

echo -e "${GREEN}🚀 Lanzando demo...${NC}"
echo -e "${YELLOW}Controles: WASD=Mover | Q/E=Rotar | +/-=Zoom | F=Debug | R=Reset | ESC=Salir${NC}"
echo ""

# Ejecutar
cd /data/data/com.termux/files/home/shield-project
./target/release/demo_hud_camera

echo ""
echo -e "${GREEN}✅ Demo cerrado correctamente${NC}"
