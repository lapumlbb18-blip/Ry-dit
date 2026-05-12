//! demo_test_gameloop.rs
//! Demo minimalista para validar el gameloop y el backend híbrido (SDL2 + RLGL)

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;

use ry_gfx::backend_sdl2::Sdl2Backend;
use events_ry::{InputManager, InputEvent, Key};

fn main() -> Result<(), String> {
    println!("🛡️ Ry-Dit - Test de Gameloop");
    println!("============================");

    let mut backend = Sdl2Backend::new("Ry-Dit | Test Gameloop", 800, 600)?;
    let mut input = InputManager::new();
    
    let mut last_time = Instant::now();
    let mut running = true;
    let mut box_x = 400.0;
    let mut box_y = 300.0;

    println!("\nControles:");
    println!("- WASD: Mover cuadrado");
    println!("- ESC: Salir");

    while running {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // 1. Sincronizar Input
        backend.actualizar_input_unificado(&mut input);
        
        for event in input.poll_events() {
            match event {
                InputEvent::WindowCloseRequested => running = false,
                InputEvent::KeyPressed { key: Key::Escape } => running = false,
                _ => {}
            }
        }

        // 2. Lógica simple
        let speed = 200.0;
        if input.is_key_down(Key::W) { box_y -= speed * dt; }
        if input.is_key_down(Key::S) { box_y += speed * dt; }
        if input.is_key_down(Key::A) { box_x -= speed * dt; }
        if input.is_key_down(Key::D) { box_x += speed * dt; }

        // 3. Renderizado
        backend.set_draw_color(Color::RGB(30, 30, 40)); 
        backend.clear();

        // Dibujar cuadrado
        backend.set_draw_color(Color::RGB(0, 255, 100));
        let _ = backend.fill_rect(Rect::new(box_x as i32 - 25, box_y as i32 - 25, 50, 50));

        // Dibujar borde
        backend.set_draw_color(Color::WHITE);
        let _ = backend.draw_rect(Rect::new(box_x as i32 - 25, box_y as i32 - 25, 50, 50));

        backend.present();
    }

    println!("Test finalizado correctamente.");
    Ok(())
}
