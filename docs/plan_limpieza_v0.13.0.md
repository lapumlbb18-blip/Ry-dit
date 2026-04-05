# 🧹 Plan de Limpieza y Consolidación v0.13.0

**Fecha**: 2026-04-04
**Versión**: v0.13.0
**Objetivo**: Eliminar basura, consolidar duplicados, mantener crates valiosos

---

## 📊 CRATES "ALTERNATIVOS" (NO eliminar, mantener como v-shield)

### ry-anim — 12 Principios Disney (3 implementados, 9 roadmap)

| Implementado | Fórmula | Estado |
|-------------|---------|--------|
| Squash & Stretch | [factor, 1/factor] | ✅ |
| Anticipation | pos + dir * amount | ✅ |
| Slow In & Slow Out (easing) | t², t(2-t) | ✅ |

| Pendientes | Descripción |
|------------|-------------|
| 4. Staging | Puesta en escena |
| 5. Straight Ahead / Pose to Pose | Dos métodos de animación |
| 6. Follow Through / Overlapping | Inercia de partes |
| 7. Slow In & Slow Out | Ya implementado como easing |
| 8. Arcs | Trayectorias curvas |
| 9. Secondary Action | Acción secundaria |
| 10. Timing | Velocidad y ritmo |
| 11. Exaggeration | Exageración |
| 12. Solid Drawing | Volumen 3D |
| 13. Appeal | Atractivo visual |

**Estrategia**: ry-anim se mantiene como crate alternativo (patrón v-shield).
- NO eliminar (tiene descargas crates.io)
- Mover easing duplicado a `math::ease_*` para acceso desde scripts
- ry-anim queda como crate especializado para animación avanzada
- Completar los 9 principios pendientes en futura versión

### v-shield — Crate de utilidades raylib

**Estado**: Mantenido en workspace, no activo. Patrón a seguir para ry-anim.

### ry-science — Geometría imposible (único)

| Función | Único? |
|---------|--------|
| `penrose_triangle()` | ✅ Sí |
| `impossible_cube()` | ✅ Sí |
| `spiral()` | ✅ Sí |
| `muller_lyer()` | ✅ Sí |
| `ponzo()` | ✅ Sí |

**Estrategia**: Mantener. No hay duplicados reales.

---

## 🗑️ BASURA SEGURA (borrar sin riesgo)

### Backups (17,422 líneas de código muerto)

| Archivo | Líneas | Motivo |
|---------|--------|--------|
| `crates/ry-rs/src/main.rs.backup_v2` | 4,617 | Usa `rydit_*` imports viejos |
| `crates/ry-rs/src/main.rs.backup_group1` | 4,617 | Idéntico al anterior |
| `crates/ry-rs/src/main.rs.backup_group2` | 4,613 | Idéntico al anterior |
| `crates/ry-rs/src/eval/mod.rs.backup_v2` | 3,483 | Evaluador obsoleto |
| `crates/ry-rs/src/repl.rs.backup_v2` | 92 | REPL obsoleto |
| **Total** | **17,422** | |

### Disabled (179 líneas)

| Archivo | Motivo |
|---------|--------|
| `crates/ry-rs/src/disabled/particles_module.rs` | Superseded por `modules/particles.rs` |

### Debug bins (históricos, una sola vez)

| Archivo | Motivo |
|---------|--------|
| `crates/ry-rs/src/bin/debug_types.rs` | Debug v0.11.4 — mover a docs/tests_referencia/ |
| `crates/ry-rs/src/bin/debug_complete.rs` | Debug v0.11.4 — mover a docs/tests_referencia/ |
| `crates/ry-rs/src/bin/debug_e0308.rs` | Debug v0.11.4 — mover a docs/tests_referencia/ |
| `crates/ry-rs/src/bin/debug_mayusculas.rs` | Debug v0.11.4 — mover a docs/tests_referencia/ |
| `crates/ry-rs/src/bin/debug_13_errors.rs` | Debug v0.11.4 — mover a docs/tests_referencia/ |
| `crates/ry-rs/src/bin/debug_estrategico.rs` | Debug v0.11.4 — mover a docs/tests_referencia/ |
| `crates/ry-rs/src/bin/debug_6_errors.rs` | Debug v0.11.4 — mover a docs/tests_referencia/ |

**Total líneas basura**: ~19,600

---

## ⚠️ DUPLICADOS PARCIALES (consolidar, no borrar)

### Collision AABB en physics.rs y collision.rs

```rust
// Ambos tienen esta lógica idéntica:
fn aabb_overlap(x1, y1, w1, h1, x2, y2, w2, h2) -> bool {
    x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
}
```

**Solución**: Extraer a función compartida en `collision.rs`, physics.rs la importa.

### Partículas: ry-anim vs ry-gfx

