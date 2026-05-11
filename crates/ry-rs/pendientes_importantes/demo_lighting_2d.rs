// demo_lighting_2d.rs
// Demo: Iluminación 2D — ry-gfx lighting
//
// cargo run --bin demo_lighting_2d --release

use raylib::prelude::*;
use ry_gfx::lighting::*;

fn main() -> Result<(), String> {
    println!("💡 RyDit — Demo Iluminación 2D");

    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("Demo Iluminación 2D — ry-gfx")
        .build();

    rl.set_target_fps(60);

    let mut lm = LightManager::new();
    lm.setup_torch_scene(450.0, 300.0);

    let mut scene_mode = 0; // 0=torch, 1=day, 2=night, 3=multi
    let mut player_x = 450.0;
    let mut player_y = 300.0;

    // Colores para cajas (decoración)
    let box_colors = [
        Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW,
        Color::PURPLE, Color::ORANGE,
    ];

    while !rl.window_should_close() {
        // Input
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            scene_mode = (scene_mode + 1) % 4;
            match scene_mode {
                0 => lm.setup_torch_scene(player_x, player_y),
                1 => lm.setup_daylight_scene(),
                2 => lm.setup_night_scene(450.0, -100.0),
                3 => {
                    lm.lights.clear();
                    lm.ambient.intensity = 0.05;
                    lm.ambient.color = ry_gfx::ColorRydit::Morado;
                    lm.add_light(Light2D::point(200.0, 200.0, ry_gfx::ColorRydit::Rojo, 150.0));
                    lm.add_light(Light2D::point(700.0, 200.0, ry_gfx::ColorRydit::Azul, 150.0));
                    lm.add_light(Light2D::point(450.0, 500.0, ry_gfx::ColorRydit::Verde, 200.0));
                }
                _ => {}
            }
        }

        // Mover jugador con WASD
        let speed = 3.0;
        if rl.is_key_down(KeyboardKey::KEY_W) { player_y -= speed; }
        if rl.is_key_down(KeyboardKey::KEY_S) { player_y += speed; }
        if rl.is_key_down(KeyboardKey::KEY_A) { player_x -= speed; }
        if rl.is_key_down(KeyboardKey::KEY_D) { player_x += speed; }
        player_x = player_x.max(20.0).min(880.0);
        player_y = player_y.max(20.0).min(580.0);

        // Actualizar luz del jugador
        if scene_mode == 0 {
            if let Some(light) = lm.lights.get_mut(0) {
                light.position = (player_x, player_y);
            }
        }

        // Render — mouse pos before begin_drawing to avoid borrow conflict
        let scene_names = ["🔥 Antorcha", "☀️ Día", "🌙 Noche", "🌈 Multi-color"];
        let mouse_pos = rl.get_mouse_position();
        let test_lx = mouse_pos.x as f32;
        let test_ly = mouse_pos.y as f32;
        let brightness = lm.compute_lighting(test_lx, test_ly);
        let bar_w = 200;
        let bar_x = 700 - bar_w / 2;
        let bar_y = 570;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(10, 10, 15, 255));

        // Dibujar cajas decorativas
        for (i, &color) in box_colors.iter().enumerate() {
            let bx = 150.0 + (i as f32) * 120.0;
            let by = 250.0 + (i % 3) as f32 * 60.0;
            let lit = lm.compute_color_rgb(bx + 25.0, by + 25.0, color);
            d.draw_rectangle(bx as i32, by as i32, 50, 50, lit);
        }

        // Dibujar jugador (círculo iluminado)
        let player_lit = lm.compute_color_rgb(player_x, player_y, Color::WHITE);
        d.draw_circle(player_x as i32, player_y as i32, 15.0, player_lit);

        // Dibujar esferas de luz (indicadores visuales)
        for light in &lm.lights {
            if light.enabled {
                let lc = light.color.to_color();
                d.draw_circle_lines(light.position.0 as i32, light.position.1 as i32, (light.radius / 3.0), lc);
            }
        }

        d.draw_text("💡 Iluminación 2D — ry-gfx lighting", 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("Modo: {} | [ESPACIO] Cambiar | [WASD] Mover", scene_names[scene_mode]), 10, 35, 16, Color::LIGHTGRAY);
        d.draw_text(&format!("Luces activas: {} | Ambient: {:.2}", lm.active_count(), lm.ambient.intensity), 10, 55, 14, Color::GRAY);

        d.draw_rectangle(bar_x, bar_y, bar_w, 10, Color::BLACK);
        d.draw_rectangle(bar_x, bar_y, (bar_w as f32 * brightness) as i32, 10, Color::YELLOW);
        d.draw_text(&format!("Luz: {:.0}%", brightness * 100.0), bar_x, bar_y - 16, 12, Color::WHITE);
    }

    println!("\n✅ Demo iluminación 2D cerrado");
    Ok(())
}
