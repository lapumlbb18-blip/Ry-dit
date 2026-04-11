# 🛡️ QWEN.md - Bitácora Técnica Ry-Dit

**Última actualización**: 2026-04-11
**Versión actual**: v0.17.0 ✅ Demo Militar + Emoji Atlas + Audio Mixer + UTF-8 fix + Organización
**Versión anterior**: v0.16.1 (Snake + Buscaminas + Action Sprite + Tilemap 2.0)
**Próxima versión**: v0.18.0 — DLSS/NIS + Bordes Suaves + Opacidad + Iluminación 2D + Sombras
**Commit**: `v0.17.0`
**Repositorio**: `https://github.com/lapumlbb18-blip/Ry-dit`
**Crates publicados**: 12 ✅

---

## 🎉 SESIÓN v0.17.0 — ÉPICA: DEMO MILITAR + EMOJI ATLAS + AUDIO MIXER

### ✅ ESTADO: 23 CRATES | 0 ERRORES | 144 TESTS | 18+ DEMOS | 12 CRATES.IO

| Demo | GPU | FPS | Features | Notas |
|------|-----|-----|----------|-------|
| **demo_militar** | zink (Adreno 610) | ~30 FPS | Soldado procedural + partículas + granadas arco + salto | WASD, física, gravedad |
| **demo_emoji_utf8** | zink (Adreno 610) | ~28 FPS | 25+ emojis como sprites procedurales PNG | UTF-8 fix, atlas de texturas |
| **demo_audio_mixer** | zink (Adreno 610) | ~30 FPS | 4 buses + spatial 2D + fade in/out | Master, SFX, Music, Ambient |
| **demo_anime_ry_v2** | zink (Adreno 610) | ~32 FPS | Snake + manzanas + bombas + entidades + minimap | WASD, flood fill, camera follow |
| **demo_buscaminas** | zink (Adreno 610) | ~29 FPS | 16×16, 40 minas, flood fill, banderas | Click izq/der, R: reiniciar |
| **demo_action_sprite** | zink (Adreno 610) | ~30 FPS | Sprite sheet + frame animation + state machine | Procedural sprites |
| **demo_hud_camera** | zink (Adreno 610) | ~30 FPS | Health bars + debug overlay + stats + minimap | Camera2D zoom+rotación |
| **demo_gpu_instancing** | zink (Adreno 610) | ~53 FPS | 50K partículas instanced | smoothstep AA ahora |
| **demo_fsr** | zink (Adreno 610) | ~48 FPS | FSR 1.0 upscale 960→1280 | FBO render-to-texture |

### Features Implementadas

| # | Feature | Detalle |
|---|---------|---------|
| 1 | **demo_militar** | Soldado procedural con primitivas (cabeza, cuerpo, arma) |
| 2 | **Granadas en arco** | Trayectoria parabólica con gravedad |
| 3 | **Sistema de partículas** | Efectos de disparo y explosión |
| 4 | **Salto del soldado** | Física de salto con input de teclado |
| 5 | **Emoji Atlas UTF-8** | TTF_RenderUTF8_Blended fix |
| 6 | **25+ emojis procedurales** | Sprites PNG generados en runtime |
| 7 | **Atlas de texturas** | Textura única con grid de emojis |
| 8 | **Audio Mixer** | 4 buses: Master, SFX, Music, Ambient |
| 9 | **Spatial 2D** | Volumen/panning según posición del oyente |
| 10 | **Fade in/out** | Transiciones suaves de volumen |
| 11 | **docs/ ignorado** | docs/ y launchers/ en .gitignore |
| 12 | **Organización git** | Archivos de desarrollo fuera del tracking |

### Bugs Fixeados

| Bug | Fix |
|-----|-----|
| TTF_RenderText no soportaba UTF-8 | Cambiar a TTF_RenderUTF8_Blended |
| Emojis no se renderizaban | Atlas procedural como sprites PNG |
| Audio sin buses | Implementar 4 buses con mixer |
| Sin spatial audio | Implementar volumen/panning 2D |

---

## 🎉 SESIÓN v0.16.1 — ÉPICA: SNAKE + BUSCAMINAS + ACTION SPRITE + TILEMAP

### ✅ ESTADO: 23 CRATES | 0 ERRORES | 144 TESTS | 15+ DEMOS | 12 CRATES.IO

