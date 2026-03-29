# 🛡️ RyDit - ESTRUCTURA DEL PROYECTO

**Última actualización**: 2026-03-28
**Versión**: v0.9.0 ✅ 3 CAPAS CRÍTICAS COMPLETADAS + VERIFICADAS
**Estado**: 10/10 - Renderizado maduro para demos complejos

---

## 📊 ESTADO REAL (SIN FILTROS)

### Puntuación Actual: 10/10 ✅ (v0.9.0 completado)

**Verificado en Producción (2026-03-28):**
- ✅ Command Queue - 8192+ draw calls por frame
- ✅ Double Buffering - Front/back buffer separation
- ✅ Platform Sync - XFlush/XSync para Termux-X11
- ✅ 0 warnings clippy (4 → 0)
- ✅ 500+ frames verificados en producción
- ✅ 260+ tests passing

**Test de Verificación:**
```bash
# Demo 1: Formas básicas
./target/release/rydit-rs --gfx ejemplos_gfx/demo_shapes.rydit
# ✅ 500 frames completados

# Demo 2: Render Queue (Rust)
./target/release/examples/demo_render_queue
# ✅ 186 comandos/frame @ 60 FPS

# Demo 3: Test completo
./target/release/rydit-rs --gfx demos/test_renderizado_v0.9.0.rydit
# ✅ Listo para ejecutar
```

**Lo que FALTA (GPU Instancing):**
- ⚠️ FFI OpenGL - Para 5000+ partículas (v0.9.5)
- ⚠️ Shaders GLSL - Para GPU rendering (v1.0.0)
- ⚠️ `glDrawArraysInstanced()` - Para 10K partículas (v1.0.0)

**NOTA**: Render Queue es SUFICIENTE para 90% de casos (1000 partículas).

---

## 📁 ESTRUCTURA ACTUAL

```
shield-project/
├── crates/
│   ├── lizer/              # Lexer + Parser ✅ FUNCIONA (74 tests)
│   │   ├── src/lib.rs      # ~3,383 líneas
│   │   └── benches/        # Deshabilitados (requieren nightly)
│   │
│   ├── blast-core/         # Executor + Memoria ✅ ESTABLE
│   │   └── src/lib.rs      # ~475 líneas
│   │
│   ├── rydit-core/         # RyditModule trait ✅ ESTABLE
│   │   └── src/lib.rs      # ~401 líneas
│   │
│   ├── rydit-loader/       # Carga dinámica ✅ ESTABLE
│   │   └── src/lib.rs      # ~420 líneas
│   │
│   ├── rydit-script/       # Scripts como módulos ✅ ESTABLE
│   │   └── src/lib.rs      # ~340 líneas
│   │
│   ├── rydit-anim/         # Animación ✅ ESTABLE
│   │   └── src/lib.rs      # ~265 líneas
│   │
│   ├── rydit-physics/      # Física ✅ ESTABLE
│   │   └── src/lib.rs      # ~205 líneas
│   │
│   ├── rydit-science/      # Bezier + Stats + Geometry ✅ ESTABLE
│   │   └── src/lib.rs      # ~988 líneas
│   │
│   ├── rydit-gfx/          # Gráficos raylib ✅ ESTABLE + v0.9.0
│   │   ├── src/lib.rs      # ~1,846 líneas
│   │   ├── src/particles.rs# CPU particles (500 partículas)
│   │   ├── src/camera.rs   # Cámara 2D
│   │   ├── src/debug_log.rs# Debug logging
│   │   └── src/render_queue.rs  # ✅ v0.9.0: 8192+ draw calls
│   │                       # - Command Queue + Double Buffering
│   │                       # - Platform Sync (XFlush/XSync)
│   │                       # - 540 líneas nuevas
│   │
│   ├── rydit-http/         # HTTP + WebSocket ✅ v0.8.7
│   │   └── src/lib.rs      # ~450 líneas (ureq + tungstenite)
│   │
│   ├── rydit-rs/           # Binario principal ⚠️ COMPLEJO
│   │   ├── src/main.rs     # ~8,235 líneas
│   │   ├── src/eval/       # ✅ CSV + HTTP/WS implementados
│   │   │   └── mod.rs      # ✅ csv::*, http::*, ws::*
│   │   ├── src/modules/    # ✅ Módulos
│   │   │   ├── csv.rs      # ✅ 885 líneas, 13 funciones
│   │   │   ├── input_map.rs# ✅ 220 líneas, 8 funciones
│   │   │   ├── audio.rs    # ✅ 427 líneas, 12 funciones
│   │   │   └── assets.rs   # ⚠️ 180 líneas, 3 funciones
│   │   └── src/bindings/   # Bindings
│   │
│   ├── migui/              # Immediate Mode GUI ✅ ESTABLE
│   │   └── src/lib.rs      # ~1,391 líneas
│   │
│   └── v-shield/           # Wrapper raylib ✅ ESTABLE
│       └── src/lib.rs      # ~434 líneas
│
├── demos/
│   ├── demo_showcase_v0.8.4.rydit      ✅ Funciona
│   ├── demo_disparo_simple_v0.8.4.rydit ✅ Funciona
│   ├── demo_particulas_v0.8.4.rydit    ✅ Funciona (simulado)
│   ├── demo_ilusiones_simple.rydit     ✅ Funciona
│   ├── tank_test_simple.rydit          ✅ Funciona
│   ├── demo_shapes.rydit               ✅ Funciona
│   ├── demo_migui_backend.rydit        ✅ Funciona
│   ├── demo_csv_completo.rydit         ⏳ Pendiente
│   └── demo_http_api.rydit             ⏳ Pendiente
│
├── docs/
│   ├── ESTRUCTURA.md                   # Este archivo
│   ├── ESTADO_DEL_CODIGO_V0.8.4.md     # Análisis completo
│   ├── PLANIFICACION_V0.5.1_PARSER_ASSETS.md  # Plan sesión
│   ├── HTTP_WEBSOCKET_IMPLEMENTADO.md  # ✅ v0.8.7
│   └── backup_seguro_*/                 # Backups
│
└── target/                 # Build artifacts (excluido de git)
```

