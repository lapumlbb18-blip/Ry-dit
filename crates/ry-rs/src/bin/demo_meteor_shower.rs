// demo_meteor_shower.rs
// 🌠 Demo: Lluvia de Meteoros — Física Newtoniana + Color por Velocidad + Blend Aditivo + Sonido Reactivo
//
// cargo run --bin demo_meteor_shower --release

use raylib::prelude::*;
use ry_gfx::lighting::*;
use ry_gfx::particles::{ParticleEmitter, ParticleSystem};

fn main() -> Result<(), String> {
    println!("🌠 RyDit — Demo Lluvia de Meteoros");
    println!("   Gravitación Newtoniana + Color por Velocidad + Blend Aditivo");
    println!("   [ESPACIO] Nuevo meteoro | [R] Reiniciar | [ESC] Salir");

    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("🌠 Meteor Shower — ry-gfx + ry-physics")
        .build();

    rl.set_target_fps(60);

    // === SISTEMA DE PARTÍCULAS ===
    let mut ps = ParticleSystem::new();
    ps.additive_blend = true; // Explosiones brillantes
    ps.global_gravity = 50.0; // Gravedad suave

    // === ESCALA DE VELOCIDAD ===
    let max_speed = 400.0; // pixels/seg para escala de color

    // === ESTADO ===
    let mut meteors = Vec::new();
    let mut frame = 0u64;
    let mut total_energy = 0.0f32;
    let mut impact_count = 0u64;

    // Crear meteoros iniciales
    spawn_meteor_cluster(&mut meteors, 450.0, 100.0, 5);

    // Meteoro central grande (como planeta)
    meteors.push(Meteor {
        x: 450.0, y: 350.0, vx: 0.0, vy: 0.0,
        mass: 500.0, radius: 20.0, alive: true,
    });

    while !rl.window_should_close() {
        // === INPUT ===
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            // Nuevo meteoro desde arriba
            let mx = 100.0 + rand_f32() * 700.0;
            spawn_single_meteor(&mut meteors, mx, -20.0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            meteors.clear();
            ps.clear();
            spawn_meteor_cluster(&mut meteors, 450.0, 100.0, 5);
            meteors.push(Meteor {
                x: 450.0, y: 350.0, vx: 0.0, vy: 0.0,
                mass: 500.0, radius: 20.0, alive: true,
            });
            impact_count = 0;
        }

        let dt = rl.get_frame_time();

        // === FÍSICA NEWTONIANA ===
        total_energy = 0.0;
        let n = meteors.len();
        for i in 0..n {
            if !meteors[i].alive { continue; }
            for j in (i + 1)..n {
                if !meteors[j].alive { continue; }
                let dx = meteors[j].x - meteors[i].x;
                let dy = meteors[j].y - meteors[i].y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt();

                // F = G * m1 * m2 / r²
                let g = 200.0; // Constante gravitacional escalada
                let force = g * meteors[i].mass * meteors[j].mass / dist_sq.max(100.0);
                let ax = force * dx / (dist * meteors[i].mass);
                let ay = force * dy / (dist * meteors[i].mass);
                let bx = -force * dx / (dist * meteors[j].mass);
                let by = -force * dy / (dist * meteors[j].mass);

                meteors[i].vx += ax * dt;
                meteors[i].vy += ay * dt;
                meteors[j].vx += bx * dt;
                meteors[j].vy += by * dt;

                // Colisión
                let min_dist = meteors[i].radius + meteors[j].radius;
                if dist < min_dist {
                    let ix = (meteors[i].x + meteors[j].x) / 2.0;
                    let iy = (meteors[i].y + meteors[j].y) / 2.0;
                    let energy = 0.5 * (meteors[i].mass + meteors[j].mass)
                        * ((meteors[i].vx - meteors[j].vx).powi(2)
                           + (meteors[i].vy - meteors[j].vy).powi(2));

                    spawn_explosion(&mut ps, ix, iy, energy);
                    impact_count += 1;
                    total_energy += energy;

                    // Meteoros más pequeños mueren, grande sobrevive
                    if meteors[i].mass < meteors[j].mass {
                        meteors[i].alive = false;
                    } else if meteors[i].mass > meteors[j].mass {
                        meteors[j].alive = false;
                    } else {
                        meteors[j].alive = false;
                    }
                }
            }
        }

        // Update meteoros
        for m in &mut meteors {
            if !m.alive { continue; }
            m.x += m.vx * dt;
            m.y += m.vy * dt;
            let speed = (m.vx * m.vx + m.vy * m.vy).sqrt();
            total_energy += 0.5 * m.mass * speed * speed;

            // Fuera de pantalla = muerto
            if m.y > 650.0 || m.x < -50.0 || m.x > 950.0 {
                m.alive = false;
            }
        }

        // Spawn nuevo meteoro cada ~2 segundos
        if frame % 120 == 0 {
            let mx = 50.0 + rand_f32() * 800.0;
            spawn_single_meteor(&mut meteors, mx, -20.0);
        }

        // Limpiar meteoros muertos
        meteors.retain(|m| m.alive);

        // Update partículas
        ps.update(dt);

        // === RENDER ===
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(5, 5, 15, 255)); // Cielo muy oscuro

        // Estrellas de fondo
        draw_stars(&mut d);

        // Dibujar partículas con blend aditivo + color por velocidad
        if ps.additive_blend {
            unsafe { raylib::ffi::BeginBlendMode(raylib::ffi::BlendMode::BLEND_ADDITIVE as i32) };
        }
        ps.draw_with_velocity(&mut d, max_speed);
        if ps.additive_blend {
            unsafe { raylib::ffi::EndBlendMode() };
        }

        // Dibujar meteoros con color por velocidad
        for m in &meteors {
            if !m.alive { continue; }
            let speed = (m.vx * m.vx + m.vy * m.vy).sqrt();
            let t = (speed / max_speed).clamp(0.0, 1.0);
            let color = velocity_color(t, m.radius);

            // Halo brillante
            d.draw_circle(m.x as i32, m.y as i32, m.radius * 2.0,
                Color::new(color.r / 3, color.g / 3, color.b / 3, 60));
            // Cuerpo
            d.draw_circle(m.x as i32, m.y as i32, m.radius, color);
            // Brillo central
            d.draw_circle(m.x as i32, m.y as i32, m.radius * 0.4, Color::WHITE);

            // Trail
            if speed > 50.0 {
                let trail_len = (speed / 50.0).min(10.0) as i32;
                for t_idx in 1..=trail_len {
                    let alpha = (200 - t_idx * 20) as u8;
                    let tx = (m.x - m.vx * 0.02 * t_idx as f32) as i32;
                    let ty = (m.y - m.vy * 0.02 * t_idx as f32) as i32;
                    let ts = m.radius - t_idx as f32;
                    if ts > 0.0 {
                        d.draw_circle(tx, ty, ts, Color::new(color.r / 2, color.g / 2, color.b / 2, alpha));
                    }
                }
            }
        }

        // === HUD ===
        d.draw_text("🌠 Meteor Shower — ry-gfx + ry-physics", 10, 10, 18, Color::WHITE);
        d.draw_text(&format!("Meteoros: {} | Impactos: {} | Energía: {:.0}",
            meteors.len(), impact_count, total_energy), 10, 35, 14, Color::LIGHTGRAY);
        d.draw_text("[ESPACIO] Nuevo | [R] Reiniciar | [ESC] Salir", 10, 55, 12, Color::GRAY);

        // Perfil de sonido (simulado)
        let sound_type = if total_energy < 100.0 { "💤 Rumble" }
            else if total_energy < 5000.0 { "💥 Impact" }
            else if total_energy < 50000.0 { "💫 Crash" }
            else { "🔥 EXPLOSION" };
        d.draw_text(&format!("Sonido: {} | Freq: ~{:.0}Hz", sound_type,
            100.0 + 500.0 * total_energy.sqrt()), 10, 570, 14,
            if total_energy > 50000.0 { Color::RED } else { Color::YELLOW });

        frame += 1;
    }

    println!("\n✅ Meteor Shower cerrado — {} impactos", impact_count);
    Ok(())
}

