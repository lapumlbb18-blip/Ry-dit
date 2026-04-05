# 🛡️ Ry-Dit - ESTRUCTURA DEL PROYECTO v0.13.0

**Última actualización**: 2026-04-04
**Versión**: v0.13.0 ✅ Math avanzado + Arrays completos + Cálculo numérico
**Commit**: Ver `git log -n 1`
**Estado**: ✅ cargo check --workspace: 0 errores | 22 crates compilando

---

## 🎯 ARQUITECTURA ACTUAL

```
shield-project/
├── Cargo.toml                  # Workspace (22 crates)
├── README.md                   # Documentación principal
├── ROADMAP.md                  # Planificación v0.13→v1.0
├── CONTRIBUTING.md             # Guía contribuidores
├── MANIFIESTO.md               # Filosofía del proyecto
├── LICENSE                     # MIT
├── .gitignore
│
├── crates/
│   ├── ry-core/                # ✅ 0.8.2  Core traits, module system, Valor
│   ├── ry-lexer/               # ✅ 0.1.0  Zero-copy lexer
│   ├── ry-parser/              # ✅ 0.1.0  Parser AST + error recovery
│   ├── ry-vm/                  # ⚠️       VM opcodes + compiler
│   ├── ry-gfx/                 # ⚠️ 0.10.7 Graphics (raylib + SDL2 + OpenGL FFI)
│   ├── ry-physics/             # ✅ 0.7.34 2D projectile + N-body (2 cuerpos)
│   ├── ry-anim/                # ✅ 0.7.34 Easing + Disney principles
│   ├── ry-ecs/                 # ✅ 0.10.0 ECS (bevy_ecs)
│   ├── ry-science/             # ⚠️       Geometry 2D + stats + Bezier
│   ├── ry-script/              # ✅ 0.8.2  Script loading
│   ├── ry-stream/              # ✅ 0.1.0  LAN streaming (WebSocket)
│   ├── ry-god/                 # ✅ 0.1.0  Security & efficiency (crates.io)
│   ├── ry-loader/              # ⚠️       Module loader
│   ├── ry-rs/                  # —        Main binary + demos + eval + modules
│   ├── ry-system-ry/           # ⚠️ 0.11.0 Universal system (SDL2)
│   ├── ry-test/                # ⚠️       Test utilities
│   ├── toolkit-ry/             # ⚠️ 0.1.0  UI toolkit (SDL2)
│   ├── migui/                  # ⚠️       Immediate mode GUI (12 widgets)
│   ├── blast-core/             # ⚠️ 0.1.0  Minimal value executor
│   ├── lizer/                  # ✅ 0.11.2 Legacy lexer wrapper
│   └── v-shield/               # ⚠️       (por definir)
│
├── crates/ry-rs/src/
│   ├── main.rs                 # Entry point + eval modo gráfico + modules
│   ├── eval/
│   │   └── mod.rs              # Evaluar expresiones (~4100 líneas)
│   │                           #   - Math: sin, cos, tan, sqrt, pow, log...
│   │                           #   - Arrays: push, pop, len, slice, insert...
│   │                           #   - Cálculo: derivada, integral (Simpson)
│   │                           #   - Strings, regex, CSV, JSON, random...
│   ├── module.rs               # Registro de módulos (math, arrays, strings...)
│   ├── json_helpers.rs         # Conversión Serde ↔ Valor
│   ├── rybot/                  # Asistente de código
│   └── modules/                # Módulos del lenguaje
│       ├── assets.rs           # Carga/dibujo de sprites PNG
│       ├── audio.rs            # SDL2_mixer (tonos, WAV)
│       ├── camera.rs           # Camera2D (posición, zoom, rotación)
│       ├── collision.rs        # Colisiones 2D (AABB, raycast)
│       ├── csv.rs              # CSV parser + queries
│       ├── entity.rs           # Sistema de entidades
│       ├── input_map.rs        # Input SDL2 mapeado
│       ├── level.rs            # Gestión de niveles/checkpoints
│       ├── physics.rs          # Física 2D (gravedad, proyectiles)
│       ├── tilemap.rs          # Tilemap system
│       └── window.rs           # Creación de ventana SDL2
│
├── crates/ry-parser/src/
│   ├── lib.rs                  # API pública
│   ├── ast.rs                  # Expr<'a>, Stmt<'a>, BinaryOp, UnaryOp
│   ├── parser.rs               # Parser completo (~1500 líneas)
│   │   ├── parse_primary()     # Literales, arrays [], vars, calls
│   │   ├── parse_term()        # *, /, +, - (precedencia)
│   │   ├── parse_expression()  # Comparaciones, lógicos
│   │   ├── parse_statement()   // Assign, if, ryda, funciones
│   │   └── parse_texto_en()    // Texto en A, B con expresiones
│   └── token.rs                # TokenKind (60+ tipos)
│
├── crates/ry-lexer/src/
│   ├── lib.rs                  # API pública
│   ├── lexer.rs                # Zero-copy Lexer
│   └── token.rs                # Token<'a> zero-copy
│
├── crates/ry-gfx/src/
│   ├── lib.rs                  # Graphics layer (~1700 líneas)
│   ├── camera.rs               # Camera2D
│   ├── gpu_instancing.rs       # OpenGL FFI + instancing
│   ├── render_queue.rs         # Cola de renderizado
│   ├── ecs_render.rs           # ECS renderer
│   ├── fsr.rs                  # FSR 1.0 upscaling
│   └── shaders/                # GLSL shaders
│       ├── vertex.glsl
│       └── fragment.glsl
│
├── demos/                      # Scripts .rydit
├── docs/
│   ├── actuales/
│   ├── antiguos/               # Docs de versiones previas
│   │   ├── sdl2/               # Guías SDL2
│   │   ├── demos/              # Documentación de demos
│   │   └── guias/              # Guías varias
│   ├── sessions/               # Logs de sesiones
│   ├── tests/
│   ├── tests_referencia/
│   └── panorama_v0.13.0.md     # 🗺️ Panorama completo v0.13→v1.0
│
├── screenshots/                # Capturas y videos MP4
├── scripts/                    # Utilidades bash/python
├── tests/                      # Tests automáticos
├── tests_rydit/                # Tests del lenguaje
└── ejemplos-gfx/               # Demos gráficos pendientes
    ├── pendientes/
    └── pendientes-revision/
```