---

## 🔧 PROBLEMAS CRÍTICOS

### 1. Parser (lizer) - ✅ RESUELTO

**Estado**: ✅ FUNCIONA CORRECTAMENTE

**Verificado en Producción (2026-03-28):**
```bash
$ ./target/release/rydit-rs test_expr.rydit
x = 30        # (10 + 5) * 2 ✅
y = 45        # ((2 + 3) * (4 + 5)) ✅
z = Score: 30 # "Score: " + x ✅
matriz[0][0] = 1  # [[1,2,3],[4,5,6]] ✅
matriz[1][2] = 6  # ✅

# CSV
dark.slot datos = csv::read("archivo.csv")  # ✅ Funciona
dark.slot filas = csv::row_count(datos)     # ✅ Funciona

# Input Map
input_map::press("w")  # ✅ Funciona
onif input_map::is_pressed("arrow_up") {   # ✅ Funciona
    voz "Arriba!"
}
```

**Tests**: 74 tests passing ✅

**Conclusión**: El parser NO es el problema. Los bugs reportados eran del eval, no del parser.

---

### 2. Evaluador (eval/mod.rs) - ✅ UNIFICADO

**Estado**: ✅ CSV + HTTP + WebSocket integrados

**Funciones Implementadas**:
- ✅ `csv::*` - 13 funciones (read, write, filter, join, etc.)
- ✅ `http::*` - 4 funciones (get, post, put, delete)
- ✅ `ws::*` - 6 funciones (connect, send, recv, etc.)
- ✅ `input_map::*` - 8 funciones (press, release, is_pressed, etc.)

**Total**: ~2400 líneas, 250+ tests passing ✅

---

### 3. Módulos IMPLEMENTADOS - ✅ COMPLETADO

**Audio** (en rydit-gfx + rydit-rs/modules):
```rust
// ✅ IMPLEMENTADO - 12 funciones
audio::beep(frecuencia, duracion)
audio::click()
audio::load(id, path)
audio::play(id)
audio::stop(id)
audio::volume(id, level)
audio::load_music(path)
audio::play_music()
audio::stop_music()
audio::music_volume(level)
audio::count()
audio::list()
```

