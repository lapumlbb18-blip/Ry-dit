# 🎮 ry-input

**Input map configurable** para Ry-Dit — mapeo de acciones a teclas, ratón y touch.

## Filosofía

Inspirado en **Godot Input Map**: en vez de hardcodear teclas en el código, defines acciones y las mapeas a múltiples inputs.

```rust
// ❌ Antes: teclas hardcodeadas
if input.is_key_pressed(Key::W) { jugador.mover_arriba(); }

// ✅ Ahora: acciones configurables
if input.is_action_pressed("move_up") { jugador.mover_arriba(); }
```

## Formato `.rydit-input`

```ini
# default.rydit-input
# Acciones → teclas/ratón/touch

move_up = W, Up
move_down = S, Down
move_left = A, Left
move_right = D, Right
jump = Space, W
shoot = MouseLeft, Z
pause = Escape, P
interact = E, Enter
sprint = LeftShift
crouch = LeftCtrl
```

## Uso

```rust
use ry_input::{InputMap, InputSource};

// Cargar desde archivo
let mut input_map = InputMap::load("default.rydit-input")?;

// Consultar por acción
if input_map.is_action_just_pressed("jump") {
    jugador.saltar();
}

// Agregar mapping en runtime
input_map.add_action("attack", vec![
    InputSource::Key("X"),
    InputSource::MouseButton("MouseRight"),
]);

// Guardar configuración del usuario
input_map.save("user.rydit-input")?;
```

## Arquitectura

| Capa | Descripción |
|------|-------------|
| **InputSource** | Key, MouseButton, TouchZone, GamepadButton |
| **InputMap** | Parser + mapeo acción → sources |
| **InputState** | Estado en tiempo real (pressed, just_pressed, just_released) |

## Tests

```bash
cargo test -p ry-input
```
