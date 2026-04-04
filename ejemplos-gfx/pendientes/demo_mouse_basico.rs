// Demo Mouse Básico - Colores Predefinidos
// Ejecutar: cargo run --bin demo_mouse_basico
// Tamaño: 1280x720

use ry_gfx::{ColorRydit, Key, RyditGfx};

fn main() {
    println!("🛡️ RyDit v0.10.3 - Demo Mouse Básico");
    println!("======================================");
    println!("🖱️  Mouse + Botones + Clicks");
    println!("🪟 Ventana: 1280x720");
    println!("======================================\n");

    let mut gfx = RyditGfx::new("Demo Mouse Básico", 1280, 720);
    gfx.set_target_fps(60);

    // Botones: (x, y, ancho, alto, color, texto, presionado)
    let mut botones: Vec<(i32, i32, i32, i32, ColorRydit, &'static str, bool)> = vec![
        (100, 100, 200, 60, ColorRydit::Azul, "Boton 1", false),
        (350, 100, 200, 60, ColorRydit::Verde, "Boton 2", false),
        (600, 100, 200, 60, ColorRydit::Rojo, "Boton 3", false),
        (100, 200, 200, 60, ColorRydit::Naranja, "Reset", false),
        (350, 200, 200, 60, ColorRydit::Cyan, "Info", false),
    ];

    // Ventana
    let ventana_x = 700;
    let ventana_y = 250;
    let ventana_ancho = 400;
    let ventana_alto = 300;

    // Contadores
    let mut frame = 0;
    let mut info_visible = false;
    let mut clicks_totales = 0;
    let mut ultimo_click = 0;

    println!("🖱️  Controles:");
    println!("   Mouse = Mover cursor");
    println!("   Click Izquierdo = Presionar botones");
    println!("   R = Reset");
    println!("   ESC = Salir");
    println!("======================================\n");

    loop {
        gfx.begin_draw();
        gfx.clear_background(ColorRydit::Negro);

        // Input
        let posicion = gfx.get_mouse_position();
        let mouse_x = posicion.0;
        let mouse_y = posicion.1;

        let click_izq =
            gfx.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT as i32);
        let mouse_down =
            gfx.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT as i32);

        if gfx.is_key_pressed(Key::Escape) {
            break;
        }

        // Reset con R
        if gfx.is_key_pressed(Key::R) {
            for i in 0..botones.len() {
                botones[i].6 = false;
            }
            info_visible = false;
            clicks_totales = 0;
            println!("🔄 Reset completado");
        }

        // Detectar clicks
        if click_izq {
            clicks_totales += 1;
            ultimo_click = frame;

            for i in 0..botones.len() {
                let (x, y, w, h, _, texto, _) = botones[i];
                if mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h {
                    botones[i].6 = true;

                    match texto {
                        "Reset" => {
                            for j in 0..botones.len() {
                                botones[j].6 = false;
                            }
                            info_visible = false;
                            println!("🔄 Reset!");
                        }
                        "Info" => {
                            info_visible = !info_visible;
                            println!("ℹ️ Info: {}", if info_visible { "ON" } else { "OFF" });
                        }
                        _ => println!("✅ {} presionado!", texto),
                    }
                }
            }
        }

        // Soltar botones
        if !mouse_down || frame - ultimo_click > 10 {
            for i in 0..botones.len() {
                botones[i].6 = false;
            }
        }

        // Ventana
        gfx.draw_rect(
            ventana_x,
            ventana_y,
            ventana_ancho,
            ventana_alto,
            ColorRydit::Gris,
        );
        gfx.draw_rect(ventana_x, ventana_y, ventana_ancho, 30, ColorRydit::Cyan);
        gfx.draw_text(
            "Ventana Informativa",
            ventana_x + 10,
            ventana_y + 20,
            18,
            ColorRydit::Blanco,
        );
        gfx.draw_text(
            "Posicion: (700, 250)",
            ventana_x + 20,
            ventana_y + 60,
            16,
            ColorRydit::Amarillo,
        );

        if info_visible {
            gfx.draw_text(
                "=== INFORMACION ===",
                ventana_x + 20,
                ventana_y + 100,
                18,
                ColorRydit::Verde,
            );
            gfx.draw_text(
                "RyDit v0.10.3 - Demo Mouse",
                ventana_x + 30,
                ventana_y + 140,
                16,
                ColorRydit::Blanco,
            );
            gfx.draw_text(
                "Termux-X11 + Zink",
                ventana_x + 30,
                ventana_y + 165,
                16,
                ColorRydit::Blanco,
            );
            gfx.draw_text(
                "1280x720 @ 60 FPS",
                ventana_x + 30,
                ventana_y + 190,
                16,
                ColorRydit::Blanco,
            );
        }

        // Botones
        for i in 0..botones.len() {
            let (x, y, w, h, color, texto, presionado) = botones[i];

            let color_final = if presionado {
                ColorRydit::Blanco
            } else {
                color
            };

            gfx.draw_rect(x, y, w, h, color_final);
            gfx.draw_line(x, y, x + w, y, ColorRydit::Blanco);
            gfx.draw_line(x + w, y, x + w, y + h, ColorRydit::Blanco);
            gfx.draw_line(x + w, y + h, x, y + h, ColorRydit::Blanco);
            gfx.draw_line(x, y + h, x, y, ColorRydit::Blanco);
            gfx.draw_text(texto, x + 50, y + 30, 20, ColorRydit::Blanco);
        }

        // Cursor
        gfx.draw_circle(mouse_x, mouse_y, 10, ColorRydit::Amarillo);
        gfx.draw_circle(mouse_x, mouse_y, 5, ColorRydit::Rojo);

        // UI
        let fps = gfx.get_fps();
        gfx.draw_text(
            "=== 🖱️ Demo Mouse Básico 🖱️ ===",
            20,
            30,
            28,
            ColorRydit::Blanco,
        );
        gfx.draw_text(&format!("FPS: {}", fps), 20, 70, 22, ColorRydit::Verde);
        gfx.draw_text(
            &format!("Mouse: ({}, {})", mouse_x, mouse_y),
            20,
            110,
            18,
            ColorRydit::Cyan,
        );
        gfx.draw_text(
            &format!("Clicks: {}", clicks_totales),
            20,
            140,
            16,
            ColorRydit::Magenta,
        );
        gfx.draw_text(
            "Mouse=Mover | Click=Botones | R=Reset | ESC=Salir",
            20,
            690,
            16,
            ColorRydit::Gris,
        );

        gfx.end_draw();
        frame += 1;
    }

    println!("\n✅ Demo: {} frames | Clicks: {}", frame, clicks_totales);
}