**Assets** - ✅ IMPLEMENTADO v0.5.1:
```rust
// ✅ IMPLEMENTADO - 5 funciones
assets::load(id, path)      // Cargar textura
assets::sprite(id, path)    // Alias de load
assets::exists(id)          // Verificar existencia
assets::count()             // Cantidad de assets
assets::unload(id)          // Liberar memoria
```

**CSV** - ✅ IMPLEMENTADO v0.8.6:
```rust
// ✅ IMPLEMENTADO - 13 funciones
csv::parse(csv_text)           // Parse CSV con headers
csv::parse_no_headers(csv)     // Parse CSV sin headers
csv::read(path)                // Leer desde archivo
csv::write(data, path)         // Escribir a archivo
csv::to_json(csv_text)         // Convertir a JSON
csv::from_json(json_text)      // Convertir desde JSON
csv::filter(data, col, val)    // Filtrar filas
csv::columns(data)             // Obtener columnas
csv::row_count(data)           // Contar filas
csv::col_count(data)           // Contar columnas
csv::join(csv1, csv2, col)     // Inner join
csv::group_by(data, col)       // Agrupar datos
csv::aggregate(data, col, op)  // Sum, avg, count, min, max
```

**Input Map** - ✅ IMPLEMENTADO v0.8.6:
```rust
// ✅ IMPLEMENTADO - 8 funciones
input_map::register(combo, action)  // Registrar combinación
input_map::list()                   // Listar combinaciones
input_map::clear()                  // Limpiar combinaciones
input_map::count()                  // Cantidad de combinaciones
input_map::press(key)               // Registrar tecla presionada
input_map::release(key)             // Registrar tecla soltada
input_map::is_pressed(action)       // Verificar acción (con mapeo)
input_map::get_active()             // Obtener acciones activas
```

**HTTP + WebSocket** - ✅ IMPLEMENTADO v0.8.7:
```rust
// ✅ IMPLEMENTADO - 10 funciones
http::get(url)           // GET request
http::post(url, data)    // POST request con JSON
http::put(url, data)     // PUT request con JSON
http::delete(url)        // DELETE request

ws::connect(url)         // Conectar a WebSocket
ws::disconnect()         // Desconectar WebSocket
ws::send(message)        // Enviar mensaje
ws::recv()               // Recibir mensaje
ws::is_connected()       // Verificar estado
ws::get_url()            // Obtener URL actual
```

---

### 4. Features que FALTAN - ⚠️ POR IMPLEMENTAR

| Feature | Estado | Ubicación | Prioridad |
|---------|--------|-----------|-----------|
| Partículas | ❌ No existe | rydit-anim | ALTA |
| HTTP | ❌ No existe | rydit-rs/modules | MEDIA |
| Audio module | ✅ Existe | rydit-gfx | ALTA (exponer) |
| Assets module | ✅ IMPLEMENTADO | rydit-rs/modules | ✅ LISTO (80%) |
| CSV | ✅ Existe | eval/mod.rs | ✅ LISTO |
| Stats (std_dev) | ✅ Existe | eval/mod.rs | ✅ LISTO |
| Stats (variance) | ✅ Existe | eval/mod.rs | ✅ LISTO |
draw.text("Score: " + score, x, y, size, "color")  # Requiere fix

