# 🔍 Documento de Fusión Importante: SDL2 + Raylib (rlgl)

**Estado**: En curso / Fase de Investigación y Diseño
**Objetivo**: Consolidar `ry-dit` como un motor multiplataforma robusto desacoplando la gestión de ventanas/input (SDL2) del renderizado gráfico (Raylib/rlgl).

---

## 1. Análisis de Viabilidad
La fusión es técnicamente viable bajo el paradigma: **SDL2 controla el ciclo de vida y eventos; Raylib (vía rlgl) controla la GPU.**

### Beneficios para ry-dit:
- **Portabilidad**: SDL2 ofrece una abstracción superior en dispositivos móviles (Android/iOS) comparado con GLFW.
- **Control**: Recuperamos el control total del *Main Loop*, permitiendo una integración profunda con los scripts de `rydit`.
- **Arquitectura**: Permite que crates como `ry-anim` sigan siendo agnósticos a la capa gráfica, facilitando el mantenimiento y escalabilidad.

---

## 2. Estrategia de Implementación

### Fase 1: Prueba de Concepto (POC)
- Crear un entorno aislado.
- Inicializar ventana/contexto con `SDL2`.
- Compilar `raylib` con `-DUSE_EXTERNAL_GLFW=OFF`.
- Inyectar el contexto de `rlgl` en la superficie de `SDL2`.
- Renderizar un objeto 3D básico (Cubo).

### Fase 2: Integración en ry-dit
- Refactorizar `ry3d-gfx` para utilizar este nuevo backend.
- Migrar sistemas de input y tipografía (TTF) a una capa de abstracción dedicada.
- Asegurar que `ry-anim` y sus módulos (partículas) sigan funcionando sin cambios.

---

## 3. Hoja de Ruta (Roadmap)
1.  **POC de Conexión SDL2 + rlgl**: Verificar el renderizado sin GLFW.
2.  **Integración del Demo Helicóptero**: Migrar la lógica del `test_py.py` a una implementación nativa en `ry-dit` utilizando la nueva arquitectura.
3.  **Refactorización de Crates**: Consolidar `ry3d-gfx` como capa de renderizado limpia.

---

*Nota: Este documento es la guía técnica para la transición de `ry-dit` hacia una arquitectura profesional multiplataforma.*
