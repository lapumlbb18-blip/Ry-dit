// crates/rydit-rs/src/bin/demo_movimiento.rs
// Demo Movimiento Lateral - v0.11.0
// Copia EXACTA de test_callback_sdl2.rs (que sí funcionaba)
//
// Ejecutar: cargo run --bin demo_movimiento --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {
    println!("🛡️ RyDit v0.11.0 - Demo Movimiento Lateral");
    println!("============================================\n");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Demo Movimiento", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Jugador
    let mut jugador_x: i32 = 400;
    let mut jugador_y: i32 = 300;
    let velocidad: i32 = 5;

    let mut running = true;

    println!("✅ Ventana: 800x600");
    println!("✅ Controles:");
    println!("   - A / ← : Izquierda");
    println!("   - D / → : Derecha");
    println!("   - W / ↑ : Arriba");
    println!("   - S / ↓ : Abajo");
    println!("   - ESC : Salir");
    println!();

    'running: while running {
        // ✅ EVENT POLLING con SDL2 (funciona en Android)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                    break 'running;
                }

                // ✅ TECLAS PRESIONADAS (evento inicial)
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::W | Keycode::Up => jugador_y -= velocidad,
                    Keycode::S | Keycode::Down => jugador_y += velocidad,
                    Keycode::A | Keycode::Left => jugador_x -= velocidad,
                    Keycode::D | Keycode::Right => jugador_x += velocidad,
                    _ => {}
                },

                // ✅ TECLAS MANTENIDAS (repeat = true) ← ESTO ES LA CLAVE
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: true,
                    ..
                } => match keycode {
                    Keycode::W | Keycode::Up => jugador_y -= velocidad,
                    Keycode::S | Keycode::Down => jugador_y += velocidad,
                    Keycode::A | Keycode::Left => jugador_x -= velocidad,
                    Keycode::D | Keycode::Right => jugador_x += velocidad,
                    _ => {}
                },

                _ => {}
            }
        }

        // Renderizado
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Cuadrado rojo (jugador)
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let jugador = Rect::new(jugador_x - 25, jugador_y - 25, 50, 50);
        canvas.fill_rect(jugador).unwrap();

        // Punto blanco (centro)
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let punto = Rect::new(jugador_x - 3, jugador_y - 3, 6, 6);
        canvas.fill_rect(punto).unwrap();

        // Referencia (círculo azul en centro)
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        let centro = Rect::new(395, 295, 10, 10);
        canvas.fill_rect(centro).unwrap();

        canvas.present();
    }

    println!("\n✅ Demo finalizado");
    println!("📍 Posición final: ({}, {})", jugador_x, jugador_y);
}
