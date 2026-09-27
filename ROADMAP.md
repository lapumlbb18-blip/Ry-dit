# Ry-Dit - ROADMAP v0.27.0 → v1.0.0

**Última actualización**: 2026-09-27
**Versión actual**: v0.27.0 (en curso) — TUI ✅ · Joysticks ✅ · Tests ✅
**Próximas versiones**: v0.28.0 Memoria + MCP → v0.29.0 Modelo local + SDK
**Análisis estratégico**: Los 3 pilares (render, input, audio) están completos y estables. La suite de tests del workspace está **verde por primera vez en meses** (575 ✓ / 0 ✗), lo que desbloquea avances. El nuevo eje es convertir los registros de tests en **memoria procedimental** para agentes de IA (locales y de nube) y entrenar un modelo propio.

---

## Estado Actual (v0.27.0)

| Métrica | Valor |
|---------|-------|
| **Arquitectura** | Ensamblador Modular (SDL2 + RLGL + Audio) backend-agnóstico |
| **Crates** | 25 (workspace) |
| **Código** | ~79,000 líneas Rust (224 archivos) |
| **Compilación** | `cargo build --release` limpio ✅ |
| **Tests** | **586 registrados: 575 ✓ / 0 ✗ / 11 ignored** ✅ |
| **Registro de memoria** | `ry-memory/test-report.yaml` por versión ✅ |
| **Input Termux-X11** | WASD + flechas + ventana de gracia 150ms ✅ |
| **Gamepads** | SDL2 GameController: dead zones, rumble, hotplug ✅ |
| **Audio** | SDL2_mixer + PulseAudio + procedural 8-bit ✅ |
| **Render** | SDL2 + RLGL batch flush sin parpadeo ✅ |
| **TUI** | Widgets de terminal (estilo Go bubbletea) ✅ |
| **Repositorio** | `github.com/lapumlbb18-blip/Ry-dit` (push retomado 2026-09-27) |

---

## 🧭 VISIÓN — Motor con memoria propia

Ry-Dit **no compite con motores de escritorio** (Godot, Bevy, Love2D). Su alcance es un **SDK de motor para Termux/Android low-end, con memoria procedimental y bot de IA**:

1. **Motor**: render + input + audio + scripting `.rydit`, modular y backend-agnóstico.
2. **Memoria**: cada versión congela sus tests en `ry-memory/*.yaml` — estado real comprimido.
3. **Agentes**: MCPs de código conectan modelos (nube o locales) al repo: leer → editar → test → YAML.
4. **Modelo propio**: fine-tune en Colab con fundamentos de Rust + registros Ry-Dit, sin contexto innecesario.

---

## Próximas Tareas (v0.27.0 — en curso)

| Tarea | Descripción | Estado |
|-------|-------------|--------|
| **TUI Línea de Comandos** | Widgets de terminal renderizados vía SDL video (estilo Go bubbletea) | ✅ `ry-rs/src/tui/` + `demo_tui` |
| **Mandos y Joysticks** | Gamepad SDL2: botones/ejes, dead zones radiales, rumble, hotplug | ✅ `events-ry/src/gamepad.rs` + `demo_gamepad` |
| **Suite de tests workspace** | Reparación de tests antiguos + registro YAML en `ry-memory/` | ✅ 575/575 verdes |
| **Texto y Números** | Render TTF/ab_glyph en curso, UTF-8, anchos variables | 🔄 ~30% |
| **HUDs y UI** | Health bars, paneles, botones, menús (migui) | 🔄 ~25% |
| **Assets Reales** | Sprites PNG, fuentes TTF, pipeline de carga tipado | 🔄 ~20% |
| **Toolkits y Apps** | Herramientas construidas con el motor (no solo juegos) | ⏳ 0% |
| **CI en GitHub Actions** | Runners multiplataforma en paralelo a los demos: variante *tests headless* (SDL dummy, sin display) y variante *demos smoke* (Xvfb). Linux estricto; macOS y Windows experimentales hasta verde. Plataformas de Rust+SDL2+raylib: Linux, macOS, Windows (Termux/Android se valida local) | 🔄 `ci.yml` creado |

---

## 🤖 Nueva Línea: Memoria + IA Local

| Versión | Foco | Componentes | Estado |
|---------|------|-------------|--------|
| **v0.28.0** | **Memoria Procedimental + MCP** | `ry-memory/vX.Y.Z/` por versión (YAML de tests + snapshot ROADMAP + diff de crates); MCPs: `mcp-tests`, `mcp-build`, `mcp-arch`, `mcp-diff` (stdio JSON-RPC) | ⏳ |
| **v0.29.0** | **Modelo local + SDK con bot** | Fine-tune en Colab (fundamentos Rust + registros Ry-Dit, sin contexto de más); inferencia local (llama.cpp/Ollama en Termux, modelos 1–3B Q4); binario `rydit agent` con loop: roadmap → tarea → patch → `cargo test` → YAML | ⏳ |
| **v0.30.0** | Editor v1.0 Alpha + Multi-ventana | Editor de escenas sobre el motor | ⏳ |
| **v1.0.0** | Motor Universal Estable | API congelada, docs completas | ⏳ |

> **Nota de alcance**: los agentes de nube y los locales comparten el mismo registro YAML. La nube no lo necesita (contexto grande), el modelo local **sí**: la memoria procedimental es su ventaja, no un extra.

---

## Versiones Completadas (Reciente)

### v0.26.0 — Audio + Render + Input Completo ✅

**Fecha**: 2026-08-31

| Feature | Estado | Detalle |
|---------|--------|---------|
| Audio Procedural 8-bit | ✅ | WAV en memoria: shoot, explosion, ambient loop |
| PulseAudio Integrado | ✅ | `ensure_pulseaudio()` al arrancar (patrón Chrome/Firefox) |
| SDL2_mixer FFI | ✅ | FFI crudo para Mix_OpenAudio en Termux-X11 |
| Fix rlgl Batch Flush | ✅ | `shim.rs::present()` antes de `gl_swap_window` |
| Fix InputManager + mapeo | ✅ | WASD + flechas + Shift/Ctrl/Alt + NumPad |
| Ventana de Gracia 150ms | ✅ | Teclado Android en Termux-X11 |

```
Progreso v0.26.0: ████████████████████ 100%
```

---

## Progreso General hacia v1.0.0

```
v0.24.0   ████████████████████ 100%
v0.25.0   ████████████████████ 100%
v0.26.0   ████████████████████ 100%
v0.27.0   ██████████░░░░░░░░░░  50% (TUI ✅ Joysticks ✅ Tests ✅ CI 🔄 | Texto/HUDs/Assets en curso)
v0.28.0   ░░░░░░░░░░░░░░░░░░░░   0% (Memoria + MCP)
v0.29.0   ░░░░░░░░░░░░░░░░░░░░   0% (Modelo local + SDK)
v1.0.0    ░░░░░░░░░░░░░░░░░░░░   0%
```

---

<div align="center">

**Ry-Dit v0.27.0 - ROADMAP ACTUALIZADO (2026-09-27)**

*Tests verdes por primera vez en meses → memoria procedimental desbloqueada.*

*Próximo: completar v0.27 (Texto/HUDs/Assets), luego v0.28 Memoria+MCP y v0.29 Modelo+SDK.*

</div>
