# 🛡️ Binarios Pendientes de Revisión

**Fecha**: 2026-04-01  
**Total**: 10 binarios seleccionados por compatibilidad  
**Propósito**: "Plan B" si Nivel 3 falla

---

## 📋 **BINARIOS SELECCIONADOS (10)**

### **Tests SDL2 Simples** (5 archivos)
- ✅ `test_sdl2_basico.rs` - Test básico de SDL2
- ✅ `test_sdl2_simple.rs` - Test simple de SDL2
- ✅ `test_sdl2_ffi.rs` - Test FFI de SDL2
- ✅ `test_audio_ffi.rs` - Test de audio FFI
- ✅ `test_input_simple.rs` - Test de input simple
- ✅ `test_minimalista.rs` - Test minimalista

### **Demos Básicos** (4 archivos)
- ✅ `demo_sdl2_puro.rs` - Demo SDL2 puro
- ✅ `demo_movimiento.rs` - Demo de movimiento
- ✅ `demo_particulas_sdl2.rs` - Demo de partículas SDL2
- ✅ `demo_toolkit_ry.rs` - Demo de Toolkit UI

---

## 🎯 **CRITERIOS DE SELECCIÓN**

### **Por qué estos 10**
1. **Menos dependencias** → Más probabilidad de compilar
2. **SDL2 nativo** → Compatible con Termux-X11
3. **Código simple** → Menos puntos de falla
4. **Esenciales para Nivel 3** → Input, audio, render básico

### **Por qué NO los demás**
- `demo_big_bang.rs` → Muy complejo
- `demo_complejo_100.rs` → Demasiadas dependencias
- `gpu_demo*.rs` → GPU instancing (avanzado)
- `ecs_demo*.rs` → ECS (avanzado)
- `demo_input_map_standalone.rs` → Input map complejo

---

## 🔧 **COMANDOS DE VERIFICACIÓN**

```bash
# Verificar todos los pendientes-revision
cd ejemplos-gfx/pendientes-revision/
for bin in *.rs; do
    nombre=$(basename $bin .rs)
    echo "=== $nombre ==="
    cargo check --bin $nombre 2>&1 | grep -E "error|Finished"
done

# Verificar uno específico
cargo check --bin test_sdl2_basico
```

---

## 📊 **ESTRATEGIA "PLAN B"**

### **Si Nivel 3 falla**
1. ✅ Verificar estos 10 pendientes-revision
2. ✅ Los que funcionen → Mover a `funcionan/`
3. ✅ Los que fallen → Mover a `no-funcionan/`
4. ✅ Usar los que funcionen como base para Nivel 3

### **Si Nivel 3 funciona**
1. ✅ Estos 10 son "respaldo"
2. ✅ Se verifican después
3. ✅ Se usan como referencia futura

---

## 🎯 **PRIORIDAD DE VERIFICACIÓN**

### **Prioridad 1** (Esenciales para Nivel 3)
1. `test_sdl2_basico.rs` → Input SDL2
2. `test_audio_ffi.rs` → Audio SDL2
3. `demo_particulas_sdl2.rs` → Render SDL2

### **Prioridad 2** (Demos básicos)
4. `demo_sdl2_puro.rs` → SDL2 puro
5. `demo_movimiento.rs` → Movimiento
6. `demo_toolkit_ry.rs` → Toolkit UI

### **Prioridad 3** (Tests FFI)
7. `test_sdl2_simple.rs` → SDL2 simple
8. `test_sdl2_ffi.rs` → FFI SDL2
9. `test_input_simple.rs` → Input simple
10. `test_minimalista.rs` → Minimalista

---

## 📝 **ESTADO ACTUAL**

| Carpeta | Cantidad | Estado |
|---------|----------|--------|
| **src/bin/** | 7 | ✅ Esenciales |
| **pendientes-revision/** | 10 | ⏳ Por verificar (Plan B) |
| **pendientes/** | 21 | ⏳ Por verificar (resto) |
| **funcionan/** | 0 | ✅ Vacío (por llenar) |
| **no-funcionan/** | 0 | ❌ Vacío (por llenar) |

---

<div align="center">

**🛡️ Pendientes de Revisión - Plan B**

*10 binarios seleccionados ✅ | Menos dependencias ✅ | Esenciales para Nivel 3 ✅*

**Próximo: Nivel 3 (gráficos manuales)**

</div>
