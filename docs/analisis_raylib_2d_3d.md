# 🎨 Análisis Completo: Raylib 2D + 3D en ry-gfx

**Fecha**: 2026-04-04
**Raylib versión**: 5.5.1 (Rust bindings)
**Estado**: 2D completo | 3D disponible pero NO envuelto

---

## 📊 RESUMEN DE CAPACIDADES

| Dimensión | Implementado en ry-gfx | Disponible en raylib | Pendiente |
|-----------|----------------------|---------------------|-----------|
| **2D** | ✅ 21 funciones | 30+ funciones | 9 funciones |
| **3D** | ❌ 0 funciones | 28 funciones | **28 funciones** |
| **Camera** | ✅ Camera2D | ✅ Camera3D + modos | ⏳ Camera3D |
| **Models** | ❌ | ✅ Model, Mesh, Material | Todo pendiente |

---

## 🎨 2D — FUNCIONES ACTUALMENTE WRAPPED (21)

### Primitivas Básicas
| Función ry-gfx | Raylib | Estado |
|----------------|--------|--------|
| `draw_circle(x, y, radius, color)` | `DrawCircle` | ✅ |
| `draw_rectangle(x, y, w, h, color)` | `DrawRectangle` | ✅ |
| `draw_line(x1, y1, x2, y2, color)` | `DrawLine` | ✅ |
| `draw_triangle(v1, v2, v3, color)` | `DrawTriangle` | ✅ |
| `draw_triangle_lines(v1, v2, v3, color)` | 3× DrawLine | ✅ |
| `draw_ellipse(cx, cy, rx, ry, color)` | `DrawEllipse` | ✅ |

### Primitivas Avanzadas
| Función ry-gfx | Raylib | Estado |
|----------------|--------|--------|
| `draw_rectangle_lines(x, y, w, h, color)` | `DrawRectangleLines` | ✅ |
| `draw_ring(center, inner_r, outer_r, color)` | DrawCircle (hack) | ⚠️ Aproximado |
| `draw_line_thick(start, end, thick, color)` | `DrawLineEx` | ✅ |
| `draw_rectangle_pro(rect, origin, angle, color)` | `DrawRectanglePro` | ✅ |

### Texto
| Función ry-gfx | Raylib | Estado |
|----------------|--------|--------|
| `draw_text(text, x, y, size, color)` | `DrawText` | ✅ |

### Texturas/Sprites
| Función ry-gfx | Raylib | Estado |
|----------------|--------|--------|
| `draw_texture(texture, x, y, color)` | `DrawTexture` | ⚠️ Placeholder |
| `draw_texture_ex(texture, pos, rot, scale, color)` | `DrawTextureEx` | ⚠️ Placeholder |
| `draw_texture_rec(texture, source, dest, color)` | `DrawTextureRec` | ⚠️ Placeholder |
| `draw_texture_scaled(texture, x, y, scale, color)` | `DrawTextureEx` | ⚠️ Placeholder |
| `Assets::draw_texture(id, x, y)` | Via Texture2D | ✅ SDL2 |
| `Assets::draw_texture_scaled(id, x, y, scale)` | Via Texture2D | ✅ SDL2 |

### ❌ 2D PENDIENTES (9 funciones)

| Función Raylib | Descripción | Prioridad |
|---------------|-------------|-----------|
| `DrawPoly` | Polígono regular | 🟡 Media |
| `DrawPolyLines` | Borde de polígono | 🟡 Media |
| `DrawCircleV` | Círculo con Vector2 | 🟢 Baja |
| `DrawCircleLines` | Borde de círculo | 🟢 Baja |
| `DrawCircleSector` | Sector circular | 🟢 Baja |
| `DrawCircleSectorLines` | Borde sector | 🟢 Baja |
| `DrawRectangleRounded` | Rect redondeado | 🟡 Media |
| `DrawRectangleRoundedLines` | Borde rect redondeado | 🟡 Media |
| `DrawSpline` | Curva spline | 🟢 Baja |

---

## 🧊 3D — FUNCIONES DISPONIBLES EN RAYLIB (28 funciones)

### ⚠️ IMPORTANTE: Ninguna está envuelta en ry-gfx
**Todas están disponibles** en el crate `raylib 5.5.1` pero **ry-gfx NO las expone**.

### Primitivas 3D (12 funciones)

