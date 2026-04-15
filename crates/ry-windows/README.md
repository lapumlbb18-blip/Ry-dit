# ry-windows

Ventana unificada configurable por plataforma para **Ry-Dit**.

## Filosofía

- **Unificar SDL2 + Raylib** bajo una API común
- **Configurable por plataforma**: Termux/openbox, Windows, Linux, Mac, Android, iOS
- **Menús/submenus/controles preconfigurables** en el buildeo
- El binario no manda herramientas por defecto — se configura qué generar

## Uso

```rust
use ry_windows::{WindowConfig, WindowBuilder, Platform, Backend};

let config = WindowConfig::new("Mi Juego", 1280, 720)
    .platform(Platform::Termux)
    .show_fps(true);

let mut win = WindowBuilder::build(&config)?;

while !win.should_close() {
    win.poll_events();
    win.begin_frame();
    // ... dibujar ...
    win.end_frame();
}
```

## Presets de plataforma

| Plataforma | Backend | Controles | Notas |
|-----------|---------|-----------|-------|
| Termux | SDL2 (X11) | Teclado | openbox, zink |
| Desktop Linux | SDL2 | Teclado + mouse | X11/Wayland |
| Windows | SDL2 | Teclado + mouse + gamepad | |
| macOS | SDL2 | Teclado + mouse + trackpad | |
| Android | Raylib | Touch controls | Joysticks on-screen |
| iOS | Raylib | Touch controls | Joysticks on-screen |

## Features

| Feature | Descripción |
|---------|-------------|
| `sdl2-backend` (default) | SDL2 para input + ventana |
| `raylib-backend` | Raylib para ventana + dibujo |
| `full` | ry-gfx + ry-input + sdl2-backend |

## Presets de configuración

```rust
let mut c = WindowConfig::default();
c.preset_game_2d();     // vsync, 60fps, show_fps
c.preset_editor();      // resizable, sin target_fps
c.preset_demo();        // 30fps, simple
```

## WindowEvent

Eventos unificados: `KeyDown`, `KeyUp`, `TextInput`, `MouseMoved`, `MouseDown`, `MouseUp`,
`MouseWheel`, `TouchDown`, `TouchMove`, `TouchUp`, `GamepadDown`, `GamepadAxis`,
`Resized`, `CloseRequested`.

## InputState

Estado agregado de teclas y mouse:
- `is_key_down("A")`
- `mouse_pos()`
- `is_mouse_down(0)`
