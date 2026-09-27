//! demo_input_gracia.rs
//! Test de input con ventana de gracia para Termux-X11
//!
//! Resuelve: Teclado Android no envía KeyRepeat al mantener presionada una tecla.
//! Solución: Si una tecla fue presionada hace <150ms, sigue "activa" para movimiento.
//!
//! Controles:
//!   WASD / Flechas = Mover cuadrado
//!   ESPACIO        = Explosión visual (teste instantáneo)
//!   C              = Cambiar color
//!   ESC            = Salir

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

use events_ry::{InputEvent, InputManager, Key, MouseButton};
use ry_gfx::backend_sdl2::Sdl2Backend;

const W: i32 = 800;
const H: i32 = 600;
const SPEED: f32 = 200.0;

fn main() -> Result<(), String> {
    println!("=== Demo Ventana de Gracia (Termux-X11) ===");
    println!("WASD / Flechas = Mover | ESPACIO = Explosión | C = Color | ESC = Salir");
    println!("Gracia: 150ms — mantener tecla presionada se simula\n");

    let mut backend = Sdl2Backend::new("RyDit - Test Gracia Input", W, H)?;
    let mut input = InputManager::new();

    // Configurar ventana de gracia de 150ms para Termux-X11
    input.input_state_mut().set_grace_ms(150);

    let mut px = W as f32 / 2.0;
    let mut py = H as f32 / 2.0;
    let mut color_mode: u8 = 0;
    let mut flash_timer: f32 = 0.0;
    let mut last_time = Instant::now();
    let mut frame = 0u64;
    let mut running = true;

    while running {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32().min(0.05);
        last_time = now;
        frame += 1;

        // === INPUT ===
        backend.actualizar_input_unificado(&mut input);

        for event in input.poll_events() {
            match event {
                InputEvent::WindowCloseRequested => running = false,
                InputEvent::KeyPressed { key: Key::Escape } => running = false,
                InputEvent::KeyPressed { key: Key::Space } => {
                    flash_timer = 0.3; // Flash visual
                    println!("[Frame {}] ESPACIO detectado (just_pressed)", frame);
                }
                InputEvent::KeyPressed { key: Key::C } => {
                    color_mode = (color_mode + 1) % 3;
                    println!("[Frame {}] Color → modo {}", frame, color_mode);
                }
                _ => {}
            }
        }

        // === MOVIMIENTO CON GRACIA ===
        // is_key_down() ahora usa la ventana de gracia de 150ms
        // Si presionaste W y la sueltaste hace <150ms, sigue activo
        let move_dt = SPEED * dt;
        if input.is_key_down(Key::W) || input.is_key_down(Key::Up) {
            py -= move_dt;
        }
        if input.is_key_down(Key::S) || input.is_key_down(Key::Down) {
            py += move_dt;
        }
        if input.is_key_down(Key::A) || input.is_key_down(Key::Left) {
            px -= move_dt;
        }
        if input.is_key_down(Key::D) || input.is_key_down(Key::Right) {
            px += move_dt;
        }

        // Clamp
        px = px.clamp(20.0, (W - 20) as f32);
        py = py.clamp(20.0, (H - 20) as f32);

        // === RENDER ===
        let bg = if flash_timer > 0.0 {
            flash_timer -= dt;
            Color::RGB(40, 40, 80)
        } else {
            Color::RGB(15, 15, 25)
        };
        backend.set_draw_color(bg);
        backend.clear();

        // Grid de fondo
        backend.set_draw_color(Color::RGBA(255, 255, 255, 8));
        for x in (0..W).step_by(50) {
            backend.draw_line((x, 0), (x, H));
        }
        for y in (0..H).step_by(50) {
            backend.draw_line((0, y), (W, y));
        }

        // Cruz de referencia
        backend.set_draw_color(Color::RGB(40, 40, 60));
        backend.draw_line((W / 2, 0), (W / 2, H));
        backend.draw_line((0, H / 2), (W, H / 2));

        // Cuadrado del jugador
        let (cr, cg, cb) = match color_mode {
            0 => (0, 200, 255),
            1 => (255, 100, 50),
            _ => (50, 255, 100),
        };

        // Aura
        backend.set_draw_color(Color::RGBA(cr, cg, cb, 40));
        let _ = backend.fill_rect(Rect::new(px as i32 - 22, py as i32 - 22, 44, 44));

        // Cuerpo
        backend.set_draw_color(Color::RGB(cr, cg, cb));
        let _ = backend.fill_rect(Rect::new(px as i32 - 14, py as i32 - 14, 28, 28));

        // Núcleo
        backend.set_draw_color(Color::WHITE);
        let _ = backend.fill_rect(Rect::new(px as i32 - 4, py as i32 - 4, 8, 8));

        // HUD
        backend.set_draw_color(Color::RGBA(0, 0, 0, 180));
        let _ = backend.fill_rect(Rect::new(10, 10, 400, 70));

        backend.set_draw_color(Color::RGB(200, 200, 200));
        let _ = backend.fill_rect(Rect::new(15, 18, 8, 8)); // W indicator
        let _ = backend.fill_rect(Rect::new(15, 32, 8, 8)); // S indicator
        let _ = backend.fill_rect(Rect::new(5, 25, 8, 8));  // A indicator
        let _ = backend.fill_rect(Rect::new(25, 25, 8, 8)); // D indicator

        // Indicadores de dirección (verde = activo por gracia)
        let w_active = input.is_key_down(Key::W);
        let s_active = input.is_key_down(Key::S);
        let a_active = input.is_key_down(Key::A);
        let d_active = input.is_key_down(Key::D);

        if w_active {
            backend.set_draw_color(Color::RGB(0, 255, 100));
            let _ = backend.fill_rect(Rect::new(15, 18, 8, 8));
        }
        if s_active {
            backend.set_draw_color(Color::RGB(0, 255, 100));
            let _ = backend.fill_rect(Rect::new(15, 32, 8, 8));
        }
        if a_active {
            backend.set_draw_color(Color::RGB(0, 255, 100));
            let _ = backend.fill_rect(Rect::new(5, 25, 8, 8));
        }
        if d_active {
            backend.set_draw_color(Color::RGB(0, 255, 100));
            let _ = backend.fill_rect(Rect::new(25, 25, 8, 8));
        }

        // Flash de explosión
        if flash_timer > 0.0 {
            let alpha = (flash_timer / 0.3 * 200.0) as u8;
            backend.set_draw_color(Color::RGBA(255, 255, 100, alpha));
            let _ = backend.fill_rect(Rect::new(
                px as i32 - 50,
                py as i32 - 50,
                100,
                100,
            ));
        }

        backend.present();

        // Log de estado cada 2 segundos
        if frame % 120 == 0 {
            println!(
                "[Frame {:05}] Pos: ({:.0},{:.0}) | WASD: {}{}{}{} | Gracia: 150ms",
                frame, px, py,
                if w_active { "W" } else { "-" },
                if s_active { "S" } else { "-" },
                if a_active { "A" } else { "-" },
                if d_active { "D" } else { "-" },
            );
        }
    }

    println!("\nDemo finalizado. Frames: {}", frame);
    Ok(())
}
