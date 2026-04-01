# 🛡️ Ejemplos Gráficos RyDit - Clasificación

**Fecha**: 2026-04-01  
**Total binarios**: 31  
**Estado**: Pendientes de verificar

---

## 📂 **ESTRUCTURA**

```
ejemplos-gfx/
├── funcionan/        # ✅ Binarios que compilan y funcionan
├── no-funcionan/     # ❌ Binarios con errores
└── pendientes/       # ⏳ Sin verificar (31 binarios)
```

---

## 📋 **BINARIOS PENDIENTES DE VERIFICAR**

### **Demos SDL2** (11 archivos)
- demo_10k_particulas.rs
- demo_assets_simple.rs
- demo_big_bang.rs
- demo_complejo_100.rs
- demo_input_map_standalone.rs
- demo_input_sdl2.rs
- demo_migui_sdl2.rs
- demo_mouse_basico.rs
- demo_movimiento.rs
- demo_particulas_sdl2.rs
- demo_platformer.rs
- demo_sdl2_puro.rs
- demo_simple_desde_cero.rs
- demo_toolkit_ry.rs

### **Tests SDL2** (10 archivos)
- test_audio_ffi.rs
- test_audio_real.rs
- test_ffi_ventana.rs
- test_input_correcto.rs
- test_input_simple.rs
- test_minimalista.rs
- test_sdl2_basico.rs
- test_sdl2_ffi.rs
- test_sdl2_simple.rs
- test_sdl2_sprite_debug.rs
- test_sdl2_sprite_simple.rs
- test_sdl2_sprites.rs
- test_sdl2_ttf.rs
- test_ventana_hd.rs

### **GPU/ECS** (3 archivos)
- ecs_demo_10k.rs
- gpu_demo_100k.rs
- gpu_demo_100k_debug.rs

---

## 🔧 **COMANDOS DE VERIFICACIÓN**

### **Verificar un binario**
```bash
# Check rápido (sin compilar)
cargo check --bin <nombre_binario>

# Compilar (debug)
cargo build --bin <nombre_binario>

# Ejecutar
./target/debug/<nombre_binario>
```

### **Verificar todos**
```bash
# Script de verificación
for bin in ejemplos-gfx/pendientes/*.rs; do
    nombre=$(basename $bin .rs)
    echo "=== Verificando: $nombre ==="
    cargo check --bin $nombre 2>&1 | grep -E "error|Finished"
done
```

---

## 📊 **CRITERIOS DE CLASIFICACIÓN**

### **✅ Funcionan**
- Compilan sin errores
- Ejecutan sin crashes
- Gráficos SDL2 visibles
- Input responde

### **❌ No Funcionan**
- Errores de compilación
- Crashes al ejecutar
- Gráficos no visibles
- Input no responde

### **⏳ Pendientes**
- Sin verificar
- Requiere Termux-X11
- Requiere assets externos

---

## 🎯 **PRÓXIMOS PASOS**

1. **Verificar cada binario** en `pendientes/`
2. **Clasificar** en `funcionan/` o `no-funcionan/`
3. **Fixear** los de `no-funcionan/`
4. **Documentar** estado de cada uno
5. **Integrar** los que funcionan en rydit-test (Nivel 3)

---

<div align="center">

**🛡️ Ejemplos Gráficos - Pendientes de Verificar**

*31 binarios | 0 verificados | Próximo: Verificar uno por uno*

</div>