---

## 📊 FUNCIONES POR MÓDULO

### math:: / matematica::
| Función | Args | Retorna |
|---------|------|---------|
| `sin, cos, tan` | 1 | f64 |
| `sqrt` | 1 | f64 |
| `pow` | 2 | f64 |
| `log, log10` | 1 | f64 |
| `exp` | 1 | f64 |
| `abs` | 1 | f64 |
| `floor, ceil, round, trunc, fract` | 1 | f64 |
| `min, max` | 2 | f64 |
| `clamp` | 3 | f64 |
| `lerp` | 3 | f64 |
| `sign` | 1 | f64 |
| `mod` | 2 | f64 |
| `hypot` | 2 | f64 |
| `cbrt` | 1 | f64 |
| `atan2` | 2 | f64 |
| `deg2rad, rad2deg` | 1 | f64 |
| **Constantes**: `PI`, `E`, `TAU`, `INF` | 0 | f64 |

### calc::
| Función | Args | Retorna |
|---------|------|---------|
| `derivada(f, x, h)` | 2-3 | f64 |
| `derivada2(f, x, h)` | 2-3 | f64 |
| `integral(f, a, b, n)` | 4 | f64 |
| `integral_trapezio(f, a, b, n)` | 4 | f64 |

### arrays::
| Función | Args | Retorna |
|---------|------|---------|
| `push(arr, elem)` | 2 | array |
| `pop(arr)` | 1 | elem |
| `shift(arr)` | 1 | elem |
| `unshift(arr, elem)` | 2 | array |
| `slice(arr, start, end)` | 3 | array |
| `reverse(arr)` | 1 | array |
| `len(arr)` | 1 | num |
| `insert(arr, idx, elem)` | 3 | array |
| `remove(arr, idx)` | 2 | elem |
| `contains(arr, elem)` | 2 | bool |
| `find(arr, elem)` | 2 | num |
| `join(arr, sep)` | 2 | texto |
| `clear(arr)` | 1 | array |
| `first(arr)` | 1 | elem |
| `last(arr)` | 1 | elem |

---

## 🏗️ PIPELINE DE EJECUCIÓN

```
Código .rydit
    │
    ▼
┌─────────────┐
│  ry-lexer   │  Zero-copy scan → tokens
└──────┬──────┘
       │ Token<'a>
       ▼
┌─────────────┐
│ ry-parser   │  Error recovery → AST
└──────┬──────┘
       │ Expr<'a>, Stmt<'a>
       ▼
┌─────────────┐
│ evaluar_expr│  Evaluar expresiones (eval/mod.rs)
│ ejecutar_stmt│ Ejecutar statements (main.rs)
└──────┬──────┘
       │ Valor (Num, Texto, Bool, Array)
       ▼
┌─────────────┐
│   ry-gfx    │  SDL2/raylib render
└─────────────┘
```

---

## 📦 CRATES PUBLICABLES

| Crate | Versión | Estado | Notas |
|-------|---------|--------|-------|
| ry-god | 0.1.0 | ✅ crates.io | Security & efficiency |
| ry-core | 0.8.2 | ✅ Listo | Core traits |
| ry-lexer | 0.1.0 | ✅ Listo | Zero-copy |
| ry-parser | 0.1.0 | ✅ Listo | Error recovery |
| ry-physics | 0.7.34 | ✅ Listo | 2D projectile |
| ry-anim | 0.7.34 | ✅ Listo | Easing |
| ry-ecs | 0.10.0 | ✅ Listo | bevy_ecs |
| ry-script | 0.8.2 | ✅ Listo | Script loading |
| ry-stream | 0.1.0 | ✅ Listo | WebSocket |
| lizer | 0.11.2 | ✅ Listo | Legacy |
| toolkit-ry | 0.1.0 | ⚠️ | Falta license |
| ry-system-ry | 0.11.0 | ⚠️ | Falta license |

---

<div align="center">

**🛡️ Ry-Dit v0.13.0 — ESTRUCTURA ACTUALIZADA**

*22 crates | 25K+ líneas Rust | Math + Arrays + Cálculo numérico*

*Última actualización: 2026-04-04*

</div>