| | ry-anim/particles.rs | ry-gfx/particles.rs |
|--|---------------------|---------------------|
| Backend | String colors, f64 | Raylib Color, f32 |
| Features | 4 presets (fire, smoke, spark, explosion) | 5 presets + emitters + sistema completo |
| Uso | Matemáticas puras | Render directo |
| Estado | ⚠️ Simplificado | ✅ Completo |

**Solución**: ry-anim particles se mantiene como versión "math-only" (útil sin dependencias gráficas). ry-gfx es la versión render. Son complementarios, no duplicados.

---

## 📦 DEMOS PENDIENTES (31 archivos)

### `ejemplos-gfx/pendientes/` (21 archivos) — Requieren update de imports

Todos tienen imports viejos `rydit_*` → `ry_*`. Son demos gráficos manuales.

**Prioridad para activar**:
1. `demo_platformer.rs` (6,504 líneas) — Juego más completo
2. `demo_10k_particulas.rs` (12,756 líneas) — Showcase partículas
3. `gpu_demo_100k.rs` (4,666 líneas) — GPU instancing
4. `ecs_demo_10k.rs` (3,810 líneas) — ECS stress test
5. `demo_input_sdl2.rs` (4,049 líneas) — Input Termux-X11

### `ejemplos-gfx/pendientes-revision/` (10 archivos)

Tests FFI viejos, probablemente superseded por módulos SDL2 actuales.

---

## 🎯 ESTRATEGIA DE ACCIÓN

### Paso 1: Limpieza segura ✅
```
Borrar:
- 5 backups (17,422 líneas)
- disabled/particles_module.rs (179 líneas)
- Carpeta disabled/ (queda vacía)

Mover a docs/tests_referencia/:
- 7 debug bins (históricos)
```

### Paso 2: Consolidar collision AABB
```
Crear: modules/collision.rs → fn aabb_overlap(x1,y1,w1,h1, x2,y2,w2,h2) → bool
Usar: modules/physics.rs → llamar a collision::aabb_overlap()
Eliminar: código duplicado en physics.rs
```

### Paso 3: Agregar easing a math:: (sin tocar ry-anim)
```
Agregar a eval/mod.rs y main.rs:
- math::ease_in(t)        → t²
- math::ease_out(t)       → t * (2 - t)
- math::ease_in_out(t)    → 2t² si t<0.5, 1-2(1-t)² si t>=0.5

ry-anim se mantiene intacto como crate especializado.
math::ease_* son atajos para scripts .rydit.
```

### Paso 4: Actualizar imports de demos clave
```
Para cada demo prioritario:
1. rydit_* → ry_* imports
2. Verificar que compile
3. Probar en Termux-X11
4. Si funciona → mover a bin/ oficial
5. Si no → archivar
```

---

## 📊 IMPACTO ESTIMADO

| Acción | Líneas afectadas | Riesgo | Beneficio |
|--------|-----------------|--------|-----------|
| Borrar backups | -17,422 | Cero | Código limpio |
| Borrar disabled | -179 | Cero | Código limpio |
| Mover debug bins | -2,000 | Cero | bin/ limpio |
| Consolidar collision | -50, +20 | Bajo | Sin duplicación |
| Agregar easing a math | +30 | Bajo | Funciones accesibles |
| ry-anim → alternativo | 0 | Bajo | Claridad de rol |

---

## 🏗️ ESTADO FINAL DESEADO

```
Crates activos principales:
├── ry-core       ✅ Core traits
├── ry-lexer      ✅ Zero-copy
├── ry-parser     ✅ Error recovery
├── ry-vm         ⚠️ VM
├── ry-gfx        ✅ Graphics (raylib + SDL2)
├── ry-physics    ✅ Math puras (proyectiles, N-body)
├── ry-ecs        ✅ ECS
├── ry-science    ✅ Geometría imposible (ÚNICO)
├── ry-anim       ⚠️ Alternativo (12 principios Disney, 3/13)
├── ry-script     ✅ Script loading
├── ry-stream     ✅ Streaming
├── ry-god        ✅ Security (crates.io)
├── ry-loader     ⚠️ Module loader
├── toolkit-ry    ⚠️ UI toolkit
├── migui         ⚠️ GUI 12 widgets
├── lizer         ✅ Legacy wrapper
├── ry-system-ry  ⚠️ Universal SDL2
├── ry-test       ⚠️ Test utils
└── v-shield      ⚠️ Alternativo (patrón ry-anim)

Crates "alternativos" (como v-shield):
├── ry-anim       ⚠️ 12 principios Disney (mantener, no borrar)
└── v-shield      ⚠️ Utilidades raylib (mantener, no borrar)
```

---

<div align="center">

**🛡️ Ry-Dit v0.13.0 — Plan de Limpieza**

*-19,600 líneas basura | ry-anim mantenido como alternativo | +5 demos funcionales*

*Estrategia: limpiar lo inútil, potenciar lo valioso*

</div>
