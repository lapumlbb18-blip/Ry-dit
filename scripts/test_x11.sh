#!/data/data/com.termux/files/usr/bin/bash
# 🛡️ RyDit v0.10.2 - Script de Diagnóstico X11
# Verifica configuración antes de ejecutar demos

set -e

echo "╔════════════════════════════════════════════════════════╗"
echo "║  🛡️  RyDit v0.10.2 - Diagnóstico de X11/Zink          ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# ============================================================================
# 1. VERIFICAR VARIABLES DE ENTORNO
# ============================================================================

echo "📋 1. Verificando variables de entorno..."
echo ""

export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

echo "   DISPLAY=$DISPLAY"
echo "   MESA_LOADER_DRIVER_OVERRIDE=$MESA_LOADER_DRIVER_OVERRIDE"
echo "   DRI3=$DRI3"
echo ""

# ============================================================================
# 2. VERIFICAR SERVIDOR X11
# ============================================================================

echo "🖥️  2. Verificando servidor X11..."

# En Termux, verificamos si DISPLAY está seteado y si hay procesos X11
if [ -n "$DISPLAY" ] && (pidof termux-x11 &>/dev/null || [ "$DISPLAY" = ":0" ]); then
    echo "   ✅ X11 disponible en DISPLAY=$DISPLAY"
    
    # Verificar si termux-x11 está corriendo
    if pidof termux-x11 &>/dev/null; then
        echo "   ✅ Termux-X11 corriendo"
    else
        echo "   ⚠️  DISPLAY=$DISPLAY pero termux-x11 no detectado"
        echo "   Puede estar en otra sesión o puerto"
    fi
else
    echo "   ❌ ERROR: X11 no disponible"
    echo ""
    echo "   Solución:"
    echo "   1. Inicia Termux-X11:"
    echo "      termux-x11 :0 -xstartup xfce4-session &"
    echo ""
    echo "   2. Espera 2-3 segundos a que inicie"
    echo ""
    echo "   3. Vuelve a ejecutar este script"
    exit 1
fi

echo ""

# ============================================================================
# 3. VERIFICAR DRIVER ZINK
# ============================================================================

echo "🎨 3. Verificando driver Zink..."

# Intentar obtener información de OpenGL
GL_INFO=$(glxinfo -B 2>&1 | head -10 || echo "glxinfo no disponible")

if echo "$GL_INFO" | grep -q "Zink"; then
    echo "   ✅ Zink detectado"
    echo "   $GL_INFO" | grep -E "(OpenGL|Zink|Device)" | sed 's/^/   /'
elif echo "$GL_INFO" | grep -q "OpenGL"; then
    echo "   ⚠️  OpenGL detectado (posiblemente Zink)"
    echo "   $GL_INFO" | grep -E "(OpenGL|Renderer)" | sed 's/^/   /'
else
    echo "   ⚠️  No se pudo verificar Zink (glxinfo no disponible o sin output)"
    echo ""
    echo "   Esto NO es crítico. RyDit puede funcionar igual."
    echo ""
    echo "   Si tienes problemas de renderizado:"
    echo "   pkg install mesa-demos"
fi

echo ""

# ============================================================================
# 4. VERIFICAR BINARIOS RYDIT
# ============================================================================

echo "📦 4. Verificando binarios RyDit..."

cd /data/data/com.termux/files/home/shield-project

BINARIOS=(
    "target/release/scene_runner"
    "target/release/ecs_demo_10k"
    "target/release/gpu_demo_100k"
    "target/release/demo_particles"
)

for binario in "${BINARIOS[@]}"; do
    if [ -f "$binario" ]; then
        SIZE=$(du -h "$binario" | cut -f1)
        echo "   ✅ $binario ($SIZE)"
    else
        echo "   ❌ $binario (no compilado)"
    fi
done

# Verificar rydit-rs (legacy)
if [ -f "target/release/rydit-rs" ]; then
    echo "   ⚠️  target/release/rydit-rs (legacy - 64 errores)"
else
    echo "   ⚠️  target/release/rydit-rs (no compilado - esperado)"
fi

echo ""

# ============================================================================
# 5. VERIFICAR DEMOS .RYDIT
# ============================================================================

echo "🎮 5. Verificando demos .rydit..."

DEMOS=(
    "demos/nivel_config.rydit"
    "demos/test_minimo.rydit"
    "demos/test_render_queue_integrada.rydit"
    "demos/test_renderizado_v0.9.0.rydit"
)

for demo in "${DEMOS[@]}"; do
    if [ -f "$demo" ]; then
        LINES=$(wc -l < "$demo")
        echo "   ✅ $demo ($LINES líneas)"
    else
        echo "   ❌ $demo (no existe)"
    fi
done

echo ""

# ============================================================================
# 6. TEST DE RENDERIZADO SIMPLE
# ============================================================================

echo "🧪 6. Ejecutando test de renderizado simple..."
echo ""

if [ -f "target/release/scene_runner" ] && [ -f "demos/test_minimo.rydit" ]; then
    echo "   🚀 Ejecutando: scene_runner demos/test_minimo.rydit"
    echo ""
    echo "   → Si se abre una ventana: ✅ TODO FUNCIONA"
    echo "   → Si hay pantalla negra: ⚠️  Revisar logs abajo"
    echo "   → Si hay error: ❌ Ver mensaje de error"
    echo ""
    echo "   (Presiona Ctrl+C para cancelar después de 3 segundos)"
    echo ""
    
    # Ejecutar con timeout de 3 segundos
    timeout 3 ./target/release/scene_runner demos/test_minimo.rydit 2>&1 || true
    
    echo ""
    echo "   ✅ Test completado"
else
    echo "   ⚠️  No se pudo ejecutar el test (faltan archivos)"
fi

echo ""

# ============================================================================
# 7. RESUMEN Y RECOMENDACIONES
# ============================================================================

echo "╔════════════════════════════════════════════════════════╗"
echo "║  📊 RESUMEN DEL DIAGNÓSTICO                            ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# Determinar estado general
if [ -n "$DISPLAY" ] && (pidof termux-x11 &>/dev/null || [ "$DISPLAY" = ":0" ]); then
    echo "   ✅ X11: Funcionando"
else
    echo "   ❌ X11: NO funcionando"
fi

if [ -f "target/release/scene_runner" ]; then
    echo "   ✅ Binarios: Compilados"
else
    echo "   ❌ Binarios: Faltan (ejecutar: cargo build --release)"
fi

if [ -f "demos/test_minimo.rydit" ]; then
    echo "   ✅ Demos: Disponibles"
else
    echo "   ❌ Demos: Faltan"
fi

echo ""
echo "════════════════════════════════════════════════════════"
echo ""

# Recomendaciones
echo "📝 RECOMENDACIONES:"
echo ""

if ! xset q &>/dev/null; then
    echo "   1. Inicia Termux-X11 primero:"
    echo "      termux-x11 :0 -xstartup xfce4-session &"
    echo ""
fi

if [ ! -f "target/release/scene_runner" ]; then
    echo "   2. Compila el proyecto:"
    echo "      cargo build --release"
    echo ""
fi

echo "   3. Para ejecutar demos:"
echo "      ./run_demo.sh demos/nivel_config.rydit"
echo ""
echo "      O directamente:"
echo "      ./target/release/scene_runner demos/test_minimo.rydit"
echo ""

echo "════════════════════════════════════════════════════════"
echo ""
echo "🛡️  Diagnóstico completado."
echo ""
