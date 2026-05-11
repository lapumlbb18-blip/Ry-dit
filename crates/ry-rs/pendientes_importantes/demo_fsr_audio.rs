// Demo: FSR 1.0 Visual Test
// Uso: cargo run --bin demo_fsr_audio --release

use ry_gfx::{ColorRydit, RyditGfx};
use ry_gfx::fsr::FsrQuality;

fn main() {
    let mut gfx = RyditGfx::new("FSR 1.0 Demo", 800, 600);
    gfx.set_target_fps(60);

    if let Err(e) = gfx.init_fsr(FsrQuality::Quality) {
        eprintln!("[FSR] {}", e);
    }

    println!("[DEMO] F=FSR  E=Toggle  ESC=Salir");

    while !gfx.should_close() {
        if gfx.is_key_pressed(ry_gfx::Key::F) {
            gfx.cycle_fsr_quality();
            eprintln!("[FSR] {:?}", gfx.fsr_quality());
        }
        if gfx.is_key_pressed(ry_gfx::Key::E) {
            let was = gfx.is_fsr_enabled();
            gfx.set_fsr_enabled(!was);
            eprintln!("[FSR] {}", if !was { "ON" } else { "OFF" });
        }
        if gfx.is_key_pressed(ry_gfx::Key::Escape) { break; }

        gfx.clear_background(ColorRydit::Negro);

        // Grid
        for x in (0..800).step_by(40) {
            gfx.draw_line(x as i32, 0, x as i32, 600, ColorRydit::Azul);
        }
        for y in (0..600).step_by(40) {
            gfx.draw_line(0, y as i32, 800, y as i32, ColorRydit::Azul);
        }

        // Formas de test FSR (bordes nítidos para ver upscale)
        gfx.draw_circle(400, 300, 80, ColorRydit::Rojo);
        gfx.draw_circle(400, 300, 50, ColorRydit::Amarillo);
        gfx.draw_circle(400, 300, 25, ColorRydit::Blanco);

        gfx.end_draw();
    }

    eprintln!("[DEMO] FSR: {:?}", gfx.fsr_quality());
}
