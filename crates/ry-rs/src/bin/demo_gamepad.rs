//! demo_gamepad.rs - Demo del sistema de gamepads (v0.27.0)
//!
//! Uso: cargo run --release --bin demo_gamepad
//!
//! Controles:
//! - Esc: Salir
//! - R: Test de rumble (vibración 300ms)
//! - Botón Start del gamepad: también vibra
//! - Botones/ejes: se muestran en tiempo real con dead zone aplicada

use events_ry::{GamepadAxis, GamepadButton, GamepadManager};
use migui::backend_sdl2::MiguiSdl2Backend;
use migui::{Color, DrawCommand, Event, Key, Migui, Rect};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const DARK: Color = Color { r: 40, g: 40, b: 55, a: 255 };
const PANEL: Color = Color { r: 30, g: 30, b: 42, a: 255 };
const BAR_BG: Color = Color { r: 55, g: 55, b: 75, a: 255 };
const BTN_OFF: Color = Color { r: 55, g: 55, b: 75, a: 255 };
const BTN_ON: Color = Color { r: 80, g: 200, b: 120, a: 255 };
const GREEN: Color = Color { r: 80, g: 200, b: 120, a: 255 };
const ACCENT: Color = Color { r: 120, g: 160, b: 255, a: 255 };
const TEXT_DIM: Color = Color { r: 150, g: 150, b: 170, a: 255 };
const TEXT: Color = Color { r: 220, g: 220, b: 235, a: 255 };
const RUMBLE: Color = Color { r: 255, g: 160, b: 80, a: 255 };

fn rect(gui: &mut Migui, x: f32, y: f32, w: f32, h: f32, c: Color) {
    gui.push_draw_command(DrawCommand::DrawRect {
        rect: Rect::new(x, y, w, h),
        color: c,
    });
}

fn text(gui: &mut Migui, s: impl Into<String>, x: f32, y: f32, size: u32, c: Color) {
    gui.push_draw_command(DrawCommand::DrawText {
        text: s.into(),
        x,
        y,
        size,
        color: c,
    });
}

fn trigger_rumble(gamepads: &mut GamepadManager) {
    let n = gamepads.rumble_all(0xFFFF, 0x8000, 300);
    if n == 0 {
        println!("[DEMO GAMEPAD] Rumble: ningún controller conectado");
    }
}

fn draw_axis_bar(
    gui: &mut Migui,
    label: &str,
    value: f32,
    centered: bool,
    x: f32,
    y: f32,
    w: f32,
) {
    text(gui, label, x, y - 16.0, 12, TEXT_DIM);
    rect(gui, x, y, w, 16.0, BAR_BG);
    const H: f32 = 16.0;
    if centered {
        let cx = x + w / 2.0;
        let fill = value * (w / 2.0);
        rect(gui, cx - 1.0, y, 2.0, H, TEXT_DIM);
        if value >= 0.0 {
            rect(gui, cx, y, fill.max(0.0), H, ACCENT);
        } else {
            rect(gui, cx + fill, y, (-fill).max(0.0), H, ACCENT);
        }
    } else {
        rect(gui, x, y, (value.clamp(0.0, 1.0)) * w, H, ACCENT);
    }
    text(gui, format!("{:+.2}", value), x + w + 6.0, y, 12, TEXT);
}

