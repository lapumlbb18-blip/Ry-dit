// Demo: Ilusiones Opticas Animadas
// Uso: cargo run --bin demo_illusions --release

use ry_gfx::{ColorRydit, RyditGfx};
use ry_anim::illusions;

fn main() {
    let mut gfx = RyditGfx::new("Ilusiones Opticas - Ry-Dit", 800, 600);
    gfx.set_target_fps(30);

    let mut current = 0;
    let mut t = 0.0;
    let names = [
        "Rotating Snakes", "Cafe Wall", "Troxler Fading",
        "Pulsing Star", "Zollner Effect", "Motion Blindness",
    ];

    println!("[DEMO] Ilusiones Opticas - 1-6 cambiar, ESC salir");

    while !gfx.should_close() {
        for (i, key) in [ry_gfx::Key::Num1, ry_gfx::Key::Num2, ry_gfx::Key::Num3,
                         ry_gfx::Key::Num4, ry_gfx::Key::Num5, ry_gfx::Key::Num6].iter().enumerate() {
            if gfx.is_key_pressed(*key) {
                current = i;
                t = 0.0;
            }
        }
        if gfx.is_key_pressed(ry_gfx::Key::Escape) { break; }

        t += 0.016;

        {
            let mut d = gfx.begin_draw();
            d.clear(ColorRydit::Negro);

            match current {
                0 => draw_snakes(&mut d, t),
                1 => draw_cafe(&mut d, t),
                2 => draw_troxler(&mut d, t),
                3 => draw_star(&mut d, t),
                4 => draw_zollner(&mut d, t),
                5 => draw_blindness(&mut d, t),
                _ => {}
            }

            d.draw_text(&format!("{} (t={:.1})", names[current], t), 10, 15, 16, ColorRydit::Blanco);
            d.draw_text("1-6 cambiar | ESC salir", 10, 575, 12, ColorRydit::Gris);
        }
    }
    println!("[DEMO] Frames: {}", (t / 0.016) as i32);
}

fn draw_snakes(d: &mut ry_gfx::DrawHandle, t: f64) {
    let colors = vec!["black".to_string(), "white".to_string(), "blue".to_string(), "yellow".to_string()];
    let segs = illusions::rotating_snakes(400.0, 300.0, 150.0, 20, t, &colors);
    for s in &segs {
        let (Some(x), Some(y)) = (s.get("x").and_then(|v| v.as_f64()), s.get("y").and_then(|v| v.as_f64())) else { continue };
        let c = match s.get("color").and_then(|v| v.as_str()).unwrap_or("") {
            "blue" => ColorRydit::Azul,
            "yellow" => ColorRydit::Amarillo,
            "black" => ColorRydit::Negro,
            _ => ColorRydit::Blanco,
        };
        d.draw_circle(x as i32, y as i32, 10, c);
    }
}

fn draw_cafe(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elems = illusions::cafe_wall(40.0, 40.0, 8, 10, 36.0, 18.0, 2.0, (t * 0.3).sin() * 0.5 + 0.5);
    for e in &elems {
        let Some(tp) = e.get("type").and_then(|v| v.as_str()) else { continue };
        if tp == "rect" {
            let (Some(x), Some(y), Some(w), Some(h)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64()), e.get("w").and_then(|v| v.as_f64()), e.get("h").and_then(|v| v.as_f64())) else { continue };
            let c = if e.get("color").and_then(|v| v.as_str()) == Some("#FFFFFF") { ColorRydit::Blanco } else { ColorRydit::Negro };
            d.draw_rectangle(x as i32, y as i32, w as i32, h as i32, c);
        } else if tp == "line" {
            let (Some(x1), Some(y1), Some(x2), Some(y2)) = (e.get("x1").and_then(|v| v.as_f64()), e.get("y1").and_then(|v| v.as_f64()), e.get("x2").and_then(|v| v.as_f64()), e.get("y2").and_then(|v| v.as_f64())) else { continue };
            d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, ColorRydit::Gris);
        }
    }
}

fn draw_troxler(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elems = illusions::troxler_fading(400.0, 300.0, 16, 120.0, 12.0, t);
    for e in &elems {
        let (Some(x), Some(y), Some(r)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64()), e.get("radius").and_then(|v| v.as_f64())) else { continue };
        let c = if e.get("color").and_then(|v| v.as_str()) == Some("#FF0000") { ColorRydit::Rojo } else { ColorRydit::Morado };
        d.draw_circle(x as i32, y as i32, r as i32, c);
    }
}

fn draw_star(d: &mut ry_gfx::DrawHandle, t: f64) {
    let pts = illusions::pulsing_star(400.0, 300.0, 100.0, 50.0, 6, t);
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        let (Some(x1), Some(x2)) = (pts[i].get("x").and_then(|v| v.as_f64()), pts[j].get("x").and_then(|v| v.as_f64())) else { continue };
        let y1 = pts[i].get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let y2 = pts[j].get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, ColorRydit::Amarillo);
    }
    d.draw_circle(400, 300, 8, ColorRydit::Naranja);
}

fn draw_zollner(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elems = illusions::zollner_effect(50.0, 80.0, 700.0, 40.0, 10, 15.0, 0.7, t * 0.3);
    for e in &elems {
        let (Some(x1), Some(y1), Some(x2), Some(y2)) = (e.get("x1").and_then(|v| v.as_f64()), e.get("y1").and_then(|v| v.as_f64()), e.get("x2").and_then(|v| v.as_f64()), e.get("y2").and_then(|v| v.as_f64())) else { continue };
        let c = if e.get("color").and_then(|v| v.as_str()) == Some("#A0A0FF") { ColorRydit::Azul } else { ColorRydit::Blanco };
        d.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32, c);
    }
}

fn draw_blindness(d: &mut ry_gfx::DrawHandle, t: f64) {
    let elems = illusions::motion_induced_blindness(400.0, 300.0, 12, 30.0, 6.0, t);
    for e in &elems {
        let Some(tp) = e.get("type").and_then(|v| v.as_str()) else { continue };
        if tp == "cross" {
            let (Some(x), Some(y), Some(s)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64()), e.get("size").and_then(|v| v.as_f64())) else { continue };
            let s = s as i32;
            d.draw_line(x as i32 - s, y as i32, x as i32 + s, y as i32, ColorRydit::Rojo);
            d.draw_line(x as i32, y as i32 - s, x as i32, y as i32 + s, ColorRydit::Rojo);
        } else if tp == "dot" {
            let (Some(x), Some(y), Some(s)) = (e.get("x").and_then(|v| v.as_f64()), e.get("y").and_then(|v| v.as_f64()), e.get("size").and_then(|v| v.as_f64())) else { continue };
            d.draw_circle(x as i32, y as i32, s as i32, ColorRydit::Azul);
        }
    }
}