| Función Raylib | Parámetros | Uso |
|---------------|------------|-----|
| `draw_cube(pos, size, color)` | Vector3, f32×3, Color | Cubo sólido |
| `draw_cube_v(pos, size, color)` | Vector3, Vector3, Color | Cubo con tamaño vector |
| `draw_cube_wires(pos, size, color)` | Vector3, f32×3, Color | Wireframe cubo |
| `draw_cube_wires_v(pos, size, color)` | Vector3, Vector3, Color | Wireframe vector |
| `draw_sphere(center, radius, color)` | Vector3, f32, Color | Esfera sólida |
| `draw_sphere_ex(center, radius, rings, slices, color)` | Vector3, f32, i32×2, Color | Esfera detallada |
| `draw_sphere_wires(center, radius, rings, slices, color)` | Vector3, f32×3, i32×2, Color | Wireframe esfera |
| `draw_cylinder(pos, r_top, r_bottom, height, slices, color)` | Vector3, f32×3, i32, Color | Cilindro |
| `draw_cylinder_ex(p1, p2, r_top, r_bottom, sides, color)` | Vector3×2, f32×2, i32, Color | Cilindro entre puntos |
| `draw_cylinder_wires(pos, r_top, r_bottom, height, slices, color)` | ... | Wireframe cilindro |
| `draw_cylinder_wires_ex(p1, p2, r_top, r_bottom, sides, color)` | ... | Wireframe entre puntos |
| `draw_plane(pos, normal, size, color)` | Vector3×2, f32, Color | Plano infinito |

### 3D Básico (6 funciones)

| Función Raylib | Parámetros | Uso |
|---------------|------------|-----|
| `draw_point3D(pos, color)` | Vector3, Color | Punto 3D |
| `draw_triangle3D(p1, p2, p3, color)` | Vector3×3, Color | Triángulo 3D |
| `draw_triangle_strip3D(points, color)` | &[Vector3], Color | Tira de triángulos |
| `draw_line_3D(start, end, color)` | Vector3×2, Color | Línea 3D |
| `draw_circle_3D(center, radius, rotation, color)` | Vector3, f32, f32, Color | Círculo 3D orientable |
| `draw_grid(slices, spacing)` | i32, f32 | Grid de referencia |

### Modelos y Meshes (8 funciones)

| Función Raylib | Parámetros | Uso |
|---------------|------------|-----|
| `draw_model(model, pos, scale, color)` | Model, Vector3, f32, Color | Modelo 3D completo |
| `draw_model_ex(model, pos, scale, rotation, color)` | Model, Vector3, f32, Vector3, Color | Modelo con rotación |
| `draw_model_wires(model, pos, scale, color)` | Model, Vector3, f32, Color | Wireframe modelo |
| `draw_model_wires_ex(model, pos, scale, rotation, color)` | Model, Vector3, f32×2, Color | Wireframe con rotación |
| `draw_mesh(mesh, material, transform)` | Mesh, Material, Matrix | Mesh raw |
| `draw_mesh_instanced(mesh, material, transforms)` | Mesh, Material, &[Matrix] | Mesh instanced |
| `draw_bounding_box(bbox, color)` | BoundingBox, Color | Caja delimitadora |
| `draw_model_points(model, color)` | Model, Color | Puntos del modelo |

### Camera 3D

| Función Raylib | Descripción |
|---------------|-------------|
| `Camera3D` | Cámara perspectiva/ortográfica |
| `begin_mode3D(camera)` | Entrar modo 3D |
| `end_mode3D()` | Salir modo 3D |
| `update_camera(camera, mode)` | Control tipo FPS/Orbital |
| `update_camera_pro(camera, movement, rotation, zoom)` | Control libre |
| `CameraMode::FirstPerson` | FPS |
| `CameraMode::ThirdPerson` | Tercera persona |
| `CameraMode::Orbital` | Orbital alrededor de punto |
| `CameraMode::Free` | Libre |
| `CameraMode::Custom` | Custom |

### Modo 3D

```rust
// API de raylib Rust
use raylib::prelude::*;

let mut rl = raylib::init();
let camera = Camera3D::perspective(
    Vector3::new(10, 10, 10),  // posición
    Vector3::new(0, 0, 0),     // target
    Vector3::new(0, 1, 0),     // up
    45.0,                      // FOV
);

while !rl.should_close() {
    let mut d = rl.begin_drawing();
    
    // Entrar modo 3D
    d.begin_mode3D(&camera);
    
    // Dibujar en 3D
    d.draw_cube(Vector3::new(0, 0, 0), 2.0, 2.0, 2.0, Color::RED);
    d.draw_sphere(Vector3::new(5, 0, 0), 1.5, Color::BLUE);
    d.draw_grid(10, 1.0);
    
    // Salir modo 3D
    d.end_mode3D();
    
    // UI 2D encima
    d.draw_text("3D Demo", 10, 10, 20, Color::WHITE);
}
```

---

## 🔧 IMPLEMENTACIÓN EXISTENTE (rl* low-level)

### ecs_render.rs ya usa raylib low-level

```rust
rlPushMatrix();
rlTranslatef(screen_x, screen_y, 0.0);
// ... dibujo 2D con transformaciones ...
rlPopMatrix();
```

**Funciones rl* disponibles**:
| Función | Uso |
|---------|-----|
| `rlPushMatrix()` | Guardar matriz actual |
| `rlPopMatrix()` | Restaurar matriz |
| `rlTranslatef(x, y, z)` | Trasladar |
| `rlRotatef(angle, x, y, z)` | Rotar |
| `rlScalef(x, y, z)` | Escalar |
| `rlBegin(mode)` | Iniciar dibujo low-level |
| `rlEnd()` | Finalizar dibujo |
| `rlVertex(x, y, z)` | Vértice individual |
| `rlColor(r, g, b, a)` | Color actual |

