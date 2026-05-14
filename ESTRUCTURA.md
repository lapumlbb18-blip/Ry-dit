# Ry-Dit - Estructura del Proyecto v0.24.0 (Ensamblador Maestro)

**Última actualización**: 2026-05-14
**Versión**: v0.24.0 ✅ Arquitectura de Ensamblador Maestro (Master Assembler)
**Estado**: Sistema Universal RY Consolidado

---

## 🏗️ La Arquitectura del Ensamblador

Ry-Dit funciona como un **Sistema de Ensamblado** donde el núcleo orquesta subsistemas independientes:

1.  **Orquestador (ry-core)**: Proporciona el `ModuleRegistry` y el trait `RyditModule` con ganchos de ciclo de vida (`on_init`, `on_update`, `on_draw`).
2.  **Módulos (ry-rs/modules)**: Implementaciones autónomas de físicas, partículas y lógica de juego.
3.  **Ensambladores (Executor & Editor)**:
    *   `crates/ry-rs/src/executor.rs`: Ensambla módulos para ejecución de scripts.
    *   `crates/ry-editor/src/editor_state.rs`: Ensambla módulos para diseño y edición visual.

---

## 📁 Árbol de Directorios (Consolidado)

```
shield-project/
├── Cargo.toml                    # Definición del Workspace
├── README.md                     # Visión general y novedades
├── ROADMAP.md                    # Plan de versiones (v0.24.0 actual)
├── TASKS.md                      # Gestión de tareas (Actualizado)
├── BITACORA.md                   # Registro histórico de hitos
├── informe_analisis/             # 🔓 Reportes técnicos (Ahora públicos para inspiración)
│
├───crates/
│   ├── ry-core/                  # ✅ El Cerebro: Trait RyditModule y ModuleRegistry
│   ├── ry-gfx/                   # ✅ El Pincel: Backend híbrido (SDL2 + RLGL/GPU)
│   ├── ry-editor/                # ✅ El Taller: Editor ensamblador de módulos
│   │   └── src/bin/main.rs       # Binario del Editor Maestro
│   ├── ry-rs/                    # ✅ El Laboratorio: Implementación de módulos universales
│   │   └── src/modules/          # Física, Partículas, Audio, etc.
│   ├── migui/                    # ✅ La Interfaz: GUI para el editor y juegos
│   └── rybot/                    # ✅ El Supervisor: Stats y diagnóstico de subsistemas
```

---

## 📡 Flujo de Datos

1.  **Entrada**: SDL2 captura eventos en `ry-gfx`.
2.  **Orquestación**: El Ensamblador (Editor o Executor) distribuye el tiempo (`dt`) a través del `ModuleRegistry`.
3.  **Simulación**: Los módulos actualizan su estado interno (`on_update`).
4.  **Renderizado**: Los módulos dibujan directamente en la GPU usando RLGL (`on_draw`) bajo la ventana gestionada por SDL2.

---

<div align="center">

**Ry-Dit v0.24.0 — Control Total para Diseño**

*Arquitectura Universal: Escribe una vez, ensámblalo en cualquier lugar.*

</div>
