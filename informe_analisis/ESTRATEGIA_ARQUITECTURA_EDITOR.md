# Informe de Análisis: Hacia el Editor Ry-Dit v0.22.0 (Arquitectura Híbrida SDL2 + Raylib)

## 1. Problema Actual
La arquitectura actual presenta inestabilidad en Termux/Android debido a la **duplicidad de contextos gráficos**:
- **Raylib/GLFW**: Intenta gestionar la ventana principal y el input global, lo cual genera conflictos con el SurfaceFlinger de Android y el servidor X11.
- **SDL2**: Está presente en el backend, pero no es el dueño absoluto, lo que impide que el input táctil sea 100% fluido y preciso para los widgets.

## 2. Visión del Editor (El "Puente")
La meta es que el editor sea un entorno donde:
- **SDL2 es el Orquestador Maestro**: Gestiona la ventana, el input nativo, los botones, los menús y los widgets de `migui`. SDL2 es el responsable de la lógica táctil precisa.
- **Raylib es el Renderizador de Viewports**: Actúa como un "esclavo" de renderizado. Solo se le pide que pinte escenas 3D en texturas (Render-to-Texture), las cuales SDL2 copiará al canvas principal.

## 3. Claves para la Funcionalidad (Sugerencias)
- **Separación de Contextos**:
  - `RybotGui` debe emitir `DrawCommand`s que el `Sdl2Backend` sepa interpretar nativamente (rectángulos, texto, botones), ignorando a Raylib para la UI.
  - El Viewport debe ser el único `DrawCommand` que delegue a `ry-gfx` (Raylib).
- **Consolidación de Input**:
  - Todo el flujo de eventos (`procesar_eventos_sdl2`) debe alimentar a `InputManager` de SDL2. La lógica del editor (clics en botones, arrastre de gizmos) debe consultar este estado unificado, nunca el de GLFW.
- **Hot Reload**:
  - Aprovechar que SDL2 permite recargar texturas de forma más limpia. Implementar un "watchdog" de archivos que notifique al `AssetsManager` cuando un sprite o shader cambie, forzando la actualización de la textura en memoria.

## 4. Hoja de Ruta Inmediata
1. **Refactor de Backend**: Asegurar que `backend_sdl2.rs` renderice todos los widgets sin depender de Raylib.
2. **Validación**: Tomar el "Demo Torreta vs Sprites" como base de referencia, ya que su lógica de eventos es estable.
3. **Gizmos Nativos**: Implementar la manipulación de gizmos (flechas de ejes) directamente como widgets de `migui` (SDL2), manteniendo el cubo 3D como simple referencia visual del viewport.

---
*Análisis generado para la sesión de trabajo sobre la consolidación del motor gráfico y UI.*
