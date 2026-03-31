// Demo Assets Simples - Rects/Circulos con Mouse
// Ejecutar: cargo run --bin demo_assets_simple
// Tamaño: 1280x720

use rydit_gfx::{RyditGfx, ColorRydit, Key};

fn main() {
    println!("🛡️ RyDit v0.10.3 - Demo Assets Simples");
    println!("========================================");
    println!("🎨 Rects/Circulos + Mouse");
    println!("🪟 Ventana: 1280x720");
    println!("========================================\n");

    let mut gfx = RyditGfx::new("Demo Assets Simples", 1280, 720);
    gfx.set_target_fps(60);

    // "Assets" como rects/circulos (simulando sprites)
    // Tank: rect verde
    let mut tank_x = 200.0;
    let mut tank_y = 300.0;
    let tank_ancho = 32;
    let tank_alto = 32;
    
    // Crate: rect marrón
    let mut crate_x = 400.0;
    let mut crate_y = 300.0;
    let crate_ancho = 24;
    let crate_alto = 24;
    
    // Platform: rect gris
    let mut platform_x = 600.0;
    let mut platform_y = 300.0;
    let platform_ancho = 64;
    let platform_alto = 16;

    // Estado del mouse
    let mut mouse_x = 0;
    let mut mouse_y = 0;
    let mut arrastrando: Option<&'static str> = None;

    // Contadores
    let mut frame = 0;

    println!("\n🖱️  Controles:");
    println!("   Mouse = Mover cursor");
    println!("   Click = Arrastrar objetos");
    println!("   R = Reset posiciones");
    println!("   ESC = Salir");
    println!("========================================\n");

    loop {
        gfx.begin_draw();
        gfx.clear_background(ColorRydit::Negro);

        // Input
        let posicion = gfx.get_mouse_position();
        mouse_x = posicion.0;
        mouse_y = posicion.1;

        let click = gfx.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT as i32);
        let mouse_down = gfx.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT as i32);

        if gfx.is_key_pressed(Key::Escape) {
            break;
        }

        // Reset
        if gfx.is_key_pressed(Key::R) {
            tank_x = 200.0;
            tank_y = 300.0;
            crate_x = 400.0;
            crate_y = 300.0;
            platform_x = 600.0;
            platform_y = 300.0;
            arrastrando = None;
            println!("🔄 Reset!");
        }

        // Detectar click para arrastrar
        if click {
            // Tank
            if mouse_x >= tank_x as i32 && mouse_x <= (tank_x + tank_ancho as f32) as i32 &&
               mouse_y >= tank_y as i32 && mouse_y <= (tank_y + tank_alto as f32) as i32 {
                arrastrando = Some("tank");
                println!("🚜 Arrastrando tank");
            }
            
            // Crate
            if mouse_x >= crate_x as i32 && mouse_x <= (crate_x + crate_ancho as f32) as i32 &&
               mouse_y >= crate_y as i32 && mouse_y <= (crate_y + crate_alto as f32) as i32 {
                arrastrando = Some("crate");
                println!("📦 Arrastrando crate");
            }
            
            // Platform
            if mouse_x >= platform_x as i32 && mouse_x <= (platform_x + platform_ancho as f32) as i32 &&
               mouse_y >= platform_y as i32 && mouse_y <= (platform_y + platform_alto as f32) as i32 {
                arrastrando = Some("platform");
                println!("🟫 Arrastrando platform");
            }
        }

        // Arrastrar objetos
        if mouse_down {
            match arrastrando {
                Some("tank") => {
                    tank_x = (mouse_x - tank_ancho/2) as f32;
                    tank_y = (mouse_y - tank_alto/2) as f32;
                }
                Some("crate") => {
                    crate_x = (mouse_x - crate_ancho/2) as f32;
                    crate_y = (mouse_y - crate_alto/2) as f32;
                }
                Some("platform") => {
                    platform_x = (mouse_x - platform_ancho/2) as f32;
                    platform_y = (mouse_y - platform_alto/2) as f32;
                }
                _ => {}
            }
        } else {
            arrastrando = None;
        }

        // Dibujar grid de fondo
        for i in (0..1280).step_by(100) {
            gfx.draw_line(i, 0, i, 720, ColorRydit::Gris);
        }
        for i in (0..720).step_by(100) {
            gfx.draw_line(0, i, 1280, i, ColorRydit::Gris);
        }

        // Dibujar "assets"
        // Tank (verde)
        gfx.draw_rect(tank_x as i32, tank_y as i32, tank_ancho, tank_alto, ColorRydit::Verde);
        gfx.draw_circle((tank_x + tank_ancho as f32/2.0) as i32, (tank_y + tank_alto as f32/2.0) as i32, 8, ColorRydit::Verde);
        
        // Crate (marrón/naranja)
        gfx.draw_rect(crate_x as i32, crate_y as i32, crate_ancho, crate_alto, ColorRydit::Naranja);
        
        // Platform (gris)
        gfx.draw_rect(platform_x as i32, platform_y as i32, platform_ancho, platform_alto, ColorRydit::Gris);

        // Dibujar cursor
        gfx.draw_circle(mouse_x, mouse_y, 8, ColorRydit::Amarillo);
        gfx.draw_circle(mouse_x, mouse_y, 4, ColorRydit::Rojo);

        // UI
        let fps = gfx.get_fps();
        gfx.draw_text("=== 🎨 Demo Assets Simples 🎨 ===", 20, 30, 28, ColorRydit::Blanco);
        gfx.draw_text(&format!("FPS: {}", fps), 20, 70, 22, ColorRydit::Verde);
        gfx.draw_text(&format!("Mouse: ({}, {})", mouse_x, mouse_y), 20, 100, 18, ColorRydit::Cyan);
        gfx.draw_text(&format!("Arrastrando: {:?}", arrastrando), 20, 130, 16, ColorRydit::Magenta);
        gfx.draw_text(&format!("Tank: ({:.0}, {:.0})", tank_x, tank_y), 20, 160, 14, ColorRydit::Verde);
        gfx.draw_text(&format!("Crate: ({:.0}, {:.0})", crate_x, crate_y), 20, 185, 14, ColorRydit::Naranja);
        gfx.draw_text(&format!("Platform: ({:.0}, {:.0})", platform_x, platform_y), 20, 210, 14, ColorRydit::Gris);
        gfx.draw_text("Click=Arrastrar | R=Reset | ESC=Salir", 20, 690, 16, ColorRydit::Gris);

        gfx.end_draw();

        frame += 1;
    }

    println!("\n✅ Demo: {} frames", frame);
}
