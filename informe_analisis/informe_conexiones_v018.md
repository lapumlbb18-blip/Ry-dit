# 🔗 MAPA DE CONEXIONES RY-DIT — v0.18.0+

**Fecha**: 2026-04-11
**Tipo**: Análisis de estado actual + conexiones pendientes
**Filosofía**: No es Godot (nodos/escenas) — es Sistema Universal Ry

---

## 📦 LO QUE YA EXISTE (parcial o completamente)

### ✅ Crates Completos (funcionan solos)

| Crate | Versión | Estado | Función | Demos que lo usan |
|-------|---------|--------|---------|-------------------|
| **ry-core** | 0.8.2 | ✅ | Traits base, RyDitModule | TODOS |
| **ry-anim** | 0.12.0 | ✅ | 12 Disney principles + action_assets + particles + transitions | demo_anime_ry_v2, demo_transitions |
| **ry-gfx** | 0.10.8 | ✅ | GPU instancing + FSR + particles + emoji_atlas + audio_mixer + transitions + font_system | demo_militar, demo_emoji_utf8, demo_audio_mixer |
| **ry3d-gfx** | 0.1.0 | ✅ | 15 primitivas 3D + DrawHandle3D + touch_controls | demo_3d_primitives, demo_3d_touch |
| **ry-science** | 0.7.34 | ✅ | Bezier + stats + illusions | — |
| **ry-physics** | 0.7.34 | ✅ | Projectile + N-body | — |
| **migui** | 0.4.1 | ⚠️ | Immediate mode GUI (parcial) | demo_panel_visual, demo_menu_bar |
| **toolkit-ry** | 0.1.0 | ⚠️ | UI toolkit + 5 themes (parcial) | — |
| **ry-stream** | 0.2.0 | ⚠️ | LAN streaming (solo test CLI) | NINGUNO (falta demo) |
| **events-ry** | 0.1.0 | ⚠️ | Input unificado (parcial) | — |

### ✅ Demos Funcionales (muestran capacidades)

| Demo | Backend | Qué demuestra | Crates que usa |
|------|---------|--------------|----------------|
| demo_militar | SDL2 | Soldado procedural + partículas + granadas arco | ry-gfx (particles) |
| demo_transitions | SDL2 | 19 transiciones tipo editor de video | ry-gfx (transitions) |
| demo_emoji_utf8 | SDL2 | Texto UTF-8 + emojis sprites | ry-gfx (emoji_atlas, font_system) |
| demo_audio_mixer | SDL2 | Mixer 4 buses + spatial 2D | ry-gfx (audio_mixer) |
| demo_3d_primitives | Raylib | Primitivas 3D + cámara orbital | ry3d-gfx (DrawHandle3D) |
| demo_3d_touch | Raylib | 3D + joysticks virtuales + botones | ry3d-gfx (touch_controls) |
| demo_anime_ry_v2 | SDL2 | Snake + manzanas + entidades | ry-anim (principles) |
| demo_gpu_instancing | SDL2 | 50K partículas | ry-gfx (gpu_instancing) |
| demo_fsr | SDL2 | FSR 1.0 upscale | ry-gfx (fsr) |
| demo_torreta_vs_sprites | SDL2 | Juego 3 niveles + cámara + AI | ry-gfx + ry-physics |

---

## 🔗 CONEXIONES PENDIENTES (rellenar, no construir)

### Bloque 1: migui + toolkit-ry → Panel Visual (v0.18.1)

| Qué existe | Qué falta | Conexión necesaria |
|-----------|-----------|-------------------|
| migui tiene botones, labels, paneles | Falta conectar con toolkit-ry themes | `migui::Widget` → `toolkit_ry::Theme` |
| toolkit-ry tiene 5 themes | Falta integrar con migui render | `toolkit_ry::apply_theme(&mut migui)` |
| SDL2 tiene TTF mejorado | migui usa su propio font system | Unificar: `migui::Font::from_sdl2_ttf()` |
| Transiciones existen | migui no las usa | `migui::Panel::transition(TransitionType::FadeIn)` |

**Lo que se ve en demos actuales:**
- `demo_panel_visual`: 4 paneles + consola → migui básico funciona
- `demo_menu_bar`: Menús Dear ImGui → migui avanzado funciona
- **Falta**: Unir ambos + themes + transiciones → Panel Visual completo

**Esfuerzo**: 4-6h (solo conectar, no construir)

