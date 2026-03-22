# 🛡️ RyDit - Motor de Videojuegos 2D + Lenguaje de Scripting en Rust para Android/Termux

<div align="center">

**"David vs Goliat - Un motor de videojuegos en Rust, construido 100% en un Redmi Note 8"**

[![Version](https://img.shields.io/badge/version-v0.4.0-blue.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Tests](https://img.shields.io/badge/tests-93%20passing-green.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![raylib](https://img.shields.io/badge/raylib-5.5-purple.svg)](https://www.raylib.com/)
[![Platform](https://img.shields.io/badge/platform-Android%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lapumlbb18-blip/Rydit_Engine/blob/main/LICENSE)

[📖 Documentación](#-documentación) • [🎨 migui GUI](#-migui---immediate-mode-gui-v040) • [🎮 Tank Combat](#-tank-combat---demo-v030) • [🐍 Snake Game](#-snake-game---demo-funcional) • [🚀 Roadmap](#-roadmap) • [📱 Construido en Android](#-construido-en-androidtermux) • [💬 Comunidad](#-comunidad)

</div>

---

## 🎯 ¿Qué es RyDit?

**RyDit** es un **motor de videojuegos 2D con lenguaje de scripting** escrito en **Rust** con **raylib**, diseñado para ejecutarse nativamente en **Android/Termux** sin necesidad de desktop, emuladores o IDEs pesados.

**No es solo un lenguaje** - es un motor completo con:
- 🎮 Game loop integrado
- 🎨 Renderizado gráfico (círculos, rectángulos, líneas, texto, triángulos, elipses)
- 🖱️ Input de mouse (posición, botones, wheel)
- 🎹 Input de teclado en tiempo real
- 📐 Math avanzado (sin, cos, tan, atan2, sqrt)
- 💥 Detección de colisiones 2D
- 🎲 Sistema de módulos (math, arrays, strings, io, random, time, json, colisiones)
- 🎨 **migui - Immediate Mode GUI** (botones, sliders, checkboxes, ventanas)
- 🧪 93 tests automáticos
- 📦 Snake Game + Tank Combat + Demo migui como demos

```rydit
# Tu primer juego en RyDit (3 líneas)
shield.init
ryda frame < 1000 {
    draw.circle(400, 300, 50, "rojo")
}
```

| Característica | RyDit | Godot | Love2D | PICO-8 |
|---------------|-------|-------|--------|--------|
| **Android Native** | ✅ Sí (Termux) | ❌ No | ❌ No | ❌ No |
| **Lenguaje** | RyDit (Español) | GDScript | Lua | Lua |
| **Backend** | Rust | C++ | C | C |
| **Binario** | ~735 KB | ~50 MB | ~10 MB | ~5 MB |
| **Sin IDE** | ✅ Sí | ❌ Requiere editor | ⚠️ VS Code | ⚠️ Editor propio |
| **Game Loop** | ✅ Integrado | ✅ Integrado | ✅ Integrado | ✅ Integrado |

---

## 🎮 Snake Game - Demo Funcional

<div align="center">

![Snake Game](screenshots/04_snake_gameplay.jpg)

*Snake Game completo con game loop, colisiones, puntuación y game over screen*

</div>

### Características del Snake
- ✅ **Cuerpo de serpiente** con arrays dinámicos
- ✅ **Comida aleatoria** con `random::int()`
- ✅ **Colisiones** con paredes y propio cuerpo
- ✅ **Puntuación** + high score
- ✅ **Velocidad progresiva**
- ✅ **Game Over** + restart con SPACE
- ✅ **Pausa** con P
- ✅ **Salir** con ESC

### Ejecutar Snake
```bash
# En Termux (Android)
cargo run --bin rydit-rs -- --gfx snake.rydit

# O con binario directo
./target/release/rydit-rs --gfx snake.rydit
```

### Controles
| Tecla | Acción |
|-------|--------|
| `↑` `→` `↓` `←` | Mover serpiente |
| `P` | Pausa |
| `SPACE` | Reiniciar |
| `ESC` | Salir |

---

## 🎨 migui - Immediate Mode GUI (v0.4.0)

<div align="center">

**¡NUEVO EN v0.4.0!** Sistema de GUI estilo raygui para interfaces rápidas

</div>

### Características de migui
- ✅ **Immediate Mode** - Sin estado complejo, fácil de usar
- ✅ **Backend agnóstico** - Funciona con raylib, terminal, web
- ✅ **8 widgets** - button, label, checkbox, slider, panel, textbox, window, message_box
- ✅ **Ventanas arrastrables** - Headers con drag-and-drop
- ✅ **Input de mouse** - Posición, clicks, estado
- ✅ **Colores personalizables** - 14 colores predefinidos
- ✅ **~600 líneas Rust** - Ligero y sin dependencias

### Ejecutar Demo migui
```bash
# En Termux (Android)
cargo run --bin rydit-rs -- --migui demos/demo_migui.rydit

# O con binario directo
./target/release/rydit-rs --migui demos/demo_migui.rydit
```

### Ejecutar Editor de Escenas
```bash
cargo run --bin rydit-rs -- --migui demos/editor_escenas.rydit
```

### Código de Ejemplo
```rydit
# demo_migui.rydit
contador = 0
slider_valor = 50
checkbox_estado = false

{
    # Panel de fondo
    migui::panel("fondo", 10, 10, 300, 400, "panel")
    
    # Botón
    si = migui::button("btn_inc", "Incrementar", 120, 60, 100, 40)
    si {
        contador = contador + 1
    }
    
    # Label
    migui::label("lbl_cont", str("Contador: ") + str(contador), 120, 110, 100, 30)
    
    # Slider
    slider_valor = migui::slider("slider", slider_valor, 0, 100, 120, 160, 160, 30)
    
    # Checkbox
    cambio = migui::checkbox("chk", "Activar", checkbox_estado, 120, 210, 150, 30)
    si cambio {
        checkbox_estado = !checkbox_estado
    }
    
    # Ventana arrastrable
    migui::window("ventana", "Mi Ventana", true, 350, 50, 300, 200)
}
```

### API Completa
| Función | Parámetros | Retorna | Descripción |
|---------|------------|---------|-------------|
| `migui::button` | id, text, x, y, w, h | bool | Botón clickeable |
| `migui::label` | id, text, x, y, w, h | - | Texto estático |
| `migui::checkbox` | id, text, checked, x, y, w, h | bool | Checkbox toggle |
| `migui::slider` | id, value, min, max, x, y, w, h | f32 | Slider numérico |
| `migui::panel` | id, x, y, w, h, color | - | Panel contenedor |
| `migui::textbox` | id, x, y, w, h | String | Input de texto |
| `migui::window` | id, title, open, x, y, w, h | bool | Ventana arrastrable |
| `migui::message_box` | title, message, buttons, x, y, w, h | i32 | Diálogo con botones |
| `migui::mouse_x` | - | f64 | Posición X del mouse |
| `migui::mouse_y` | - | f64 | Posición Y del mouse |
| `migui::mouse_position` | - | [x, y] | Posición completa |
| `migui::is_mouse_pressed` | - | bool | Estado del click |

---

## 🎮 Tank Combat - Demo v0.3.0

<div align="center">

**¡NUEVO EN v0.3.0!** Tanque de combate con seguimiento de mouse y disparos

</div>

### Características de Tank Combat
- ✅ **Input de mouse** - La torreta sigue el cursor
- ✅ **Disparos** - Click izquierdo para disparar
- ✅ **Matemáticas avanzadas** - atan2, sin, cos para trayectorias
- ✅ **Colisiones** - Detección bala-enemigo
- ✅ **IA enemiga** - Tanque enemigo que persigue al jugador
- ✅ **Health bars** - Sistema de vida
- ✅ **Victory/Game Over** - Pantallas de fin de juego

### Ejecutar Tank Combat
```bash
# En Termux (Android)
cargo run --bin rydit-rs -- --gfx demos/tank_combat.rydit

# O con binario directo
./target/release/rydit-rs --gfx demos/tank_combat.rydit
```

### Controles
| Control | Acción |
|---------|--------|
| `Mouse` | Apuntar torreta |
| `Click Izquierdo` | Disparar |
| `ESC` | Salir |

### Código de Ejemplo
```rydit
shield.init
import math
import colisiones

dark.slot tanque_x = 400
dark.slot tanque_y = 300

ryda frame < 10000 {
    # Input de mouse
    dark.slot mouse_x = input::mouse_x()
    dark.slot mouse_y = input::mouse_y()
    
    # Calcular ángulo hacia mouse
    dark.slot angulo = math::atan2(mouse_y - tanque_y, mouse_x - tanque_x)
    
    # Dibujar tanque
    draw.circle(tanque_x, tanque_y, 30, "verde")
    
    # Disparar con click
    onif input::is_mouse_button_pressed(0) == 1 {
        # Lógica de disparo...
    }
    
    # Colisiones
    onif colisiones::circulo_circulo(...) == 1 {
        # Impacto detectado
    }
}
```

---

## 🎨 Demo Visual - Formas y Colores

<div align="center">

![Demo Shapes](screenshots/03_demo_shapes_circulos.jpg)

*Demo de formas geométricas con draw.circle(), draw.rect(), draw.line(), draw.text()*

</div>

---

## 📸 Galería de Capturas

<div align="center">

| Demo rydit-gfx v0.0.7 | Demo Shapes | Snake Gameplay |
|--------------|--------------|----------------|
| ![Demo rydit-gfx](screenshots/02_demo_rydit_gfx_completo.jpg) | ![Demo Shapes](screenshots/03_demo_shapes_circulos.jpg) | ![Snake](screenshots/04_snake_gameplay.jpg) |
| Círculo rojo animado, rectángulo verde, línea azul | Círculos concéntricos, rectángulos de colores | Snake en movimiento, grid retro, comida roja |

| Snake Game Over | Menú Demo |
|--------------|--------------|
| ![Game Over](screenshots/05_snake_gameover.jpg) | ![Menú](screenshots/01_demo_rydit_gfx_menu.jpg) |
| Pantalla de Game Over, puntuación, high score | Menú de selección de demos Termux-X11 |

</div>

**Todas las capturas fueron tomadas en un Redmi Note 8 con Termux-X11 + raylib** 📱🎮

```rydit
shield.init

ryda frame < 500 {
    draw.circle(400, 200, 80, "rojo")
    draw.rect(200, 350, 60, 60, "naranja")
    draw.line(100, 500, 300, 500, "blanco")
    draw.text("Demo RyDit", 250, 50, "amarillo")
}
```

---

## 📖 Sintaxis del Lenguaje

### Funciones Básicas
```rydit
rytmo saludar {
    voz "Hola Mundo"
    return 1
}

saludar()
```

### Funciones con Parámetros
```rydit
rytmo saludar(nombre) {
    voz "Hola " + nombre
}

saludar("Mundo")
```

### Condicionales
```rydit
dark.slot x = 10
onif x > 5 voz "Mayor" blelse voz "Menor"
```

### Ciclos
```rydit
dark.slot x = 3
ryda x {
    voz x
    dark.slot x = x - 1
}
```

### Arrays
```rydit
# Array básico
dark.slot lista = [1, 2, 3]

# Multidimensional (tablero)
dark.slot tablero = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]

# Asignación por índice
dark.slot lista[0] = 5
```

### Sistema de Módulos
```rydit
# Importar módulos
import math
import arrays
import strings

# Usar funciones con namespace
dark.slot suma = math::sumar(10, 3)
dark.slot len = arrays::length([1, 2, 3])
dark.slot upper = strings::upper("hola")
```

### Gráficos (Modo Ventana)
```rydit
shield.init

# Dibujar formas
draw.circle(400, 300, 50, "rojo")
draw.rect(100, 100, 100, 100, "verde")
draw.line(0, 0, 800, 600, "azul")
draw.text("RyDit v0.1.9", 300, 50, "blanco")
```

---

## 🏗️ Arquitectura

```
┌─────────────────────────────────────────────────────────┐
│  RyDit Core (Rust)                                      │
│  ├── lizer       → Lexer + Parser + AST (~2,452 líneas) │
│  ├── blast-core  → Executor + Memoria (~465 líneas)     │
│  ├── rydit-gfx   → Gráficos raylib (~481 líneas)        │
│  ├── rydit-rs    → Binario + stdlib (~2,491 líneas)     │
│  └── v-shield    → Wrapper raylib (~120 líneas)         │
└─────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────┐
│  Scripts RyDit (.rydit)                                 │
│  ├── Snake Game                                         │
│  ├── Demos visuales                                     │
│  ├── Módulos stdlib (math, arrays, strings, io, etc.)   │
│  └── Juegos de la comunidad                             │
└─────────────────────────────────────────────────────────┘
```

### Métricas del Proyecto
```
Líneas totales:     ~8,000 líneas (+800 en v0.3.0)
├── Rust:           ~6,200 líneas
└── RyDit:          ~1,800 líneas (demos + módulos + tests)

Tests automáticos:  90 pasando ✅
Demos funcionales:  12/12 ✅ (Snake + Tank Combat + Math Demo)
Warnings activos:   0 ✅

Binarios:
├── rydit-rs:       ~760 KB (release)
└── snake:          ~505 KB
```

---

## 📱 Construido en Android/Termux

<div align="center">

| Setup | Especificaciones |
|-------|-----------------|
| **Dispositivo** | Redmi Note 8 |
| **OS** | Android 11 |
| **Terminal** | Termux |
| **RAM** | 4 GB |
| **Almacenamiento** | 64 GB |
| **IDE** | Ninguno (vim/nano) |
| **Teclado** | Pantalla táctil + Bluetooth |

</div>

**Este proyecto fue construido 100% en un teléfono Android**, sin laptop, sin desktop, sin IDE. Solo:
- 📱 Teléfono Redmi Note 8
- ⌨️ Terminal Termux
- 🦀 Rust + Cargo
- 🎨 Raylib (nativo)

**Filosofía:** Demostrar que el desarrollo serio es posible en dispositivos móviles cuando tienes arquitectura clara, tests automatizados, buena documentación y determinación.

---

## 🚀 Roadmap

<div align="center">

| Versión | Estado | Features Principales | Fecha |
|---------|--------|---------------------|-------|
| **v0.0.1-v0.0.14** | ✅ | CLI → Snake Game | 2026-03-14 a 2026-03-16 |
| **v0.1.0** | ✅ | Snake Game Completo | 2026-03-17 |
| **v0.1.1** | ✅ | Sistema de Módulos | 2026-03-17 |
| **v0.1.4** | ✅ | Strings + IO + Arrays | 2026-03-18 |
| **v0.1.6** | ✅ | Random + Time | 2026-03-18 |
| **v0.1.8** | ✅ | Maduración + Gráficos | 2026-03-20 |
| **v0.1.9** | ✅ | **110 Tests Checkpoint** | 2026-03-20 |
| **v0.2.0** | ✅ | +12 Colores + 5 Formas + rybot.sh | 2026-03-21 |
| **v0.3.0** | ✅ | **Tank Combat** (Mouse + Math + Colisiones) | 2026-03-21 |
| **v0.4.0** | 🔜 | **migui** (Immediate Mode GUI) | Próxima |
| **v0.5.0** | 🔮 | **Ecosistema Maduro** (integración) | 2 meses |
| **v0.6.0** | 🔮 | **Motor de Escenas** (nodos, señales) | 3 meses |
| **v1.0.0** | 🔮 | Production Ready | 4-6 meses |

</div>

---

## 🎯 Estado del Proyecto

### ✅ Completado (v0.3.0 - Tank Combat)
- [x] Lexer + Parser con AST
- [x] Executor con memoria y scopes
- [x] Sistema de módulos (import)
- [x] 90 tests automáticos
- [x] 12 demos funcionales
- [x] Snake Game completo
- [x] **Tank Combat** (input mouse, math, colisiones)
- [x] Gráficos con raylib (9 formas)
- [x] **Input de mouse** (posición, botones, wheel)
- [x] **Math avanzado** (atan2, sin, cos, tan, sqrt)
- [x] **Módulo colisiones** (5 funciones)
- [x] Strings, IO, Arrays maduros
- [x] Soporte JSON
- [x] Random + Time ligeros
- [x] UTF-8 completo
- [x] Escapes en strings
- [x] Símbolos en identificadores
- [x] +12 colores (18 totales)
- [x] rybot.sh (bot de ayuda)

### 🔜 Próximamente (v0.4.0 - migui)
- [ ] migui (Immediate Mode GUI ~1000 líneas)
- [ ] Ventanas arrastrables
- [ ] Botones, sliders, labels
- [ ] Message boxes, paneles
- [ ] Editor de escenas visual
- [x] Símbolos en identificadores

### 🔜 Próximamente (v0.2.0 - v0.6.0)
- [ ] Module system avanzado (cache, cíclicos)
- [ ] Chatbot asistente de código
- [ ] migui (Immediate Mode GUI ~1000 líneas)
- [ ] Ecosistema maduro (integración con otras herramientas)
- [ ] Motor de escenas (nodos, señales, prefabs)
- [ ] Editor visual de escenas
- [ ] Ecosistema de frameworks (RPG, platformer)
- [ ] Asset store comunitario

---

## 🧪 Evaluación de la Comunidad

Este proyecto está siendo revisado por la comunidad de desarrolladores. Las evaluaciones detalladas de asistentes de IA se incluirán en la próxima actualización cuando el repositorio sea público.

> **"Espero tu evaluación de este proyecto con buena intención. Es mostrar lo que se hace en un celular gama baja, y que a futuras versiones con su apoyo, osea la comunidad, crezca en ecosistema con la capacidad enorme de la comunidad, para que creen sus escenas en hardware modesto sin depender de IA que hace un video rápido y sin experiencia propia. Esa es una de las claves."**

---

## 💬 Comunidad

### 🌐 Únete y Evalúa este Proyecto

| Plataforma | Enlace |
|-----------|--------|
| **Discord Mouredev** | https://discord.gg/mouredev |
| **Reddit r/rust** | https://reddit.com/r/rust |
| **Reddit r/gamedev** | https://reddit.com/r/gamedev |
| **Reddit r/AndroidGaming** | https://reddit.com/r/AndroidGaming |
| **X (Twitter)** | #RyDit #RustLang #AndroidDev |

### 💌 Tu Opinión Importa

¿Qué piensas de este proyecto? ¿Crees que es posible crear un motor de videojuegos completo en un celular gama baja?

**Tu evaluación ayuda a:**
- 📱 Demostrar que el desarrollo en Android es posible
- 🤝 Construir una comunidad que comparte conocimiento real
- 🎮 Crear un ecosistema donde todos pueden hacer sus juegos
- 🚀 Inspirar a otros con hardware modesto

### 🔜 Próximamente

- **Servidor Discord propio** - Espacio dedicado para RyDit
- **Evaluaciones públicas de IA** - Análisis detallado del código
- **Asset store comunitario** - Frameworks, escenas, herramientas

### 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Lee [CONTRIBUTING.md](CONTRIBUTING.md) para más detalles.

```bash
# Clonar repositorio
git clone https://github.com/alucard18/shield-project.git

# Build
cd shield-project
cargo build

# Tests
cargo test

# Ejecutar demo
cargo run --bin rydit-rs -- --gfx demo_shapes.rydit
```

---

## 📚 Documentación

| Documento | Descripción |
|-----------|-------------|
| **[README.md](README.md)** | Documentación técnica interna |
| **[LIBRO_RYDIT.md](LIBRO_RYDIT.md)** | Guía completa del lenguaje (~400 líneas) |
| **[ROADMAP.md](ROADMAP.md)** | Planificación detallada |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | Guía de contribuciones |
| **[CHANGELOG_v0.1.8.md](CHANGELOG_v0.1.8.md)** | Cambios de versión |
| **[diagnostico/](diagnostico/)** | Logs de desarrollo (25 sesiones) |

---

## 🎮 Ejemplos de la Comunidad

### RPG Simple
```rydit
import rpg::engine

rytmo juego {
    rpg.iniciar("mi_rpg.rydit")
    rpg.crear_personaje("heroe", ["espada", "escudo"])
    rpg.iniciar_dialogo("npc", ["Hola", "Adiós"])
}
```

### Platformer
```rydit
import platformer::physics

platformer.fisica.gravedad(9.8)
dark.slot jugador = platformer.crear_jugador(100, 200)

ryda frame < 10000 {
    platformer.mover(jugador, "derecha")
    onif platformer.colision(jugador, "suelo") {
        platformer.saltar(jugador)
    }
}
```

### Visual Novel
```rydit
shield.init

dark.slot nombre = input("¿Cómo te llamas?")

ryda frame < 500 {
    draw.text("Hola " + nombre, 200, 100, "blanco")
    draw.text("¿Qué haces hoy?", 200, 200, "blanco")
    
    onif gui.button("Estudiar", 100, 300, 200, 50) {
        draw.text("¡Buena decisión!", 200, 400, "verde")
    }
}
```

---

## 🏆 Logros

### Sesión 26 (v0.1.9) - Checkpoint 100 Tests
- ✅ **110 tests pasando** (meta 100 superada)
- ✅ **0 warnings, 0 errors**
- ✅ **8/8 demos funcionales**
- ✅ **Termux-X11 activado** (5 screenshots)
- ✅ **Backup Google Drive sincronizado**

### General
- ✅ **27 sesiones en 8 días** (v0.0.1 → v0.4.0)
- ✅ **6 crates funcionales** (lizer, blast-core, rydit-gfx, v-shield, rydit-rs, migui)
- ✅ **~9,200 líneas de código**
- ✅ **Documentación completa**
- ✅ **93 tests automáticos**

---

## 💾 Backup

- **Google Drive:** `alucard18:/shield-project-rydit`
- **Archivos:** 120+
- **Tamaño:** ~200 KB (sin `target/`)
- **Última sync:** 2026-03-22 (v0.4.0)

---

## 📄 Licencia

MIT License - Ver [LICENSE](LICENSE) para más detalles.

---

## 🙏 Agradecimientos

- **Comunidad Mouredev** - Discord: https://discord.gg/mouredev - Por el apoyo y espacio para compartir proyectos
- **raylib** (https://www.raylib.com/) - Por la biblioteca gráfica más ligera y fácil de usar
- **Rust** (https://www.rust-lang.org/) - Por el lenguaje más amado por 8 años consecutivos
- **Termux** - Por hacer posible el desarrollo en Android sin root

---

<div align="center">

## 🚀 "Construido con ❤️ en Android/Termux"

**"No necesitas una laptop cara para crear software impresionante. Solo necesitas un teléfono, determinación y mucha café."** ☕

**"Este proyecto es una invitación a la comunidad: miren lo que se puede hacer en un celular gama baja. Mi sueño es que a futuras versiones, con su apoyo, crezcamos en ecosistema. Que todos puedan crear sus escenas y juegos en hardware modesto, sin depender de herramientas que hacen todo rápido pero sin experiencia propia. Esa es la clave: aprender creando, no solo consumiendo."**

---

*¿Quieres evaluar este proyecto?* Únete al **Discord Mouredev**: https://discord.gg/mouredev y comparte tu opinión en #mostrar-proyecto

*Próxima actualización:* v0.4.1 con backend raylib para migui y más widgets

*Última actualización:* 2026-03-22 (v0.4.0 - migui Immediate Mode GUI)
*Próxima versión:* v0.4.1 (migui backend raylib + más widgets)
*Estado:* ✅ **93 TESTS - 0 WARNINGS - v0.4.0 COMPLETADA**

[⬆️ Volver arriba](#-rydit---rust-gaming--scripting-engine-for-androidtermux)

</div>
