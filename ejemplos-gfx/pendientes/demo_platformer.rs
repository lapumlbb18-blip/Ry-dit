// crates/rydit-rs/src/bin/demo_platformer.rs
// Demo Platformer Simplificado - v0.11.0
// Solo rydit_gfx + SDL2 (sin migui, sin parser)
//
// Ejecutar: cargo run --bin demo_platformer --release

use ry_gfx::camera::Camera2D;

fn main() {
    println!("🛡️ RyDit v0.11.0 - Demo Platformer Simplificado");
    println!("================================================\n");

    // Inicializar SDL2
    let sdl_context = sdl2::init().expect("Failed to init SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to init video");

    let window = video_subsystem
        .window("Demo Platformer", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to create canvas");
    let mut event_pump = sdl_context.event_pump().expect("Failed to init event pump");

    // Inicializar cámara
    let mut camera = Camera2D::new();

    // Jugador (struct simple)
    struct Jugador {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: (u8, u8, u8),
    }

    let mut jugador = Jugador {
        x: 100.0,
        y: 300.0,
        width: 50.0,
        height: 50.0,
        color: (255, 50, 50), // Rojo
    };

    println!("✅ Demo inicializado");
    println!("✅ Ventana: 800x600");
    println!("✅ Controles:");
    println!("   - D / → : Mover derecha");
    println!("   - A / ← : Mover izquierda");
    println!("   - W / ↑ / SPACE : Saltar");
    println!("   - ESC : Salir");
    println!();

    let mut running = true;
    let mut velocidad_x = 0.0;
    let mut velocidad_y = 0.0;
    let gravedad = 500.0;
    let salto = -400.0;
    let mut en_suelo = true;

    // Game loop
    while running {
        // Eventos
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    running = false;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    running = false;
                }

                // Input jugador - Primera pulsación
                sdl2::event::Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => match key {
                    sdl2::keyboard::Keycode::D | sdl2::keyboard::Keycode::Right => {
                        velocidad_x = 200.0;
                    }
                    sdl2::keyboard::Keycode::A | sdl2::keyboard::Keycode::Left => {
                        velocidad_x = -200.0;
                    }
                    sdl2::keyboard::Keycode::W
                    | sdl2::keyboard::Keycode::Up
                    | sdl2::keyboard::Keycode::Space => {
                        if en_suelo {
                            velocidad_y = salto;
                            en_suelo = false;
                        }
                    }
                    _ => {}
                },

                // Input jugador - Tecla mantenida (movimiento continuo)
                sdl2::event::Event::KeyDown {
                    keycode: Some(key),
                    repeat: true,
                    ..
                } => match key {
                    sdl2::keyboard::Keycode::D | sdl2::keyboard::Keycode::Right => {
                        velocidad_x = 200.0;
                    }
                    sdl2::keyboard::Keycode::A | sdl2::keyboard::Keycode::Left => {
                        velocidad_x = -200.0;
                    }
                    _ => {}
                },

                sdl2::event::Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    sdl2::keyboard::Keycode::D
                    | sdl2::keyboard::Keycode::Right
                    | sdl2::keyboard::Keycode::A
                    | sdl2::keyboard::Keycode::Left => {
                        velocidad_x = 0.0;
                    }
                    _ => {}
                },

                _ => {}
            }
        }

        // Físicas simples
        let dt = 0.016; // 60 FPS (~16ms por frame)

        // Aplicar gravedad
        velocidad_y += gravedad * dt;

        // Actualizar posición
        jugador.x += velocidad_x * dt;
        jugador.y += velocidad_y * dt;

        // Colisión con suelo (simple)
        if jugador.y > 500.0 {
            jugador.y = 500.0;
            velocidad_y = 0.0;
            en_suelo = true;
        }

        // Límites de pantalla
        if jugador.x < 0.0 {
            jugador.x = 0.0;
        }
        if jugador.x > 750.0 {
            jugador.x = 750.0;
        }

        // Cámara follow (suave)
        let target_x = jugador.x - 400.0 + (jugador.width / 2.0);
        let target_y = 300.0 - (jugador.height / 2.0);
        camera.set_position(target_x, target_y);

        // Render
        canvas.set_draw_color(sdl2::pixels::Color::RGB(30, 30, 30));
        canvas.clear();

        // Dibujar suelo
        canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
        canvas
            .fill_rect(sdl2::rect::Rect::new(0, 550, 800, 50))
            .ok();

        // Dibujar plataformas (decoración)
        canvas.set_draw_color(sdl2::pixels::Color::RGB(80, 80, 80));
        canvas
            .fill_rect(sdl2::rect::Rect::new(200, 450, 150, 20))
            .ok();
        canvas
            .fill_rect(sdl2::rect::Rect::new(450, 380, 150, 20))
            .ok();
        canvas
            .fill_rect(sdl2::rect::Rect::new(100, 300, 150, 20))
            .ok();

        // Dibujar jugador con cámara
        let (screen_x, screen_y) = camera.apply_sdl2(jugador.x, jugador.y, 800, 600);

        canvas.set_draw_color(sdl2::pixels::Color::RGB(
            jugador.color.0,
            jugador.color.1,
            jugador.color.2,
        ));
        canvas
            .fill_rect(sdl2::rect::Rect::new(
                screen_x - (jugador.width / 2.0) as i32,
                screen_y - (jugador.height / 2.0) as i32,
                jugador.width as u32,
                jugador.height as u32,
            ))
            .ok();

        canvas.present();
    }

    println!("\n✅ Demo finalizado");
    println!("📍 Posición final: ({:.1}, {:.1})", jugador.x, jugador.y);
}