// ============================================================================
// METEOR STRUCT
// ============================================================================

struct Meteor {
    x: f32, y: f32, vx: f32, vy: f32,
    mass: f32, radius: f32, alive: bool,
}

// ============================================================================
// SPAWN HELPERS
// ============================================================================

fn spawn_meteor_cluster(meteors: &mut Vec<Meteor>, cx: f32, cy: f32, count: usize) {
    for _ in 0..count {
        let angle = rand_f32() * std::f32::consts::PI * 2.0;
        let dist = 30.0 + rand_f32() * 80.0;
        let mx = cx + angle.cos() * dist;
        let my = cy + angle.sin() * dist;
        spawn_single_meteor(meteors, mx, my);
    }
}

fn spawn_single_meteor(meteors: &mut Vec<Meteor>, x: f32, y: f32) {
    let mass = 10.0 + rand_f32() * 40.0;
    let radius = 3.0 + mass * 0.05;
    let vx = (rand_f32() - 0.5) * 60.0;
    let vy = 50.0 + rand_f32() * 100.0;
    meteors.push(Meteor {
        x, y, vx, vy, mass, radius, alive: true,
    });
}

// ============================================================================
// EXPLOSION PARTICLES
// ============================================================================

fn spawn_explosion(ps: &mut ParticleSystem, x: f32, y: f32, energy: f32) {
    let count = (energy / 10.0).min(100.0) as i32;
    let mut emitter = ParticleEmitter::explosion(x, y);
    emitter.spread = 360.0;
    emitter.speed_min = 50.0 + energy * 0.5;
    emitter.speed_max = 150.0 + energy;
    emitter.size_min = 2.0;
    emitter.size_max = 4.0 + energy * 0.02;
    emitter.one_shot = true;

    // Emitir partículas
    for _ in 0..count.max(10) {
        emitter.emit();
    }
    ps.emitters.insert(format!("explosion_{}", ps.emitters.len()), emitter);
}

