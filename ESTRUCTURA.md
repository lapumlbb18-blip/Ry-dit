# Ry-Dit - Estructura del Proyecto v0.23.0 (Híbrida)

**Última actualización**: 2026-05-10
**Versión**: v0.23.0 ✅ Fusión SDL2 (Ventana/Input) + RLGL (GPU)
**Estado**: Consolidación de Núcleo Completada

---

## 📁 Estructura del Workspace

```
shield-project/
├── Cargo.toml                    # Workspace definition
├── README.md                     # Documentación principal
├── ROADMAP.md                    # Plan de versiones
├── TASKS.md                      # Tareas completadas y pendientes
├── BITACORA.md                   # Bitácora técnica
├── ESTRUCTURA.md                 # Este archivo (ACTUALIZADO)
├── MANIFIESTO.md                 # Filosofía Low-End First
│
├───crates/
│   ├── ry-gfx/                   # ✅ Motor Gráfico Híbrido
│   │   └── src/
│   │       ├── backend_sdl2.rs   # Sdl2Backend (Sin Canvas, usa rlgl)
│   │       ├── shim.rs           # Shims de compatibilidad (set_draw_color, fill_rect, etc.)
│   │       ├── renderer.rs       # Trait Renderer local
│   │       └── renderer_impl.rs  # Implementación Sdl2Renderer vía raylib-ffi/rlgl
│   ├── ry-rs/                    # Crate Principal (Binario)
│   │   └── src/
│   │       ├── main.rs           # Entry point
│   │       └── bin/              # Demos activos (Vacío temporalmente)
│   ├── ry-rs/pendientes_importantes/ # 🆕 Demos Legacy (Pendientes de migración a RLGL)
│   ├── migui/                    # ✅ GUI con backend SDL2 actualizado
│   ├── ...                       # Otros 22 crates del motor (rybot, ry-anim, etc.)
```

---

## 🏗️ Arquitectura de Fusión (v0.23.0)

El motor ha transicionado de una dependencia total de `SDL2::Canvas` a una arquitectura híbrida profesional:
*   **SDL2**: Gestiona la ventana, el contexto OpenGL y el Input (Event Pump).
*   **RLGL (Raylib-FFI)**: Controla directamente la GPU mediante comandos de bajo nivel compatibles con OpenGL 3.3 Core.
*   **Compatibilidad**: Se mantienen "Shims" en `ry-gfx` para permitir que el código existente siga funcionando mientras se migra a la nueva API de renderizado.

---

**Estado de los Demos**

✅ **Gameloop Validado**: Se ha confirmado que el patrón `Sdl2Backend` + `InputManager` (`events-ry`) es el estándar operativo para la arquitectura híbrida.

Todos los demos antiguos han sido movidos a `crates/ry-rs/pendientes_importantes/`. 
**Razón**: El cambio a la arquitectura sin `Canvas` rompe la compatibilidad binaria directa. 

**Plan de Recuperación**:
1.  Re-habilitar demos uno a uno en `src/bin/`.
2.  **Patrón estándar**: Usar `Sdl2Backend` + `InputManager`.
3.  Actualizar firmas (ej: `draw_text` -> `draw_text_old`).
4.  Migrar renderizado de texturas de `SDL_Texture` a texturas de Raylib (`Texture2D`).

---

<div align="center">

**Ry-Dit v0.23.0 — Hacia un Motor Multiplataforma**

*Fusión exitosa: SDL2 para el sistema, RLGL para el arte.*

</div>
