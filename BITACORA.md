# 🛡️ Ry-Dit (Shield Project)
## Proyección: Consolidación del Editor v0.22.0

**Estado actual (14 Mayo 2026)**:
- **Consolidación del Ensamblador Maestro (v0.24.0)**: El motor ahora orquesta módulos con ciclo de vida completo (`on_init`, `on_update`, `on_draw`).
- **Renderizado GPU Activo**: Integración exitosa de RLGL en el Renderer, permitiendo dibujo nativo en contexto SDL2.
- **Editor Orquestador**: El `ry-editor` ya ensambla físicamente los subsistemas de física y partículas.
- **Workspace Estable**: Compilación global verificada y limpia.

**Próximos Pasos**:
1. Implementación de demos gráficos de prueba para el nuevo patrón.
2. Primera versión funcional (Alpha) del editor visual.
3. Migración de assets a Texture2D de Raylib.

---

## Sesión 27 Agosto 2026 — v0.25.0 Estabilidad Total + Input Termux-X11

**Problemas resueltos**:
1. **Parpadeo de pantalla**: `shim.rs::present()` ahora llama `renderer.present()` (flush rlgl batch) antes de `window.gl_swap_window()`. Confirmado estable por el usuario.
2. **WASD no funcionaba**: `InputManager::is_key_down()` consultaba `MockBackend` (siempre `false`). Ahora consulta `input_state` interno via `input_state.is_key_pressed(key.name())`.
3. **Flechas no mapeadas**: `map_sdl_keycode_to_ry()` no incluía Up/Down/Left/Right. Expandido con flechas + teclas comunes (Tab, Shift, etc.).
4. **Teclado Android sin KeyRepeat**: `InputState` con `grace_ms: 150` mantiene teclas "activas" 150ms después de soltar. Resuelve el problema del teclado virtual de Termux-X11.

**Nuevo demo**: `demo_input_gracia` — Test de WASD/flechas con ventana de gracia.

**Archivos modificados**:
- `crates/ry-gfx/src/shim.rs` — Fix parpadeo
- `crates/events-ry/src/manager.rs` — Fix input queries
- `crates/ry-gfx/src/backend_sdl2.rs` — Fix mapeo flechas
- `crates/ry-input/src/lib.rs` — Ventana de gracia 150ms
- `crates/ry-rs/src/bin/demo_input_gracia.rs` — Nuevo demo

---

## Sesión 31 Agosto 2026 — v0.26.0 Audio + Render + Input Completo

**Problemas resueltos**:
1. **Audio no funcionaba en demos**: `AudioFFI::init()` usaba `sdl2::mixer::init()` que fallaba. Solución: FFI crudo directo (`Mix_OpenAudio`) como el test que sí funcionaba.
2. **PulseAudio no accesible**: SDL2 no encontraba el servidor PulseAudio. Solución: `ensure_pulseaudio()` al inicio del demo (patrón Chrome/Firefox en Termux-X11).
3. **Variables de entorno rompían audio**: `SDL_AUDIODRIVER=pulse` y `PULSE_SERVER` forzadas causaban "Could not connect to PulseAudio". Solución: NO forzar variables, dejar que SDL2 auto-detecte.
4. **Doble cleanup AudioMixer**: `AudioMixer::drop()` y `AudioFFI::drop()` ambos llamaban `Mix_CloseAudio/Mix_Quit`. Solución: limpiar solo en `AudioFFI::drop()`.

**Nuevo demo**: `demo_war_spacio_v2` — Galaga con audio procedural 8-bit + input completo + ventana de gracia.

**Archivos modificados**:
- `crates/ry-gfx/src/sdl2_ffi.rs` — Fix AudioFFI (FFI crudo + SDL_Init)
- `crates/ry-gfx/src/audio_mixer.rs` — Fix doble cleanup
- `crates/ry-rs/src/bin/demo_war_spacio_v2.rs` — Nuevo demo con audio
