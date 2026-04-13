# 🤖 Rybot — Guía del Motor Central

**Versión**: v0.19.2
**Última actualización**: 2026-04-13

---

## 📋 Índice

1. [¿Qué es Rybot?](#qué-es-rybot)
2. [Arquitectura](#arquitectura)
3. [Los 6 Subsistemas](#los-6-subsistemas)
4. [API de cada Subsistema](#api-de-cada-subsistema)
5. [Ejemplo de Uso](#ejemplo-de-uso)
6. [Conectar desde Scripts .rydit](#conectar-desde-scripts-rydit)
7. [Escenas con SceneTree](#escenas-with-scenetree)
8. [CLI de Rybot](#cli-de-rybot)
9. [GUI de Rybot (4 Paneles)](#gui-de-rybot-4-paneles)

---

## ¿Qué es Rybot?

**Rybot** es el **motor central** de Ry-Dit — el orquestador que conecta y coordina todos los crates del ecosistema. Es como el **SceneTree de Godot** + **GDScript**: el sistema que mueve los hilos del motor.

```text
Rybot
├── Escena activa
│   ├── Nodos (entidades, cámaras, luces)
│   ├── Subsistemas (input, físicas, animación, render, red)
│   └── Recursos (texturas, sonidos, scripts)
├── CLI → Crear proyectos, compilar, deploy
└── GUI → Inspector visual, propiedades, escena
```

Rybot no reescribe nada — **conecta crates existentes**:

| Crate | Función |
|-------|---------|
| `ry-input` | Input map configurable |
| `ry-physics` | Proyectiles, N-body, gravedad newtoniana |
| `ry-anim` | 12 principios Disney, action sprite |
| `ry-science` | Bezier, estadísticas, ilusiones |
| `ry-gfx` + `ry3d-gfx` | Render 2D + 3D, partículas, GPU instancing |
| `ry-stream` | WebSocket LAN server, streaming |
| `migui` | GUI inmediata, widgets, temas |
| `ry-config` | Parser de niveles, entidades |

---

## Arquitectura

### RybotEngine

El motor central que orquesta todo:

```rust
use rybot::RybotEngine;

let mut engine = RybotEngine::new();
engine.load_scene("mi_nivel.ryscene");
while engine.is_running() {
    engine.update(0.016); // ~60 FPS
}
```

### Game Loop

Cada frame del motor ejecuta los subsistemas en orden:

```
1. Input    → ry-input: procesar teclas, ratón, gamepad, touch
2. Physics  → ry-physics: gravedad newtoniana, proyectiles, N-body
3. Animation→ ry-anim: Disney principles, action sprite
4. Science  → ry-science: Bezier curves, simulaciones
5. Network  → ry-stream: WebSocket LAN, streaming
6. Scene    → SceneTree: update nodos de escena
7. Render   → ry-gfx + ry3d-gfx: dibujar frame
```

### EngineState

```rust
pub enum EngineState {
    Initializing,   // Inicializando
    Running,        // Corriendo normalmente
    Paused,         // Pausado
    Loading,        // Cargando escena
    ShuttingDown,   // Apagando
}
```

---

## Los 6 Subsistemas

### 1. InputSubsystem → ry-input

Mapeo de acciones a teclas/ratón/touch/gamepad.

**Archivo de configuración**: `.rydit-input`

```ini
# Movimiento
move_up = W, Up
move_down = S, Down
move_left = A, Left
move_right = D, Right

# Acción
jump = Space, W
attack = J, MouseLeft
```

**Macros ergonómicos**:

| Macro | Tipo | Ejemplo |
|-------|------|---------|
| `K!("W")` | Keyboard | `K!("Space")`, `K!("Up")` |
| `M!("MouseLeft")` | Mouse button | `M!("MouseRight")` |
| `P!("A")` | Gamepad button | `P!("X")`, `P!("DPadUp")` |
| `PA!("LeftX")` | Gamepad axis | `PA!("LeftY")`, `PA!("RightX")` |

### 2. PhysicsSubsystem → ry-physics

Física de proyectiles, N-body, gravedad newtoniana.

**Features**:
- Gravedad Newtoniana: `F = G·m₁·m₂/r²` entre cuerpos
- Simulación de proyectiles con trayectorias parabólicas
- N-body simulation con gravedad configurable

### 3. AnimationSubsystem → ry-anim

12 principios de animación Disney + action sprite.

**Disney principles**:
- Arc path, follow through, overlapping action
- Secondary action, squash & stretch
- Anticipation, staging, slow in/out

### 4. ScienceSubsystem → ry-science

Curvas Bezier, estadísticas, geometría.

**Features**:
- Curvas Bezier cúbicas manuales
- Generación de puntos de curva
- Simulaciones científicas (ondas, L-System)

### 5. RenderSubsystem → ry-gfx + ry3d-gfx

Render 2D + 3D con partículas y GPU instancing.

**Features 2D (ry-gfx)**:
- GPU instancing (50K-250K partículas)
- FSR 1.0 upscaling
- Color por velocidad
- Blend aditivo para explosiones
- Audio reactivo por impacto

**Features 3D (ry3d-gfx)**:
- 15 primitivas 3D (cubos, esferas, cilindros)
- Mesh3D con GenMesh raylib
- Skeleton3D con 22 bones humanoides
- Letras 3D con fondo
- Controles táctiles

### 6. NetworkSubsystem → ry-stream

WebSocket LAN server para streaming y multiplayer.

**Features**:
- Servidor WebSocket
- Portal web para espectadores
- Clientes conectados en tiempo real
- Streaming LAN

---

## API de cada Subsistema

### InputSubsystem

```rust
// Crear
let mut input = InputSubsystem::new();

// Cargar mapa desde archivo
input.load_input_map(".rydit-input")?;

// Cada frame
input.update();
input.update_key("W", true);

// Consultar
input.is_action_pressed("move_up")    // bool
input.is_action_just_pressed("jump")   // bool
input.is_action_just_released("pause") // bool

// Remapear en runtime
input.rebind_action("move_up", vec![K!("I"), K!("K")]);

// Contar acciones
input.action_count()  // usize
```

### PhysicsSubsystem

```rust
// Crear
let mut physics = PhysicsSubsystem::new();

// Configurar gravedad Newtoniana
physics.set_newtonian(true);
physics.set_gravity_constant(50.0);

// Agregar cuerpos
physics.add_body("earth", 0.0, 0.0, 0.0, 0.0, 100.0, 5.0);
physics.add_body("moon", 50.0, 0.0, 0.0, 10.0, 10.0, 2.0);

// Cada frame
physics.update(delta_time);

// Simular proyectil
let (xf, yf, flight_time, max_height, range) =
    physics.simulate_projectile(0.0, 0.0, 10.0, 45.0);

// Acceder cuerpos
physics.bodies()       // &[Body]
physics.bodies_mut()   // &mut [Body]
```

### AnimationSubsystem

```rust
// Crear
let mut anim = AnimationSubsystem::new();

// Cada frame
anim.update(delta_time);

// Disney helpers
let (x, y) = anim.arc_path((0.0, 0.0), (100.0, 0.0), 50.0, 0.5);
let squash = anim.follow_through(1.0, 2.0, 0.5);
let (primary, secondary) = anim.secondary_action(0.0, 0.5, 1.0);

// Estado
anim.time()             // f64 — tiempo acumulado
anim.enabled()          // bool
anim.set_enabled(true)  // activar/desactivar
```

### ScienceSubsystem

```rust
// Crear
let mut sci = ScienceSubsystem::new();
sci.set_enabled(true);

// Curva Bezier cúbica
let pt = sci.bezier(
    (0.0, 0.0),  // p0
    (0.5, 1.0),  // p1 (control)
    (0.5, 1.0),  // p2 (control)
    (1.0, 0.0),  // p3
    0.5          // t (0.0 - 1.0)
);

// Generar puntos de curva
let points = sci.bezier_curve(
    (0.0, 0.0), (0.5, 1.0), (0.5, 1.0), (1.0, 0.0),
    100  // pasos
);
```

### RenderSubsystem

```rust
// Crear
let mut render = RenderSubsystem::new();

// Configurar modo 3D
render.set_mode_3d(true);

// Inicializar partículas
render.init_particles();
let ps = render.particles_mut();  // Option<&mut ParticleSystem>

// Cada frame
render.update();

// FPS
render.set_fps(60.0);
render.fps();
render.frame_count();
```

### NetworkSubsystem

```rust
// Crear
let mut net = NetworkSubsystem::new();

// Iniciar servidor
net.start_server("127.0.0.1", 9876);

// Cada frame
net.update(delta_time);

// Consultar clientes
net.client_count();  // usize

// Detener servidor
net.stop_server();
```

---

## Ejemplo de Uso

### Crear RybotEngine y usar subsistemas

```rust
use rybot::RybotEngine;

fn main() {
    // 1. Crear motor
    let mut engine = RybotEngine::new();

    // 2. Configurar input
    engine.input_mut().load_input_map(".rydit-input").unwrap();

    // 3. Activar gravedad Newtoniana
    engine.physics_mut().set_newtonian(true);
    engine.physics_mut().add_body("star", 400.0, 300.0, 0.0, 0.0, 1000.0, 10.0);

    // 4. Activar animación
    engine.animation_mut().set_enabled(true);

    // 5. Game loop
    while engine.is_running() {
        // Input
        engine.input_mut().update();

        // Física
        engine.physics_mut().update(0.016);

        // Animación
        engine.animation_mut().update(0.016);

        // Render
        engine.render().update();

        // Frame completo
        engine.update(0.016);
    }
}
```

### Acceder a cualquier subsistema

```rust
// Input
engine.input()           // &InputSubsystem
engine.input_mut()       // &mut InputSubsystem

// Physics
engine.physics()         // &PhysicsSubsystem
engine.physics_mut()     // &mut PhysicsSubsystem

// Animation
engine.animation()       // &AnimationSubsystem
engine.animation_mut()   // &mut AnimationSubsystem

// Science
engine.science()         // &ScienceSubsystem

// Render
engine.render()          // &RenderSubsystem
engine.render_mut()      // &mut RenderSubsystem

// Network
engine.network()         // &NetworkSubsystem
engine.network_mut()     // &mut NetworkSubsystem
```

### Estadísticas del motor

```rust
let stats = engine.get_stats();
println!("{}", stats);
// 🛡️ Rybot Engine Stats
//   Frame: 1234
//   Estado: Running
//   Nodos en escena: 15
//   Acciones input: 16
//   Físicas: ✅
//   Animación: ✅
//   Red: ❌
//   Target FPS: 60
```

---

## Conectar desde Scripts .rydit

Los scripts `.rydit` pueden interactuar con los subsistemas de Rybot a través del evaluator:

```rydit
# mi_script.rydit — conectar con Rybot

# Input
si accion_pulsada("jump") entonces
    jugador.saltar()
fin

# Física
entidad "bala" {
    tipo = "projectile"
    velocidad = 500
    angulo = 45
    fisica = true
}

# Animación
entidad "personaje" {
    sprite = "sprites/player.png"
    animacion = "run"
    disney_squash = true
}

# Render
camara {
    follow = "jugador"
    zoom = 1.5
    limite = { x = 0, y = 0, ancho = 2400, alto = 1800 }
}

# Network
servidor {
    host = "127.0.0.1"
    port = 9876
    streaming = true
}
```

### Flujo de conexión

```
Script .rydit
    ↓
ry-config parsea entidades
    ↓
SceneTree crea nodos
    ↓
RybotEngine conecta subsistemas
    ↓
Game loop actualiza todo
```

---

## Escenas con SceneTree

### Parsear escena desde archivo .ryscene

```rust
use rybot::{RybotEngine, SceneTree};

// Cargar escena
let mut engine = RybotEngine::new();
engine.load_scene("nivel1.ryscene")?;
```

### Formato .ryscene

```ryscene
# nivel1.ryscene

# Nodos raíz
root
  tipo = "SceneRoot"
  hijos = 3

  entidad "jugador"
    tipo = "Sprite"
    sprite = "sprites/player.png"
    posicion = (100, 300)
    fisica = "dynamic"

  entidad "enemigo"
    tipo = "Sprite"
    sprite = "sprites/enemy.png"
    posicion = (500, 300)
    fisica = "dynamic"

  entidad "camara"
    tipo = "Camera2D"
    follow = "jugador"
    zoom = 1.0
```

### SceneTree API

```rust
use rybot::{SceneTree, SceneNode, NodeType};

// Crear escena programáticamente
let mut tree = SceneTree::new();

// Parsear desde string
let content = r#"
root
  entidad "player" { ... }
"#;
let tree = SceneTree::parse(content)?;

// Consultar nodos
tree.node_count();           // usize — total nodos
tree.root();                 // &SceneNode
tree.find("player");         // Option<&SceneNode>

// Cada frame
tree.update(delta_time);
```

### SceneNode

```rust
pub enum NodeType {
    Root,
    Entity,
    Camera2D,
    Camera3D,
    Light,
    Particle,
    UI,
    Sound,
}
```

---

## CLI de Rybot

Crear proyectos desde terminal:

```bash
# Crear proyecto nuevo (game 2D)
rybot new mi_juego --template game2d

# Crear proyecto 3D
rybot new mi_juego_3d --template game3d

# Crear proyecto de animación
rybot new mi_animacion --template animation

# Listar templates disponibles
rybot templates

# Ver información del motor
rybot info
```

### Templates disponibles

| Template | Descripción |
|----------|-------------|
| `game2d` | Juego 2D básico con input, física, render |
| `game3d` | Juego 3D con cámara orbital, primitivas |
| `animation` | Showcase de animación Disney |
| `science` | Simulaciones científicas (Bezier, ondas) |
| `network` | Demo de networking LAN |

### Estructura de proyecto generado

```
mi_juego/
├── Cargo.toml
├── src/
│   └── main.rs          # Entry point con RybotEngine
├── .rydit-input         # Input map
├── nivel1.ryscene       # Escena inicial
└── assets/              # Sprites, sonidos
```

---

## GUI de Rybot (4 Paneles)

La GUI de Rybot usa **migui** con 4 paneles interactivos:

### Panel 1: New Project

- Crear proyecto nuevo
- Seleccionar template (game2d, game3d, animation, etc.)
- Configurar nombre, resolución, FPS target

### Panel 2: Inspector

- Ver/editar propiedades del nodo seleccionado
- Posición, rotación, escala
- Componentes (sprite, física, animación)

### Panel 3: Scene Tree

- Árbol de escena visual
- Agregar/remover nodos
- Reordenar jerarquía
- Seleccionar nodos

### Panel 4: Engine Stats

- FPS actual
- Frame count
- Nodos activos
- Subsistemas habilitados
- Clientes conectados

### Ejemplo de uso GUI

```rust
use rybot::gui::RybotGui;
use rybot::RybotEngine;

let mut engine = RybotEngine::new();
let mut gui = RybotGui::new(&engine);

// Cada frame (después del render)
gui.render(&engine);

// Consultar estado GUI
if gui.should_create_project() {
    // Abrir diálogo de nuevo proyecto
}
```

---

## SDL2 Helpers para Demos

Rybot provee helpers SDL2 reutilizables para demos rápidas:

```rust
use ry_gfx::sdl2_helpers::*;

// Color por velocidad (azul → amarillo → rojo → blanco)
let color = velocity_color_sdl2(speed, max_speed);
canvas.set_draw_color(color);

// Blend aditivo para explosiones
set_blend_additive(&mut canvas);
draw_particles_sdl2(&mut canvas, &particle_system, max_speed);
set_blend_normal(&mut canvas);

// Gravedad newtoniana 2D
apply_newtonian_gravity_2d(&positions, &mut velocities, &masses, g, dt);

// Audio procedural
let wave_data = generate_wave_data(0.1, 22050, 800.0, "shoot");
```

### Demo War Spacio

Demo completa tipo Galaga usando SDL2 + helpers:

```bash
cargo run --bin demo_war_spacio --release
```

**Controles**:
| Tecla | Acción |
|-------|--------|
| WASD | Mover nave |
| SPACE | Disparar |
| R | Reiniciar |
| ESC | Salir |

**Features demostradas**:
- Gravedad Newtoniana entre enemigos
- Color por velocidad en partículas
- Blend aditivo para explosiones
- Audio procedural por impacto
- SDL2 helpers reutilizables

---

## Resumen

Rybot es el **corazón del ecosistema Ry-Dit** — conecta 8 crates especializados en un solo motor coherente. No reescribe nada, **orquestra lo que ya funciona**.

| Subsistema | Crate | Función principal |
|-----------|-------|------------------|
| Input | `ry-input` | Input map configurable + macros K! M! P! PA! |
| Physics | `ry-physics` | Gravedad Newtoniana + proyectiles + N-body |
| Animation | `ry-anim` | 12 principios Disney + action sprite |
| Science | `ry-science` | Bezier curves + simulaciones |
| Render | `ry-gfx` + `ry3d-gfx` | Partículas + GPU instancing + 3D |
| Network | `ry-stream` | WebSocket LAN server |

**25 crates · ~260 tests · 24+ demos · 0 errores**

<div align="center">

**🛡️ Rybot v0.19.2 — Guía del Motor Central**

*El orquestador que conecta todos los crates de Ry-Dit*

</div>
