// Demo 10K Partículas - Límite de Render
// Ejecutar: cargo run --bin demo_10k_particulas
// Tamaño: 1280x720 (HD)
// Partículas: 10,000 (estrés de CPU render)

use ry_gfx::{ColorRydit, Key, RyditGfx};

fn main() {
    println!("🛡️ RyDit v0.10.2 - 10K Partículas EXTREME");
    println!("==========================================");
    println!("🔥 ESTRÉS DE CPU RENDER");
    println!("🪟 Ventana: 1280x720");
    println!("⚛️  Partículas: 10,000");
    println!("🎮 Controles:");
    println!("   W = Explosión central");
    println!("   A = Lluvia de meteoros");
    println!("   S = Torbellino espiral");
    println!("   D = Fuente de partículas");
    println!("   R = Reiniciar todo");
    println!("   ESC = Salir");
    println!("==========================================\n");

    // Crear ventana HD
    let mut gfx = RyditGfx::new("10K Partículas EXTREME", 1280, 720);
    gfx.set_target_fps(60);

    // 10,000 partículas
    let mut particulas: Vec<(f32, f32, f32, f32, ColorRydit, u8)> = Vec::with_capacity(10000);
    // (x, y, vx, vy, color, vida)

    // Estrellas de fondo (1000 puntos)
    let mut estrellas: Vec<(f32, f32, f32)> = Vec::new();
    for i in 0..1000 {
        let x = (i * 17 % 1280) as f32;
        let y = (i * 31 % 720) as f32;
        let brillo = ((i % 100) as f32) / 100.0;
        estrellas.push((x, y, brillo));
    }

    let mut frame = 0;
    let mut modo = 0; // 0=ninguno, 1=explosión, 2=lluvia, 3=torbellino, 4=fuente

    // Crear explosión inicial de 10K partículas
    crear_explosion_masiva(&mut particulas, 640.0, 360.0, 10000);
    println!("💥 10,000 partículas creadas");

    loop {
        gfx.begin_draw();

        // Fondo negro
        gfx.clear_background(ColorRydit::Negro);

        // Input
        if gfx.is_key_pressed(Key::Escape) {
            break;
        }

        if gfx.is_key_pressed(Key::W) {
            particulas.clear();
            crear_explosion_masiva(&mut particulas, 640.0, 360.0, 10000);
            modo = 1;
            println!("💥 Explosión central - 10K partículas");
        }

        if gfx.is_key_pressed(Key::A) {
            particulas.clear();
            modo = 2;
            println!("☄️  Lluvia de meteoros");
        }

        if gfx.is_key_pressed(Key::S) {
            particulas.clear();
            crear_torbellino(&mut particulas, 640.0, 360.0, 10000);
            modo = 3;
            println!("🌪️  Torbellino espiral - 10K partículas");
        }

        if gfx.is_key_pressed(Key::D) {
            particulas.clear();
            modo = 4;
            println!("⛲ Fuente de partículas");
        }

        if gfx.is_key_pressed(Key::R) {
            particulas.clear();
            crear_explosion_masiva(&mut particulas, 640.0, 360.0, 10000);
            modo = 0;
            println!("🔄 Reiniciado - 10K partículas");
        }

        // Dibujar estrellas de fondo (parpadeo)
        for i in 0..estrellas.len() {
            let (x, y, brillo_base) = estrellas[i];
            let brillo = brillo_base + (frame as f32 * 0.05 + i as f32 * 0.01).sin() * 0.5;
            let size = 1.0 + brillo * 3.0;

            let color = if brillo > 0.8 {
                ColorRydit::Blanco
            } else if brillo > 0.5 {
                ColorRydit::Gris
            } else {
                ColorRydit::Azul
            };

            gfx.draw_circle(x as i32, y as i32, size as i32, color);
        }

        // Actualizar según modo
        match modo {
            2 => {
                // Lluvia de meteoros - generar continuamente
                if particulas.len() < 10000 {
                    for _ in 0..100 {
                        let x = (frame as f32 * 7.0 + particulas.len() as f32 * 13.0) % 1280.0;
                        let y = -10.0;
                        let vx = (frame as f32 * 0.01).sin() * 2.0;
                        let vy = 8.0 + (particulas.len() as f32 % 10.0);
                        let color = match frame % 6 {
                            0 => ColorRydit::Blanco,
                            1 => ColorRydit::Amarillo,
                            2 => ColorRydit::Naranja,
                            3 => ColorRydit::Rojo,
                            4 => ColorRydit::Cyan,
                            _ => ColorRydit::Magenta,
                        };
                        particulas.push((x, y, vx, vy, color, 200));
                    }
                }
            }
            4 => {
                // Fuente - generar desde abajo
                if particulas.len() < 10000 {
                    for _ in 0..100 {
                        let x = 640.0 + (frame as f32 * 0.5).sin() * 200.0;
                        let y = 720.0;
                        let vx = (particulas.len() as f32 - 5000.0) / 1000.0;
                        let vy = -10.0 - (frame % 20) as f32 * 0.3;
                        let color = match frame % 8 {
                            0 => ColorRydit::Blanco,
                            1 => ColorRydit::Amarillo,
                            2 => ColorRydit::Naranja,
                            3 => ColorRydit::Rojo,
                            4 => ColorRydit::Magenta,
                            5 => ColorRydit::Cyan,
                            6 => ColorRydit::Azul,
                            _ => ColorRydit::Verde,
                        };
                        particulas.push((x, y, vx, vy, color, 150));
                    }
                }
            }
            _ => {}
        }

        // Actualizar y dibujar todas las partículas
        // OPTIMIZACIÓN: Batch por color para reducir draw calls
        let mut particulas_a_eliminar: Vec<usize> = Vec::new();

        for i in 0..particulas.len() {
            let (x, y, vx, vy, color, mut vida) = particulas[i];

            // Actualizar posición
            let nueva_x = x + vx;
            let nueva_y = y + vy;

            // Gravedad y fricción
            let nueva_vy = vy + 0.1;
            let nueva_vx = vx * 0.99;

            // Dibujar partícula
            let size = if vida > 100 { 4 } else { (vida / 25) as i32 };
            gfx.draw_circle(nueva_x as i32, nueva_y as i32, size.max(1), color);

            // Estela/cometa (solo para algunas)
            if i % 3 == 0 && vida > 50 {
                gfx.draw_circle(
                    (nueva_x - nueva_vx * 3.0) as i32,
                    (nueva_y - nueva_vy * 3.0) as i32,
                    2,
                    ColorRydit::Gris,
                );
            }

            // Actualizar vida
            vida = vida.saturating_sub(1);

            // Marcar para eliminar si muere o sale de pantalla
            if vida == 0 || nueva_x < -50.0 || nueva_x > 1330.0 || nueva_y > 770.0 {
                particulas_a_eliminar.push(i);
            } else {
                // Actualizar partícula
                particulas[i] = (nueva_x, nueva_y, nueva_vx, nueva_vy, color, vida);
            }
        }

        // Eliminar partículas muertas (en orden inverso)
        for &idx in particulas_a_eliminar.iter().rev() {
            particulas.remove(idx);
        }

        // Reponer partículas si hay menos de 10K
        if particulas.len() < 10000 && modo != 0 {
            let faltantes = 10000 - particulas.len();
            for _ in 0..faltantes.min(500) {
                match modo {
                    1 => {
                        // Explosión desde centro
                        let angulo =
                            (particulas.len() as f32 / 10000.0) * std::f32::consts::PI * 2.0;
                        let velocidad = 3.0 + (particulas.len() as f32 % 100.0) / 20.0;
                        particulas.push((
                            640.0,
                            360.0,
                            angulo.cos() * velocidad,
                            angulo.sin() * velocidad,
                            match particulas.len() % 8 {
                                0 => ColorRydit::Blanco,
                                1 => ColorRydit::Amarillo,
                                2 => ColorRydit::Naranja,
                                3 => ColorRydit::Rojo,
                                4 => ColorRydit::Magenta,
                                5 => ColorRydit::Cyan,
                                6 => ColorRydit::Azul,
                                _ => ColorRydit::Verde,
                            },
                            200,
                        ));
                    }
                    2 => {
                        // Lluvia (ya manejado arriba)
                    }
                    3 => {
                        // Torbellino (ya creado)
                    }
                    4 => {
                        // Fuente (ya manejado arriba)
                    }
                    _ => {}
                }
            }
        }

        // UI
        let fps = gfx.get_fps();
        gfx.draw_text(
            "=== 🔥 10K PARTÍCULAS EXTREME 🔥 ===",
            20,
            30,
            28,
            ColorRydit::Blanco,
        );
        gfx.draw_text(
            &format!("FPS: {}", fps),
            20,
            70,
            24,
            if fps > 50 {
                ColorRydit::Verde
            } else if fps > 30 {
                ColorRydit::Amarillo
            } else {
                ColorRydit::Rojo
            },
        );
        gfx.draw_text(
            &format!("Frame: {}", frame),
            20,
            105,
            20,
            ColorRydit::Amarillo,
        );
        gfx.draw_text(
            &format!("Partículas: {} / 10000", particulas.len()),
            20,
            140,
            20,
            ColorRydit::Cyan,
        );
        gfx.draw_text(
            &format!(
                "Modo: {}",
                match modo {
                    0 => "Explosión inicial",
                    1 => "Explosión central",
                    2 => "Lluvia de meteoros",
                    3 => "Torbellino espiral",
                    4 => "Fuente de partículas",
                    _ => "Desconocido",
                }
            ),
            20,
            175,
            18,
            ColorRydit::Magenta,
        );
        gfx.draw_text(
            "W=Explosión | A=Lluvia | S=Torbellino | D=Fuente | R=Reiniciar",
            20,
            680,
            16,
            ColorRydit::Gris,
        );
        gfx.draw_text("ESC=Salir", 20, 705, 16, ColorRydit::Gris);

        gfx.end_draw();

        frame += 1;

        // Log cada segundo
        if frame % 60 == 0 {
            println!(
                "📊 Frame: {} | Partículas: {} | FPS: {}",
                frame,
                particulas.len(),
                fps
            );
        }
    }

    println!("\n✅ Test completado: {} frames", frame);
    println!("🛡️ 10K partículas: CPU render al límite");
}

