//! demo_test1.rs
//! Demo de diagnostico vertical (600x800) con instrumentacion y logs de Gameloop y Framebuffer
//!
//! Monitorea:
//! - Estados de OpenGL (Errores, FBO Binding, Viewport)
//! - Latencia de calculo vs Latencia de gl_swap_window()
//! - Deteccion de caidas de buffer y pautas de oscurecimiento

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

use events_ry::{InputEvent, InputManager, Key, MouseButton};
use ry_gfx::backend_sdl2::Sdl2Backend;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 800;
const PARTICLE_COUNT: usize = 150;

#[derive(Clone, Copy)]
struct Star {
    x: f32,
    y: f32,
    speed: f32,
    size: u32,
    brightness: u8,
}

unsafe fn check_gl_state(stage: &str, frame: u64) {
    let err = gl::GetError();
    if err != gl::NO_ERROR {
        eprintln!("[GL-ERR][Frame {}][{}] Codigo de Error OpenGL: 0x{:04X} ({})", frame, stage, err, err);
    }
    
    // Inspeccion periodica del Framebuffer y Viewport
    if frame % 60 == 1 {
        let mut fbo: i32 = -1;
        let mut viewport = [0i32; 4];
        let mut draw_buffer: i32 = -1;
        gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fbo);
        gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr());
        gl::GetIntegerv(gl::DRAW_BUFFER, &mut draw_buffer);
        println!(
            "[GL-STATE][Frame {}][{}] FBO: {} | Viewport: [{}, {}, {}, {}] | DrawBuffer: 0x{:04X}",
            frame, stage, fbo, viewport[0], viewport[1], viewport[2], viewport[3], draw_buffer
        );
    }
}

