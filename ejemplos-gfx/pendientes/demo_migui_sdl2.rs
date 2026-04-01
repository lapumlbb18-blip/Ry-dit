// Demo MiGUI + SDL2 - RyDit v0.10.9
// Ejecutar: cargo run --bin demo_migui_sdl2 --release

use migui::backend_sdl2::MiguiSdl2Backend;
use migui::{Color, Migui, Rect, WidgetId};

fn main() {
    println!("🛡️ RyDit v0.10.9 - Demo MiGUI + SDL2");
    println!("=====================================\n");

    // Inicializar SDL2
    let sdl_context = sdl2::init().expect("Failed to init SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to init video");

    let window = video_subsystem
        .window("MiGUI Demo", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to create canvas");
    let mut event_pump = sdl_context.event_pump().expect("Failed to init event pump");

    // Crear backend MiGUI + SDL2
    let mut backend = MiguiSdl2Backend::new(canvas);

    // Crear MiGUI
    let mut gui = Migui::new();

    // Estado del demo
    let mut contador = 0;
    let mut slider_value = 50.0;
    let mut checkbox_checked = false;
    let mut running = true;

    println!("✅ MiGUI + SDL2 inicializado");
    println!("✅ Botones: Incrementar, Decrementar, Reset");
    println!("✅ Slider: Ajusta valor (0-100)");
    println!("✅ Checkbox: Activar/Desactivar");
    println!("✅ ESC para salir\n");

    // Game loop
    while running {
        // Iniciar frame de MiGUI
        gui.begin_frame();

        // Procesar eventos SDL2 uno por uno
        for sdl_event in event_pump.poll_iter() {
            running = !backend.process_events(&mut gui, &sdl_event);
        }

        // Título
        gui.label(
            WidgetId::new("title"),
            "MiGUI + SDL2 Demo",
            Rect::new(20.0, 20.0, 300.0, 40.0),
        );

        // Contador
        gui.label(
            WidgetId::new("counter_label"),
            &format!("Contador: {}", contador),
            Rect::new(20.0, 80.0, 200.0, 30.0),
        );

        // Botones
        let btn_w = 120.0;
        let btn_h = 40.0;
        let btn_y = 130.0;
        let btn_spacing = 140.0;

        if gui.button(
            WidgetId::new("btn_inc"),
            Rect::new(20.0, btn_y, btn_w, btn_h),
            "Incrementar",
        ) {
            contador += 1;
        }

        if gui.button(
            WidgetId::new("btn_dec"),
            Rect::new(20.0 + btn_spacing, btn_y, btn_w, btn_h),
            "Decrementar",
        ) {
            if contador > 0 {
                contador -= 1;
            }
        }

        if gui.button(
            WidgetId::new("btn_reset"),
            Rect::new(20.0 + btn_spacing * 2.0, btn_y, btn_w, btn_h),
            "Reset",
        ) {
            contador = 0;
        }

        // Slider
        gui.label(
            WidgetId::new("slider_label"),
            &format!("Slider: {:.0}", slider_value),
            Rect::new(20.0, 200.0, 200.0, 30.0),
        );

        slider_value = gui.slider(
            WidgetId::new("slider"),
            slider_value,
            0.0,
            100.0,
            Rect::new(20.0, 240.0, 300.0, 30.0),
        );

        // Checkbox
        gui.checkbox(
            WidgetId::new("checkbox"),
            "Opción activada",
            &mut checkbox_checked,
            Rect::new(20.0, 300.0, 200.0, 30.0),
        );

        // Panel de información
        let panel_color = if checkbox_checked {
            Color::GREEN
        } else {
            Color::GRAY
        };
        gui.panel(
            WidgetId::new("info_panel"),
            Rect::new(20.0, 350.0, 400.0, 150.0),
            panel_color,
        );

        gui.label(
            WidgetId::new("info_text"),
            &format!(
                "Estado:\n  Contador: {}\n  Slider: {:.0}\n  Checkbox: {}",
                contador,
                slider_value,
                if checkbox_checked { "✓" } else { "✗" }
            ),
            Rect::new(30.0, 360.0, 380.0, 130.0),
        );

        // Finalizar frame y renderizar
        gui.end_frame();
        backend.render(&mut gui);
    }

    println!("\n✅ Demo finalizado");
}
