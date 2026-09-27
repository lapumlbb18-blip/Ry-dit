# 🛡️ Ry-Dit

**Motor de juegos 2D + lenguaje de scripting `.rydit`, escrito en Rust y diseñado para Termux/Android (gama baja, low-end first).**

`v0.27.0 (en curso)` · 25 crates · ~79,000 líneas Rust · **586 tests: 575 ✓ / 0 ✗ / 11 ignored**

---

## Pilares funcionales

| Pilar | Estado | Detalle |
|-------|--------|---------|
| **Render** | ✅ | SDL2 + RLGL con batch flush sin parpadeo, GPU instancing, FSR 1.0 |
| **Input** | ✅ | Teclado/mouse con ventana de gracia 150ms (Termux-X11) + **gamepads SDL2** (dead zones, rumble, hotplug) |
| **Audio** | ✅ | SDL2_mixer + PulseAudio + sonidos procedurales 8-bit |
| **Texto/TUI** | 🔄 | Fuentes TTF (ab_glyph + SDL2_ttf), widgets TUI estilo bubbletea |
| **HUDs/UI** | 🔄 | Health bars, paneles, menús (migui) |
| **Assets** | 🔄 | Pipeline PNG/TTF con tipos fuertes (AssetServer) |

## Inicio rápido

```bash
# Requisitos (Termux)
pkg install rust clang SDL2 SDL2_image SDL2_ttf SDL2_mixer raylib x11-repo

# Compilar (siempre release para builds finales)
cargo build --release

# Demo principal
./target/release/rydit-rs

# Demo de gamepad (panel de botones + ejes + rumble)
./target/release/demo_gamepad

# Demo TUI
./target/release/demo_tui

# Tests del workspace (verde = 0 fallos)
cargo test --workspace --no-fail-fast -j 4
```

Guía completa: **[GUIA_USUARIO.md](GUIA_USUARIO.md)** · Ruta de desarrollo: **[ROADMAP.md](ROADMAP.md)**

## Estructura

```
crates/
├── ry-core / ry-god        # Núcleo y Ensamblador (ModuleRegistry)
├── ry-rs                   # Puerta principal: prelude, RybotEngine, demos, TUI
├── ry-backend / ry-gfx     # Ventana+GL (SDL2/RLGL), render queue, audio, HUD
├── events-ry / ry-input    # Input unificado 3 capas: teclado, mouse, gamepad
├── ry-lexer / ry-parser    # Front-end del lenguaje .rydit
├── ry-anim / ry-physics    # Animación (12 principios Disney) y física
├── migui                   # UI inmediata (paneles, menús, fuentes ab_glyph)
└── ry-loader / ry-config   # Assets tipados y configuración
```

## Memoria procedural (nuevo)

Los tests del workspace se registran por versión en **`ry-memory/`** como YAML:

```
ry-memory/
├── raw-test-output.txt   # log crudo de cargo test (local)
├── test-report.yaml      # reporte estructurado: suites, tests, fallos
└── parse_cargo_test.py   # parser reproducible crudo → YAML
```

Este registro es **memoria procedimental**: permite que un modelo de IA (local o nube) sepa en segundos el estado real del motor sin releer 79k líneas. Cada versión congelada = un snapshot de memoria.

## Camino de IA local

Ry-Dit no compite con editores de escritorio (Godot/Bevy): su alcance es **SDK de motor + memoria + bot**.

1. **Registros por versión** en `ry-memory/` (hecho) — el insumo de entrenamiento y de contexto.
2. **MCPs de código** — servidores `mcp-tests` (lee YAMLs), `mcp-build` (cargo test/build), `mcp-arch` (grafo de crates).
3. **Modelo propio** — fine-tune en Google Colab con bases de Rust + los registros Ry-Dit, sin contexto innecesario.
4. **SDK con bot** — binario `rydit` con agentes (nube y locales) que ciclan: leer roadmap → editar → `cargo test` → regenerar YAML.

## Roadmap resumido

| Versión | Foco | Estado |
|---------|------|--------|
| v0.24–v0.26 | Ensamblador, input Termux, audio+render | ✅ |
| **v0.27.0** | TUI, Joysticks, Texto, HUDs, Assets | 🔄 45% (TUI ✅ Joysticks ✅ tests ✅) |
| v0.28.0 | Memoria procedural + MCPs | ⏳ |
| v0.29.0 | Modelo local (Colab) + SDK `rydit agent` | ⏳ |
| v1.0.0 | Motor universal estable | ⏳ |

---

**Filosofía**: *Modularidad Extrema, Control Total.* Un motor para ensamblar el futuro del desarrollo híbrido — con memoria propia.
