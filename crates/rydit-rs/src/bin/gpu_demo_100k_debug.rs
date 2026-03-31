// Demo GPU 100K con Debug Extendido
// Ejecutar: cargo run --bin gpu_demo_100k_debug
// Tamaño: 1280x720 + logs detallados

use rydit_gfx::gpu_instancing::{GPUInstancer, ParticleData};
use rydit_gfx::{RyditGfx, ColorRydit, Key};
use gl;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("🛡️ RyDit v0.10.2 - GPU Instancing 100K DEBUG");
    println!("==============================================");
    
    // Debug: Variables de entorno
    println!("\n📋 Variables de entorno:");
    println!("   DISPLAY={}", env::var("DISPLAY").unwrap_or_else(|_| "no definido".to_string()));
    println!("   MESA_LOADER_DRIVER_OVERRIDE={}", env::var("MESA_LOADER_DRIVER_OVERRIDE").unwrap_or_else(|_| "no definido".to_string()));
    println!("   DRI3={}", env::var("DRI3").unwrap_or_else(|_| "no definido".to_string()));
    println!("   Working Directory: {:?}", env::current_dir().unwrap());
    
    // Debug: Verificar shaders
    println!("\n🔍 Verificando shaders...");
    let mut base_path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    base_path.push("crates/rydit-gfx/shaders");
    
    let vertex_path = base_path.join("vertex.glsl");
    let fragment_path = base_path.join("fragment.glsl");
    
    println!("   Vertex shader: {:?}", vertex_path);
    println!("   Fragment shader: {:?}", fragment_path);
    
    if !vertex_path.exists() {
        println!("   ❌ ERROR: vertex.glsl no existe!");
        return;
    } else {
        println!("   ✅ vertex.glsl existe");
    }
    
    if !fragment_path.exists() {
        println!("   ❌ ERROR: fragment.glsl no existe!");
        return;
    } else {
        println!("   ✅ fragment.glsl existe");
    }

    // Crear ventana HD (1280x720)
    println!("\n🪟 Creando ventana 1280x720...");
    let mut gfx = RyditGfx::new("RyDit v0.10.2 - GPU 100K DEBUG", 1280, 720);
    gfx.set_target_fps(60);
    println!("   ✅ Ventana creada");

    // Inicializar OpenGL
    println!("\n🎨 Inicializando OpenGL...");
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    println!("   ✅ OpenGL inicializado");

    // Crear GPU Instancer
    println!("\n🚀 Creando GPU Instancer...");
    let mut gpu = GPUInstancer::new();
    println!("   ✅ GPU Instancer creado");

    // Cargar shaders
    println!("\n📜 Cargando shaders...");
    match gpu.load_shaders(
        vertex_path.to_str().unwrap(),
        fragment_path.to_str().unwrap(),
    ) {
        Ok(_) => println!("   ✅ Shaders cargados exitosamente"),
        Err(e) => {
            println!("   ❌ ERROR cargando shaders: {}", e);
            println!("   Verifica que los shaders sean válidos");
            return;
        }
    }

    // Configurar proyección para 1280x720
    gpu.set_projection(1280.0, 720.0);
    gpu.set_camera(0.0, 0.0);
    println!("   ✅ Proyección configurada: 1280x720");

    // Crear 100K partículas
    println!("\n⚛️  Creando 100,000 partículas...");
    let mut particles: Vec<ParticleData> = Vec::with_capacity(100000);

    for i in 0..100000 {
        let x = ((i % 1000) as f32) * 1.28 + 50.0;
        let y = ((i / 1000) as f32) * 0.72 + 50.0;

        // Colores variados (RGB)
        let r = ((i % 255) as f32) / 255.0;
        let g = (((i / 255) % 255) as f32) / 255.0;
        let b = (((i / 65025) % 255) as f32) / 255.0;

        particles.push(ParticleData::new(x, y, 6.0, r, g, b, 0.8));
    }

    println!("   ✅ {} partículas creadas", particles.len());

    // Subir partículas a GPU
    println!("\n📤 Subiendo partículas a GPU...");
    gpu.set_particles(&particles);
    println!("   ✅ Partículas en GPU");

    // Variables de control
    let mut frame = 0;
    let mut last_fps = 0;
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    println!("\n🎮 Controles:");
    println!("   W, A, S, D = Mover cámara");
    println!("   R = Reiniciar");
    println!("   ESC = Salir");
    println!("   + = Más partículas (hasta 100K)");
    println!("   - = Menos partículas");
    println!("==============================================\n");

    // Game loop
    while !gfx.should_close() {
        gfx.begin_draw();
        gfx.clear_background(ColorRydit::Negro);

        // Input
        if gfx.is_key_pressed(Key::Escape) {
            println!("\n👋 Saliendo...");
            break;
        }

        if gfx.is_key_pressed(Key::R) {
            offset_x = 0.0;
            offset_y = 0.0;
            gpu.set_camera(0.0, 0.0);
            println!("   🔄 Cámara reiniciada");
        }

        // Mover cámara
        let speed = 10.0;
        if gfx.is_key_down(Key::D) { offset_x += speed; }
        if gfx.is_key_down(Key::A) { offset_x -= speed; }
        if gfx.is_key_down(Key::S) { offset_y += speed; }
        if gfx.is_key_down(Key::W) { offset_y -= speed; }

        gpu.set_camera(offset_x, offset_y);

        // Actualizar partículas (animación)
        if frame % 60 == 0 {
            for i in 0..particles.len() {
                particles[i].offset[1] += 1.0;
                if particles[i].offset[1] > 720.0 {
                    particles[i].offset[1] = 50.0;
                }
            }
            gpu.set_particles(&particles);
        }

        // Renderizar con GPU Instancing
        gpu.draw();

        // Debug UI
        let current_fps = gfx.get_fps();
        if frame % 60 == 0 && current_fps != last_fps {
            last_fps = current_fps;
            println!("📊 FPS: {} | Partículas: {} | Cámara: ({}, {})", 
                     current_fps, particles.len(), offset_x, offset_y);
        }

        // UI en pantalla
        gfx.draw_text("RyDit v0.10.2 - GPU Instancing 100K", 10, 25, 24, ColorRydit::Blanco);
        gfx.draw_text(&format!("Partículas: {}", particles.len()), 10, 55, 18, ColorRydit::Verde);
        gfx.draw_text(&format!("FPS: {}", current_fps), 10, 80, 18, ColorRydit::Cyan);
        gfx.draw_text(&format!("Cámara: ({:.1}, {:.1})", offset_x, offset_y), 10, 105, 16, ColorRydit::Gris);
        gfx.draw_text("W,A,S,D=Mover | R=Reiniciar | ESC=Salir", 10, 690, 14, ColorRydit::Gris);

        gfx.end_draw();

        frame += 1;
    }

    println!("\n✅ Demo completado: {} frames totales", frame);
    println!("🛡️ GPU Instancing: {} partículas @ {} FPS", particles.len(), last_fps);
}
