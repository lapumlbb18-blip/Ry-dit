# 🛡️ RyDit - ESTRUCTURA DEL PROYECTO

**Última actualización**: 2026-03-26
**Versión**: v0.5.0
**Estado**: ⚠️ REQUIERE REFACTORIZACIÓN MASIVA

---

## 📊 ESTADO REAL (SIN FILTROS)

### Puntuación Actual: 5/10 ⬇️ (era 8/10)

**Razones del descenso:**
- ❌ Parser débil - no soporta expresiones complejas
- ❌ Demos simplificados constantemente (no reflejan capacidad real)
- ❌ Arrays limitados - no hay operaciones completas
- ❌ Concatenación rota - requiere fix constante
- ❌ Líneas largas - parser se rompe
- ❌ 270 tests passing pero demos reales no funcionan

---

## 📁 ESTRUCTURA ACTUAL

```
shield-project/
├── crates/
│   ├── lizer/              # Lexer + Parser ⚠️ DEBIL
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
│   ├── rydit-anim/         # Animación ⚠️ FALTA PARTICLES
│   │   └── src/lib.rs      # ~265 líneas
│   │
│   ├── rydit-physics/      # Física ✅ ESTABLE
│   │   └── src/lib.rs      # ~205 líneas
│   │
│   ├── rydit-science/      # Bezier + Stats + Geometry ⚠️ FALTA CSV
│   │   └── src/lib.rs      # ~988 líneas
│   │
│   ├── rydit-gfx/          # Gráficos raylib ⚠️ FALTA ASSETS EXPOSED
│   │   └── src/lib.rs      # ~1,846 líneas
│   │
│   ├── rydit-rs/           # Binario principal ⚠️ COMPLEJO DEMASIADO
│   │   ├── src/main.rs     # ~8,235 líneas (DEMASIADO)
│   │   ├── src/eval/       # Evaluador
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
│   └── demo_migui_backend.rydit        ✅ Funciona
│
└── docs/
    ├── ESTADO_DEL_CODIGO_V0.8.4.md     # Análisis completo
    ├── PLANIFICACION_V0.5.1_AUDIO_HTTP.md
    └── backup_seguro_*/                 # Backups
```

---

## 🔧 PROBLEMAS CRÍTICOS

### 1. Parser (lizer) - PRIORIDAD ALTA ⚠️⚠️⚠️

**Problemas:**
- ❌ Paréntesis fallan en ciertos contextos
- ❌ Expresiones largas se rompen
- ❌ Arrays multidimensionales limitados
- ❌ Concatenación string + variable requiere fix
- ❌ Comentarios largos causan problemas

**Síntomas:**
```rydit
# ESTO FALLA:
dark.slot x = (10 + 5) * 2  # A veces funciona, a veces no

# ESTO FALLA:
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
