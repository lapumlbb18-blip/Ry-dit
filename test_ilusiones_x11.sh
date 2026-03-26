#!/bin/bash
# test_ilusiones_x11.sh - Probar demo de ilusiones ópticas en Termux-X11

echo "🛡️ RyDit - Test Ilusiones Ópticas en Termux-X11"
echo "================================================"
echo ""

# Configurar Termux-X11
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

echo "✅ Variables de entorno configuradas:"
echo "   DISPLAY=$DISPLAY"
echo "   MESA_LOADER_DRIVER_OVERRIDE=$MESA_LOADER_DRIVER_OVERRIDE"
echo "   DRI3=$DRI3"
echo ""

# Verificar si el demo existe
if [ ! -f "demos/demo_ilusiones_opticas.rydit" ]; then
    echo "❌ Error: demos/demo_ilusiones_opticas.rydit no existe"
    exit 1
fi

echo "📦 Compilando en modo release..."
cargo build --release --bin rydit-rs 2>&1 | tail -5

if [ $? -ne 0 ]; then
    echo "❌ Error en la compilación"
    exit 1
fi

echo ""
echo "🎮 Ejecutando demo de ilusiones ópticas..."
echo ""
echo "Controles:"
echo "  ← →  : Cambiar entre ilusiones"
echo "  ESC  : Salir"
echo ""

./target/release/rydit-rs --gfx demos/demo_ilusiones_opticas.rydit

echo ""
echo "✅ Demo finalizado"
