// demo_3d_touch.rs
// Demo: ry3d-gfx — Primitivas 3D + Controles Táctiles (como RayGunz)
//
// 100% raylib — sin SDL2
// TouchControls: joysticks virtuales + botones en pantalla
//
// Controles:
// Joystick izq: Orbitar cámara (horizontal)
// Joystick der: Subir/bajar cámara (vertical)
// Botón A: Toggle controles visuales
// Botón B: Reset cámara
// Mouse: click+arrastrar orbita | rueda zoom

use raylib::prelude::*;
use ry3d_gfx::DrawHandle3D;
use ry3d_gfx::touch_controls::TouchControls;

// Colores
use ry_gfx::ColorRydit;

fn main() -> Result<(), String> {
    println!("🎮 RyDit - Demo 3D Touch (100% raylib)");

    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("Demo 3D Touch - ry3d-gfx + TouchControls")
        .build();

    // Cámara orbital manual
    let mut cam_dist = 12.0;
    let mut cam_theta = 0.8;
    let mut cam_phi = 0.5;
    let cam_target = raylib::prelude::Vector3::new(0.0, 1.0, 0.0);

    // Controles táctiles
    let mut touch = TouchControls::new(900.0, 600.0);

    rl.set_target_fps(60);

    println!("✅ Ventana 3D creada");
    println!("   Touch: Joysticks virtuales + botones A/B");
    println!("   Mouse: Click+arrastrar orbita | Rueda zoom");
    println!("   ESC: Salir");

    while !rl.window_should_close() {
        // ---- INPUT TOUCH ----
        let touching = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let touch_pos = rl.get_mouse_position();
        touch.update(touching, touch_pos.x, touch_pos.y);

        // Joystick izq → orbitar (theta)
        let (jx, _jy) = touch.joy_left.axis();
        cam_theta += jx * 0.03;

        // Joystick der → altura (phi)
        let (_rx, ry) = touch.joy_right.axis();
        cam_phi = (cam_phi + ry * 0.02).max(0.1).min(3.0);

        // Botón A → toggle
        if touch.btn_a.just_pressed {
            touch.toggle();
        }

        // Botón B → reset
        if touch.btn_b.just_pressed {
            cam_dist = 12.0;
            cam_theta = 0.8;
            cam_phi = 0.5;
        }

        // Zoom con rueda del mouse
        let wheel = rl.get_mouse_wheel_move();
        cam_dist = (cam_dist - wheel * 0.5).max(3.0).min(30.0);

        // Calcular posición de cámara
        let cam_x = cam_target.x + cam_dist * cam_phi.sin() * cam_theta.cos();
        let cam_y = cam_target.y + cam_dist * cam_phi.cos();
        let cam_z = cam_target.z + cam_dist * cam_phi.sin() * cam_theta.sin();

        let camera = Camera3D::perspective(
            raylib::prelude::Vector3::new(cam_x, cam_y, cam_z),
            cam_target,
            raylib::prelude::Vector3::new(0.0, 1.0, 0.0),
            45.0,
        );

        // ---- RENDER ----
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // ---- MODO 3D (DrawHandle3D de ry3d-gfx) ----
        let mut h3d = DrawHandle3D::new(&camera);

        h3d.clear_3d(ColorRydit::Negro);
        h3d.draw_grid_3d(20, 1.0);

        // Ejes
        h3d.draw_line_3d((0.0, 0.0, 0.0), (3.0, 0.0, 0.0), ColorRydit::Rojo);
        h3d.draw_line_3d((0.0, 0.0, 0.0), (0.0, 3.0, 0.0), ColorRydit::Verde);
        h3d.draw_line_3d((0.0, 0.0, 0.0), (0.0, 0.0, 3.0), ColorRydit::Azul);

        // CUBOS
        h3d.draw_cube_3d((0.0, 1.0, 0.0), (2.0, 2.0, 2.0), ColorRydit::Rojo);
        h3d.draw_cube_wires_3d((-3.5, 1.0, 0.0), (2.0, 2.0, 2.0), ColorRydit::Verde);
        h3d.draw_cube_3d((3.5, 0.5, 0.0), (1.0, 1.0, 1.0), ColorRydit::Azul);

        // ESFERAS
        h3d.draw_sphere_3d((0.0, 1.0, -4.0), 1.0, ColorRydit::Amarillo);
        h3d.draw_sphere_wires_3d((-3.5, 1.0, -4.0), 1.0, ColorRydit::Magenta);
        h3d.draw_sphere_3d((3.5, 0.5, -4.0), 0.5, ColorRydit::Cyan);

        // CILINDROS
        h3d.draw_cylinder_3d((0.0, 1.0, -8.0), 0.5, 0.8, 2.0, ColorRydit::Naranja);
        h3d.draw_cylinder_wires_3d((-3.5, 1.0, -8.0), 0.5, 0.8, 2.0, ColorRydit::Blanco);
        h3d.draw_cylinder_3d((3.5, 1.0, -8.0), 0.0, 0.8, 2.0, ColorRydit::Morado);

        // PLANO
        h3d.draw_plane_3d((0.0, 0.01, -12.0), 4.0, ColorRydit::Gris);

        // PIRÁMIDE
        let base = [
            (-5.0, 0.0, -12.0), (-4.0, 0.0, -12.0),
            (-4.0, 0.0, -13.0), (-5.0, 0.0, -13.0),
        ];
        let apex = (-4.5, 2.0, -12.5);
        for i in 0..4 {
            h3d.draw_line_3d(base[i], apex, ColorRydit::Rojo);
            h3d.draw_line_3d(base[i], base[(i+1)%4], ColorRydit::Verde);
        }

        // TRIÁNGULO
        h3d.draw_triangle_3d(
            (5.0, 0.0, -12.0), (7.0, 0.0, -12.0), (6.0, 2.0, -12.0),
            ColorRydit::Naranja,
        );

        // PUNTOS
        for i in 0..10 {
            let x = -5.0 + i as f32;
            h3d.draw_point_3d((x, 3.0 + i as f32 * 0.3, -14.0), ColorRydit::Blanco);
        }

        // BBOX
        h3d.draw_bounding_box_3d((5.0, 0.0, -4.0), (7.0, 2.0, -2.0), ColorRydit::Verde);

        // Fin modo 3D
        drop(h3d);

        // ---- TOUCH CONTROLS (2D overlay) ----
        touch.draw();

        // ---- HUD ----
        d.draw_text("🎮 Demo 3D Touch — 100% raylib + TouchControls", 10, 10, 18, Color::WHITE);
        let info = format!("Cam: ({:.1},{:.1},{:.1}) Dist: {:.1}", cam_x, cam_y, cam_z, cam_dist);
        d.draw_text(&info, 10, 32, 14, Color::LIGHTGRAY);
        d.draw_text("Joy izq: Orbitar | Joy der: Altura | A: Toggle | B: Reset", 10, 570, 14, Color::DARKGRAY);
        d.draw_text("Mouse: Click+arrastre orbita | Rueda zoom | ESC salir", 10, 588, 12, Color::DARKGRAY);
    }

    println!("\n✅ Demo 3D Touch cerrado");
    Ok(())
}
