# 🛡️ Ry-Dit — Sistema de Creación en Rust para Android/Termux

> **Ry-Dit no es solo un motor de juegos.**  
> Es un sistema de creación para desarrolladores, artistas, soñadores y streamers  
> que quieren construir lo que imaginen — sin límites de categoría ni de hardware.
> low-end first
> *Construido desde un Redmi Note 8 (4GB RAM, Adreno 610) en Termux. Sin PC. Sin equipo.*

![Logo](screenshots/logo.png)

**"Construido sin prisa, madurado con paciencia"** — Filosofía *Low-End First*

[![Version](https://img.shields.io/badge/version-v0.18.0-blue.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Errors](https://img.shields.io/badge/errors-0-green.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Tests](https://img.shields.io/badge/tests-147%2F147-brightgreen.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![SDL2](https://img.shields.io/badge/SDL2-0.37-red.svg)](https://www.libsdl.org/)
[![Raylib](https://img.shields.io/badge/raylib-5.0-orange.svg)](https://www.raylib.com/)
[![Platform](https://img.shields.io/badge/platform-Android%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/lapumlbb18-blip/Ry-dit)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-12%20publicados-purple.svg)](https://crates.io/crates/ry-anim)

[🚀 Inicio Rápido](#-inicio-rápido) • [🎮 Demos](#-demos-funcionales) • [🆕 Novedades](#-novedades-v0180) • [🏆 Logros](#-logros) • [📦 Crates](#-crates-publicados) • [🤖 IA en el Desarrollo](#-ia-en-el-desarrollo) • [🎯 Roadmap](#-roadmap)

---

## ¿Qué puedes crear con Ry-Dit?

| Categoría | Ejemplo |
|-----------|---------|
| 🎮 Juegos 2D/3D | Snake, Buscaminas, Torreta, Platformers |
| 🛠️ Editores y herramientas | Editor visual, IDE, dashboard |
| 🎓 Software educativo | Simulaciones, visualizaciones interactivas |
| 🔬 Ciencia | Bezier, ondas, L-System, ilusiones ópticas |
| 📡 Streaming LAN | Servidor WebSocket + portal web |
| 🎬 Animaciones | 12 principios Disney, sprite animation |

---

## 🚀 Inicio Rápido

```bash
# Clonar
git clone https://github.com/lapumlbb18-blip/Ry-dit.git
cd Ry-dit

# Compilar
cargo build -p ry-rs --bin rydit-rs --release

# Tests
cargo test --workspace
```

### Scripting con .rydit

```
# mi_nivel.rydit
tilemap 2400 1800
  tileset "sprites/tileset.png" 32 32
  layer 0 "mapa.csv"

camera follow player
  zoom 1.0

entity player
  sprite "player.png"
  physics dynamic
  collider rect 32 48
```

### Stack técnico

| Capa | Tecnología |
|------|-----------|
| **Lenguaje** | Rust |
| **Scripting** | `.rydit` (lenguaje propio) |
| **2D/3D** | Raylib + rlgl + SDL2 + ry3d-gfx |
| **GPU** | RyScale (upscaling propio) + GPU Instancing |
| **Físicas** | rapier2d + ry-physics |
| **Audio** | SDL2 Mixer + 4 buses + spatial 2D |
| **Red** | Tokio + Tungstenite + LAZOS JSON/RPC |
| **GUI** | migui (12+ widgets) + controles táctiles |
| **ECS** | bevy_ecs algunas features de inspiracion |
| **CI/CD** | GitHub Actions |

**Plataformas**: `Android` · `Linux` · `Windows` · *(WASM en roadmap)*
`MacOS`
---

## 🆕 Novedades v0.18.0

**Estado**: 23 crates · 0 errores · 147 tests · 12 crates.io · 20+ demos

| Feature | Detalle |
|---------|---------|
| **3D con controles en pantalla** | Escena 3D interactiva — cámara orbital + controles táctiles funcionales |
| **ry3d-gfx v0.1.0** | 15 primitivas: cubos, esferas, cilindros, líneas, grid, ejes, bbox |
| **19 Transiciones** | Fade, Slide, Wipe, Zoom, Circle, Blinds, Dissolve, Spiral... |
| **Audio Mixer** | 4 buses + spatial 2D + fade in/out |
| **RyScale** | Sistema de upscaling espacial propio (base FSR 1.0 + experimental NIS-style) |
| **UTF-8 Fix** | TTF\_RenderUTF8\_Blended → acentos y emojis correctos |
| **Emoji Atlas** | 25+ emojis como sprites procedurales PNG |

> **[ACTUALIZAR]** Agrega aquí los datos específicos de la sesión más reciente (líneas añadidas, bugs fixeados, commits).

---

## 🎮 Demos Funcionales

| Demo | Descripción | Tamaño |
|------|-------------|--------|
| **demo\_3d\_primitives** | 🧊 Escena 3D: cubos, esferas, cilindros + controles táctiles | — |
| **demo\_transitions** | 🎬 19 transiciones tipo editor de video | — |
| **demo\_militar** | 🎖️ Soldado procedural + partículas + granadas en arco + salto | — |
| **demo\_emoji\_utf8** | 😀 25+ emojis sprites + texto UTF-8 | — |
| **demo\_audio\_mixer** | 🎵 Mixer interactivo 4 buses + spatial 2D | — |
| **demo\_anime\_ry\_v2** | 🐍 Snake completo + cámara follow + minimap | — |
| **demo\_buscaminas** | 💣 Buscaminas 16×16 con flood fill | — |
| **demo\_gpu\_instancing** | ⚡ 50K partículas GPU instancing a 48 FPS | ~500K |
| **demo\_fsr** | 🔍 RyScale upscaling 960→1280 | ~480K |
| **demo\_torreta\_vs\_sprites** | 🎮 Juego completo: 3 niveles + AI + audio | 434K |
| **demo\_rigidbody** | Física + colisiones + sprites PNG | 446K |
| **demo\_hud\_camera** | HUD + Cámara 2D rotación/zoom | — |
| **demo\_action\_sprite** | 🎬 Sprite animation con state machine | — |
| **demo\_platformer\_completo** | Plataformas + gravedad + salto | — |
| **demo\_panel\_visual** | 4 paneles + consola interactiva | 339K |
| **demo\_50k\_particulas** | 50K partículas — benchmark de referencia | 313K |

---

## 🏆 Logros

| # | Logro | Versión | Detalle |
|---|-------|---------|---------|
| 1 | **GPU Instancing** | v0.15.0 | 50K partículas a 48 FPS en Adreno 610 |
| 2 | **RyScale (FSR 1.0)** | v0.15.0 | 960×540 → 1280×720 a 48 FPS |
| 3 | **12 Crates publicados** | v0.16.0 | ry-god, ry-stream, v-shield, ry-backend, migui, ry-gfx, ry-core, ry-anim, toolkit-ry, ry-config, ry-physics, ry-science |
| 4 | **147 Tests** | v0.16.1 | ry-anim: 65, toolkit-ry: 14, ry-physics: 6, ry-science: 21 + más |
| 5 | **3D funcional en Android** | v0.18.0 | ry3d-gfx con controles táctiles en Termux-X11 |
| 6 | **20+ Demos funcionales** | v0.18.0 | Todos corriendo en Redmi Note 8 |
| 7 | **0 Errores workspace** | v0.18.0 | 23 crates compilando limpio |
| 8 | **~30K líneas en 27 días** | v0.17–v0.18 | 1 desarrollador, Android/Termux, sin PC |

---

## 📦 Crates Publicados

| # | Crate | Versión | Descripción |
|---|-------|---------|-------------|
| 1 | ry-god | 0.1.0 | Security & Efficiency core |
| 2 | ry-stream | 0.2.0 | LAN streaming |
| 3 | v-shield | 0.2.0 | Platform layer + sync |
| 4 | ry-backend | 0.1.0 | Dual backend (raylib + SDL2) |
| 5 | migui | 0.4.1 | Immediate Mode GUI — 12+ widgets |
| 6 | ry-gfx | 0.10.8 | GPU Instancing + RyScale |
| 7 | ry-core | 0.8.2 | Core trait + registry |
| 8 | ry-anim | 0.12.0 | 12 principios Disney + action\_sprite |
| 9 | toolkit-ry | 0.1.0 | UI toolkit + 5 themes + HUD |
| 10 | ry-config | 0.1.0 | Config parser zero-deps |
| 11 | ry-physics | 0.7.34 | Projectile, N-body, gravity |
| 12 | ry-science | 0.7.34 | Bezier, stats, optical illusions |

---

## 🖼️ Galería

### Videos — Demos en Termux-X11 (Adreno 610)

**Recorrido general (4 partes — 4:29 min total)**

| Parte 1 — 1:07 | Parte 2 — 1:07 |
|----------------|----------------|
| ![Demo Parte 1](ry-galery_contenido/varios_demos_parte_1.gif) | ![Demo Parte 2](ry-galery_contenido/varios_demos_parte_2.gif) |

| Parte 3 — 1:07 | Parte 4 — 1:08 |
|----------------|----------------|
| ![Demo Parte 3](ry-galery_contenido/varios_demos_parte_3.gif) | ![Demo Parte 4](ry-galery_contenido/varios_demos_parte_4.gif) |

**Demo GPU Instancing — 50K Partículas a 48 FPS**

![Partículas GPU](ry-galery_contenido/demo_particles.gif)

**Demo Torreta vs Sprites — Juego Completo**

![Torreta Demo](ry-galery_contenido/demo_torreta_vs_sprites.gif)

---

## 🤖 IA en el Desarrollo

Ry-Dit usa IA como **herramienta de síntesis bajo dirección del autor**, no como generador automático. Esto es relevante en 2026, cuando la comunidad exige transparencia.

### Metodología real

El autor actúa como **arquitecto y guía**. Las IA (Claude, Qwen) aportan:
- Síntesis de patrones técnicos
- Revisión de código propuesto
- Análisis comparativo de opciones de diseño

Lo que **no hacen las IA**: decidir arquitectura, elegir qué entra al proyecto, validar que algo funciona en Adreno 610 en Termux. Eso es trabajo del autor.

### Comparativa honesta de herramientas usadas

| IA | Uso en RyDit | Fortaleza real | Limitación real |
|----|-------------|----------------|-----------------|
| **Claude** | Arquitectura, análisis técnico profundo, feedback sin filtros | Razonamiento técnico con contexto largo | Sin acceso directo al repo en tiempo real |
| **Qwen** | Sesiones de código en Termux, bitácora técnica | Disponible sin suscripción, útil en mobile | Tiende a generar código sin validar arquitectura |

> Los tests, demos y benchmarks en este repo son la prueba de que el código funciona.  
> No hay afirmaciones sin evidencia ejecutable.

### ¿Por qué esto importa?

La comunidad técnica está saturada de proyectos generados por IA sin autor real. Ry-Dit tiene 360 commits, 147 tests que pasan, binarios que corren en hardware de gama baja real, y un solo desarrollador que puede explicar cada decisión de arquitectura.

---

## 🎯 Roadmap

**3 Pilares**: 🎮 Gaming · 🎬 Animaciones+Ciencia · 📡 Streaming+Comunidad

| Versión | Features | Estado |
|---------|----------|--------|
| **v0.18.0** | 3D + controles táctiles + Transiciones + Audio Mixer + RyScale + UTF-8 | ✅ En curso |
| **v0.19.0** | Letras 3D + Escenas (.ryscene) + Input map + Rybot CLI+GUI | 🔜 |
| **v0.20.0** | Editor visual + Asset pipeline + LAZOS (Python+C++) + Multiplayer LAN | 🔜 |
| **v1.0.0** | GitHub Actions binaries + Motor completo + Debugger + Comunidad | 🎯 Meta |

> Ver `ROADMAP.md` para plan detallado. Ver `docs/DEVLOG.md` para bitácora técnica sesión a sesión.

---

## 📚 Documentación

| Archivo | Descripción |
|---------|-------------|
| `ROADMAP.md` | Plan de versiones con análisis comparativo de motores |
| `TASKS.md` | Tareas completadas y pendientes |
| `MANIFIESTO.md` | Filosofía Low-End First |
| `ESTRUCTURA.md` | Estructura del workspace y crates |
| `BITACORA.md` | Bitácora técnica — analisis evaluaciones sesiones, bugs, decisiones |
| `GUIA_USUARIO.md` | Guía de instalación y uso |
| `CONTRIBUTING.md` | Guía de contribución |

---

## Desarrollo

Proyecto desarrollado por **1 persona** desde Android/Termux.  
IA utilizada como herramienta de síntesis bajo dirección del autor.  
Cada línea tiene un commit. Cada demo tiene evidencia ejecutable.

---

**🛡️ Ry-Dit v0.18.0 — Low-End First**

*23 crates · 147 tests · 12 crates.io · 20+ demos · 0 errores*

*🎮 Gaming · 🎬 Animaciones+Ciencia · 📡 Streaming+Comunidad*

**Próximo: v0.19.0 — Letras 3D + Escenas + Input map + Rybot**
