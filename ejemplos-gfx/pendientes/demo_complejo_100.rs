// Demo Complejo v0.10.2 - 100+ Partículas
// Ejecutar: cargo run --bin demo_complejo_100
// Tamaño: 1280x720 (HD)
// Partículas: 100+ en movimiento

use ry_gfx::render_queue::{DrawCommand, RenderQueue};
use ry_gfx::{Assets, ColorRydit, Key, RyditGfx};

fn main() {
    println!("🛡️ RyDit v0.10.2 - Demo Complejo 100 Partículas");
    println!("================================================");
    println!("Tamaño: 1280x720 (HD)");
    println!("Partículas: 100+ (gris y azul)");
    println!("Controles: ESC - Salir");
    println!("================================================");

    // Crear ventana HD (1280x720)
    let mut gfx = RyditGfx::new("RyDit v0.10.2 - Demo Complejo 100", 1280, 720);
    gfx.set_target_fps(60);

    // Crear Assets vacío
    let assets = Assets::new();

    println!("[RYDIT-GFX]: Ventana creada 1280x720");
    println!("[RYDIT-GFX]: Rust = Arquitecto, Raylib = Pincel");
    println!(
        "[RYDIT-GFX]: DISPLAY={}",
        std::env::var("DISPLAY").unwrap_or_else(|_| "no definido".to_string())
    );

    // Crear Render Queue (más grande para 100+ partículas)
    let mut queue = RenderQueue::with_capacity(16384);
    println!("[RENDER QUEUE] Command Queue creada: capacidad=16384");

    // Crear 100 partículas con posiciones y velocidades
    let mut particulas: Vec<(f32, f32, f32, f32, ColorRydit)> = Vec::new();
    for i in 0..100 {
        let x = (i % 10) as f32 * 128.0 + 64.0;
        let y = (i / 10) as f32 * 72.0 + 36.0;
        let vx = (i as f32 * 0.1).sin() * 2.0;
        let vy = (i as f32 * 0.1).cos() * 2.0;
        let color = if i % 2 == 0 {
            ColorRydit::Gris
        } else {
            ColorRydit::Azul
        };
        particulas.push((x, y, vx, vy, color));
    }

    println!("✅ 100 partículas creadas (50 gris, 50 azul)");

    let mut frame = 0;
    let mut last_time = std::time::Instant::now();

    // Game loop nativo
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

        // Dibujar las 100 partículas
        for i in 0..100 {
            let (x, y, vx, vy, color) = particulas[i];

            // Actualizar posición (movimiento)
            let nueva_x = x + vx * (frame as f32 * 0.05).sin();
            let nueva_y = y + vy * (frame as f32 * 0.05).cos();

            // Dibujar partícula
            queue.push(DrawCommand::Circle {
                x: nueva_x as i32,
                y: nueva_y as i32,
                radius: 8 + (i % 5) as i32,
                color: color,
            });
        }

        // Dibujar líneas conectando partículas cercanas (efecto red)
        for i in 0..50 {
            let (x1, y1, _, _, _) = particulas[i];
            let (x2, y2, _, _, _) = particulas[i + 50];

            let x1_anim = x1 + particulas[i].2 * (frame as f32 * 0.05).sin();
            let y1_anim = y1 + particulas[i].3 * (frame as f32 * 0.05).cos();
            let x2_anim = x2 + particulas[i + 50].2 * (frame as f32 * 0.05).sin();
            let y2_anim = y2 + particulas[i + 50].3 * (frame as f32 * 0.05).cos();

            queue.push(DrawCommand::Line {
                x1: x1_anim as i32,
                y1: y1_anim as i32,
                x2: x2_anim as i32,
                y2: y2_anim as i32,
                color: ColorRydit::Gris,
            });
        }

        // Dibujar círculos concéntricos en el centro (efecto "sol")
        for i in 0..10 {
            let radio = 30.0 + (i as f32 * 15.0) + (frame as f32 * 0.1).sin() * 10.0;
            queue.push(DrawCommand::Circle {
                x: 640,
                y: 360,
                radius: radio as i32,
                color: if i % 2 == 0 {
                    ColorRydit::Azul
                } else {
                    ColorRydit::Gris
                },
            });
        }

        // UI (información en pantalla)
        let fps = gfx.get_fps();
        queue.push(DrawCommand::Text {
            text: "=== RyDit v0.10.2 - Demo Complejo ===".to_string(),
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
            text: "Tamaño: 1280x720 (HD)".to_string(),
            x: 10,
            y: 75,
            size: 16,
            color: ColorRydit::Cyan,
        });
        queue.push(DrawCommand::Text {
            text: format!("Partículas: {}", particulas.len()),
            x: 10,
            y: 95,
            size: 16,
            color: ColorRydit::Magenta,
        });
        queue.push(DrawCommand::Text {
            text: "Líneas: 50 | Círculos centro: 10".to_string(),
            x: 10,
            y: 115,
            size: 16,
            color: ColorRydit::Gris,
        });
        queue.push(DrawCommand::Text {
            text: "ESC: Salir".to_string(),
            x: 10,
            y: 690,
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