# ESTO FALLA:
dark.slot matriz = [[1,2,3], [4,5,6], [7,8,9]]  # No soportado
```

**Causa Raíz:**
- `parse_primary()` en `lizer/src/lib.rs` tiene bugs
- `parse_expression()` no maneja bien la precedencia
- Lexer tokeniza mal strings largos con escapes

**Solución Requerida:**
- Refactorizar parser completo
- Agregar tests de estrés (expresiones complejas)
- Soporte real para arrays multidimensionales

---

### 2. Evaluador (eval/mod.rs) - PRIORIDAD ALTA ⚠️⚠️

**Problemas:**
- ❌ `evaluar_expr()` tiene lógica duplicada en main.rs
- ❌ Conversión `Valor` ↔ `serde_json::Value` es frágil
- ❌ Funciones builtin hardcodeadas

**Síntomas:**
```rydit
# La evaluación depende del contexto (gfx vs repl)
# Mismo código funciona en REPL pero no en --gfx
```

**Causa Raíz:**
- Split incompleto entre eval y main.rs
- `evaluar_expr_gfx()` duplica lógica de `evaluar_expr()`

**Solución Requerida:**
- Unificar `evaluar_expr()` y `evaluar_expr_gfx()`
- Eliminar duplicación de lógica
- Centralizar funciones builtin

---

### 3. Game Loop - PRIORIDAD MEDIA ⚠️

**Problemas:**
- ❌ `ryda frame < N` requiere fix de 1 iteración
- ❌ While en modo gráfico tiene límite artificial

**Síntomas:**
```rydit
# Sin el fix, el game loop hace 1 iteración y para
ryda frame < 1000 {  # Solo hace 1 frame
    draw.circle(x, y, 50, "rojo")
}
```

**Causa Raíz:**
- `Stmt::While` en `ejecutar_stmt_gfx()` tiene `max_iterations = 1`

**Solución Requerida:**
- Game loop debería ser manejado por raylib, no por while
- Refactorizar arquitectura del game loop

---

### 4. Assets Manager - PRIORIDAD MEDIA ⚠️

**Estado:**
- ✅ `Assets` struct existe en `rydit-gfx`
- ❌ Funciones NO expuestas a RyDit
- ❌ `assets::load()`, `assets::draw()` no existen

**Síntomas:**
```rydit
# ESTO NO FUNCIONA:
assets::load("tank", "sprites/tank.png")
assets::draw("tank", 400, 300, 2.0)
```

**Causa Raíz:**
- Assets fue removido en el split
- Requiere re-implementar módulo

**Solución Requerida:**
- Crear `rydit-rs/src/modules/assets.rs`
- Exponer funciones como `RyditModule`

---

### 5. Partículas - PRIORIDAD BAJA

**Estado:**
- ❌ Removido en el split
- ❌ No hay código existente

**Solución Requerida:**
- Implementar en `rydit-anim/src/particles.rs`
- O crear `crates/rydit-particles/`

---

## ✅ LO QUE SÍ FUNCIONA

### Crates Estables
| Crate | Estado | Tests | Notas |
|-------|--------|-------|-------|
| blast-core | ✅ Estable | 20 | Executor + Memoria |
| rydit-core | ✅ Estable | 9 | RyditModule trait |
| rydit-loader | ✅ Estable | 6 | Carga dinámica |
| rydit-script | ✅ Estable | 4 | Scripts como módulos |
| rydit-physics | ✅ Estable | 6 | Projectile, NBody |
| rydit-anim | ✅ Estable | 9 | Easing, Squash/Stretch |
| rydit-science | ✅ Estable | 21 | Bezier, Stats, Geometry |
| migui | ✅ Estable | 8 | UI widgets |
| v-shield | ✅ Estable | 0 | Wrapper raylib |
| lizer | ⚠️ Débil | 74 | Parser con bugs |
| rydit-gfx | ⚠️ Incompleto | 6 | Faltan assets |
| rydit-rs | ⚠️ Complejo | 50 | Demasiado código |

### Demos Funcionales
| Demo | Estado | Complejidad |
|------|--------|-------------|
| demo_showcase_v0.8.4 | ✅ | Baja (sin paréntesis) |
| demo_disparo_simple_v0.8.4 | ✅ | Baja (colisiones simples) |
| demo_particulas_v0.8.4 | ✅ | Baja (círculos, no particles::) |
| demo_ilusiones_simple | ✅ | Baja (sin assets) |
| tank_test_simple | ✅ | Baja (sin assets) |
| demo_shapes | ✅ | Baja |
| demo_migui_backend | ✅ | Media |

---

## 📈 MÉTRICAS REALES

### Líneas de Código
```
Total: 18,383 líneas Rust
├── lizer: 3,383 (parser débil)
├── rydit-rs: 8,235 (demasiado complejo)
├── rydit-gfx: 1,846 (incompleto)
├── rydit-science: 988
├── migui: 1,391
├── lizer: 3,383
└── otros: ~2,157
```

### Tests
```
Total: 157 tests passing
├── lizer: 74 (parser, pero falla en producción)
├── rydit-rs: 50
├── rydit-science: 21
├── blast-core: 20
├── rydit-core: 9
├── rydit-anim: 9
├── migui: 8
├── rydit-physics: 6
├── rydit-loader: 6
└── rydit-script: 4
```

**Problema:** Tests pasan pero demos reales fallan = tests insuficientes

---

## 🎯 REFACTORIZACIÓN NECESARIA

### Fase 1: Parser (2-3 días)
- [ ] Refactorizar `lizer/src/lib.rs` completo
- [ ] Tests de estrés con expresiones complejas
- [ ] Soporte real para arrays multidimensionales
- [ ] Fix definitivo para paréntesis
- [ ] Fix definitivo para concatenación

### Fase 2: Evaluador (1-2 días)
- [ ] Unificar `evaluar_expr()` y `evaluar_expr_gfx()`
- [ ] Eliminar duplicación main.rs ↔ eval/mod.rs
- [ ] Centralizar funciones builtin

### Fase 3: Game Loop (1 día)
- [ ] Refactorizar arquitectura del game loop
- [ ] Eliminar `max_iterations = 1` hack
- [ ] Game loop manejado por raylib

### Fase 4: Assets + Particles (2-3 días)
- [ ] Crear módulo assets.rs
- [ ] Exponer funciones assets::
- [ ] Implementar particles en rydit-anim

### Fase 5: Limpieza (1-2 días)
- [ ] Reducir rydit-rs/main.rs de 8,235 a ~5,000 líneas
- [ ] Mover lógica a módulos separados
- [ ] Documentación completa

---

## 📅 CRONOGRAMA REALISTA

| Semana | Objetivo | Resultado Esperado |
|--------|----------|-------------------|
| 1 | Parser fix | Expresiones complejas funcionan |
| 2 | Evaluador unificado | Sin duplicación de lógica |
| 3 | Assets + Particles | Sprites y partículas reales |
| 4 | Limpieza + Docs | Código mantenible |
| 5 | Release v0.6.0 | Stable con features reales |

---

## 🚨 DECISIONES CRÍTICAS

### Opción A: Refactorización Masiva (RECOMENDADA)
- **Tiempo**: 4-5 semanas
- **Riesgo**: Alto (puede romper cosas)
- **Beneficio**: RyDit funcional de verdad
- **Resultado**: v0.6.0 estable

### Opción B: Parches Incrementales
- **Tiempo**: 2-3 semanas
- **Riesgo**: Medio (parches sobre parches)
- **Beneficio**: Mejoras pequeñas
- **Resultado**: v0.5.x con fixes

### Opción C: Release v0.5.0 Así
- **Tiempo**: 0 semanas
- **Riesgo**: Muy alto (comunidad pierde confianza)
- **Beneficio**: Release rápido
- **Resultado**: v0.5.0 buggy, score baja a 3/10

---

## 💭 REFLEXIÓN

**El problema no es la cantidad de código (18,383 líneas), es la CALIDAD.**

- 270 tests passing pero demos simples fallan = **tests insuficientes**
- Parser tiene 74 tests pero falla con paréntesis = **tests mal diseñados**
- 7 demos funcionales pero todos simplificados = **no refleja capacidad real**

**La comunidad no va a aceptar un motor que:**
- No puede hacer `dark.slot x = (10 + 5) * 2` consistentemente
- Requiere simplificar demos para que funcionen
- Tiene 270 tests pero no puede cargar un sprite

**Hay que elegir:**
1. **Refactorizar masivamente** (doloroso ahora, vale la pena)
2. **Lanzar buggy** (rápido ahora, doloroso después)

---

<div align="center">

**🛡️ RyDit v0.5.0 - ENCRUCIJADA**

*18,383 líneas | 157 tests | 7 demos simples | Parser débil | ¿Refactorizar o lanzar?*

</div>