fn main() -> Result<(), String> {
    println!("==================================================");
    println!("Ry-Dit -- Diagnostico de Gameloop y Framebuffer (600x800)");
    println!("Controles:");
    println!("   - WASD / Flechas : Mover nave");
    println!("   - Mouse / Touch  : Posicionar nave");
    println!("   - ESPACIO        : Disparar proyectiles");
    println!("   - ESC            : Salir");
    println!("==================================================");

    let mut backend = Sdl2Backend::new("Ry-Dit Test 1 (600x800)", WIDTH as i32, HEIGHT as i32)?;
    let mut input = InputManager::new();

    // Fondo de estrellas
    let mut stars = Vec::with_capacity(PARTICLE_COUNT);
    let mut rng_state = 987654321u32;
    let mut rand_fn = || -> f32 {
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        ((rng_state >> 9) as f32) / 8388608.0
    };

    for _ in 0..PARTICLE_COUNT {
        stars.push(Star {
            x: rand_fn() * WIDTH as f32,
            y: rand_fn() * HEIGHT as f32,
            speed: 40.0 + rand_fn() * 140.0,
            size: if rand_fn() > 0.8 { 3 } else { 2 },
            brightness: (120.0 + rand_fn() * 135.0) as u8,
        });
    }

    // Nave interactiva
    let mut ship_x = (WIDTH as f32) / 2.0;
    let mut ship_y = (HEIGHT as f32) * 0.8;
    let mut ship_target_x = ship_x;
    let mut ship_target_y = ship_y;

    // Disparos
    let mut bullets: Vec<(f32, f32)> = Vec::new();

    let mut last_time = Instant::now();
    let mut timer = 0.0f32;
    let mut frame_count = 0u64;
    let mut last_log_time = Instant::now();
    let mut running = true;

    println!("[LOG] Gameloop iniciado con seguimiento de etapas");

    while running {
        let frame_start = Instant::now();
        let mut dt = frame_start.duration_since(last_time).as_secs_f32();
        last_time = frame_start;
        if dt > 0.05 {
            dt = 0.05;
        }
        timer += dt;
        frame_count += 1;

        // --- ETAPA 1: INPUT ---
        let t_input_start = Instant::now();
        backend.actualizar_input_unificado(&mut input);

        for event in input.poll_events() {
            match event {
                InputEvent::WindowCloseRequested => {
                    println!("[EVENT] Solicitud de cierre de ventana recibida");
                    running = false;
                }
                InputEvent::KeyPressed { key: Key::Escape } => {
                    println!("[EVENT] Tecla ESC presionada, saliendo");
                    running = false;
                }
                InputEvent::KeyPressed { key: Key::Space } => {
                    bullets.push((ship_x, ship_y - 20.0));
                }
                InputEvent::MouseMoved { x, y } => {
                    ship_target_x = x as f32;
                    ship_target_y = y as f32;
                }
                _ => {}
            }
        }

        // Movimiento WASD
        let speed = 350.0 * dt;
        if input.is_key_down(Key::W) || input.is_key_down(Key::Up) {
            ship_y -= speed;
            ship_target_y = ship_y;
        }
        if input.is_key_down(Key::S) || input.is_key_down(Key::Down) {
            ship_y += speed;
            ship_target_y = ship_y;
        }
        if input.is_key_down(Key::A) || input.is_key_down(Key::Left) {
            ship_x -= speed;
            ship_target_x = ship_x;
        }
        if input.is_key_down(Key::D) || input.is_key_down(Key::Right) {
            ship_x += speed;
            ship_target_x = ship_x;
        }

        // Movimiento suave con mouse/touch
        if input.is_mouse_button_down(MouseButton::Left) {
            ship_x += (ship_target_x - ship_x) * 12.0 * dt;
            ship_y += (ship_target_y - ship_y) * 12.0 * dt;
        }

        ship_x = ship_x.clamp(20.0, WIDTH as f32 - 20.0);
        ship_y = ship_y.clamp(20.0, HEIGHT as f32 - 20.0);
        let t_input_end = Instant::now();

        // --- ETAPA 2: FISICAS Y LOGICA ---
        for star in stars.iter_mut() {
            star.y += star.speed * dt;
            if star.y > HEIGHT as f32 {
                star.y = 0.0;
                star.x = rand_fn() * WIDTH as f32;
            }
        }

        bullets.retain_mut(|b| {
            b.1 -= 600.0 * dt;
            b.1 > -10.0
        });

        // --- ETAPA 3: RENDERIZADO (DRAW CALLS) ---
        let t_draw_start = Instant::now();
        unsafe { check_gl_state("PRE-CLEAR", frame_count); }

        // Fondo azul espacial
        backend.set_draw_color(Color::RGB(10, 12, 22));
        backend.clear();

        unsafe { check_gl_state("POST-CLEAR", frame_count); }

        // Dibujar estrellas
        for star in &stars {
            let b = star.brightness;
            backend.set_draw_color(Color::RGB(b, b, (b as f32 * 1.1).min(255.0) as u8));
            let _ = backend.fill_rect(Rect::new(star.x as i32, star.y as i32, star.size, star.size));
        }

        // Dibujar balas
        backend.set_draw_color(Color::RGB(255, 230, 50));
        for b in &bullets {
            let _ = backend.fill_rect(Rect::new(b.0 as i32 - 2, b.1 as i32, 4, 12));
        }

        // Dibujar Nave
        backend.set_draw_color(Color::RGB(40, 120, 240));
        let _ = backend.fill_rect(Rect::new(ship_x as i32 - 20, ship_y as i32 + 5, 40, 8));

        backend.set_draw_color(Color::RGB(0, 200, 255));
        let _ = backend.fill_rect(Rect::new(ship_x as i32 - 8, ship_y as i32 - 16, 16, 32));

        backend.set_draw_color(Color::WHITE);
        let _ = backend.fill_rect(Rect::new(ship_x as i32 - 4, ship_y as i32 - 10, 8, 12));

        let flame_h = (10.0 + (timer * 30.0).sin() * 5.0) as u32;
        backend.set_draw_color(Color::RGB(255, 100, 0));
        let _ = backend.fill_rect(Rect::new(ship_x as i32 - 4, ship_y as i32 + 16, 8, flame_h));

        // HUD Superior
        backend.set_draw_color(Color::RGBA(20, 30, 50, 220));
        let _ = backend.fill_rect(Rect::new(10, 10, WIDTH - 20, 40));
        backend.set_draw_color(Color::RGB(80, 140, 255));
        let _ = backend.draw_rect(Rect::new(10, 10, WIDTH - 20, 40));

        // Indicador de estado parpadeante (para saber si el GPU dibuja)
        let led_color = if (frame_count / 15) % 2 == 0 {
            Color::RGB(0, 255, 120)
        } else {
            Color::RGB(255, 200, 0)
        };
        backend.set_draw_color(led_color);
        let _ = backend.fill_rect(Rect::new(20, 22, 16, 16));

        unsafe { check_gl_state("POST-DRAW", frame_count); }
        let t_draw_end = Instant::now();

        // --- ETAPA 4: PRESENT & SWAP BUFFER ---
        let t_swap_start = Instant::now();
        backend.present();
        unsafe { check_gl_state("POST-SWAP", frame_count); }
        let t_swap_end = Instant::now();

        let dt_input = t_input_end.duration_since(t_input_start).as_secs_f64() * 1000.0;
        let dt_draw = t_draw_end.duration_since(t_draw_start).as_secs_f64() * 1000.0;
        let dt_swap = t_swap_end.duration_since(t_swap_start).as_secs_f64() * 1000.0;
        let dt_total = Instant::now().duration_since(frame_start).as_secs_f64() * 1000.0;

        // Log de pautas del Gameloop cada 1 segundo
        if frame_start.duration_since(last_log_time) >= Duration::from_secs(1) {
            let fps = 1.0 / dt.max(0.0001);
            println!(
                "[PAUTA-LOOP][Frame {:05}] FPS: {:.1} | Total: {:.2}ms [Input: {:.2}ms, Draw: {:.2}ms, Swap: {:.2}ms] | Balas: {}",
                frame_count, fps, dt_total, dt_input, dt_draw, dt_swap, bullets.len()
            );
            last_log_time = frame_start;
        }

        // Limitador de tasa de refresco a 60 FPS (16.6ms) para sincronizacion perfecta con Termux-X11
        let target_frame = Duration::from_micros(16666);
        let elapsed = frame_start.elapsed();
        if elapsed < target_frame {
            std::thread::sleep(target_frame - elapsed);
        }
    }

    println!("[LOG] Test finalizado tras {} frames.", frame_count);
    Ok(())
}