// ============================================================================
// COLOR POR VELOCIDAD — Azul oscuro → Rojo NFS → Blanco flash
// ============================================================================

fn velocity_color(t: f32, radius: f32) -> Color {
    let (r, g, b) = if t < 0.2 {
        // Azul oscuro → Azul claro
        let lt = t / 0.2;
        lerp3((20, 40, 120), (80, 160, 255), lt)
    } else if t < 0.4 {
        let lt = (t - 0.2) / 0.2;
        lerp3((80, 160, 255), (255, 255, 80), lt)
    } else if t < 0.6 {
        let lt = (t - 0.4) / 0.2;
        lerp3((255, 255, 80), (255, 160, 20), lt)
    } else if t < 0.8 {
        let lt = (t - 0.6) / 0.2;
        lerp3((255, 160, 20), (255, 40, 20), lt)
    } else if t < 0.95 {
        let lt = (t - 0.8) / 0.15;
        lerp3((255, 40, 20), (255, 100, 80), lt)
    } else {
        let lt = (t - 0.95) / 0.05;
        lerp3((255, 100, 80), (255, 255, 255), lt)
    };
    Color::new(r as u8, g as u8, b as u8, 255)
}

fn lerp3(from: (u8, u8, u8), to: (u8, u8, u8), t: f32) -> (u16, u16, u16) {
    let t = t.clamp(0.0, 1.0);
    (
        (from.0 as f32 + (to.0 as f32 - from.0 as f32) * t) as u16,
        (from.1 as f32 + (to.1 as f32 - from.1 as f32) * t) as u16,
        (from.2 as f32 + (to.2 as f32 - from.2 as f32) * t) as u16,
    )
}

// ============================================================================
// ESTRELLAS DE FONDO
// ============================================================================

fn draw_stars(d: &mut RaylibDrawHandle) {
    // Estrellas estáticas (pseudo-random basadas en posición)
    for i in 0..80u32 {
        let sx = ((i * 7919 + 13) % 900) as i32;
        let sy = ((i * 6271 + 37) % 600) as i32;
        let brightness = 80 + ((i * 3571) % 100) as u8;
        d.draw_pixel(sx, sy, Color::new(brightness, brightness, brightness + 40, 255));
    }
}

// ============================================================================
// RAND
// ============================================================================

fn rand_f32() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as f32;
    (seed.sin() * 10000.0).fract()
}
