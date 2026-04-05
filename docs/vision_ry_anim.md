# 🔮 ry-anim — Visión Estratégica y Plan de Implementación

**Fecha**: 2026-04-05
**Versión actual**: v0.7.3 (parcial: 3/12 principios Disney)
**Versión objetivo**: v0.12.0 — Motor de animación integral

---

## 📊 DESCRIPCIÓN PARA CRATES.IO

```
ry-anim — Animation Engine for Science, Education & Gaming

A comprehensive animation toolkit with:
• 12 Disney animation principles (easing, squash/stretch, anticipation)
• Animated optical illusions (Troxler, rotating snakes, cafe wall)
• Anamorphic art generators (perspective, cylindrical, mirror)
• Scientific animations (chemical reactions, biological cycles, L-systems)
• Special effects (neon glow, motion blur, chromatic aberration)
• Sprite animation system (frame-based, state machines, blending)

Perfect for:
• Science content creators (Discovery Channel-style animations)
• Educators demonstrating concepts visually
• Game developers needing polished animation systems
• Digital artists creating optical illusions

Built in Rust. Zero external dependencies.
```

---

## 🎯 VISIÓN DEL CRATE

**ry-anim** es el motor de creación de animaciones de Ry-Dit — el "After Effects" integrado.

### Público Objetivo

| Audiencia | Uso |
|-----------|-----|
| **Creadores de contenido ciencia** | Animar ciclos de crecimiento, reacciones químicas, evolución biológica |
| **Educadores** | Demostrar conceptos con animaciones (Tusi, ilusiones, experimentos) |
| **Game developers** | Efectos especiales, sprites animados, eventos visuales |
| **Artistas digitales** | Arte anamórfico, ilusiones ópticas, efectos neón |

### Diferenciador Único

| Motor | Animación | Ilusiones | Ciencia |
|-------|-----------|-----------|---------|
| **Godot** | ✅ AnimationPlayer | ❌ | ❌ |
| **Unity** | ✅ Animator | ❌ | ❌ |
| **ry-anim** | ✅ Disney + Easing | ✅ Ópticas | ✅ Científicas |

**Ningún motor tiene ilusiones ópticas animadas integradas.**

---

## 🧩 DIFERENCIACIÓN CON OTROS CRATES

### ry-science vs ry-anim

| ry-science | ry-anim |
|-----------|---------|
| **Geometría estática** (Penrose, impossible cube) | **Animaciones** de esas mismas ilusiones |
| **Matemáticas** (Bezier, estadísticas) | **Movimiento** (easing, transiciones) |
| **Fórmulas** (qué es) | **Tiempo** (cómo se mueve) |
| Ejemplo: `geometry::penrose()` → coordenadas | Ejemplo: `illusion::rotating_penrose(t)` → animación |

### ry-physics vs ry-anim

| ry-physics | ry-anim |
|-----------|---------|
| **Cálculo físico** (fuerzas, gravedad) | **Efecto visual** (glow, blur, morph) |
| **Simulación** real | **Apariencia** de movimiento |
| Ejemplo: `nbody_simulate()` → posiciones reales | Ejemplo: `trail::motion_blur(entity)` → estela visual |

### El Puente

```
ry-science (qué) + ry-physics (cómo se mueve físicamente)
         ↓
    ry-anim (cómo se VE el movimiento)
         ↓
    ry-gfx / ry3d-gfx (render final)
```

---

## 🏗️ ARQUITECTURA PROPUESTA — 7 CAPAS

```
ry-anim/
├── easing.rs          ✅ Ya existe (parcial)
│   ├── Curvas de easing (20+ tipos)
│   ├── Animation curves custom
│   └── Timeline interpolation
│
├── disney.rs          ⏳ 9 principios pendientes
│   ├── Follow Through & Overlapping
│   ├── Arcs
│   ├── Secondary Action
│   ├── Timing
│   ├── Exaggeration
│   ├── Solid Drawing
│   ├── Appeal
│   ├── Straight Ahead vs Pose-to-Pose
│   └── Slow In & Out (ya existe como easing)
│
├── illusions.rs       🔮 NUEVO — Ilusiones ópticas animadas
│   ├── Troxler fading (desvanecimiento por fijación)
│   ├── Motion-induced blindness
│   ├── Rotating snakes (ilusión de movimiento)
│   ├── Cafe wall animation
│   ├── Fraser spiral animation
│   └── Zöllner effect animation
│
├── anamorphic.rs      🔮 NUEVO — Arte anamórfico
│   ├── Perspective correction
│   ├── Cylindrical anamorphosis
│   ├── Mirror anamorphosis
│   └── Street art 3D illusion
│
├── science_anim.rs    🔮 NUEVO — Animaciones científicas
│   ├── Chemical reactions (growth, crystallization)
│   ├── Biological cycles (cell division, growth curves)
│   ├── Fauna movement (walk cycles, flight patterns)
│   ├── Flora growth (L-systems animated)
│   ├── Tusi couple (historical mathematical animation)
│   └── Pendulum waves
│
├── effects.rs         🔮 NUEVO — Efectos especiales
│   ├── Neon glow
│   ├── Motion blur
│   ├── Chromatic aberration
│   ├── Bloom effect
│   ├── Particle trails
│   └── Morphing shapes
│
├── sprite_anim.rs     🔮 NUEVO — Animación de sprites
│   ├── Frame-based animation
│   ├── Sprite sheet parser
│   ├── Animation states (idle, run, jump)
│   └── Blend between animations
│
└── particles.rs       ✅ Ya existe (fire, smoke, spark, explosion)
    └── + presets nuevos (chemical, biological, magical)
```