fn draw_ui(gui: &mut Migui, gamepads: &GamepadManager, fps: f64, rumble_on: bool) {
    // --- Barra de título ---
    rect(gui, 0.0, 0.0, SCREEN_WIDTH as f32, 26.0, DARK);
    text(
        gui,
        format!("ry-dit Gamepad Demo | FPS: {:.1} | Esc=salir R=rumble", fps),
        8.0,
        5.0,
        13,
        GREEN,
    );
    if rumble_on {
        text(gui, "~~ RUMBLE ~~", 640.0, 5.0, 13, RUMBLE);
    }

    // --- Estado de conexión ---
    rect(gui, 10.0, 36.0, 780.0, 56.0, PANEL);
    text(
        gui,
        format!("Controllers conectados: {}", gamepads.count()),
        20.0,
        44.0,
        14,
        TEXT,
    );
    let names: Vec<String> = gamepads
        .connected()
        .iter()
        .map(|(id, name)| format!("[{}] {}", id, name))
        .collect();
    text(gui, names.join("  "), 20.0, 66.0, 12, TEXT_DIM);

    if gamepads.count() == 0 {
        text(
            gui,
            "Conecta un gamepad (USB / Bluetooth)...",
            20.0,
            66.0,
            12,
            RUMBLE,
        );
    }

    // --- Panel de botones ---
    rect(gui, 10.0, 102.0, 385.0, 300.0, PANEL);
    text(gui, "Botones", 20.0, 110.0, 14, TEXT);

    let buttons: [(GamepadButton, &str); 14] = [
        (GamepadButton::FaceDown, "A"),
        (GamepadButton::FaceRight, "B"),
        (GamepadButton::FaceLeft, "X"),
        (GamepadButton::FaceUp, "Y"),
        (GamepadButton::LeftShoulder, "LB"),
        (GamepadButton::RightShoulder, "RB"),
        (GamepadButton::Back, "Back"),
        (GamepadButton::Start, "Start"),
        (GamepadButton::LeftStick, "LS"),
        (GamepadButton::RightStick, "RS"),
        (GamepadButton::DPadUp, "D-Up"),
        (GamepadButton::DPadDown, "D-Down"),
        (GamepadButton::DPadLeft, "D-Left"),
        (GamepadButton::DPadRight, "D-Right"),
    ];
    let (bx, by, bw, bh, gap) = (20.0, 138.0, 86.0, 40.0, 8.0);
    for (i, (btn, label)) in buttons.iter().enumerate() {
        let col = (i % 4) as f32;
        let row = (i / 4) as f32;
        let x = bx + col * (bw + gap);
        let y = by + row * (bh + gap);
        let down = gamepads.is_button_down(*btn);
        rect(gui, x, y, bw, bh, if down { BTN_ON } else { BTN_OFF });
        text(
            gui,
            if down {
                format!("[{}]", label)
            } else {
                label.to_string()
            },
            x + 8.0,
            y + 11.0,
            13,
            if down { DARK } else { TEXT },
        );
    }

    // --- Panel de ejes ---
    rect(gui, 405.0, 102.0, 385.0, 300.0, PANEL);
    text(gui, "Ejes (dead zone = 0.15, radial)", 415.0, 110.0, 14, TEXT);

    let bars: [(GamepadAxis, &str, bool); 6] = [
        (GamepadAxis::LeftX, "Left Stick X", true),
        (GamepadAxis::LeftY, "Left Stick Y", true),
        (GamepadAxis::RightX, "Right Stick X", true),
        (GamepadAxis::RightY, "Right Stick Y", true),
        (GamepadAxis::LeftTrigger, "LT (trigger)", false),
        (GamepadAxis::RightTrigger, "RT (trigger)", false),
    ];
    for (i, (axis, label, centered)) in bars.iter().enumerate() {
        let y = 150.0 + i as f32 * 38.0;
        let v = gamepads.axis_value(*axis);
        draw_axis_bar(gui, label, v, *centered, 425.0, y, 240.0);
    }

    // --- Visualización del stick izquierdo ---
    rect(gui, 10.0, 412.0, 180.0, 170.0, PANEL);
    text(gui, "Stick izq. (visual)", 20.0, 420.0, 13, TEXT);
    let (sx, sy, ss) = (40.0, 446.0, 110.0);
    rect(gui, sx, sy, ss, ss, BAR_BG);
    rect(gui, sx + ss / 2.0 - 1.0, sy, 2.0, ss, TEXT_DIM);
    rect(gui, sx, sy + ss / 2.0 - 1.0, ss, 2.0, TEXT_DIM);
    let lx = gamepads.axis_value(GamepadAxis::LeftX);
    let ly = gamepads.axis_value(GamepadAxis::LeftY);
    let dot = sx + ss / 2.0 + lx * (ss / 2.0 - 6.0);
    let dot_y = sy + ss / 2.0 + ly * (ss / 2.0 - 6.0);
    rect(gui, dot - 5.0, dot_y - 5.0, 10.0, 10.0, GREEN);

    // --- Info / hints ---
    rect(gui, 200.0, 412.0, 590.0, 170.0, PANEL);
    text(gui, "Info", 210.0, 420.0, 13, TEXT);
    text(
        gui,
        "Hotplug: conectar/desconectar gamepad en caliente",
        210.0,
        444.0,
        12,
        TEXT_DIM,
    );
    text(
        gui,
        "R: rumble 300ms (low=0xFFFF, high=0x8000)",
        210.0,
        464.0,
        12,
        TEXT_DIM,
    );
    text(
        gui,
        "Start del gamepad: rumble tambien",
        210.0,
        484.0,
        12,
        TEXT_DIM,
    );
    text(
        gui,
        "events_ry::GamepadManager → open_all/handle_event/poll/rumble",
        210.0,
        510.0,
        12,
        ACCENT,
    );
    text(
        gui,
        "Dead zone radial: aplica a sticks; triggers usan umbral",
        210.0,
        530.0,
        12,
        TEXT_DIM,
    );
    let st = gamepads
        .state(0)
        .map(|s| s.pressed_buttons().len())
        .unwrap_or(0);
    text(
        gui,
        format!("state(0): {} botones presionados (crudos)", st),
        210.0,
        554.0,
        12,
        TEXT_DIM,
    );
}

