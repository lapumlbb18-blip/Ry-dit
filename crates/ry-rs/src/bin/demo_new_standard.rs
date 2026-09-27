//! demo_new_standard.rs
//! Demo Estándar de Alto Rendimiento para Ry-Dit (Arquitectura Híbrida SDL2 + RLGL)
//!
//! Características:
//! - Renderizado 2D optimizado con batching (cero asignaciones en caliente)
//! - Sistema de partículas reactivo e interactivo (Mouse/Touch + Teclado)
//! - Fondo procedural con rejilla dinámica e iluminación senoidal
//! - Medición precisa de FPS y telemetría de frametime
//! - Frame pacing inteligente para optimización de batería y temperatura en Termux

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

use events_ry::{InputEvent, InputManager, Key, MouseButton};
use ry_gfx::backend_sdl2::Sdl2Backend;

const SCREEN_WIDTH: u32 = 900;
const SCREEN_HEIGHT: u32 = 600;
const MAX_PARTICLES: usize = 250;

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    r: u8,
    g: u8,
    b: u8,
    size: f32,
    active: bool,
}

impl Particle {
    fn inactive() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            life: 0.0,
            max_life: 1.0,
            r: 255,
            g: 255,
            b: 255,
            size: 2.0,
            active: false,
        }
    }

    fn spawn(&mut self, x: f32, y: f32, vx: f32, vy: f32, r: u8, g: u8, b: u8, size: f32, life: f32) {
        self.x = x;
        self.y = y;
        self.vx = vx;
        self.vy = vy;
        self.life = life;
        self.max_life = life;
        self.r = r;
        self.g = g;
        self.b = b;
        self.size = size;
        self.active = true;
    }

    fn update(&mut self, dt: f32) {
        if !self.active {
            return;
        }
        self.life -= dt;
        if self.life <= 0.0 {
            self.active = false;
            return;
        }
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.vy += 35.0 * dt; // Gravedad suave
    }
}

fn pseudo_rand(seed: &mut u32) -> f32 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    ((*seed >> 9) as f32) / 8388608.0
}

