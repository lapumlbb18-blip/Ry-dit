// Demo: Fase 2 Completo - Ry-Dit
// Integra: L-System, Sistema Solar (Kepler), Radiación y Genética
// Uso: cargo run --bin demo_fase2_completo --release

use ry_gfx::{ColorRydit, RyditGfx, DrawHandle, Key};
use ry_physics::PhysicsModule;
use ry_science::ScienceModule;
use ry_core::RyditModule;
use serde_json::json;

fn main() {
    let mut gfx = RyditGfx::new("Ry-Dit Fase 2: Ciencia y Física", 1000, 700);
    gfx.set_target_fps(60);

    let physics = PhysicsModule;
    let science = ScienceModule;

    let mut current_view = 0; // 0: L-System, 1: Sistema Solar, 2: Genética/Radiación
    let mut t = 0.0;
    
    // Datos para Sistema Solar
    let planets = json!([
        {"name": "Mercurio", "a": 60.0, "e": 0.205, "period": 0.24, "angle_start": 0.0},
        {"name": "Venus", "a": 100.0, "e": 0.007, "period": 0.61, "angle_start": 1.0},
        {"name": "Tierra", "a": 150.0, "e": 0.017, "period": 1.0, "angle_start": 2.0},
        {"name": "Marte", "a": 220.0, "e": 0.093, "period": 1.88, "angle_start": 3.0}
    ]);

    // Datos para Genética
    let mut dna_sequence = "ATCGATCGATCGATCGATCGATCGATCGATCG".to_string();
    let mut mutations_total = 0;

    println!("[DEMO] Fase 2 Completa - 1: L-System, 2: Sistema Solar, 3: Genética, ESC: Salir");

    while !gfx.should_close() {
        // Input
        if gfx.is_key_pressed(Key::Num1) { current_view = 0; t = 0.0; }
        if gfx.is_key_pressed(Key::Num2) { current_view = 1; t = 0.0; }
        if gfx.is_key_pressed(Key::Num3) { current_view = 2; t = 0.0; }
        if gfx.is_key_pressed(Key::Escape) { break; }

        t += 0.016;

        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);

            match current_view {
                0 => draw_lsystem_view(&mut d, &science, t),
                1 => draw_solar_view(&mut d, &physics, &planets, t),
                2 => draw_genetics_view(&mut d, &science, &physics, &mut dna_sequence, &mut mutations_total, t),
                _ => {}
            }

            // HUD
            d.draw_text("Fase 2: Ciencia Avanzada + Física Nuclear", 10, 10, 20, ColorRydit::Amarillo);
            d.draw_text("1: L-System | 2: Sistema Solar | 3: Genética | ESC: Salir", 10, 670, 16, ColorRydit::Gris);
        }
    }
}

fn draw_lsystem_view(d: &mut DrawHandle, science: &ScienceModule, t: f64) {
    d.draw_text("Visualización L-System (Procedural)", 350, 50, 18, ColorRydit::Verde);
    
    // Ciclo entre presets cada 4 segundos
    let presets = ["binary_tree", "fern", "coral", "bush", "koch_plant"];
    let idx = (t as usize / 4) % presets.len();
    let name = presets[idx];
    
    let iterations = 4;
    let params = json!([name, iterations, 8.0, 500.0, 600.0]);
    
    if let Ok(res) = science.execute("lsystem::preset", params) {
        if let Some(result) = res["result"].as_object() {
            if let Some(segments) = result["segments"].as_array() {
                for seg in segments {
                    if let Some(s) = seg.as_array() {
                        let x1 = s[0].as_f64().unwrap_or(0.0) as i32;
                        let y1 = s[1].as_f64().unwrap_or(0.0) as i32;
                        let x2 = s[2].as_f64().unwrap_or(0.0) as i32;
                        let y2 = s[3].as_f64().unwrap_or(0.0) as i32;
                        d.draw_line(x1, y1, x2, y2, ColorRydit::Verde);
                    }
                }
            }
        }
    }
    d.draw_text(&format!("Preset: {}", name), 10, 60, 16, ColorRydit::Blanco);
}

