// crates/rydit-rs/src/bin/test_sdl2_ttf.rs
// Test de SDL2_ttf - Renderizado de texto real
// v0.11.0 - SDL2_ttf FFI

use rydit_gfx::backend_sdl2::Sdl2Backend;

fn main() {
    println!("[TEST SDL2_TTF]: Iniciando...");

    // Crear backend SDL2
    let mut backend = match Sdl2Backend::new("Test SDL2_ttf", 800, 600) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[TEST SDL2_TTF]: Error creando backend: {}", e);
            return;
        }
    };

    // Intentar cargar una fuente del sistema
    // En Termux-X11, las fuentes están en /usr/share/fonts/
    let font_paths = [
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/DroidSans.ttf",
        "/system/fonts/DroidSans.ttf",
        "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
    ];

    let font_loaded = font_paths.iter().find_map(|path| {
        if std::path::Path::new(path).exists() {
            println!("[TEST SDL2_TTF]: Fuente encontrada: {}", path);
            match backend.load_font(path, 24) {
                Ok(_) => Some(path),
                Err(e) => {
                    eprintln!("[TEST SDL2_TTF]: Error cargando fuente: {}", e);
                    None
                }
            }
        } else {
            None
        }
    });

    if font_loaded.is_none() {
        println!("[TEST SDL2_TTF]: No se encontró ninguna fuente, usando fallback");
    }

    println!("[TEST SDL2_TTF]: Comenzando game loop...");

    // Game loop
    let mut frame_count = 0;
    let mut running = true;

    while running {
        // Procesar eventos - esto actualiza backend.input
        if backend.procesar_eventos() {
            running = false;  // should_close = true
        }

        // Clear
        backend.canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 20, 30));
        backend.canvas.clear();

        // Dibujar texto
        if font_loaded.is_some() {
            // Texto con SDL2_ttf
            backend.draw_text("¡SDL2_ttf FUNCIONA!", 200, 100, 32, 255, 255, 255);
            backend.draw_text("RyDit v0.11.0", 280, 150, 24, 0, 255, 128);
            backend.draw_text("FPS: 60 (vsync)", 300, 200, 20, 128, 128, 255);
        } else {
            // Fallback (rectángulos)
            backend.draw_text("SDL2_ttf", 300, 100, 24, 255, 255, 255);
        }

        // Dibujar información
        backend.draw_rect(50, 500, 700, 2, 100, 100, 100);  // Línea separadora
        backend.draw_text("Presiona ESC para salir", 280, 520, 16, 200, 200, 200);
        backend.draw_text(&format!("Frame: {}", frame_count), 50, 520, 16, 150, 150, 150);

        frame_count += 1;

        // Presentar (vsync se encarga del timing)
    }

    println!("[TEST SDL2_TTF]: Cerrando... ({} frames)", frame_count);
}
