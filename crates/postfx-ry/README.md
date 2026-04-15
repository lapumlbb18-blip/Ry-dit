# postfx-ry

Post-processing effects y sistema de materiales para **Ry-Dit**.

## Efectos GPU

| Efecto | Descripción | Uso |
|--------|-------------|-----|
| **Bloom** | Brillo difuso en zonas claras | Explosiones, neón, magia |
| **Blur** | Gaussian blur configurable | Motion blur, profundidad de campo |
| **Sharpen** | Enfoque de imagen | Contraste, nitidez |
| **Color Grading** | Corrección por sombras/medios/luces | Cyberpunk, natural, retro |
| **Chromatic Aberration** | Separación RGB en bordes | Glitch, retro, sci-fi |
| **Vignette** | Oscurecimiento en bordes | Cinemático, dramático |

## Presets

```rust
let mut chain = PostFxChain::new(1280, 720)?;
chain.init_passes()?;

chain.preset_cyberpunk();  // Bloom + sharpen + color + chromatic + vignette
chain.preset_natural();    // Bloom suave + sharpen + color
chain.preset_retro();      // Sharpen + chromatic alto + vignette
chain.reset();             // Sin efectos
```

## Pipeline

```
Scene render → FBO input → Bloom → Blur(2-pass) → Sharpen → Color Grade → Chromatic → Vignette → Screen
```

Cada efecto se puede activar/desactivar individualmente. Los FBOs intermedios se reutilizan.

## Filosofía

- **OpenGL ES 2.0** — Compatible Adreno 610 y superior
- **Encadenamiento** — Efectos se componen en orden configurable
- **FBO intermedio** — Ping-pong entre dos framebuffers
- **Presets** — Cyberpunk, Natural, Retro listos para usar

## Próximo (v0.2.0)

- **Materiales**: Goma, lava, vidrio, metal
- **Química**: Mezcla, reacción, fusión
- **Transformación visual**: Cortar, mojar, endurecer, reventar, estirar

## Compatibilidad

- Adreno 610 (Termux-X11, SDL2, raylib)
- OpenGL ES 2.0+
- GLSL 100
