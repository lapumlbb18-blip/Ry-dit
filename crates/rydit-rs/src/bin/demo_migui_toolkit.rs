// crates/rydit-rs/src/bin/demo_migui_toolkit.rs
// Demo de MiGUI Toolkit v2.0
// Muestra botones, labels y paneles

use migui::toolkit::{App, Button, Label, Panel, VBox, Theme};

fn main() {
    println!("[MIGUI TOOLKIT DEMO]: Iniciando...");

    // Crear aplicación
    let mut app = App::new("MiGUI Toolkit Demo - RyDit v0.11.0", 800, 600).unwrap();
    app.set_theme(Theme::dark());

    // Crear panel principal
    let panel = Panel::new(400, 300)
        .position(200, 150)
        .title("MiGUI Toolkit v2.0");

    // Crear labels
    let title = Label::new("¡HOLA MUNDO!")
        .position(250, 180)
        .size(24);

    let subtitle = Label::new("MiGUI Toolkit + RyBot")
        .position(280, 220)
        .size(16);

    // Crear botones
    let btn1 = Button::new("Jugar")
        .position(300, 300)
        .size(150, 40);

    let btn2 = Button::new("Opciones")
        .position(300, 350)
        .size(150, 40);

    let btn3 = Button::new("Salir")
        .position(300, 400)
        .size(150, 40);

    // Agregar widgets (la App los renderizará)
    // Nota: En esta versión simplificada, los widgets se dibujan directamente
    
    println!("[MIGUI TOOLKIT DEMO]: Widgets creados:");
    println!("  - Panel: 400x300");
    println!("  - Labels: 2 (titulo, subtitle)");
    println!("  - Botones: 3 (Jugar, Opciones, Salir)");
    println!("[MIGUI TOOLKIT DEMO]: Presiona ESC para salir");

    // Ejecutar (usamos el backend directamente por ahora)
    app.run();
}