---

### Bloque 2: Input Configurable (v0.18.2)

| Qué existe | Qué falta | Conexión necesaria |
|-----------|-----------|-------------------|
| SDL2 lee teclado físico | Solo hardcodeado en cada demo | `.rydit-input` archivo de configuración |
| Raylib tiene touch_controls | Solo en demo_3d_touch | `TouchControls::from_config(path)` |
| events-ry crate existe | Sin demo, sin integración | events-ry → SDL2 + Raylib input unificado |
| ry-stream funciona | Solo test CLI | Demo de streaming + input remoto |

**Teclado virtual completo:**
- Raylib puede crear viewport + mandos en pantalla
- SDL2 maneja teclado físico en Termux-X11
- Solo falta: `InputConfig` struct + `.rydit-input` parser

**Esfuerzo**: 6-8h

---

### Bloque 3: ry-anim Action Player (v0.18.3)

| Qué existe | Qué falta | Conexión necesaria |
|-----------|-----------|-------------------|
| ry-anim tiene 12 Disney principles | Sin demo integrado | Demo que use principles + action_assets |
| action_assets existe en ry-anim | Sin player visual | `ActionPlayer` widget en migui |
| particles system funciona | Solo en demos sueltos | `ParticleSystem` widget en panel |
| transitions existen | Solo en demo_transitions | Integrar con migui panels |

**Lo que se ve en demos:**
- `demo_anime_ry_v2`: Snake usa principios Disney
- action_assets: frame animation, sprite sheet parse, state machine
- **Falta**: Action Player visual en panel → ver animaciones en UI

**Esfuerzo**: 8-12h

---

### Bloque 4: Viewport 2D + 3D (v0.18.4)

| Qué existe | Qué falta | Conexión necesaria |
|-----------|-----------|-------------------|
| SDL2 viewport 2D | Solo en demos individuales | `Viewport2D` struct reutilizable |
| Raylib viewport 3D | Solo en demo_3d_touch | `Viewport3D` struct reutilizable |
| DrawHandle3D funciona | Sin wrapper de viewport | `Viewport3D::begin()` / `end()` |
| TouchControls funciona | Solo en demo_3d_touch | `Viewport3D::draw_controls()` |

**Viewport 2D:**
- SDL2 ya dibuja rects, circles, lines, texturas
- Solo necesita wrapper: `Viewport2D { canvas, width, height }`

**Viewport 3D:**
- Raylib + DrawHandle3D ya funciona
- Solo necesita wrapper: `Viewport3D { camera, handle }`

**Esfuerzo**: 6-8h

---

### Bloque 5: Rybot CLI + Inspector (v0.18.5)

| Qué existe | Qué falta | Conexión necesaria |
|-----------|-----------|-------------------|
| rybot.sh existe | Solo script básico | CLI completo con subcomandos |
| .rydit formato | No existe aún | Parser de proyecto |
| toolkit-ry tiene themes | Sin inspector panel | `InspectorPanel` widget |
| ry-anim action_assets | Sin script panel | `ScriptPanel` que cargue .rydit |

**Rybot como toolkit del editor:**
- `rybot new proyecto` → crea estructura .rydit
- `rybot build` → compila proyecto
- `rybot run` → lanza demo
- `rybot inspect` → abre panel inspector

**Esfuerzo**: 10-15h

---

### Bloque 6: Hot Reload (v0.18.6)

| Qué existe | Qué falta | Conexión necesaria |
|-----------|-----------|-------------------|
| rybot puede build/launch | Sin file watcher | `watch_changes()` en background |
| Demos se compilan | Sin reload automático | `on_file_change → rebuild + relaunch` |
| ry-stream existe | Sin sync en vivo | `sync_state()` entre instancias |

**Flujo hot reload:**
1. Editas archivo `.rydit` o script
2. Rybot detecta cambio
3. Recompila solo lo necesario
4. Relanza demo con nuevo estado
5. Mantiene estado de la escena (no reset completo)

**Esfuerzo**: 6-8h

---

### Bloque 7: Render Avanzado (v0.18.7)

| Qué existe | Qué falta | Conexión necesaria |
|-----------|-----------|-------------------|
| GPU instancing (50K) | Sin NIS | NIS shader como feature |
| FSR 1.0 funciona | Sin FSR 2.0 | FSR 2.0 temporal upscaling |
| Render queue existe | Sin ECS render | ECS + render pipeline |
| Post-processing en roadmap | Sin bloom/glow | Shader post-process |