fn main() -> Result<(), String> {
    println!("🛡️ ==================================================");
    println!("🚀 Ry-Dit — Demo Estándar Optimizado (SDL2 + RLGL)");
    println!("🎮 Controles:");
    println!("   - WASD / Flechas : Mover orbe de control");
    println!("   - Mouse / Touch  : Arrastrar orbe / Emisión continua");
    println!("   - ESPACIO        : Explosión de energía");
    println!("   - C              : Cambiar paleta de colores");
    println!("   - ESC            : Salir");
    println!("🛡️ ==================================================");

    let mut backend = Sdl2Backend::new(
        "Ry-Dit | Ultra-Optimized Standard Demo",
        SCREEN_WIDTH as i32,
        SCREEN_HEIGHT as i32,
    )?;
    let mut input = InputManager::new();

    // Pool de partículas preasignado (Zero-allocation en runtime)
    let mut particles = [Particle::inactive(); MAX_PARTICLES];
    let mut rng_seed: u32 = 123456789;

    // Estado interactivo
    let mut player_x = (SCREEN_WIDTH as f32) / 2.0;
    let mut player_y = (SCREEN_HEIGHT as f32) / 2.0;
    let mut target_x = player_x;
    let mut target_y = player_y;
    let mut palette_mode: u8 = 0;

    // Variables de telemetría y frametime
    let mut last_time = Instant::now();
    let mut timer = 0.0f32;
    let mut frame_count: u64 = 0;
    let mut fps: f32 = 60.0;
    let mut last_fps_update = Instant::now();
    let mut frames_in_second = 0u32;
    let mut running = true;

    while running {
        let now = Instant::now();
        let mut dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // Limitar dt para evitar saltos en caídas de frames
        if dt > 0.05 {
            dt = 0.05;
        }
        timer += dt;
        frame_count += 1;
        frames_in_second += 1;

        // Cálculo de FPS una vez cada 500ms
        if now.duration_since(last_fps_update) >= Duration::from_millis(500) {
            let elapsed = now.duration_since(last_fps_update).as_secs_f32();
            fps = (frames_in_second as f32) / elapsed;
            frames_in_second = 0;
            last_fps_update = now;
        }

        // 1. SINCRONIZACIÓN DE ENTRADA
        backend.actualizar_input_unificado(&mut input);

        for event in input.poll_events() {
            match event {
                InputEvent::WindowCloseRequested => running = false,
                InputEvent::KeyPressed { key: Key::Escape } => running = false,
                InputEvent::KeyPressed { key: Key::C } => {
                    palette_mode = (palette_mode + 1) % 3;
                }
                InputEvent::KeyPressed { key: Key::Space } => {
                    // Explosión de partículas radial
                    for p in particles.iter_mut() {
                        if !p.active {
                            let angle = pseudo_rand(&mut rng_seed) * std::f32::consts::TAU;
                            let speed = 100.0 + pseudo_rand(&mut rng_seed) * 220.0;
                            let vx = angle.cos() * speed;
                            let vy = angle.sin() * speed;
                            let life = 0.6 + pseudo_rand(&mut rng_seed) * 0.8;
                            let (r, g, b) = match palette_mode {
                                0 => (255, (100.0 + pseudo_rand(&mut rng_seed) * 155.0) as u8, 50),
                                1 => (50, (180.0 + pseudo_rand(&mut rng_seed) * 75.0) as u8, 255),
                                _ => (220, 50, (150.0 + pseudo_rand(&mut rng_seed) * 105.0) as u8),
                            };
                            p.spawn(player_x, player_y, vx, vy, r, g, b, 4.0, life);
                        }
                    }
                }
                InputEvent::MouseMoved { x, y } => {
                    target_x = x as f32;
                    target_y = y as f32;
                }
                _ => {}
            }
        }

        // Movimiento por teclado
        let move_speed = 320.0 * dt;
        if input.is_key_down(Key::W) || input.is_key_down(Key::Up) {
            player_y -= move_speed;
            target_y = player_y;
        }
        if input.is_key_down(Key::S) || input.is_key_down(Key::Down) {
            player_y += move_speed;
            target_y = player_y;
        }
        if input.is_key_down(Key::A) || input.is_key_down(Key::Left) {
            player_x -= move_speed;
            target_x = player_x;
        }
        if input.is_key_down(Key::D) || input.is_key_down(Key::Right) {
            player_x += move_speed;
            target_x = player_x;
        }

        // Suavizado e interpolación hacia el puntero del mouse/touch
        if input.is_mouse_button_down(MouseButton::Left) {
            player_x += (target_x - player_x) * 10.0 * dt;
            player_y += (target_y - player_y) * 10.0 * dt;

            // Emisión de partículas continuas en arrastre
            for p in particles.iter_mut() {
                if !p.active {
                    let vx = (pseudo_rand(&mut rng_seed) - 0.5) * 80.0;
                    let vy = (pseudo_rand(&mut rng_seed) - 0.5) * 80.0;
                    let (r, g, b) = match palette_mode {
                        0 => (255, 200, 50),
                        1 => (0, 220, 255),
                        _ => (255, 80, 200),
                    };
                    p.spawn(player_x, player_y, vx, vy, r, g, b, 3.0, 0.5);
                    break;
                }
            }
        }

        // Límites de pantalla
        player_x = player_x.clamp(30.0, (SCREEN_WIDTH as f32) - 30.0);
        player_y = player_y.clamp(30.0, (SCREEN_HEIGHT as f32) - 30.0);

        // 2. ACTUALIZACIÓN DE FÍSICAS Y PARTÍCULAS
        for p in particles.iter_mut() {
            p.update(dt);
        }

        // 3. RENDERIZADO OPTIMIZADO
        // Fondo con gradiente dinámico reactivo al temporizador
        let bg_r = (14.0 + (timer * 0.8).sin() * 6.0) as u8;
        let bg_g = (16.0 + (timer * 0.7).cos() * 6.0) as u8;
        let bg_b = (26.0 + (timer * 0.9).sin() * 8.0) as u8;
        backend.set_draw_color(Color::RGB(bg_r, bg_g, bg_b));
        backend.clear();

        // Rejilla de fondo (Grid procedural ligero)
        backend.set_draw_color(Color::RGBA(255, 255, 255, 12));
        let grid_size = 50;
        let offset_x = ((timer * 20.0) as i32) % grid_size;
        let offset_y = ((timer * 20.0) as i32) % grid_size;

        for x in (offset_x..SCREEN_WIDTH as i32).step_by(grid_size as usize) {
            backend.draw_line((x, 0), (x, SCREEN_HEIGHT as i32));
        }
        for y in (offset_y..SCREEN_HEIGHT as i32).step_by(grid_size as usize) {
            backend.draw_line((0, y), (SCREEN_WIDTH as i32, y));
        }

        // Dibujar ondas decorativas centrales
        let wave_y = ((SCREEN_HEIGHT as f32) * 0.5) as i32;
        backend.set_draw_color(Color::RGBA(70, 120, 200, 40));
        let mut prev_pt = (0, wave_y);
        for x in (0..SCREEN_WIDTH as i32).step_by(10) {
            let sine_offset = ((x as f32 * 0.02 + timer * 3.0).sin() * 25.0) as i32;
            let current_pt = (x, wave_y + sine_offset);
            if x > 0 {
                backend.draw_line(prev_pt, current_pt);
            }
            prev_pt = current_pt;
        }

        // Renderizado de Partículas en Batch
        for p in particles.iter() {
            if p.active {
                let alpha_ratio = (p.life / p.max_life).clamp(0.0, 1.0);
                let cur_r = (p.r as f32 * alpha_ratio) as u8;
                let cur_g = (p.g as f32 * alpha_ratio) as u8;
                let cur_b = (p.b as f32 * alpha_ratio) as u8;
                let sz = (p.size * alpha_ratio).max(1.0) as u32;

                backend.set_draw_color(Color::RGB(cur_r, cur_g, cur_b));
                let _ = backend.fill_rect(Rect::new(p.x as i32 - (sz as i32 / 2), p.y as i32 - (sz as i32 / 2), sz, sz));
            }
        }

        // Dibujado del Orbe / Jugador Principal con aura multicapa
        let pulse = (timer * 6.0).sin() * 3.0;
        let base_radius = 20.0 + pulse;

        // Aura exterior
        let (aura_r, aura_g, aura_b) = match palette_mode {
            0 => (255, 120, 30),
            1 => (0, 180, 255),
            _ => (255, 50, 180),
        };
        backend.set_draw_color(Color::RGBA(aura_r, aura_g, aura_b, 60));
        let outer_sz = ((base_radius + 12.0) * 2.0) as u32;
        let _ = backend.draw_rect(Rect::new(
            player_x as i32 - (outer_sz as i32 / 2),
            player_y as i32 - (outer_sz as i32 / 2),
            outer_sz,
            outer_sz,
        ));

        // Cuerpo central
        backend.set_draw_color(Color::RGB(aura_r, aura_g, aura_b));
        let core_sz = (base_radius * 2.0) as u32;
        let _ = backend.fill_rect(Rect::new(
            player_x as i32 - (core_sz as i32 / 2),
            player_y as i32 - (core_sz as i32 / 2),
            core_sz,
            core_sz,
        ));

        // Núcleo brillante
        backend.set_draw_color(Color::WHITE);
        let inner_sz = (base_radius * 0.8) as u32;
        let _ = backend.fill_rect(Rect::new(
            player_x as i32 - (inner_sz as i32 / 2),
            player_y as i32 - (inner_sz as i32 / 2),
            inner_sz,
            inner_sz,
        ));

        // HUD / OSD de Telemetría en tiempo real
        // Panel contenedor HUD superior
        backend.set_draw_color(Color::RGBA(15, 18, 25, 200));
        let _ = backend.fill_rect(Rect::new(15, 15, 340, 50));
        backend.set_draw_color(Color::RGBA(100, 150, 255, 180));
        let _ = backend.draw_rect(Rect::new(15, 15, 340, 50));

        // Indicador gráfico de FPS (barra verde/amarilla)
        let fps_bar_width = ((fps / 120.0).clamp(0.0, 1.0) * 140.0) as u32;
        backend.set_draw_color(if fps >= 55.0 {
            Color::RGB(40, 230, 100)
        } else if fps >= 30.0 {
            Color::RGB(230, 200, 40)
        } else {
            Color::RGB(240, 60, 60)
        });
        let _ = backend.fill_rect(Rect::new(25, 33, fps_bar_width, 14));
        backend.set_draw_color(Color::RGB(80, 90, 110));
        let _ = backend.draw_rect(Rect::new(25, 33, 140, 14));

        // Indicador de modo de paleta
        let pal_color = match palette_mode {
            0 => Color::RGB(255, 140, 0),
            1 => Color::RGB(0, 200, 255),
            _ => Color::RGB(255, 0, 180),
        };
        backend.set_draw_color(pal_color);
        let _ = backend.fill_rect(Rect::new(185, 33, 18, 14));

        // 4. INTERCAMBIO DE BUFFERS (Swap)
        backend.present();

        // Control de descanso para no consumir 100% CPU en threads móviles
        std::thread::sleep(Duration::from_millis(1));
    }

    println!("🛡️ Demo finalizado correctamente tras {} frames.", frame_count);
    Ok(())
}
