//! demo_new_standard.rs
//! Demo con instrumentación para análisis de fallos de renderizado

use sdl2::pixels::Color;
use std::time::{Instant, Duration};

use ry_gfx::backend_sdl2::Sdl2Backend;
use events_ry::{InputManager, InputEvent, Key};

fn main() -> Result<(), String> {
    println!("DEBUG: Iniciando demo_new_standard...");

    let mut backend = Sdl2Backend::new("Ry-Dit Debug | Zink", 800, 600)?;
    let mut input = InputManager::new();
    
    let mut last_time = Instant::now();
    let mut running = true;
    let mut frame_count = 0u64;
    let mut timer = 0.0f32;
    let mut last_log = Instant::now();

    println!("DEBUG: Gameloop iniciado");

    while running {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;
        timer += dt;
        frame_count += 1;

        // Log cada 1 segundo para no saturar
        if now.duration_since(last_log) >= Duration::from_secs(1) {
            println!("DEBUG: Frame {} | Timer: {:.2}s | DT: {:.4}s", frame_count, timer, dt);
            last_log = now;
        }

        // 1. Sincronizar Input
        backend.actualizar_input_unificado(&mut input);
        
        for event in input.poll_events() {
            match event {
                InputEvent::WindowCloseRequested => {
                    println!("DEBUG: Solicitud de cierre detectada");
                    running = false;
                },
                InputEvent::KeyPressed { key: Key::Escape } => running = false,
                _ => {}
            }
        }

        // 2. Renderizado
        let r = ((timer.sin() * 0.5 + 0.5) * 255.0) as u8;
        let g = (((timer + 2.0).sin() * 0.5 + 0.5) * 255.0) as u8;
        
        backend.set_draw_color(Color::RGB(r, g, 150)); 
        backend.clear(); 

        // 3. Swap
        backend.present();

        // Pequeño delay para no consumir 100% CPU en el test
        std::thread::sleep(Duration::from_millis(1));
    }

    println!("DEBUG: Demo finalizado tras {} frames", frame_count);
    Ok(())
}