fn main() {
    let mut backend = MiguiSdl2Backend::new("ry-dit Gamepad Demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("Backend init failed");
    let mut gui = Migui::new();

    // GamepadManager (Capa 1 de events-ry)
    let subsystem = backend
        .core()
        .game_controller()
        .expect("SDL2 game_controller subsystem");
    let mut gamepads = GamepadManager::new(subsystem, 0.15);
    let opened = gamepads.open_all().unwrap_or(0);
    println!("[DEMO GAMEPAD] Controllers abiertos al inicio: {opened}");

    let mut running = true;
    let mut frame_count = 0u64;
    let mut fps = 0.0f64;
    let mut last_fps = std::time::Instant::now();
    let mut rumble_until: Option<std::time::Instant> = None;
    let mut start_was_down = false;

    while running {
        frame_count += 1;
        let elapsed = last_fps.elapsed().as_secs_f64();
        if elapsed >= 1.0 {
            fps = frame_count as f64 / elapsed;
            frame_count = 0;
            last_fps = std::time::Instant::now();
        }

        // Eventos SDL2 → MiGUI (ventana/teclado/mouse)
        if backend.process_events(&mut gui) {
            running = false;
        }

        // Eventos crudos SDL2 → GamepadManager (hotplug + botones + ejes)
        let raw = backend.take_raw_events();
        for ev in &raw {
            gamepads.handle_event(ev);
        }

        // Teclado
        for ev in gui.drain_events() {
            if let Event::KeyDown { key } = ev {
                match key {
                    Key::Escape => running = false,
                    Key::R => {
                        trigger_rumble(&mut gamepads);
                        rumble_until =
                            Some(std::time::Instant::now() + std::time::Duration::from_millis(300));
                    }
                    _ => {}
                }
            }
        }

        // Refrescar estado desde hardware + purgar desconectados
        gamepads.poll();

        // Start del gamepad → rumble
        let start_down = gamepads.is_button_down(GamepadButton::Start);
        if start_down && !start_was_down {
            trigger_rumble(&mut gamepads);
            rumble_until = Some(std::time::Instant::now() + std::time::Duration::from_millis(300));
        }
        start_was_down = start_down;

        let rumble_on = rumble_until.map_or(false, |t| std::time::Instant::now() < t);

        gui.begin_frame();
        draw_ui(&mut gui, &gamepads, fps, rumble_on);
        gui.end_frame();

        backend.render(&mut gui);
    }

    println!("[DEMO GAMEPAD] Shutdown. FPS: {:.1}", fps);
}