---

## 📋 ROADMAP DE IMPLEMENTACIÓN

### Fase 1: Completar Disney (v0.8.0) — 10-15h

| Principio | Fórmula/Función | Esfuerzo |
|-----------|----------------|----------|
| Follow Through | `follow_through(amplitude, decay, t)` | 2h |
| Overlapping Action | `overlap(base, offset, t)` | 2h |
| Arcs | `arc_path(start, end, curvature, t)` | 2h |
| Secondary Action | `secondary(primary, offset, t)` | 2h |
| Timing | `timing(duration, keyframes, t)` | 2h |
| Exaggeration | `exaggerate(value, factor, t)` | 1h |
| Solid Drawing | `solid_rotation(x, y, z, t)` | 2h |
| Appeal | `appeal(base_shape, charm, t)` | 2h |
| Straight Ahead / Pose-to-Pose | `pose_to_pose(keyframes, t)` | 2h |

### Fase 2: Ilusiones Ópticas (v0.9.0) — 15-20h

| Ilusión | Descripción | Esfuerzo |
|---------|-------------|----------|
| Troxler fading | Desvanecimiento por fijación visual | 3h |
| Rotating snakes | Ilusión de movimiento circular | 3h |
| Cafe wall | Líneas paralelas que parecen inclinadas | 2h |
| Fraser spiral | Espiral que en realidad son círculos | 3h |
| Zöllner effect | Líneas paralelas con segmentossesgados | 2h |
| Motion-induced blindness | Objetos que desaparecen | 3h |
| Pulsing star | Estrella que pulsa | 2h |

### Fase 3: Efectos Especiales (v0.10.0) — 10-15h

| Efecto | Descripción | Esfuerzo |
|--------|-------------|----------|
| Neon glow | Resplandor neón configurable | 3h |
| Motion blur | Desenfoque de movimiento | 3h |
| Chromatic aberration | Separación RGB en bordes | 2h |
| Bloom effect | Brillo difuso en zonas claras | 3h |
| Particle trails | Estelas de partículas | 2h |
| Morphing shapes | Transición entre formas | 3h |

### Fase 4: Ciencia Animada (v0.11.0) — 20-30h

| Animación | Descripción | Esfuerzo |
|-----------|-------------|----------|
| Chemical reactions | Cristalización, crecimiento | 5h |
| Biological cycles | División celular, crecimiento | 5h |
| Fauna movement | Ciclos de caminata, vuelo | 5h |
| Flora growth | L-systems animados | 5h |
| Tusi couple | Pareja de Tusi (histórica) | 3h |
| Pendulum waves | Ondas de péndulos | 3h |
| Wave interference | Interferencia de ondas | 3h |

### Fase 5: Sprite Animation (v0.12.0) — 15-20h

| Feature | Descripción | Esfuerzo |
|---------|-------------|----------|
| Frame-based animation | Animación cuadro por cuadro | 4h |
| Sprite sheet parser | Parsear hojas de sprites | 3h |
| Animation states | idle, run, jump, etc. | 4h |
| Blend animations | Transición suave entre animaciones | 4h |
| Sprite events | Eventos al cambiar estado | 2h |
| Timeline editor | Editor visual de timeline | 5h |

---

## 📊 MÉTRICAS ESPERADAS

| Métrica | Actual | v0.8.0 | v0.9.0 | v0.10.0 | v0.11.0 | v0.12.0 |
|---------|--------|--------|--------|---------|---------|---------|
| **Funciones** | 6 | 15 | 22 | 28 | 35 | 45+ |
| **Tests** | 9 | 20 | 30 | 40 | 50 | 60+ |
| **Líneas** | ~380 | ~600 | ~900 | ~1,200 | ~1,600 | ~2,200 |
| **Principios Disney** | 3/12 | 12/12 | 12/12 | 12/12 | 12/12 | 12/12 |
| **Ilusiones** | 0 | 0 | 7 | 7 | 7 | 7 |
| **Efectos** | 0 | 0 | 0 | 6 | 6 | 6 |
| **Ciencia** | 0 | 0 | 0 | 0 | 7 | 7 |
| **Sprite anim** | 0 | 0 | 0 | 0 | 0 | 6 |

---

## 🎯 PRIMER PASO INMEDIATO

Comenzar con **Fase 1: Completar Disney** — los 9 principios restantes.

Cada principio será una función pública en `disney.rs` con:
- Fórmula matemática
- Parámetros configurables
- Tests unitarios
- Documentación con ejemplo de uso

---

<div align="center">

**🔮 ry-anim v0.7.3 → v0.12.0**

*De 6 funciones a motor integral de animación*

*Disney + Ilusiones + Ciencia + Efectos + Sprites*

*Único en crates.io*

</div>