| Demo | GPU | FPS | Features | Notas |
|------|-----|-----|----------|-------|
| **demo_anime_ry_v2** | zink (Adreno 610) | ~32 FPS | Snake + manzanas + bombas + entidades + minimap | WASD, flood fill, camera follow |
| **demo_buscaminas** | zink (Adreno 610) | ~29 FPS | 16×16, 40 minas, flood fill, banderas | Click izq/der, R: reiniciar |
| **demo_action_sprite** | zink (Adreno 610) | ~30 FPS | Sprite sheet + frame animation + state machine | Procedural sprites |
| **demo_hud_camera** | zink (Adreno 610) | ~30 FPS | Health bars + debug overlay + stats + minimap | Camera2D zoom+rotación |
| **demo_gpu_instancing** | zink (Adreno 610) | ~53 FPS | 50K partículas instanced | smoothstep AA ahora |
| **demo_fsr** | zink (Adreno 610) | ~48 FPS | FSR 1.0 upscale 960→1280 | FBO render-to-texture |

### Features Implementadas

| # | Feature | Detalle |
|---|---------|---------|
| 1 | **action_sprite module** | SpriteSheet, AnimationClip, AnimatedSprite, RenderCommand |
| 2 | **demo_action_sprite** | Sprite sheet procedural + clips + state machine + flip |
| 3 | **Tilemap v2.0** | Texturas reales + CSV import/export + camera culling (95% menos) |
| 4 | **demo_buscaminas** | 16×16 grid, 40 minas, flood fill, banderas, game over |
| 5 | **demo_anime_ry_v2** | Snake controlable, manzanas, bombas, 8 entidades, minimap |
| 6 | **8 Launchers** | Auto-detección DISPLAY + Zink para todos los demos principales |
| 7 | **Bordes suaves** | smoothstep anti-aliasing + alpha blending en GPU instancing |
| 8 | **ry3d-gfx mejorado** | Texto 3D + modelos GLTF/OBJ/IQM/VOX/MDL |
| 9 | **ry-config publicado** | README + Cargo.toml fix |
| 10 | **ry-physics publicado** | README + Cargo.toml fix |
| 11 | **ry-science publicado** | README + Cargo.toml fix |
| 12 | **ry-test eliminado** | Código muerto removido |
| 13 | **GUIA_USUARIO.md** | Guía completa creada |
| 14 | **6 docs actualizados** | README, QWEN, TASKS, ROADMAP, ESTRUCTURA, GUIA_USUARIO |

### Bugs Fixeados

| Bug | Fix |
|-----|-----|
| ry-gfx sin feature migui | Agregar `features = ["migui"]` en ry-rs |
| demo_render_queue assets | Agregar `Assets::new()` |
| Texture::width unwrap_or | q.width es `u32` directo |
| f32→i32 type casts | Múltiples fixes en demos nuevos |

---

## 📦 Lista de Crates (23)

| Crate | Versión | Estado | Publicado | Notas |
|-------|---------|--------|-----------|-------|
| ry-core | 0.8.2 | ✅ | ✅ crates.io | Core trait + registry |
| ry-lexer | 0.1.0 | ✅ | ❌ | Zero-copy lexer |
| ry-parser | 0.1.0 | ✅ | ❌ | AST parser |
| ry-vm | — | ⚠️ | ❌ | VM opcodes |
| ry-gfx | 0.10.8 | ✅ | ✅ crates.io | GPU Instancing + FSR |
| ry-physics | 0.7.34 | ✅ | ✅ crates.io | Projectile + N-body |
| ry-anim | 0.12.0 | ✅ | ✅ crates.io | 12 Disney + action_sprite |
| ry-science | 0.7.34 | ✅ | ✅ crates.io | Bezier + stats + illusions |
| ry-script | 0.8.2 | ✅ | ❌ | Script loading |
| ry-stream | 0.2.0 | ✅ | ✅ crates.io | LAN streaming |
| ry-god | 0.1.0 | ✅ | ✅ crates.io | Security |
| ry-loader | — | ⚠️ | ❌ | Module loader |
| ry-rs | — | Main | N/A | Binary + lib |
| ry-test | — | 🗑️ | — | ELIMINADO |
| ry-backend | 0.1.0 | ✅ | ✅ crates.io | Dual backend |
| ry-config | 0.1.0 | ✅ | ✅ crates.io | Config parser |
| toolkit-ry | 0.1.0 | ✅ | ✅ crates.io | UI toolkit + 5 themes |
| migui | 0.4.1 | ✅ | ✅ crates.io | Immediate mode GUI |
| blast-core | 0.1.0 | ✅ | ❌ | Minimal executor |
| lizer | 0.11.2 | ✅ | ❌ | Legacy + AST cache |
| v-shield | 0.2.0 | ✅ | ✅ crates.io | Platform layer + sync |
| ry3d-gfx | 0.1.0 | ✅ | ❌ | 3D graphics |
| events-ry | 0.1.0 | ✅ | ❌ | Input unificado |

