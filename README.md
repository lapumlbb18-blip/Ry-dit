# 🛡️ Ry-Dit: Motor de Desarrollo Consolidado (v0.23.0)

**Ry-Dit** es un ecosistema de desarrollo de juegos y simulaciones diseñado específicamente para entornos de alto rendimiento y portabilidad (Android/Termux, Linux, Windows). Basado en el **Patrón Fusional**, combina la estabilidad de **SDL2** con la potencia gráfica de **Raylib 6.0**.

## 🚀 Estado Actual: v0.23.0 (Fusional Alpha)
Hemos restaurado el flujo de trabajo liderado por **SDL2** para el editor y las herramientas de usuario, relegando a Raylib a tareas de renderizado especializado.

### ✅ Logros Recientes
- **Arquitectura Híbrida**: SDL2 gestiona la ventana, el input y el texto TTF; Raylib gestiona el 3D y las partículas.
- **Editor Restaurado**: Soporte nativo para Termux-X11 con estabilidad mejorada.
- **Puente FFI**: Renderizado de viewports de Raylib directamente sobre canvas de SDL2.
- **Limpieza de Workspace**: Todos los crates (`migui`, `rybot`, `ry-gfx`, `ry-rs`) verificados y compilando.

### 🛠️ Tareas Pendientes (Próxima Sesión)
1.  **Compilación Raylib 6.0**: Construir la librería personalizada con el backend de SDL2 activo.
2.  **Sincronización de Contexto**: Validar el intercambio de texturas OpenGL entre SDL2 y Raylib sin copia de CPU.
3.  **Touch Avanzado**: Implementar el mapeo de gestos multitouch de SDL2 hacia la cámara 3D de Raylib.
4.  **Estabilización de UI**: Migrar todos los widgets de `toolkit-ry` al nuevo backend de texto profesional.

## 🏗️ Estructura del Proyecto
- `crates/ry-gfx`: Capa de abstracción gráfica dual (Líder SDL2/Raylib).
- `crates/ry-editor`: Entorno de desarrollo unificado.
- `crates/migui`: GUI de modo inmediato pura en Rust con soporte SDL2 TTF.
- `crates/rybot`: Orquestador central y subsistemas del motor.

---
**Nota**: El proyecto se encuentra en una fase de transición hacia Raylib 6.0 para maximizar la compatibilidad en dispositivos Android mediante Termux-X11.