fn crear_explosion_masiva(
    particulas: &mut Vec<(f32, f32, f32, f32, ColorRydit, u8)>,
    x: f32,
    y: f32,
    cantidad: usize,
) {
    for i in 0..cantidad {
        let angulo = (i as f32 / cantidad as f32) * std::f32::consts::PI * 2.0;
        let velocidad = 2.0 + (i as f32 % 200.0) / 40.0;
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
            _ => ColorRydit::Verde,
        };

        particulas.push((x, y, vx, vy, color, 200));
    }
}

fn crear_torbellino(
    particulas: &mut Vec<(f32, f32, f32, f32, ColorRydit, u8)>,
    x: f32,
    y: f32,
    cantidad: usize,
) {
    for i in 0..cantidad {
        let radio = 50.0 + (i as f32 / cantidad as f32) * 500.0;
        let angulo = (i as f32 / cantidad as f32) * std::f32::consts::PI * 16.0;

        let px = x + angulo.cos() * radio;
        let py = y + angulo.sin() * radio;

        // Velocidad tangencial (espiral)
        let vx = -angulo.sin() * 3.0 + angulo.cos() * 0.5;
        let vy = angulo.cos() * 3.0 + angulo.sin() * 0.5;

        let color = match i % 8 {
            0 => ColorRydit::Blanco,
            1 => ColorRydit::Cyan,
            2 => ColorRydit::Azul,
            3 => ColorRydit::Magenta,
            4 => ColorRydit::Blanco,
            5 => ColorRydit::Cyan,
            6 => ColorRydit::Azul,
            _ => ColorRydit::Magenta,
        };

        particulas.push((px, py, vx, vy, color, 255));
    }
}
