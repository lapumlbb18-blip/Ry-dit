# Ry-Dit - ROADMAP v0.13.0 -> v1.0.0

**Última actualización**: 2026-04-05
**Versión actual**: v0.13.0 ✅ Math + Arrays + Vec2 + toolkit-ry + ry3d-gfx

---

## Estado Actual

| Métrica | Valor |
|---------|-------|
| **Crates** | 23 (21 ry-* + 2 externos) |
| **Líneas Rust** | ~25K+ |
| **Compilación** | 0 errores |
| **Warnings** | ~43 (demos, dead_code) |
| **Parser** | 95% features |
| **Tests parser** | test_parser funcional |
| **Repositorio** | `github.com/lapumlbb18-blip/Ry-dit` |

---

## Versiones Planificadas

### v0.12.0 - Parser Infalible (COMPLETADA)

**Fecha**: 2026-04-04
**Commits**: `68c8593` -> `3d80f39` (7 commits)

| Feature | Estado |
|---------|--------|
| Rebrand: rydit-* -> ry-* | |
| Repositorio: Ry-Dit | |
| Separar Asignar (=) de Igual (==) | |
| `romper` alias de `break` | |
| `ryda { }` sin condición | |
| Array literals [1, 2, 3] | |
| `dark.slot[] name = [...]` | |
| `ident = expr` asignación | |
| `matematica::` namespace | |
| `fps()` builtin | |
| `texto "X" en A, B` syntax | |
| Fix raíz: self.advance() | |
| Tests viejos movidos | |
| Documentación parser | |


### v0.12.1 - Ry-god publicado + CI/CD (COMPLETADA)

**Fecha**: 2026-04-04
**Commits**: `8ff8814` -> `22252bc` (6 commits)

| Feature | Estado |
|---------|--------|
| ry-god creado (Security & Efficiency) | |
| Test revelacion: 15/15 tests | |
| Verificacion: 13/13 crates compilando | |
| ry-god publicado en crates.io v0.1.0 | |
| GitHub Actions CI/CD configurado | |
| 0 errores compilacion workspace | |

### v0.13.0 - Math + Arrays + Vec2 + toolkit-ry + ry3d-gfx (COMPLETADA)

**Fecha**: 2026-04-05
**Commits**: `2f210b7` -> `7b010d1` (HEAD, 8 commits)

| Feature | Estado |
|---------|--------|
| Math avanzado: 23 funciones (pow, log, exp, PI, E, TAU...) | |
| Cálculo numérico: derivada, derivada2, integral, integral_trapezio | |
| Arrays completos: 16 funciones (push, pop, slice, contains...) | |
| Vec2 tipo nativo: 22 operaciones (add, sub, normalize, dot...) | |
| toolkit-ry v0.1.0: 5 temas + 20+ widgets | |
| ry3d-gfx v0.1.0: 15 funciones 3D | |
| Fix input Android: SDL_TEXTINPUT + 7 hints | |
| Limpieza: -17,604 líneas basura | |
| Organización docs en actuales/, antiguos/, sessions/ | |
| 23 crates compilando | 0 errores |

### v0.14.0 - Quest System + Save/Load (EN PROGRESO)

**Prioridad**: ALTA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| Quest system | | 15-20h |
| Save/Load game | | 8-12h |
| One-way platforms | | 4-6h |
| ry-stream v0.1.1 crates.io | | 4-6h |
| Tilemap hexagonal | | 15-20h |
| ry-anim: neon.rs, fx.rs, bw.rs | | 10-15h |

### v0.15.0 - Physics + Science Avanzada

**Prioridad**: MEDIA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| v-shield platform layer | | 15-20h |
| ry-physics N-cuerpos >2 | | 15-20h |
| ry-science FFT, fractales | | 15-20h |
| ry-geometry Vec3/Mat4 | | 12-16h |

### v0.16.0 - Integraciones + Editor

**Prioridad**: MEDIA

| Feature | Estado | Tiempo est. |
|---------|--------|-------------|
| LAZOS Python bridge | | 20-30h |
| Camera3D + DrawCube | | 12-16h |
| Editor visual | | 24-32h |
| Discord bot integration | | 10-15h |

### v1.0.0 - Lanzamiento Público

**Prioridad**: META

| Feature | Estado | Notas |
|---------|--------|-------|
| Parser 100% funcional | | Sin errores conocidos |
| 5+ demos funcionales | | Termux-X11 |
| Crates publicados | | crates.io |
| Documentación completa | | Guía usuario + dev |
| Videos tutoriales | | YouTube |
| README completo | | Con galería |
| GitHub Actions CI | | Build automático |

---

## Progreso General

```
v0.12.0 100%
v0.12.1 100%
v0.13.0 100%
v0.14.0 0%
v0.15.0 0%
v0.16.0 0%
v1.0.0  10%
```

---

## Tabla de Versiones

| Versión | Fecha | Commits | Crates | Errores | Warnings | Features Clave |
|---------|-------|---------|--------|---------|----------|----------------|
| v0.11.4 | 2026-04-02 | 20+ | 18 | 0 | ~3 | Lifetimes fix |
| v0.11.5 | 2026-04-02 | 10+ | 18 | 0 | ~5 | 0 errores final |
| v0.12.0 | 2026-04-04 | 7 | 22 | 0 | ~30 | Rebrand + Parser |
| v0.12.1 | 2026-04-04 | 6 | 22 | 0 | ~30 | ry-god crates.io |
| **v0.13.0** | **2026-04-05** | **8** | **23** | **0** | **~43** | **Math+Arrays+Vec2+toolkit+3D** |
| v0.14.0 | 2026-04-xx | - | 23 | - | - | Quest+SaveLoad |
| v1.0.0 | Futuro | - | 25+ | - | - | Motor completo |

---

## Objetivos a Largo Plazo

1. **Motor 2D completo** para Termux-X11
2. **Lenguaje de scripting** en español
3. **Comunidad** de desarrolladores
4. **Multiplataforma**: Android, Linux, Windows
5. **Editor visual** integrado

---

<div align="center">

**Ry-Dit v0.13.0 - ROADMAP**

*Math avanzado | Arrays completos | Vec2 nativo | toolkit-ry | ry3d-gfx | 23 crates*

</div>
