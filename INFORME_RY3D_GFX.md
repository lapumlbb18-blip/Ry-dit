# 🧊 ry3d-gfx — Estado Actual y Hoja de Ruta

**Fecha**: 2026-04-10
**Versión del crate**: v0.1.0
**Ubicación**: `crates/ry3d-gfx/`
**Estado**: ✅ Compila | ⚠️ Sin demos | ❌ Sin publicar

---

## 📦 ¿Qué es ry3d-gfx?

Crate independiente de `ry-gfx` (2D) que proporciona **gráficos 3D** usando raylib como backend.

| Métrica | Valor |
|---------|-------|
| **Líneas de código** | ~360 (single-file `lib.rs`) |
| **Dependencias** | `ry-gfx`, `raylib 5.5.1` |
| **Tests** | 3 pasando |
| **Publicado** | ❌ No está en crates.io |
| **Demos** | ❌ 0 demos existen |
| **Consumidores** | ❌ Ningún otro crate lo usa |

---

## ✅ Lo que SÍ funciona (implementado)

### Primitivas 3D (15 funciones)

| Método | Raylib FFI | Estado |
|--------|-----------|--------|
| `draw_cube_3d(pos, size, color)` | `DrawCube` | ✅ |
| `draw_cube_wires_3d(pos, size, color)` | `DrawCubeWires` | ✅ |
| `draw_sphere_3d(center, radius, color)` | `DrawSphere` | ✅ |
| `draw_sphere_wires_3d(center, radius, color)` | `DrawSphereWires` | ✅ |
| `draw_cylinder_3d(pos, rt, rb, h, color)` | `DrawCylinder` | ✅ |
| `draw_cylinder_wires_3d(pos, rt, rb, h, color)` | `DrawCylinderWires` | ✅ |
| `draw_plane_3d(center, size, color)` | `DrawPlane` | ✅ |
| `draw_grid_3d(slices, spacing)` | `DrawGrid` | ✅ |
| `draw_point_3d(pos, color)` | `DrawPoint3D` | ✅ |
| `draw_line_3d(start, end, color)` | `DrawLine3D` | ✅ |
| `draw_triangle_3d(p1, p2, p3, color)` | `DrawTriangle3D` | ✅ |
| `draw_bounding_box_3d(min, max, color)` | `DrawBoundingBox` | ✅ |
| `draw_axes_gizmo(length)` | 3× `DrawLine3D` | ✅ |
| `clear_3d(color)` | `ClearBackground` | ✅ |

### Camera3DBuilder

| Método | Estado |
|--------|--------|
| `Camera3DBuilder::new()` | ✅ Defaults: pos(10,10,10), target(0,0,0), up(0,1,0), fovy 45 |
| `.position(Vector3)` | ✅ |
| `.target(Vector3)` | ✅ |
| `.up(Vector3)` | ✅ |
| `.fovy(f32)` | ✅ |
| `.perspective() -> Camera3D` | ✅ |
| `.orthographic() -> Camera3D` | ✅ |

### Model3D

| Método | Estado |
|--------|--------|
| `Model3D::load(path) -> Result<Self>` | ✅ FFI `LoadModel` |
| `is_loaded() -> bool` | ✅ |
| `Drop` → `UnloadModel` | ✅ RAII |

### DrawHandle3D (RAII)

| Aspecto | Detalle |
|---------|---------|
| Constructor | `DrawHandle3D::new(&camera)` → llama `BeginMode3D` |
| Drop | Llama `EndMode3D` automáticamente |
| Patrón | RAII seguro — no puede haber fuga de modo 3D |

---

## 🔧 Stubs recién arreglados (v0.18.0)

| Método | Antes | Después |
|--------|-------|---------|
| `draw_model(model, pos, scale, tint)` | ❌ `{}` vacío | ✅ `DrawModel` FFI real |
| `draw_model_ex(model, pos, rot_axis, rot_angle, scale, tint)` | ❌ `{}` vacío | ✅ `DrawModelEx` FFI real |
| `draw_text_3d(pos, text, size, color)` | ❌ `let _ = ...` no-op | ✅ `DrawText3D` FFI real |

---

## ❌ Lo que FALTA implementar

### Primitivas faltantes
| Función raylib | Descripción |
|---------------|-------------|
| `DrawCubeV` / `DrawCubeWiresV` | Cubo con tamaño Vector3 |
| `DrawSphereEx` | Esfera con rings/slices custom |
| `DrawCylinderEx` | Cilindro entre dos puntos (eje arbitrario) |
| `DrawCapsule` / `DrawCapsuleWires` | Cápsula 3D |
| `DrawTorus` | Toroide |
| `DrawCircle3D` | Círculo en espacio 3D |
| `DrawRay` | Rayo visual |

### Mesh Generation
| Función | Descripción |
|---------|-------------|
| `GenMeshCube` | Generar mesh de cubo |
| `GenMeshSphere` | Generar mesh de esfera |
| `GenMeshCylinder` | Generar mesh de cilindro |
| `GenMeshTorus` | Generar mesh de toroide |
| `GenMeshKnot` | Generar nudo toroidal |
| `GenMeshHeightmap` | Terreno desde heightmap |
| `GenMeshTerrain` | Terreno procedural |
| `DrawMesh` / `DrawMeshInstanced` | Dibujar meshes |

### Modelos y Animaciones
| Función | Descripción |
|---------|-------------|
| `DrawModelWires` / `DrawModelWiresEx` | Modelos en wireframe |
| `LoadModelAnimations` | Cargar animaciones |
| `UpdateModelAnimation` | Actualizar animación |
| `UnloadModelAnimation` | Liberar animación |

