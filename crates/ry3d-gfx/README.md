# ry3d-gfx

**3D Graphics for Ry-Dit — Primitives, Camera3D, Models (GLTF/OBJ), Text 3D**

[![Crates.io](https://img.shields.io/crates/v/ry3d-gfx.svg)](https://crates.io/crates/ry3d-gfx)
[![Documentation](https://docs.rs/ry3d-gfx/badge.svg)](https://docs.rs/ry3d-gfx)
[![License](https://img.shields.io/crates/l/ry3d-gfx.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)

## Overview

`ry3d-gfx` is the 3D graphics layer for the Ry-Dit game engine. It provides primitives, camera management, model loading, and text rendering in 3D space — built on top of raylib's FFI.

**ry-gfx** = 2D (circles, rects, lines, sprites)  
**ry3d-gfx** = 3D (cubes, spheres, cylinders, models, text)

They share `ColorRydit` and raylib but are separate crates.

## Features

### 📐 3D Primitives
| Function | Description |
|----------|-------------|
| `draw_cube_3d` | Solid cube |
| `draw_cube_wires_3d` | Wireframe cube |
| `draw_sphere_3d` | Solid sphere |
| `draw_sphere_wires_3d` | Wireframe sphere |
| `draw_cylinder_3d` | Solid cylinder |
| `draw_cylinder_wires_3d` | Wireframe cylinder |
| `draw_plane_3d` | XZ plane surface |
| `draw_line_3d` | 3D line |
| `draw_triangle_3d` | 3D triangle |
| `draw_point_3d` | 3D point |

### 📦 Reference Helpers
| Function | Description |
|----------|-------------|
| `draw_grid_3d` | Reference grid floor |
| `draw_axes_gizmo` | XYZ axes (R=green, G=Y, B=Z) |
| `draw_bounding_box_3d` | Bounding box visualization |

### 🎥 Camera
| Feature | Description |
|---------|-------------|
| `Camera3DBuilder` | Fluent builder for cameras |
| `perspective()` | Perspective projection |
| `orthographic()` | Orthographic projection |

### 📝 Text 3D
| Function | Description |
|----------|-------------|
| `draw_text_3d` | Billboard text in 3D space |

### 🎨 Models
| Feature | Description |
|---------|-------------|
| `Model3D::load()` | Load from `.gltf`, `.glb`, `.obj`, `.iqm`, `.vox`, `.mdl` |
| `draw_model()` | Draw model at position with scale |
| `draw_model_ex()` | Draw model with rotation (XYZ) |

## Installation

```toml
[dependencies]
ry3d-gfx = "0.1.0"
ry-gfx = "0.10.8"
raylib = "5.5.1"
```

## Quick Start

```rust
use ry3d_gfx::{Camera3DBuilder, DrawHandle3D, Model3D};
use ry_gfx::ColorRydit;

// Create camera
let camera = Camera3DBuilder::new()
    .position(Vector3::new(10.0, 10.0, 10.0))
    .target(Vector3::zero())
    .fovy(45.0)
    .perspective();

// Inside raylib game loop:
// Begin 3D mode
let mut h3d = DrawHandle3D::new(&camera);
h3d.clear_3d(ColorRydit::Negro);

// Draw primitives
h3d.draw_cube_3d((0.0, 1.0, 0.0), (2.0, 2.0, 2.0), ColorRydit::Rojo);
h3d.draw_sphere_3d((5.0, 2.0, 0.0), 1.5, ColorRydit::Azul);
h3d.draw_grid_3d(20, 1.0);
h3d.draw_axes_gizmo(5.0);

// Draw 3D text
h3d.draw_text_3d((0.0, 3.0, 0.0), "Hello 3D!", 1.0, ColorRydit::Blanco);

// Load and draw model
let model = Model3D::load("assets/model.glb")?;
h3d.draw_model(&model, (0.0, 0.0, 0.0), 1.0, ColorRydit::Blanco);

// Exit 3D mode (automatic via Drop)
drop(h3d);

// Now draw 2D HUD on top
let mut d = gfx.begin_draw();
d.draw_text("FPS: 60", 10, 10, 20, ColorRydit::Blanco);
```

## Supported Model Formats

| Format | Extension | Notes |
|--------|-----------|-------|
| **glTF** | `.gltf` | Open standard, JSON + binary |
| **glTF Binary** | `.glb` | Single file, recommended |
| **Wavefront OBJ** | `.obj` | Simple geometry + materials |
| **IQM** | `.iqm` | With skeletal animations |
| **MagicaVoxel** | `.vox` | Voxel models |
| **Quake MDL** | `.mdl` | Legacy game models |

## Architecture

```
DrawHandle3D (RAII)
├── new(camera)  → BeginMode3D()
├── draw_*()     → FFI calls to raylib
└── drop()       → EndMode3D() (automatic)

Model3D
├── load(path)   → LoadModel(FFI)
├── is_loaded()  → bool check
└── drop()       → UnloadModel() (automatic)
```

The RAII pattern ensures `BeginMode3D()` / `EndMode3D()` pairs are always balanced, even on early returns or panics.

## Examples

### 3D Scene with Multiple Objects

```rust
let mut h3d = DrawHandle3D::new(&camera);
h3d.clear_3d(ColorRydit::Cielo);

// Ground grid
h3d.draw_grid_3d(40, 1.0);

// Cubes in a circle
for i in 0..8 {
    let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0;
    let x = angle.cos() * 5.0;
    let z = angle.sin() * 5.0;
    h3d.draw_cube_3d((x, 1.0, z), (1.0, 2.0, 1.0), ColorRydit::Verde);
}

// Axis gizmo at center
h3d.draw_axes_gizmo(3.0);

// Labels
h3d.draw_text_3d((0.0, 3.0, 0.0), "Center", 0.8, ColorRydit::Blanco);
```

### Loading and Rotating a Model

```rust
// Load once (outside game loop)
let mut model = Model3D::load("assets/character.glb")?;

// In game loop:
let rotation = (elapsed_time * 30.0) % 360.0;
h3d.draw_model_ex(&model, (0.0, 0.0, 0.0), (0.0, rotation, 0.0), 1.0, ColorRydit::Blanco);
```

## Performance

- **Zero-overhead FFI** — direct calls to raylib C functions
- **RAII safety** — no manual BeginMode3D/EndMode3D management
- **Model caching** — load once, draw many times
- **3 tests** ensuring camera and model correctness

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ry-gfx` | 0.10.8 | ColorRydit shared types |
| `raylib` | 5.5.1 | 3D rendering via FFI |

## Roadmap

- [ ] Real 3D text with texture billboards
- [ ] Material system (PBR, roughness, metallic)
- [ ] Lighting (directional, point, spot)
- [ ] Mesh generation (torus, knot, terrain)
- [ ] Raycasting (pick objects with mouse)
- [ ] Model animations (skeletal, morph targets)
- [ ] Skybox / environment mapping

## Contributing

Contributions are welcome! This crate is part of the **Ry-Dit** game engine project.

- **Repository**: https://github.com/lapumlbb18-blip/Ry-dit
- **Issues**: https://github.com/lapumlbb18-blip/Ry-dit/issues
- **Pull Requests**: Welcome!

Please read [CONTRIBUTING.md](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE) for details.

---

<div align="center">

**ry3d-gfx** — 3D graphics for Ry-Dit game engine 🎮🧊

*3 tests · Primitives + Camera3D + Models + Text 3D*

</div>
