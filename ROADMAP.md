# 🛡️ RyDit Engine - ROADMAP v0.9.0

**Última actualización**: 2026-03-28
**Versión actual**: v0.9.0 ✅ 3 CAPAS CRÍTICAS COMPLETADAS
**Próxima versión**: v0.9.1 - GPU Particles (FFI Experimental)

---

## 📊 ESTADO ACTUAL (v0.9.0)

### ✅ Completado
- [x] **Command Queue** - 8192+ draw calls por frame
- [x] **Double Buffering** - Front/back buffer separation
- [x] **Platform Sync** - XFlush/XSync para Termux-X11
- [x] **0 warnings** clippy (4 → 0)
- [x] **Tests verificados** - 500+ frames en producción
- [x] **260+ tests** passing
- [x] **Documentación** completa

### ⚠️ Deuda Técnica
| Ítem | Estado | Prioridad | Impacto |
|------|--------|-----------|---------|
| **GPU Instancing** | ⚠️ Pendiente | Media | 10K+ partículas |
| **Shaders GLSL** | ⚠️ Pendiente | Media | GPU rendering |
| **FFI OpenGL** | ⚠️ Pendiente | Baja | Opcional |

---

## 🎯 ROADMAP EVOLUTIVO

### v0.9.0 - 3 Capas Críticas ✅ COMPLETADO

**Fecha**: 2026-03-28

**Features**:
- ✅ Command Queue (8192+ draw calls)
- ✅ Double Buffering (front/back)
- ✅ Platform Sync (XFlush/XSync)
- ✅ 0 warnings clippy
- ✅ 500+ frames verificados

**Archivos clave**:
- `crates/rydit-gfx/src/render_queue.rs` (540 líneas)
- `crates/rydit-gfx/examples/demo_render_queue.rs` (200 líneas)
- `docs/3_CAPAS_CRITICAS_V0.9.0.md`

**Rendimiento**:
- 1000 partículas @ 60 FPS (límite práctico)
- 8192+ draw calls acumulados
- 1 begin_draw() por frame

---

### v0.9.1 - GPU Particles (FFI Experimental) 🔜

**Fecha**: Próxima sesión (2-3 semanas)

**Features**:
- [ ] Investigar `gl-rs` crate
- [ ] Prototipo shader vertex/fragment GLSL
- [ ] `glDrawArraysInstanced()` básico
- [ ] Demo: 5000 partículas @ 60 FPS
- [ ] FFI OpenGL seguro

**Riesgos**:
- ⚠️ Requiere `unsafe` massivo
- ⚠️ Duplica código de raylib
- ⚠️ Gestión manual de memoria GPU

**Beneficios**:
- ✅ 5x más partículas (5000 vs 1000)
- ✅ Shaders GLSL custom
- ✅ Aprendizaje de GPU programming

---

### v0.9.2 - Optimización Render Queue 🔜

**Fecha**: Después de v0.9.1 (1-2 semanas)

**Features**:
- [ ] Separar por tipo (círculos, rects, líneas)
- [ ] Mejor batching interno
- [ ] Posible: 2000 partículas @ 60 FPS
- [ ] Statistics por tipo

**Riesgos**:
- ✅ Bajo (optimización incremental)

**Beneficios**:
- ✅ 2x más partículas sin FFI
- ✅ Sin unsafe
- ✅ Fácil de mantener

---

### v0.9.5 - FFI OpenGL en rydit-gfx 🔮

**Fecha**: Futuro (4-6 semanas)

**Ubicación**: `crates/rydit-gfx/src/gpu_instancing.rs`

**Features**:
- [ ] Agregar `gl-rs` crate a rydit-gfx
- [ ] Shaders GLSL (vertex + fragment)
- [ ] VAO + VBO para instancing
- [ ] `glDrawArraysInstanced()` básico
- [ ] Demo: 10,000+ partículas @ 60 FPS
- [ ] Fallback a Render Queue si no hay GPU

**Riesgos**:
- ⚠️ Requiere unsafe (FFI OpenGL)
- ⚠️ Complejidad media-alta
- ⚠️ Testing más difícil

**Beneficios**:
- ✅ 10x más partículas (10,000 vs 1000)
- ✅ Shaders GLSL custom
- ✅ Binarios .rs pueden usarlo directo

---

### v1.0.0 - GPU Instancing Maduro 🔮

**Fecha**: Futuro (6-8 semanas)

**Ubicación**: `crates/rydit-gfx/src/gpu_instancing.rs`

**Features**:
- [ ] 10,000+ partículas reales
- [ ] 1 draw call por frame
- [ ] Shaders GLSL custom
- [ ] API unificada
- [ ] Documentación completa

**Riesgos**:
- ⚠️ Requiere FFI OpenGL estable
- ⚠️ Testing en múltiples plataformas

