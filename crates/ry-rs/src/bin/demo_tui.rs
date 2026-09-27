//! demo_tui.rs - Demo del subsistema TUI unificado con TTF real
//!
//! Uso: cargo run --release --bin demo_tui
//!
//! Controles:
//! - F1: Toggle consola
//! - `: Toggle modo input
//! - Flechas ↑↓: Navegar historial
//! - Tab: Autocompletar
//! - Enter: Ejecutar comando
//! - Escape: Limpiar consola

use migui::{DrawCommand, Event, Migui, Rect, Color};
use migui::backend_sdl2::MiguiSdl2Backend;
use ry_rs::tui::{TuiSystem, render_tui};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn main() {
    let mut backend = MiguiSdl2Backend::new("ry-dit TUI Demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("Backend init failed");
    let mut gui = Migui::new();
    let mut tui = TuiSystem::new();
    tui.register_engine_commands();

    // Mensaje de bienvenida
    tui.execute_command("help");

    let mut running = true;
    let mut frame_count = 0u64;
    let mut fps = 0.0f64;
    let mut last_fps = std::time::Instant::now();

    unsafe { sdl2::sys::SDL_StartTextInput(); }

    while running {
        frame_count += 1;
        let elapsed = last_fps.elapsed().as_secs_f64();
        if elapsed >= 1.0 {
            fps = frame_count as f64 / elapsed;
            frame_count = 0;
            last_fps = std::time::Instant::now();
            tui.state_mut().fps = fps;
        }

        // Eventos SDL2 -> Migui
        if backend.process_events(&mut gui) {
            running = false;
        }

        // Consumir eventos de migui -> TUI
        let events = gui.drain_events();
        for event in &events {
            match event {
                Event::KeyDown { key } => {
                    tui.on_migui_key(key);
                }
                Event::CharTyped { ch } => {
                    tui.on_text(&ch.to_string());
                }
                _ => {}
            }
        }

        // Frame de migui
        gui.begin_frame();

        // Panel derecho (demostración widgets)
        gui.panel(
            migui::WidgetId::new("panel"),
            Rect::new(415.0, 30.0, 375.0, (SCREEN_HEIGHT - 45) as f32),
            Color { r: 40, g: 40, b: 55, a: 255 },
        );
        gui.label(
            migui::WidgetId::new("lbl1"),
            "Panel de control",
            Rect::new(425.0, 40.0, 355.0, 24.0),
        );
        gui.label(
            migui::WidgetId::new("lbl2"),
            "TTF rendering via ry-backend",
            Rect::new(425.0, 64.0, 355.0, 24.0),
        );

        // Agregar DrawCommands del TUI
        let tui_cmds = render_tui(&tui, 10.0, 10.0, 395.0, (SCREEN_HEIGHT - 20) as f32);
        gui.extend_draw_commands(tui_cmds);

        // Status bar
        gui.push_draw_command(DrawCommand::DrawRect {
            rect: Rect::new(0.0, 0.0, SCREEN_WIDTH as f32, 22.0),
            color: Color { r: 40, g: 40, b: 55, a: 255 },
        });
        gui.push_draw_command(DrawCommand::DrawText {
            text: format!("ry-dit TUI | FPS: {:.1} | F1=consola Tab=auto Esc=clear", fps),
            x: 6.0,
            y: 4.0,
            size: 12,
            color: Color { r: 80, g: 200, b: 120, a: 255 },
        });

        gui.end_frame();

        // Render con TTF
        backend.render(&mut gui);
    }

    unsafe { sdl2::sys::SDL_StopTextInput(); }
    println!("[DEMO TUI] Shutdown. FPS: {:.1}", fps);
}