### Materiales e Iluminación
| Feature | Descripción |
|---------|-------------|
| `LoadMaterialDefault` | Material por defecto |
| `SetMaterialTexture` | Texturas en materiales |
| `LoadShader` / `SetShaderValue` | Shaders custom |
| Lighting | Iluminación 3D |

### Cámara y Utilidades
| Feature | Descripción |
|---------|-------------|
| `CameraMode` | First-person, third-person, orbital, free |
| `UpdateCamera` | Controlador de cámara integrado |
| `GetWorldToScreen` | Proyectar 3D → 2D |
| `GetRayCollision*` | Raycasting (7 funciones) |

### Texturas 3D
| Feature | Descripción |
|---------|-------------|
| `DrawBillboard` | Texturas billboards en 3D |
| `DrawBillboardRec` | Billboard con sub-rect |
| Texturas en modelos | Materiales con mapas |

---

## 📐 Arquitectura Actual

```
ry3d-gfx/
├── Cargo.toml          # v0.1.0, raylib 5.5.1 + ry-gfx
├── README.md           # Documentación en inglés
└── src/
    └── lib.rs          # ~360 líneas, TODO el crate

Estructura interna de lib.rs:
├── to_ffi_color()          # ColorRydit → ffi::Color
├── Re-exports              # Camera3D, Vector3, type Camera
├── struct Model3D          # Wrapper RAII de raylib::ffi::Model
├── struct Camera3DBuilder  # Fluent builder para Camera3D
├── struct DrawHandle3D     # RAII BeginMode3D/EndMode3D
│   ├── 15 primitivas 3D    # Cubos, esferas, cilindros, etc.
│   ├── draw_text_3d()      # ✅ FFI real
│   ├── draw_model()        # ✅ FFI real
│   └── draw_model_ex()     # ✅ FFI real
└── tests (3)               # Camera3D builder, Model3D
```

### Patrones de diseño

| Patrón | Dónde se usa |
|--------|-------------|
| **RAII** | `DrawHandle3D` (BeginMode3D/EndMode3D), `Model3D` (LoadModel/UnloadModel) |
| **Builder** | `Camera3DBuilder` con métodos encadenables |
| **FFI directo** | `unsafe { raylib::ffi::* }` sin abstracción intermedia |
| **Type alias** | `pub type Camera = Camera3D` |
| **Re-exports** | Re-exporta `Camera3D` y `Vector3` de raylib |

---

## 📄 Documentos de referencia

| Archivo | Contenido |
|---------|-----------|
| `docs/analisis_raylib_2d_3d.md` | Análisis de 28 funciones 3D de raylib (2026-04-04) |
| `docs/sesion_control_total_v0.13.0.md` | Sesión de creación de ry3d-gfx (commit 7b010d1) |
| `crates/ry3d-gfx/README.md` | README del crate en inglés con ejemplos |

---

## 🎯 Hoja de Ruta Planificada

### Fase 1: Demo + Validación (AHORA)
- [ ] `demo_3d_primitives` — Mostrar cubos, esferas, cilindros, grid, ejes
- [ ] `demo_3d_models` — Cargar modelo GLTF/OBJ y rotarlo
- [ ] Validar que todo funciona visualmente

### Fase 2: Mesh Generation (v0.19.0)
- [ ] `GenMeshCube`, `GenMeshSphere`, `GenMeshCylinder`
- [ ] `GenMeshTorus`, `GenMeshKnot`
- [ ] `DrawMesh` / `DrawMeshInstanced`

### Fase 3: Materiales + Iluminación (v0.20.0)
- [ ] `LoadMaterialDefault`, `SetMaterialTexture`
- [ ] `LoadShader`, `SetShaderValue`
- [ ] Iluminación básica (ambient + directional)

### Fase 4: Utilidades Avanzadas (v0.21.0)
- [ ] Camera modes (first-person, orbital, free)
- [ ] `UpdateCamera` integrado
- [ ] Raycasting: `GetRayCollisionModel`, `GetRayCollisionSphere`
- [ ] `GetWorldToScreen` (proyección 3D→2D)

### Fase 5: Publicación
- [ ] README profesional en español
- [ ] Tests de integración
- [ ] Publicar en crates.io

---

## ⚠️ Problemas Conocidos

| # | Problema | Severidad |
|---|----------|-----------|
| 1 | **No hay demos** — nadie ha visto ry3d-gfx en acción | 🔴 Alta |
| 2 | **Sin consumidores** — ningún crate del workspace lo usa | 🟡 Media |
| 3 | **Hardcoded segments** — spheres/cylinders siempre 16 segmentos | 🟡 Media |
| 4 | **DrawPlane solo cuadrado** — raylib acepta Vector2, aquí solo scalar | 🟢 Baja |
| 5 | **to_ffi_color usa transmute** — frágil, mejor field-by-field | 🟢 Baja |
| 6 | **No publicado en crates.io** | 🟡 Media |

---

## 🧪 Cómo Probar (cuando haya demo)

```bash
# Demo de primitivas
cargo run --bin demo_3d_primitives --release

# Demo con modelos
cargo run --bin demo_3d_models --release
```

---

<div align="center">

**🧊 ry3d-gfx v0.1.0 — Estado: Compila ✅ | Sin demos ❌**

*15 funciones 3D + Camera3D + Model3D — todo FFI raylib*

*Próximo: Demo de primitivas → Mesh generation → Materiales → Publicar*

</div>
