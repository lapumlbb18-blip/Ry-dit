// crates/rydit-rs/src/bin/demo_toolkit_ry.rs
// Demo de Toolkit RyDit - UI en rydit-gfx
// v0.11.0 - Toolkit integrado en rydit-gfx

use rydit_gfx::backend_sdl2::Sdl2Backend;
use rydit_gfx::toolkit::{Button, Label, Panel, Theme};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {
    println!("[TOOLKIT RY DEMO]: Iniciando...");

    // Crear backend SDL2
    let mut backend = Sdl2Backend::new("Toolkit RyDit v0.11.0", 800, 600).unwrap();

    // Tema
    let theme = Theme::dark();

    // Crear widgets
    let panel = Panel::new(400, 300)
        .position(200, 150)
        .title("Toolkit RyDit");

    let title = Label::new("¡HOLA MUNDO!").position(280, 180).size(24);

    let subtitle = Label::new("UI Toolkit en rydit-gfx")
        .position(260, 220)
        .size(16);

    let btn_jugar = Button::new("Jugar").position(300, 300).size(150, 40);

    let btn_opciones = Button::new("Opciones").position(300, 350).size(150, 40);

    let btn_salir = Button::new("Salir").position(300, 400).size(150, 40);

    println!("[TOOLKIT RY DEMO]: Widgets creados");
    println!("[TOOLKIT RY DEMO]: Presiona ESC para salir");

    // Game loop
    let mut frame = 0u32;
    let mut running = true;

    while running {
        // Eventos
        for event in backend.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    // Verificar clicks en botones
                    if btn_jugar.is_clicked(x, y) {
                        println!("[TOOLKIT RY DEMO]: Click en JUGAR");
                    }
                    if btn_opciones.is_clicked(x, y) {
                        println!("[TOOLKIT RY DEMO]: Click en OPCIONES");
                    }
                    if btn_salir.is_clicked(x, y) {
                        println!("[TOOLKIT RY DEMO]: Click en SALIR");
                        running = false;
                    }
                }
                _ => {}
            }
        }

        // Clear
        backend.canvas.set_draw_color(Color::RGB(20, 20, 40));
        backend.canvas.clear();

        // Render widgets
        panel.render(&mut backend, &theme);
        title.render(&mut backend, &theme);
        subtitle.render(&mut backend, &theme);
        btn_jugar.render(&mut backend, &theme);
        btn_opciones.render(&mut backend, &theme);
        btn_salir.render(&mut backend, &theme);

        // Presentar
        backend.canvas.present();
        frame += 1;

        if frame % 60 == 0 {
            println!("[TOOLKIT RY DEMO]: Frame {}", frame);
        }
    }

    println!("[TOOLKIT RY DEMO]: Cerrando... ({} frames)", frame);
}