**Beneficios**:
- ✅ 100,000+ partículas posibles
- ✅ GPU-bound (no CPU-bound)
- ✅ Comparable a Python ModernGL

---

### v1.1.0 - ECS (Entity Component System) 🔮

**Fecha**: Futuro (8-10 semanas)

**Ubicación**: `crates/rydit-ecs/` (crate nuevo)

**Features**:
- [ ] Crate separado: `crates/rydit-ecs/`
- [ ] ENTT o bevy_ecs
- [ ] Components: Position, Velocity, Sprite
- [ ] Systems: Movement, Render, Physics
- [ ] Integración en executor.rs

**Riesgos**:
- ⚠️ Crate nuevo que mantener
- ⚠️ Curva de aprendizaje ECS

**Beneficios**:
- ✅ 100,000+ entities estables
- ✅ Reutilizable
- ✅ Testing independiente

---

### v1.1.0 - Parser Maduro 🔮

**Fecha**: Futuro (8-10 semanas)

**Features**:
- [ ] Refactorizar `lizer/src/lib.rs` completo
- [ ] Paréntesis que funcionen SIEMPRE
- [ ] Expresiones complejas sin dolor
- [ ] Arrays multidimensionales reales
- [ ] Comentarios en cualquier posición

**Riesgos**:
- 🔴 Alto (puede romper código existente)

**Beneficios**:
- ✅ Lenguaje más robusto
- ✅ Mejor experiencia de desarrollo
- ✅ Soporte para código complejo

---

## 📈 COMPARATIVA DE RENDIMIENTO

| Versión | Partículas | Draw Calls | FPS | Complejidad |
|---------|------------|------------|-----|-------------|
| **v0.8.x** | 500 | 500 | 30 | Baja |
| **v0.9.0** | 1000 | 1000 | 60 | Media |
| **v0.9.1** | 5000 | 100 | 60 | Alta |
| **v0.9.5** | 5000 | 10 | 60 | Alta |
| **v1.0.0** | 10,000+ | 1 | 60 | Muy Alta |

---

## 🎯 DECISIONES CRÍTICAS

### ¿GPU Instancing ahora o después?

**AHORA (v0.9.1-v1.0.0)**:
- ✅ Para demos masivos de partículas
- ✅ Aprendizaje de GPU programming
- ⚠️ Requiere 4-6 semanas adicionales

**DESPUÉS (post v1.0.0)**:
- ✅ Render Queue es SUFICIENTE para 90% de casos
- ✅ Enfocarse en otras features
- ⚠️ Límite de 1000 partículas

### ¿FFI OpenGL o wgpu?

**FFI OpenGL (recomendado)**:
- ✅ Mantiene compatibilidad con raylib
- ⚠️ Requiere unsafe
- ✅ Más control directo

**wgpu (alternativa)**:
- ✅ API moderna (Vulkan/Metal/DX12)
- ❌ Cambia arquitectura completa
- ❌ Pierde ventajas de raylib

---

## 📊 MÉTRICAS DE ÉXITO

### v0.9.0 ✅
- [x] 8192+ draw calls
- [x] 0 warnings clippy
- [x] 500+ frames verificados
- [x] 60 FPS estables

### v0.9.1 (Objetivo)
- [ ] 5000 partículas @ 60 FPS
- [ ] Shaders GLSL funcionando
- [ ] Demo de partículas masivas

### v1.0.0 (Objetivo)
- [ ] 10,000+ partículas @ 60 FPS
- [ ] 1 draw call por frame
- [ ] API unificada

---

## 🔗 REFERENCIAS

### Documentos
- [3_CAPAS_CRITICAS_V0.9.0.md](docs/3_CAPAS_CRITICAS_V0.9.0.md)
- [PANORAMA_GPU_INSTANCING_V0.9.x.md](docs/PANORAMA_GPU_INSTANCING_V0.9.x.md)
- [VERIFICACION_PRODUCCION_V0.9.0.md](docs/VERIFICACION_PRODUCCION_V0.9.0.md)

### Código
- `crates/rydit-gfx/src/render_queue.rs`
- `crates/rydit-gfx/examples/demo_render_queue.rs`

### Externas
- [Learn OpenGL - Instancing](https://learnopengl.com/Advanced-OpenGL/Instancing)
- [gl-rs crate](https://github.com/bjz/gl-rs)
- [wgpu](https://github.com/gfx-rs/wgpu)

---

<div align="center">

**🛡️ RyDit Engine - ROADMAP v0.9.0**

*v0.9.0 ✅ | v0.9.1 🔜 | v1.0.0 🔮*

**Próxima sesión: v0.9.1 - GPU Particles (FFI) o Optimización**

</div>