fn draw_solar_view(d: &mut DrawHandle, physics: &PhysicsModule, planets_data: &serde_json::Value, t: f64) {
    d.draw_text("Simulación Orbital (Leyes de Kepler)", 350, 50, 18, ColorRydit::Azul);
    
    let sun_x = 500.0;
    let sun_y = 350.0;
    d.draw_circle(sun_x as i32, sun_y as i32, 15, ColorRydit::Amarillo); // El Sol

    let params = json!([planets_data, sun_x, sun_y, t]);
    if let Ok(res) = physics.execute("solar_system::step", params) {
        if let Some(planets) = res.as_array() {
            let colors = [ColorRydit::Gris, ColorRydit::Naranja, ColorRydit::Azul, ColorRydit::Rojo];
            for (i, p) in planets.iter().enumerate() {
                let x = p["x"].as_f64().unwrap_or(0.0) as i32;
                let y = p["y"].as_f64().unwrap_or(0.0) as i32;
                let name = p["name"].as_str().unwrap_or("?");
                let color = colors[i % colors.len()];
                
                d.draw_circle(x, y, 6, color);
                d.draw_text(name, x + 10, y - 10, 12, ColorRydit::Blanco);
                
                // Dibujar órbita (ahora con líneas de verdad)
                let dist = p["distance"].as_f64().unwrap_or(0.0) as f32;
                d.draw_circle_lines(sun_x as i32, sun_y as i32, dist, ColorRydit::Gris);
            }
        }
    }
}

fn draw_genetics_view(d: &mut DrawHandle, science: &ScienceModule, physics: &PhysicsModule, dna: &mut String, total: &mut usize, t: f64) {
    d.draw_text("Genética + Radiación (Evolución)", 350, 50, 18, ColorRydit::Rojo);

    // 1. Simular Radiación (Exposición)
    let radiation_energy = 0.002 * (1.0 + (t * 0.5).sin()); // Variable en el tiempo
    let rad_params = json!([radiation_energy, 1]); // Alpha particles (wr=20)
    let exposure = physics.execute("radiation::exposure", rad_params).unwrap();
    let msv = exposure[1].as_f64().unwrap_or(0.0);
    let risk = exposure[2].as_str().unwrap_or("?");

    // 2. Mutar ADN basado en la radiación (cada 2 segundos aprox)
    if (t * 60.0) as usize % 120 == 0 {
        let mut_params = json!({
            "sequence": dna.as_str(),
            "rate": 0.01,
            "radiation": msv / 50.0
        });
        if let Ok(res) = science.execute("dna::mutate", mut_params) {
            *dna = res["sequence"].as_str().unwrap_or(dna).to_string();
            *total += res["mutations"].as_u64().unwrap_or(0) as usize;
        }
    }

    // 3. Expresar fenotipo
    let exp_params = json!({"sequence": dna.as_str(), "gene_length": 4});
    let phenotype = science.execute("dna::express", exp_params).unwrap();
    let p_arr = phenotype["phenotype"].as_array().unwrap();

    // Visualización
    d.draw_text(&format!("Radiación: {:.2} mSv (Riesgo: {})", msv, risk), 50, 100, 16, ColorRydit::Rojo);
    d.draw_text(&format!("ADN: {}", dna), 50, 130, 14, ColorRydit::Verde);
    d.draw_text(&format!("Mutaciones totales: {}", total), 50, 150, 16, ColorRydit::Amarillo);

    // Dibujar "criatura" mutante basada en el fenotipo
    let cx = 500;
    let cy = 400;
    for (i, val) in p_arr.iter().enumerate() {
        let v = val.as_f64().unwrap_or(0.0);
        let size = (20.0 + v * 50.0) as i32;
        let angle = (i as f64 * 45.0).to_radians();
        let ox = (angle.cos() * 100.0) as i32;
        let oy = (angle.sin() * 100.0) as i32;
        
        let color = if v > 0.7 { ColorRydit::Rojo } else if v > 0.4 { ColorRydit::Verde } else { ColorRydit::Azul };
        d.draw_circle(cx + ox, cy + oy, size, color);
    }
    d.draw_circle(cx, cy, 30, ColorRydit::Blanco);
}
