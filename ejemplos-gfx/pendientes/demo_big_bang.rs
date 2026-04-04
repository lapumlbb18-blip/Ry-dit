// Demo Big Bang - Explosión Cósmica
// Ejecutar: cargo run --bin demo_big_bang
// Tamaño: 1280x720 (HD)
// Efectos: Explosión, partículas, flash

use ry_gfx::{ColorRydit, Key, RyditGfx};

fn main() {
    println!("🛡️ RyDit v0.10.2 - Demo Big Bang");
    println!("==================================");
    println!("🌟 Explosión Cósmica");
    println!("🪟 Ventana: 1280x720");
    println!("🎮 Controles:");
    println!("   ESPACIO = Nueva explosión");
    println!("   ESC = Salir");
    println!("==================================\n");

    // Crear ventana HD
    let mut gfx = RyditGfx::new("Big Bang - Explosión Cósmica", 1280, 720);
    gfx.set_target_fps(60);

    // Estado de la explosión
    let mut explosion_activa = false;
    let mut explosion_frame = 0;
    let mut explosion_x = 640.0;
    let mut explosion_y = 360.0;

    // Partículas de la explosión (200 partículas)
    let mut particulas: Vec<(f32, f32, f32, f32, ColorRydit)> = Vec::new();

    // Estrellas de fondo (500 puntos)
    let mut estrellas: Vec<(f32, f32, f32)> = Vec::new();
    for i in 0..500 {
        let x = (i * 7 % 1280) as f32;
        let y = (i * 13 % 720) as f32;
        let brillo = ((i % 100) as f32) / 100.0;
        estrellas.push((x, y, brillo));
    }

    let mut frame = 0;
    let mut flash_intensity = 0.0;

    // Crear explosión inicial
    crear_explosion(&mut particulas, 640.0, 360.0);
    explosion_activa = true;
    flash_intensity = 1.0;

    loop {
        gfx.begin_draw();

        // Fondo negro con degradé azul oscuro
        gfx.clear_background(ColorRydit::Negro);

        // Input
        if gfx.is_key_pressed(Key::Escape) {
            break;
        }

        // Nueva explosión con ESPACIO
        if gfx.is_key_pressed(Key::Space) {
            let (mx, my) = (
                640.0 + (frame as f32 * 0.1).sin() * 300.0,
                360.0 + (frame as f32 * 0.1).cos() * 200.0,
            );
            crear_explosion(&mut particulas, mx, my);
            explosion_x = mx;
            explosion_y = my;
            explosion_activa = true;
            explosion_frame = 0;
            flash_intensity = 0.5;
            println!("💥 Explosión en ({:.0}, {:.0})", mx, my);
        }

        // Dibujar estrellas de fondo
        for i in 0..estrellas.len() {
            let (x, y, brillo_base) = estrellas[i];
            let brillo = brillo_base + (frame as f32 * 0.1 + i as f32 * 0.01).sin() * 0.3;
            let size = 1.0 + brillo * 2.0;

            // Parpadeo de estrellas
            let color = if brillo > 0.7 {
                ColorRydit::Blanco
            } else if brillo > 0.4 {
                ColorRydit::Gris
            } else {
                ColorRydit::Azul
            };

            gfx.draw_circle(x as i32, y as i32, size as i32, color);
        }

        // Actualizar explosión
        if explosion_activa {
            explosion_frame += 1;

            // Flash inicial (cámara)
            if flash_intensity > 0.0 {
                flash_intensity -= 0.05;
                // Dibujar flash blanco
                for i in 0..10 {
                    let radio = 50.0 + i as f32 * 80.0 + explosion_frame as f32 * 5.0;
                    let alpha = flash_intensity / (i as f32 + 1.0);
                    gfx.draw_circle(
                        explosion_x as i32,
                        explosion_y as i32,
                        radio as i32,
                        ColorRydit::Blanco,
                    ); // Flash intenso
                }
            }

            // Círculos concéntricos de la explosión (Big Bang)
            for i in 0..15 {
                let radio_base = 20.0 + i as f32 * 40.0;
                let radio = radio_base + explosion_frame as f32 * 8.0;

                // Colores que cambian con el tiempo
                let color = match (explosion_frame / 10 + i) % 6 {
                    0 => ColorRydit::Blanco,
                    1 => ColorRydit::Amarillo,
                    2 => ColorRydit::Naranja,
                    3 => ColorRydit::Rojo,
                    4 => ColorRydit::Magenta,
                    _ => ColorRydit::Cyan,
                };

                gfx.draw_circle(explosion_x as i32, explosion_y as i32, radio as i32, color);
            }

            // Actualizar y dibujar partículas
            for i in 0..particulas.len() {
                let (x, y, vx, vy, color) = particulas[i];

                // Mover partícula
                let nueva_x = x + vx;
                let nueva_y = y + vy + 0.5; // Gravedad

                // Dibujar partícula (cometa con cola)
                gfx.draw_circle(nueva_x as i32, nueva_y as i32, 4, color);

                // Cola del cometa
                gfx.draw_circle(
                    (nueva_x - vx * 2.0) as i32,
                    (nueva_y - vy * 2.0) as i32,
                    2,
                    ColorRydit::Gris,
                );

                // Actualizar partícula
                particulas[i].0 = nueva_x;
                particulas[i].1 = nueva_y;
            }

            // Eliminar partículas fuera de pantalla
            particulas
                .retain(|(x, y, _, _, _)| *x > -50.0 && *x < 1330.0 && *y > -50.0 && *y < 770.0);

            // Desactivar explosión cuando termine
            if explosion_frame > 200 && particulas.is_empty() {
                explosion_activa = false;
            }
        }

        // UI
        let fps = gfx.get_fps();
        gfx.draw_text(
            "=== 🌟 BIG BANG - Explosión Cósmica 🌟 ===",
            20,
            30,
            28,
            ColorRydit::Blanco,
        );
        gfx.draw_text(&format!("FPS: {}", fps), 20, 70, 22, ColorRydit::Verde);
        gfx.draw_text(
            &format!("Frame: {}", frame),
            20,
            100,
            20,
            ColorRydit::Amarillo,
        );
        gfx.draw_text(
            &format!("Partículas: {}", particulas.len()),
            20,
            130,
            18,
            ColorRydit::Cyan,
        );

        if explosion_activa {
            gfx.draw_text("💥 EXPLOSIÓN ACTIVA", 20, 160, 24, ColorRydit::Rojo);
        }

        gfx.draw_text(
            "ESPACIO = Nueva explosión | ESC = Salir",
            20,
            680,
            18,
            ColorRydit::Gris,
        );

        gfx.end_draw();

        frame += 1;
    }

    println!("\n✅ Demo completado: {} frames", frame);
}

fn crear_explosion(particulas: &mut Vec<(f32, f32, f32, f32, ColorRydit)>, x: f32, y: f32) {
    // Crear 200 partículas en todas direcciones
    for i in 0..200 {
        let angulo = (i as f32 / 200.0) * std::f32::consts::PI * 2.0;
        let velocidad = 5.0 + (i % 50) as f32 * 0.3;
        let vx = angulo.cos() * velocidad;
        let vy = angulo.sin() * velocidad;

        let color = match i % 8 {
            0 => ColorRydit::Blanco,
            1 => ColorRydit::Amarillo,
            2 => ColorRydit::Naranja,
            3 => ColorRydit::Rojo,
            4 => ColorRydit::Magenta,
            5 => ColorRydit::Cyan,
            6 => ColorRydit::Azul,
            _ => ColorRydit::Gris,
        };

        particulas.push((x, y, vx, vy, color));
    }
}
