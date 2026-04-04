// Demo Simple desde Cero v0.10.2
// Ejecutar: cargo run --bin demo_simple_desde_cero
// Tamaño: 800x600 (como demo_particles)
// Render nativo Rust (sin parser .rydit)

use ry_gfx::render_queue::{DrawCommand, RenderQueue};
use ry_gfx::{Assets, ColorRydit, Key, RyditGfx};

fn main() {
    println!("🛡️ RyDit v0.10.2 - Demo Simple desde Cero");
    println!("==========================================");
    println!("Tamaño: 800x600");
    println!("Controles: ESC - Salir");
    println!("==========================================");

    // Crear ventana (800x600 como demo_particles)
    let mut gfx = RyditGfx::new("RyDit v0.10.2 - Demo Simple", 800, 600);
    gfx.set_target_fps(60);

    // Crear Assets vacío (no usamos sprites en este demo)
    let assets = Assets::new();

    println!("[RYDIT-GFX]: Ventana creada 800x600");
    println!("[RYDIT-GFX]: Rust = Arquitecto, Raylib = Pincel");
    println!(
        "[RYDIT-GFX]: DISPLAY={}",
        std::env::var("DISPLAY").unwrap_or_else(|_| "no definido".to_string())
    );

    // Crear Render Queue
    let mut queue = RenderQueue::with_capacity(8192);
    println!("[RENDER QUEUE] Command Queue creada: capacidad=8192");

    let mut frame = 0;
    let mut last_time = std::time::Instant::now();

    // Game loop nativo (como demo_particles)
    while !gfx.should_close() {
        // Calcular delta time
        let now = std::time::Instant::now();
        let _dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // Input
        let escape = gfx.is_key_pressed(Key::Escape);

        // FASE 1: Acumular comandos en queue
        queue.push(DrawCommand::Clear {
            color: ColorRydit::Negro,
        });

        // Círculo central (como fuego en demo_particles)
        queue.push(DrawCommand::Circle {
            x: 400,
            y: 300,
            radius: 50,
            color: ColorRydit::Naranja,
        });
        queue.push(DrawCommand::Circle {
            x: 400,
            y: 300,
            radius: 40,
            color: ColorRydit::Amarillo,
        });
        queue.push(DrawCommand::Circle {
            x: 400,
            y: 300,
            radius: 30,
            color: ColorRydit::Rojo,
        });

        // Círculos animados (como partículas)
        let offset = (frame as f32 / 20.0).sin() * 100.0;
        queue.push(DrawCommand::Circle {
            x: (200.0 + offset) as i32,
            y: 300,
            radius: 20,
            color: ColorRydit::Cyan,
        });
        queue.push(DrawCommand::Circle {
            x: (600.0 - offset) as i32,
            y: 300,
            radius: 20,
            color: ColorRydit::Magenta,
        });

        // UI (como demo_particles)
        let fps = gfx.get_fps();
        queue.push(DrawCommand::Text {
            text: "=== RyDit v0.10.2 - Demo Simple ===".to_string(),
            x: 10,
            y: 10,
            size: 20,
            color: ColorRydit::Blanco,
        });
        queue.push(DrawCommand::Text {
            text: format!("FPS: {}", fps),
            x: 10,
            y: 35,
            size: 16,
            color: ColorRydit::Verde,
        });
        queue.push(DrawCommand::Text {
            text: format!("Frame: {}", frame),
            x: 10,
            y: 55,
            size: 16,
            color: ColorRydit::Amarillo,
        });
        queue.push(DrawCommand::Text {
            text: "Tamaño: 800x600".to_string(),
            x: 10,
            y: 75,
            size: 16,
            color: ColorRydit::Gris,
        });
        queue.push(DrawCommand::Text {
            text: "ESC: Salir".to_string(),
            x: 10,
            y: 570,
            size: 16,
            color: ColorRydit::Gris,
        });

        // FASE 2: Ejecutar queue (1 begin_draw por frame)
        queue.execute(&mut gfx, &assets);

        frame += 1;

        if escape {
            break;
        }
    }

    println!("Demo finalizado. ¡Hasta luego!");
}