---

## 🔴 LECCIONES CRÍTICAS

### 1. ry-rs lib vs bin — NO REFACTORIZAR AHORA
**Problema**: main.rs ~5000 líneas con ciclo de dependencia circular
**Análisis**: El costo (10-14h) supera el beneficio (nadie usa ry_rs como lib)
**Decisión**: Dejar como está hasta que haya un consumidor real

### 2. Model3D necesita contexto raylib activo
**Problema**: `LoadModel` FFI requiere ventana inicializada
**Solución**: Cargar modelos dentro del game loop, no fuera
**Lección**: FFI de raylib necesita contexto de ventana

### 3. Texture no implementa Debug
**Problema**: `#[derive(Debug)]` falla en sdl2::render::Texture
**Solución**: Implementar `Debug` manualmente sin el campo texture

### 4. Borrow checker en snake game
**Problema**: `snake.head()` inmutable + mutación de `snake.alive`
**Solución**: Copiar posición primero (`let hp = snake.head_pos()`)

### 5. NUNCA usar sed para código estructural
- ✅ sed seguro SOLO para imports simples
- ❌ sed peligroso para código multilínea
- **Siempre usar el tool `edit`**

---

## 🚀 PRÓXIMOS PASOS (v0.18.0)

### **Lo que traerás en la próxima sesión:**
- [ ] DLSS/NIS implementación
- [ ] Bordes suaves + texturas + opacidad final
- [ ] Iluminación 2D dinámica
- [ ] Sombras 2D con raycasting
- [ ] Opacidad/transparencia en texturas
- [ ] Fade in/out transiciones entre escenas
- [ ] Letras 3D en demos
- [ ] Rybot CLI + GUI
- [ ] Editor separado o 2-in-1
- [ ] LAZOS: Python + C++ + C
- [ ] GitHub Actions completo
- [ ] SAZ formato
- [ ] v1.0 de Ry-Dit

### **Prioridades técnicas:**
1. NIS/FSR 2.0 → upscaling calidad
2. Iluminación 2D → Godot Light2D style
3. Sombras 2D → raycasting
4. Opacidad → PNG con canal alpha
5. Transiciones → fade entre escenas

---

## 📝 COMANDOS ÚTILES

### Build y Test
```bash
# Build release
cargo build -p ry-rs --bin rydit-rs --release

# Tests workspace
cargo test --workspace

# Check workspace
cargo check --workspace
```

### Launchers
```bash
./launcher_militar.sh        # 🎖️ Militar
./launcher_emoji_utf8.sh     # 😀 Emoji Atlas
./launcher_audio_mixer.sh    # 🎵 Audio Mixer
./launcher_anime_v2.sh      # 🐍 Snake
./launcher_buscaminas.sh     # 💣 Buscaminas
./launcher_hud_camera.sh     # HUD + Cámara
./launcher_gpu_instancing.sh # 50K partículas
./launcher_fsr.sh            # FSR 1.0
./launcher_torreta.sh        # Torreta vs Sprites
```

---

<div align="center">

**🛡️ RyDit v0.17.0 — Demo Militar + Emoji Atlas + Audio Mixer**

*23 crates · 144 tests · 12 crates.io · 18+ demos · 11 launchers · 0 errores*

**Próximo: v0.18.0 — DLSS/NIS + Bordes Suaves + Opacidad + Iluminación 2D + Sombras**

**LECCIÓN v0.17.0: TTF_RenderUTF8_Blended para emojis | Atlas procedural | 4 buses de audio**

</div>