**Lo que ya tenemos:**
- `gpu_instancing.rs`: 50K partículas a 48 FPS
- `fsr.rs`: FSR 1.0 upscale 960→1280
- `render_queue.rs`: Command queue + double buffering

**Falta:**
- NIS (NVIDIA Image Scaling) → shader similar a FSR
- Post-processing → bloom, glow, blur, color grade

**Esfuerzo**: 8-12h

---

### Bloque 8: ry-stream Demo (v0.18.8)

| Qué existe | Qué falta | Conexión necesaria |
|-----------|-----------|-------------------|
| ry-stream crate | Solo test CLI | Demo visual de streaming |
| WebSocket server | Sin cliente visual | Panel que muestre stream remoto |
| LAN streaming | Sin multi-client | Demo multi-player LAN |

**Esfuerzo**: 4-6h

---

## 📊 RESUMEN POR BLOQUE

| Bloque | Tarea | Esfuerzo | Depende de | Impacto |
|--------|-------|----------|-----------|---------|
| **v0.18.1** | migui + toolkit-ry → Panel | 4-6h | migui, toolkit-ry | Medio |
| **v0.18.2** | Input Configurable | 6-8h | events-ry, SDL2, Raylib | Alto |
| **v0.18.3** | ry-anim Action Player | 8-12h | ry-anim, migui | Alto |
| **v0.18.4** | Viewport 2D + 3D | 6-8h | SDL2, ry3d-gfx | Alto |
| **v0.18.5** | Rybot CLI + Inspector | 10-15h | toolkit-ry, .rydit | Muy Alto |
| **v0.18.6** | Hot Reload | 6-8h | rybot, file watcher | Alto |
| **v0.18.7** | Render Avanzado (NIS) | 8-12h | FSR, GPU instancing | Medio |
| **v0.18.8** | ry-stream Demo | 4-6h | ry-stream | Medio |
| **TOTAL** | **8 bloques** | **52-75h** | — | — |

---

## 🎯 ORDEN LÓGICO DE EJECUCIÓN

### Fase 1: Conectar UI (v0.18.1-18.2)
1. migui + toolkit-ry → Panel Visual
2. Input Configurable

### Fase 2: Conectar Animación (v0.18.3-18.4)
3. ry-anim Action Player
4. Viewport 2D + 3D

### Fase 3: Conectar Editor (v0.18.5-18.6)
5. Rybot CLI + Inspector
6. Hot Reload

### Fase 4: Conectar Render (v0.18.7-18.8)
7. Render Avanzado (NIS)
8. ry-stream Demo

---

## 🔍 DETALLE: Qué NO es necesario construir

| Lo que NO necesitamos | Por qué |
|----------------------|---------|
| ~~Sistema de escenas tipo Godot~~ | Ry-Dit tiene su propio sistema |
| ~~Node graph~~ | No es nodos, es sistema universal Ry |
| ~~Editor desde cero~~ | Solo es panel 2D viewport que carga lo que ya tenemos |
| ~~Render pipeline nuevo~~ | Ya tenemos GPU instancing + FSR + render queue |
| ~~Input system nuevo~~ | SDL2 + Raylib ya funcionan, solo unificar |
| ~~Animation system nuevo~~ | ry-anim ya tiene 12 Disney + action_assets |

---

## ✅ DETALLE: Qué SÍ necesitamos (conectar/rellenar)

| Lo que SÍ necesitamos | Qué es realmente |
|----------------------|------------------|
| **migui ↔ toolkit-ry** | Solo conectar 2 crates que ya existen |
| **Input config** | Archivo `.rydit-input` + parser simple |
| **Action Player** | Widget migui que use ry-anim existente |
| **Viewport wrappers** | Wrappers simples alrededor de SDL2/Raylib |
| **Rybot CLI** | Script → CLI con subcomandos |
| **Hot Reload** | File watcher + rebuild automático |
| **NIS shader** | Similar a FSR, solo otro shader |
| **ry-stream demo** | Demo visual del crate existente |

---

<div align="center">

**🔗 Mapa de Conexiones — v0.18.0+**

*No es construir desde cero — es conectar, completar y rellenar*

*8 bloques · 52-75h total · 0 construcciones nuevas*

*Sistema Universal Ry ≠ Godot (nodos/escenas)*

</div>