**Estas funciones funcionan en 3D** pero actualmente solo se usan para transformaciones 2D (z=0).

---

## 📋 ROADMAP DE IMPLEMENTACIÓN 3D

### Fase 1: Primitivas 3D básicas (v0.14.0)

| Función ry-gfx wrapper | Raylib | Esfuerzo |
|-----------------------|--------|----------|
| `draw_cube_3d(pos, size, color)` | `DrawCube` | Bajo |
| `draw_sphere_3d(center, radius, color)` | `DrawSphere` | Bajo |
| `draw_cylinder_3d(pos, rt, rb, h, color)` | `DrawCylinder` | Bajo |
| `draw_plane_3d(pos, normal, size, color)` | `DrawPlane` | Bajo |
| `draw_line_3d(start, end, color)` | `DrawLine3D` | Bajo |
| `draw_triangle_3d(p1, p2, p3, color)` | `DrawTriangle3D` | Bajo |
| `draw_grid_3d(slices, spacing)` | `DrawGrid` | Bajo |
| `draw_point_3d(pos, color)` | `DrawPoint3D` | Bajo |

**Tiempo estimado**: 4-6 horas

### Fase 2: Camera3D (v0.14.0)

| Función ry-gfx | Raylib | Esfuerzo |
|---------------|--------|----------|
| `Camera3D::perspective(...)` | `Camera3D::perspective` | Bajo |
| `Camera3D::orthographic(...)` | `Camera3D::orthographic` | Bajo |
| `camera.begin_mode3d()` | `begin_mode3D` | Bajo |
| `camera.end_mode3d()` | `end_mode3D` | Bajo |
| `camera.update(mode)` | `update_camera` | Medio |

**Tiempo estimado**: 4-6 horas

### Fase 3: Models (v0.15.0)

| Función | Raylib | Esfuerzo |
|---------|--------|----------|
| `load_model(path)` | `LoadModel` | Medio |
| `draw_model(model, pos, scale)` | `DrawModel` | Medio |
| `unload_model(model)` | `UnloadModel` | Bajo |
| `load_mesh(path)` | `LoadMesh` | Medio |

**Tiempo estimado**: 8-12 horas

### Fase 4: 3D avanzado (v0.16.0)

| Feature | Esfuerzo |
|---------|----------|
| Instanced drawing | Alto |
| Material system | Alto |
| 3D lighting | Alto |
| 3D textures | Alto |
| Skybox | Medio |

---

## 🎯 ARQUITECTURA PROPUESTA PARA 3D

```
ry-gfx/src/
├── lib.rs              # 2D actual + init 3D
├── camera.rs           # Camera2D actual
├── camera3d.rs         # 🆕 Camera3D wrapper
├── draw3d.rs           # 🆕 Primitivas 3D
├── models.rs           # 🆕 Model loading + drawing
└── mesh.rs             # 🆕 Mesh utilities
```

```rust
// Ejemplo de API propuesta
let mut gfx = RyditGfx::new("3D Demo", 800, 600);
let camera = Camera3D::perspective(
    vec3(10.0, 10.0, 10.0),  // posición
    vec3(0.0, 0.0, 0.0),     // target
    vec3(0.0, 1.0, 0.0),     // up
    45.0,                     // FOV
);

while !gfx.should_close() {
    let mut d = gfx.begin_draw();
    
    // Modo 3D
    d.begin_3d(&camera);
    d.draw_cube_3d(vec3(0, 0, 0), vec3(2, 2, 2), ColorRydit::Rojo);
    d.draw_sphere_3d(vec3(5, 0, 0), 1.5, ColorRydit::Azul);
    d.draw_cylinder_3d(vec3(-5, 0, 0), 0.5, 1.0, 3.0, ColorRydit::Verde);
    d.draw_grid_3d(10, 1.0);
    d.end_3d();
    
    // UI 2D encima
    d.draw_text("FPS: 60", 10, 10, 20, ColorRydit::Blanco);
}
```

---

## 🔥 DATO CLAVE: Todo está disponible, solo falta envolver

**Raylib 5.5.1 ya tiene TODAS las funciones 3D** compiladas y funcionales.
**Los Rust bindings ya las exponen** en `raylib::prelude::*`.
**ry-gfx solo necesita wrappers** que traduzcan `ColorRydit` → `raylib::Color` y expongan las funciones.

**No hay riesgo técnico** — es trabajo de wrapping puro.

---

<div align="center">

**🧊 ry-gfx 3D — Todo disponible, 0 envuelto**

*28 funciones listas | Camera3D con 5 modos | Models + Meshes*

*Solo necesita wrappers → 16-24 horas de trabajo*

</div>
