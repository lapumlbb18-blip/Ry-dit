// demo_text_3d.rs
// Demo exclusivo para letras 3D de ry3d-gfx
//
// cargo run --bin demo_text_3d --release

use raylib::prelude::*;
use ry3d_gfx::*;
use ry_gfx::ColorRydit;

fn main() -> Result<(), String> {
    println!("🔤 RyDit — Demo Letras 3D");

    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("Demo Letras 3D — ry3d-gfx")
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 6.0, 16.0),
        Vector3::new(0.0, 3.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.set_target_fps(60);

    let mut show_bg = true;

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, raylib::ffi::CameraMode::CAMERA_ORBITAL);

        if rl.is_key_pressed(KeyboardKey::KEY_B) {
            show_bg = !show_bg;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(180, 180, 190, 255)); // Gris claro

        // ---- MODO 3D ----
        let mut h3d = DrawHandle3D::new(&camera);
        h3d.clear_3d(ColorRydit::Gris); // Fondo gris en 3D
        h3d.draw_grid_3d(20, 1.0);
        h3d.draw_axes_gizmo(3.0);

        // Cubos de referencia para posicionar texto
        h3d.draw_cube_3d((-6.0, 1.0, 0.0), (2.0, 2.0, 2.0), ColorRydit::Rojo);
        h3d.draw_cube_3d((-2.0, 1.0, -4.0), (2.0, 2.0, 2.0), ColorRydit::Verde);
        h3d.draw_cube_3d((2.0, 1.0, -8.0), (2.0, 2.0, 2.0), ColorRydit::Azul);
        h3d.draw_cube_3d((6.0, 1.0, -12.0), (2.0, 2.0, 2.0), ColorRydit::Amarillo);

        // Cilindros
        h3d.draw_cylinder_3d((-4.0, 0.0, -4.0), 0.8, 0.8, 3.0, ColorRydit::Cyan);
        h3d.draw_cylinder_3d((4.0, 0.0, -8.0), 0.8, 0.8, 3.0, ColorRydit::Magenta);

        // Esferas
        h3d.draw_sphere_3d((0.0, 2.0, -6.0), 1.0, ColorRydit::Naranja);

        // ---- LETRAS 3D ----
        // Texto muy cerca de cámara para probar
        h3d.draw_text_3d_with_bg((0.0, 5.0, 4.0), "¡LETRAS 3D!", 32.0,
            ColorRydit::Rojo, ColorRydit::Negro);

        // Texto cerca de cada cubo
        h3d.draw_text_3d_with_bg((-6.0, 3.5, 0.0), "ROJO", 28.0, ColorRydit::Negro, ColorRydit::Blanco);
        h3d.draw_text_3d_with_bg((-2.0, 3.5, -4.0), "VERDE", 28.0, ColorRydit::Negro, ColorRydit::Blanco);
        h3d.draw_text_3d_with_bg((2.0, 3.5, -8.0), "AZUL", 28.0, ColorRydit::Negro, ColorRydit::Blanco);
        h3d.draw_text_3d_with_bg((6.0, 3.5, -12.0), "AMARILLO", 24.0, ColorRydit::Negro, ColorRydit::Blanco);

        // Texto sobre cilindros
        h3d.draw_text_3d_with_bg((-4.0, 4.5, -4.0), "CILINDRO 1", 22.0,
            ColorRydit::Negro, ColorRydit::Cyan);
        h3d.draw_text_3d_with_bg((4.0, 4.5, -8.0), "CILINDRO 2", 22.0,
            ColorRydit::Negro, ColorRydit::Magenta);

        // Texto sobre esfera
        h3d.draw_text_3d_with_bg((0.0, 4.5, -6.0), "ESFERA", 24.0,
            ColorRydit::Negro, ColorRydit::Amarillo);

        // Texto simple sin fondo (blanco puro)
        h3d.draw_text_3d((0.0, 8.0, -2.0), "Texto sin fondo", 20.0,
            ColorRydit::Blanco);

        // Texto en el suelo
        h3d.draw_text_3d((0.0, 0.5, 3.0), "[B] Toggle fondo texto", 16.0,
            ColorRydit::Negro);

        drop(h3d);

        // ---- HUD 2D ----
        d.draw_text("🔤 Demo Letras 3D — ry3d-gfx", 10, 10, 20, Color::WHITE);
        d.draw_text("Mouse: Orbitar camara | B: Toggle fondo | ESC: Salir", 10, 35, 16, Color::LIGHTGRAY);

        let cam_pos = format!(
            "Cam: ({:.1}, {:.1}, {:.1})",
            camera.position.x, camera.position.y, camera.position.z
        );
        d.draw_text(&cam_pos, 10, 55, 14, Color::GRAY);
    }

    println!("\n✅ Demo Letras 3D cerrado");
    Ok(())
}
