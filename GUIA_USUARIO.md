# 🛡️ Ry-Dit - Guía del Usuario

**Versión**: v0.27.0 (en curso)
**Última actualización**: 2026-09-27

---

## 📋 Índice

1. [¿Qué es Ry-Dit?](#qué-es-ry-dit)
2. [Rybot Engine](#rybot-engine)
3. [Requisitos](#requisitos)
4. [Instalación](#instalación)
5. [Ejecutar Demos](#ejecutar-demos)
6. [Controles de los Demos](#controles-de-los-demos)
7. [Mandos y Joysticks (nuevo v0.27.0)](#mandos-y-joysticks-nuevo-v0270)
8. [TUI en Línea de Comandos (nuevo v0.27.0)](#tui-en-línea-de-comandos-nuevo-v0270)
9. [Scripting .rydit](#scripting-rydit)
10. [Crear Niveles con .rydit](#crear-niveles-con-rydit)
11. [Tests y Memoria Procedimental](#tests-y-memoria-procedimental)
12. [Troubleshooting](#troubleshooting)

---

## ¿Qué es Ry-Dit?

Ry-Dit es un **motor de juegos 2D + lenguaje de scripting en Rust**, diseñado para funcionar en dispositivos de gama baja como el Redmi Note 8 (Adreno 610) corriendo Android/Termux.

### Características principales

- **Motor 2D completo**: Sprites PNG, texto TTF, física, colisiones, audio
- **Gamepads y joysticks** (v0.27.0): soporte SDL2 GameController con dead zones, rumble y hotplug
- **TUI** (v0.27.0): widgets de terminal renderizados vía SDL (estilo Go bubbletea)
- **Lenguaje de scripting .rydit**: Scripting en español con matemáticas, arrays, Vec2
- **GPU Instancing**: Hasta 150K partículas en un solo draw call
- **FSR 1.0**: Upscaling AMD para mejorar rendimiento
- **Health Bars + HUD**: Sistema de HUD world-space con color dinámico
- **Cámara 2D avanzada**: Zoom, rotación, follow suave, límites de mapa
- **25 crates** en workspace
- **Suite de tests verde**: 586 tests (575 ✓ / 0 ✗) con registro en `ry-memory/*.yaml`
- **Multi-plataforma**: Android/Termux, Linux, Windows

---

## Puerta Principal (ry-rs)

A partir de la v0.23.0, Ry-Dit utiliza un sistema de **Puerta Principal Unificada**. No necesitas importar múltiples crates para empezar a desarrollar.

### El Prelude Maestro
Añade esto al inicio de tu archivo de Rust para tener acceso a todo el motor:

```rust
use ry_rs::prelude::*;
```

Esto te dará acceso a:
- `RybotEngine`: El orquestador central.
- `InputManager`: Gestión unificada de teclado/mouse/gamepad.
- `RyditGfx`: El motor de renderizado 2D.
- `PhysicsBrush`: Pinceles procedimentales de ryArt.
- `AssetServer`: Carga de recursos con tipos fuertes.

---

## Rybot Engine

Ry-Dit ahora incluye **Rybot**, el motor central que orquesta todos los crates del ecosistema.

### ¿Qué es Rybot?

Rybot conecta 6 subsistemas especializados en un solo motor coherente:

| Subsistema | Crate | Función |
|-----------|-------|---------|
| **Input** | \`events-ry\` + \`ry-input\` | Input unificado + acciones (.rydit-input) |
| **Física** | \`ry-physics\` | Gravedad Newtoniana, proyectiles, N-body |
| **Animación** | \`ry-anim\` | 12 principios Disney, action sprite |
| **Ciencia** | \`ry-science\` | Bezier curves, simulaciones |
| **Arte** | \`ry-art\` | ryArt: Arte generativo y pinceles físicos |
| **Render** | \`ry-gfx\` + \`ry3d-gfx\` | Partículas, GPU instancing, 3D |
| **Red** | \`ry-stream\` | WebSocket LAN server |

### Input Map Configurable (.rydit-input)

Rybot incluye un sistema de input map tipo Godot — mapea acciones a teclas, ratón, gamepad o touch desde un archivo simple:

```ini
# .rydit-input
move_up = W, Up
move_down = S, Down
attack = J, MouseLeft
jump = Space
```

Se puede remapear en runtime:
```rust
engine.input_mut().rebind_action("move_up", vec![K!("I"), K!("K")]);
```

### Física Newtoniana en Game Loop

Rybot ejecuta gravedad Newtoniana `F = G·m₁·m₂/r²` entre cuerpos cada frame:

```rust
engine.physics_mut().set_newtonian(true);
engine.physics_mut().add_body("earth", 0.0, 0.0, 0.0, 0.0, 100.0, 5.0);
engine.physics_mut().update(0.016);
```

### Color por Velocidad en Partículas

Las partículas cambian de color según su velocidad:
- 🔵 Azul oscuro (lento) → 🔵 Azul → 🟡 Amarillo → 🟠 Naranja → 🔴 Rojo → ⚪ Blanco (rápido)

```rust
use ry_gfx::sdl2_helpers::velocity_color_sdl2;
let color = velocity_color_sdl2(speed, max_speed);
```

### Blend Aditivo para Explosiones

Los colores se SUMAN al superponerse — ideal para explosiones y efectos de energía:

```rust
set_blend_additive(&mut canvas);
draw_particles_sdl2(&mut canvas, &ps, max_speed);
set_blend_normal(&mut canvas);
```

### Audio Reactivo por Impacto

Audio procedural generado por tipo de evento (disparo, explosión, powerup):

```rust
let wave_data = generate_wave_data(0.1, 22050, 800.0, "shoot");
// "shoot" = tono descendente con envelope
// "explosion" = noise con envelope
// "powerup" = sweep ascendente de frecuencia
```

### SDL2 Helpers para Demos

Módulo reutilizable `ry_gfx::sdl2_helpers` con helpers para demos SDL2 rápidas:

| Helper | Función |
|--------|---------|
| `velocity_color_sdl2()` | Color por velocidad (ramp azul→rojo) |
| `set_blend_additive()` | Blend aditivo para explosiones |
| `draw_particles_sdl2()` | Dibujar partículas con color |
| `apply_newtonian_gravity_2d()` | Gravedad entre cuerpos 2D |
| `generate_wave_data()` | Audio procedural (shoot/explosion/powerup) |

### Demo War Spacio

Demo tipo Galaga que usa todos los SDL2 helpers:

```bash
cargo run --bin demo_war_spacio --release
```

**Controles**: WASD mover, SPACE disparar, R reiniciar, ESC salir.

---

### ryArt — Motor de Expresión Generativa

Ry-Dit incluye **ryArt**, un sistema para crear arte digital, texturas y efectos visuales de forma procedimental utilizando la física y matemática del motor:

| Herramienta | Descripción |
|-------------|-------------|
| \`PhysicsBrush\` | Pincel con inercia y masa para trazos orgánicos. |
| \`OrganicNoise\` | Generador de texturas basado en ruido de Perlin/Simplex. |
| \`ArtCanvas\` | Superficie de dibujo persistente integrada con el Asset Pipeline. |

**Ejemplo de uso**:
\`\`\`rust
let mut brush = PhysicsBrush::new(ColorRydit::Rojo, 15.0);
brush.stroke(&mut canvas, mouse_x, mouse_y, pressure);
\`\`\`

---

## Requisitos

### Mínimos
- **Android 8+** con Termux + Termux-X11
- **RAM**: 2GB mínimo
- **GPU**: Adreno 610 o equivalente (soporte OpenGL ES 3.2)
- **Almacenamiento**: 500MB libre

### Recomendados
- **Linux** (Ubuntu/Debian) o **Windows** con WSL2
- **Rust 1.70+** instalado
- **SDL2, SDL2_ttf, SDL2_image, SDL2_mixer** development libraries
- **raylib** (opcional, para features 3D)

---

## Instalación

### 1. Clonar el repositorio

```bash
git clone https://github.com/lapumlbb18-blip/Ry-dit.git
cd Ry-dit
```

### 2. Instalar dependencias (Termux)

```bash
pkg update && pkg upgrade
pkg install rust pkg-config sdl2 sdl2_ttf sdl2_image sdl2_mixer
pkg install zink mesa virglrenderer
```

### 3. Instalar dependencias (Linux Ubuntu/Debian)

```bash
sudo apt install build-essential pkg-config libssl-dev
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev
```

### 4. Compilar

```bash
# Debug (rápido, para desarrollo)
cargo build -p ry-rs --bin rydit-rs

# Release (optimizado, para jugar)
cargo build -p ry-rs --bin rydit-rs --release
```

### 5. Ejecutar

```bash
# Motor principal
cargo run -p ry-rs --bin rydit-rs --release

# O directamente el ELF compilado
./target/release/rydit-rs
```

---

## Ejecutar Demos

### Demos principales

```bash
# Juego completo: torreta vs sprites (3 niveles)
cargo run -p ry-rs --bin demo_torreta_vs_sprites --release

# GPU Instancing: 50K partículas
cargo run -p ry-rs --bin demo_gpu_instancing --release

# FSR 1.0: Upscaling 960x540 → 1280x720
cargo run -p ry-rs --bin demo_fsr --release

# Health Bars + Cámara 2D + HUD + Minimap
cargo run -p ry-rs --bin demo_hud_camera --release

# Física + colisiones + audio + TTF
cargo run -p ry-rs --bin demo_rigidbody --release

# Showcase ry-anim (12 principios Disney)
cargo run -p ry-rs --bin demo_anime_ry --release
```

### Con Launchers Zink (Termux-X11)

```bash
# Auto-detección DISPLAY + Zink + GPU Adreno
./launcher_hud_camera.sh
./launcher_gpu_instancing.sh
./launcher_fsr.sh
./launcher_sdl2.sh
./launcher_torreta.sh
```

### Lista completa de demos

| Demo | Descripción |
|------|-------------|
| `demo_hud_camera` | Health bars + Cámara 2D + Debug overlay + Minimap |
| `demo_gpu_instancing` | 50K partículas GPU instancing |
| `demo_fsr` | FSR 1.0 upscaling |
| `demo_torreta_vs_sprites` | Juego completo: 3 niveles, boss fights |
| `demo_rigidbody` | Física + colisiones + audio |
| `demo_anime_ry` | Showcase ry-anim |
| `demo_panel_visual` | 4 paneles + consola interactiva |
| `demo_menu_bar` | Menús Dear ImGui |
| `demo_ttf_sprites` | Texto TTF + sprites PNG |
| `demo_platformer_completo` | Plataformas + gravedad + salto |
| `demo_50k_particulas` | 50K partículas simples |
| `demo_colisiones` | Sistema de colisiones |
| `demo_input_gracia` | Test WASD/flechas con ventana de gracia (Termux-X11) |
| `demo_war_spacio_v2` | Galaga con audio procedural 8-bit + input completo |
| `demo_gamepad` | Panel de gamepad: botones, ejes con dead zone, rumble, hotplug |
| `demo_tui` | TUI: consola, input con historial, shell integrada |

---

## Controles de los Demos

### demo_torreta_vs_sprites

| Tecla | Acción |
|-------|--------|
| ← → ó A/D | Mover torreta |
| W ó ↑ | Saltar |
| S ó ↓ | Bajar rápido |
| SPACE | Disparar |
| P | Pausa |
| R | Reiniciar nivel |
| ESC | Salir / Volver menú |

### demo_hud_camera

| Tecla | Acción |
|-------|--------|
| ← → ↑ ↓ ó WASD | Mover cámara |
| + / - | Zoom in/out (0.2x - 5.0x) |
| Q / E | Rotación (-/+ 15°) |
| R | Reset cámara |
| D | Toggle debug overlay |
| M | Toggle minimap |
| H | Toggle health bars |
| ESC | Salir |

### demo_gpu_instancing

| Tecla | Acción |
|-------|--------|
| 1-6 | 10K/25K/50K/75K/100K/150K partículas |
| ← → ↑ ↓ ó WASD | Mover cámara |
| + / - | Tamaño de partículas |
| P | Pausar animación |
| R | Regenerar partículas |
| ESC | Salir |

### demo_fsr

| Tecla | Acción |
|-------|--------|
| F | Cycle calidad (Quality → Balanced → Performance) |
| E | Toggle FSR ON/OFF |
| A | Toggle auto-detect |
| ESC | Salir |

### Controles generales (panel visual)

| Tecla | Acción |
|-------|--------|
| 1-4 | Cambiar panel (Screen, Console, Input, Controls) |
| ESC | Salir |

---

## Mandos y Joysticks (nuevo v0.27.0)

Soporte completo de **gamepads SDL2 GameController** en la Capa 1 de input (`events-ry`): botones, ejes analógicos, triggers, rumble y conexión/desconexión en caliente.

### Ejecutar el demo

```bash
cargo build --release -p ry-rs
./target/release/demo_gamepad
```

### Controles del demo_gamepad

| Elemento | Acción |
|----------|--------|
| Botones A/B/X/Y/Start/Back/Shoulders | Se iluminan al presionar |
| Stick izquierdo/derecho | Visual radial + valores de ejes X/Y con dead zone |
| Triggers LT/RT | Barras de valor (0.0 → 1.0) |
| R o Start | Rumble (vibración) — requiere mando con soporte |
| Conectar/Desconectar | Hotplug: el mando aparece/desaparece en vivo |
| ESC | Salir |

### API para desarrolladores

```rust
use events_ry::GamepadManager;

// Habilitar gamepads en el backend SDL2:
//   backend.enable_gamepads(sdl.game_controller()?);
let mut gamepads = GamepadManager::new(subsystem, 0.15); // dead zone 15%
gamepads.open_all()?; // detectar mandos conectados

// En el loop:
gamepads.handle_event(&event); // SDL2 → botones, ejes y hotplug
gamepads.poll(); // estados activos y desconexiones
if gamepads.is_button_down_on(id, GamepadButton::A) { /* ... */ }
let x = gamepads.axis_value_on(id, GamepadAxis::LeftX); // con dead zone radial
gamepads.rumble(id, 0xFFFF, 0x8000, 300)?; // low, high, duración ms
// o vibrar todos: gamepads.rumble_all(0xFFFF, 0x8000, 300);
```

Detalles técnicos:
- Dead zone **radial** (no por eje) para sticks — evita deriva diagonal.
- Triggers con dead zone propia (0.1 por defecto).
- `GamepadState` expone `buttons`, `axes` y `connected` por mando.
- Backend agnóstico: fuera de SDL2, el input sigue funcionando con mock (tests sin ventana).

---

## TUI en Línea de Comandos (nuevo v0.27.0)

Widgets de terminal renderizados vía SDL video driver (estilo **Go bubbletea**), sin depender de ncurses: sirve para editores, consolas y menús dentro del motor.

### Ejecutar el demo

```bash
./target/release/demo_tui
```

### Qué incluye

| Componente | Descripción |
|------------|-------------|
| `TuiSystem` | Estado unificado: consola, input, historial, autocompletado y keybindings |
| Panel | Fondo con borde y título (vía migui) |
| Consola | Líneas con severidad (info/warn/error) + scroll |
| Input | Edición de una línea con historial (↑↓) y autocompletado (Tab) |
| Shell integrada | Comandos `:help`, carga de assets, reutiliza `events_ry::Shell` |
| Status bar | Barra de estado inferior (`render_tui_status`) |

### API

```rust
// Módulo: ry-rs/src/tui/
use ry_rs::tui::{TuiSystem, render_tui, render_tui_status};

let mut tui = TuiSystem::new();
tui.execute_command(":help");          // shell integrada
let cmds = render_tui(&tui, 10.0, 10.0, 395.0, h); // → Vec<DrawCommand>
```

---

## Scripting .rydit

Ry-Dit incluye un **lenguaje de scripting en español** que permite crear lógica de juego sin compilar.

### Sintaxis básica

```rydit
# Variables
mi_variable = 10
nombre = "Hola Mundo"
posicion = vec2(100, 200)

# Matemáticas
resultado = sin(PI / 2)
distancia = sqrt(pow(x, 2) + pow(y, 2))

# Condicionales
si vida > 0 entonces
    imprimir("Jugador vivo")
sino
    imprimir("Game Over")
fin

# Bucles
repetir 10 veces
    imprimir("Iteración")
fin

# Funciones
funcion saludar(nombre)
    imprimir("Hola " + nombre)
fin

saludar("Mundo")
```

### Funciones disponibles

#### Matemáticas
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `sin(x)` | Seno | `sin(PI / 2)` → 1.0 |
| `cos(x)` | Coseno | `cos(0)` → 1.0 |
| `tan(x)` | Tangente | `tan(PI / 4)` → 1.0 |
| `sqrt(x)` | Raíz cuadrada | `sqrt(16)` → 4.0 |
| `pow(x, y)` | Potencia | `pow(2, 3)` → 8.0 |
| `log(x)` | Logaritmo natural | `log(E)` → 1.0 |
| `abs(x)` | Valor absoluto | `abs(-5)` → 5.0 |
| `floor(x)` | Redondear abajo | `floor(3.7)` → 3.0 |
| `ceil(x)` | Redondear arriba | `ceil(3.2)` → 4.0 |
| `lerp(a, b, t)` | Interpolación lineal | `lerp(0, 10, 0.5)` → 5.0 |

#### Arrays
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `push(arr, elem)` | Agregar elemento | `push(lista, 5)` |
| `pop(arr)` | Remover último | `pop(lista)` |
| `len(arr)` | Longitud | `len(lista)` |
| `contains(arr, elem)` | Contiene elemento | `contains(lista, 3)` |
| `join(arr, sep)` | Unir con separador | `join(lista, ", ")` |

#### Vec2
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `vec2(x, y)` | Crear vector | `vec2(100, 200)` |
| `add(a, b)` | Sumar vectores | `add(v1, v2)` |
| `normalize(v)` | Normalizar | `normalize(vec2(3, 4))` → (0.6, 0.8) |
| `dist(a, b)` | Distancia | `dist(jugador, enemigo)` |
| `lerp(a, b, t)` | Interpolar | `lerp(pos1, pos2, 0.5)` |

### Ejecutar scripts

```bash
# Desde el motor
./target/release/rydit-rs mi_script.rydit

# Modo REPL interactivo
./target/release/rydit-rs
```

---

## Crear Niveles con .rydit

### Estructura de un nivel

Los niveles se definen con el módulo `ry-config` que parsea archivos de configuración:

```rydit
# nivel1.rydit

# Entidades
entidad "jugador" {
    x = 100
    y = 300
    vida = 100
    sprite = "sprites/jugador.png"
}

entidad "enemigo" {
    x = 500
    y = 300
    vida = 50
    sprite = "sprites/enemigo.png"
    ai = "patrol"
}

# Plataformas
plataforma {
    x = 0
    y = 400
    ancho = 800
    alto = 20
}

plataforma {
    x = 200
    y = 300
    ancho = 100
    alto = 10
}

# Cámara
camara {
    follow = "jugador"
    zoom = 1.0
    limites = { x = 0, y = 0, ancho = 1200, alto = 800 }
}
```

### Checkpoints

```rydit
checkpoint {
    x = 400
    y = 200
    nombre = "Mitad del nivel"
}
```

### HUD Configuration

```rydit
hud {
    health_bars = true
    debug_overlay = false
    minimap = true
    stats = {
        score = true
        tiempo = true
        nivel = "Nivel 1"
    }
}
```

---

## Audio en Termux-X11

### Requisitos
- `pulseaudio` instalado: `pkg install pulseaudio`
- `sdl2-mixer` instalado: `pkg install sdl2-mixer`

### Cómo funciona
Ry-Dit usa SDL2_mixer con FFI crudo. Al iniciar un demo, `ensure_pulseaudio()` verifica que PulseAudio esté corriendo. El audio procedural genera WAV en memoria (tonos, ruido, sweeps) sin necesidad de archivos externos.

### Variables de entorno (NO forzar)
```bash
# NO pongas esto — SDL2 detecta PulseAudio solo:
# export PULSE_SERVER=...
# export SDL_AUDIODRIVER=pulse

# Si audio no funciona, prueba:
unset PULSE_SERVER
unset SDL_AUDIODRIVER
./target/release/demo_war_spacio_v2
```

### Audio procedural disponible
| Sonido | Tipo | Frecuencia |
|--------|------|-----------|
| shoot | Tonos descendentes con envolvente | 120 Hz |
| explosion | Ruido + envolvente exponencial | 150 Hz |
| ambient | Drone bajo modulado en loop | 65 Hz |
| music | Secuencia melódica C-E-G-C' | 220 Hz |

---

## Termux-X11: Ventana de Gracia (150ms)

En Termux-X11, el teclado Android **no envía eventos KeyDown continuos** al mantener presionada una tecla (sin KeyRepeat). Esto hace que WASD/flechas no funcionen para movimiento continuo.

### Solución: Ventana de Gracia

Ry-Dit v0.25.0 implementa una **ventana de gracia de 150ms**: al presionar una tecla, se registra el timestamp. Si la tecla se suelta antes de 150ms, `is_key_pressed()` sigue retornando `true` durante ese período.

```
Presionaste W → KeyDown registrado → timestamp guardado
Soltaste W    → KeyUp → pressed = false
100ms después → is_key_down(W) → 100ms < 150ms → retorna TRUE
160ms después → 160ms > 150ms → retorna FALSE (expiró)
```

### Configurar el valor de gracia

```rust
// En tu código, ajustar según necesidad:
engine.input_mut().input_state_mut().set_grace_ms(200); // 200ms para teclado lento
```

### Probar el input

```bash
# Demo de prueba para WASD + flechas con ventana de gracia
cargo run --bin demo_input_gracia --release
```

---

## Troubleshooting

### Error: SDL2 no encontrado

**Síntoma**: `error: could not find native static library`

**Solución**:
```bash
# Termux
pkg install sdl2 sdl2_ttf sdl2_image sdl2_mixer

# Ubuntu/Debian
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev
```

### Error: DISPLAY no configurado (Termux-X11)

**Síntoma**: `Cannot initialize SDL video`

**Solución**:
```bash
# Usar launchers con auto-detección
./launcher_hud_camera.sh

# O configurar manualmente
export DISPLAY=:0
```

### Rendimiento bajo en GPU Instancing

**Síntoma**: Menos de 30 FPS con 50K partículas

**Solución**:
```bash
# Forzar Zink (GPU) en vez de llvmpipe (CPU)
export MESA_LOADER_DRIVER_OVERRIDE=zink
export GALLIUM_DRIVER=zink

# Verificar GPU activa
glxinfo | grep "OpenGL renderer"
```

### Error: Shaders no encontrados

**Síntoma**: Crash al iniciar demo con shaders

**Causa**: Shaders desde path relativo no se encuentran

**Solución**: Ya fixeado en v0.15.0 - shaders embebidos con `include_str!()`

### Error: Texto no se muestra

**Síntoma**: Pantalla sin texto TTF

**Solución**:
```bash
# Verificar SDL2_ttf instalado
pkg install sdl2_ttf  # Termux
sudo apt install libsdl2-ttf-dev  # Linux

# Verificar fuentes del sistema
ls /usr/share/fonts/
```

### Error: Audio no funciona

**Síntoma**: Sin sonido al jugar

**Solución**:
```bash
# Verificar SDL2_mixer
pkg install sdl2_mixer  # Termux
sudo apt install libsdl2-mixer-dev  # Linux
```

### Crash en demo con muchos sprites

**Síntoma**: Panic con "out of memory"

**Solución**:
- Reducir número de entidades en el nivel
- Usar GPU Instancing para partículas (1 draw call vs N draw calls)
- Activar FSR 1.0 para reducir resolución interna

### Problemas de compilación

```bash
# Limpiar y recompilar
cargo clean
cargo build -p ry-rs --release

# Verificar workspace
cargo check --workspace

# Ejecutar tests (verde = 0 fallos; -j 4 evita crashes de linker en Termux)
cargo test --workspace --no-fail-fast -j 4
```

### Problemas con crates.io

```bash
# Actualizar índice de crates
cargo update

# Verificar crates publicados
cargo search ry-anim
cargo search ry-god
cargo search ry-stream
cargo search v-shield
```

---

## Tests y Memoria Procedimental

Cada versión del motor deja un **registro de memoria** en `ry-memory/`: el estado real de los tests, comprimido en YAML. Este registro es el insumo para agentes de IA (nube o locales) y para el modelo propio que se entrenará en Colab.

### Ejecutar la suite completa

```bash
cargo test --workspace --no-fail-fast -j 4
echo $?   # 0 = verde
```

Estado actual (v0.27.0): **586 tests: 575 ✓ / 0 ✗ / 11 ignored**.

### Estructura de `ry-memory/`

```
ry-memory/
├── raw-test-output.txt    # log crudo de cargo test (se queda local, *.txt ignorado)
├── test-report.yaml       # reporte estructurado: meta, summary, suites, tests, fallos
└── parse_cargo_test.py    # parser reproducible: crudo → YAML
```

### Regenerar el registro tras cada cambio importante

```bash
cargo test --workspace --no-fail-fast -j 4 > ry-memory/raw-test-output.txt 2>&1
python3 ry-memory/parse_cargo_test.py \
    ry-memory/raw-test-output.txt \
    ry-memory/test-report.yaml --exit-code $?
```

### Contenido del YAML (resumen)

```yaml
meta:     # comando, fecha, exit_code
summary:  # suites, tests, passed, failed, ignored, warnings
failures: # lista plana de fallos (vacía en verde)
suites:   # por binario/doc-test: nombre, tipo, lista de tests con estado
```

Para congelar la memoria de una versión, copiar el YAML a `ry-memory/vX.Y.Z/test-report.yaml` junto con un snapshot del `ROADMAP.md` — así el modelo local recibe el estado de la versión sin releer el código completo.

---

## Recursos Adicionales

| Recurso | URL |
|---------|-----|
| **Repositorio** | `https://github.com/lapumlbb18-blip/Ry-dit` |
| **README** | `README.md` |
| **Roadmap** | `ROADMAP.md` |
| **Estructura del proyecto** | `ESTRUCTURA.md` |
| **Tareas pendientes** | `TASKS.md` |
| **Manifiesto** | `MANIFIESTO.md` |
| **Memoria de tests (YAML)** | `ry-memory/test-report.yaml` |
| **crates.io** | `https://crates.io/crates/ry-anim` |

---

<div align="center">

**🛡️ Ry-Dit v0.27.0 - Guía del Usuario**

*Construido sin prisa, madurado con paciencia*

*25 crates · 586 tests (575 ✓ / 0 ✗) · TUI + Joysticks nuevos · 27+ demos · 0 errores · Low-End First*

</div>
