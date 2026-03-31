// Demo Input Map - Sistema standalone
// Ejecutar: cargo run --bin demo_input_map_standalone
// Tamaño: 1280x720

use rydit_gfx::{RyditGfx, ColorRydit, Key};
use std::collections::HashMap;

/// Input Map simple - Mapeo de teclas a acciones
struct InputMap {
    combinaciones: HashMap<String, String>,
    teclas_presionadas: HashMap<String, bool>,
}

impl InputMap {
    fn new() -> Self {
        let mut map = Self {
            combinaciones: HashMap::new(),
            teclas_presionadas: HashMap::new(),
        };

        // Mapeo por defecto
        map.bind("w", "mover_arriba");
        map.bind("a", "mover_izquierda");
        map.bind("s", "mover_abajo");
        map.bind("d", "mover_derecha");
        map.bind("space", "saltar");
        map.bind("r", "reset");
        map.bind("escape", "salir");
        
        // Termux VolUP combos
        map.bind("volup_w", "arrow_up");
        map.bind("volup_a", "arrow_left");
        map.bind("volup_s", "arrow_down");
        map.bind("volup_d", "arrow_right");

        map
    }

    fn bind(&mut self, tecla: &str, accion: &str) {
        self.combinaciones.insert(tecla.to_string(), accion.to_string());
    }

    fn press(&mut self, tecla: &str) {
        self.teclas_presionadas.insert(tecla.to_string(), true);
    }

    fn release(&mut self, tecla: &str) {
        self.teclas_presionadas.insert(tecla.to_string(), false);
    }

    fn is_pressed(&self, tecla: &str) -> bool {
        *self.teclas_presionadas.get(tecla).unwrap_or(&false)
    }

    fn is_action_pressed(&self, accion: &str) -> bool {
        // Buscar teclas que mapean a esta acción
        for (tecla, mapped_accion) in &self.combinaciones {
            if mapped_accion == accion && self.is_pressed(tecla) {
                return true;
            }
        }
        false
    }

    fn get_active_actions(&self) -> Vec<String> {
        let mut acciones = Vec::new();
        for (tecla, mapped_accion) in &self.combinaciones {
            if self.is_pressed(tecla) {
                acciones.push(mapped_accion.clone());
            }
        }
        acciones
    }
}

fn main() {
    println!("🛡️ RyDit v0.10.3 - Demo Input Map Standalone");
    println!("============================================");
    println!("🎮 Sistema de Acciones Mapeadas");
    println!("🪟 Ventana: 1280x720");
    println!("============================================\n");

    let mut gfx = RyditGfx::new("Demo Input Map", 1280, 720);
    gfx.set_target_fps(60);

    let mut input = InputMap::new();
    
    let mut jugador_x: f32 = 640.0;
    let mut jugador_y: f32 = 360.0;
    let velocidad = 5.0;
    let mut color_idx = 0;
    let mut estela: Vec<(f32, f32)> = Vec::new();
    let mut frame = 0;
    let mut saltos = 0;
    let mut resets = 0;

    println!("🎮 Controles:");
    println!("   W, A, S, D = Mover");
    println!("   Space = Saltar (cambia color)");
    println!("   R = Reset");
    println!("   ESC = Salir");
    println!("============================================\n");

    let mut space_prev = false;

    loop {
        gfx.begin_draw();
        gfx.clear_background(ColorRydit::Negro);

        // Input directo
        if gfx.is_key_pressed(Key::W) {
            input.press("w");
        } else {
            input.release("w");
        }
        
        if gfx.is_key_pressed(Key::A) {
            input.press("a");
        } else {
            input.release("a");
        }
        
        if gfx.is_key_pressed(Key::S) {
            input.press("s");
        } else {
            input.release("s");
        }
        
        if gfx.is_key_pressed(Key::D) {
            input.press("d");
        } else {
            input.release("d");
        }

        let space_actual = gfx.is_key_pressed(Key::Space);
        if space_actual {
            input.press("space");
        } else {
            input.release("space");
        }

        if gfx.is_key_pressed(Key::R) {
            input.press("r");
        } else {
            input.release("r");
        }

        if gfx.is_key_pressed(Key::Escape) {
            break;
        }

        // Detectar just pressed para saltar
        if space_actual && !space_prev {
            saltos += 1;
            color_idx = (color_idx + 1) % 8;
            println!("🦘 Salto #{} - Color: {}", saltos, color_idx);
        }
        space_prev = space_actual;

        // Detectar just pressed para reset
        if gfx.is_key_pressed(Key::R) && frame % 20 == 0 {
            jugador_x = 640.0;
            jugador_y = 360.0;
            estela.clear();
            resets += 1;
            println!("🔄 Reset #{}", resets);
        }

        // Movimiento con Input Map
        if input.is_action_pressed("mover_arriba") {
            jugador_y -= velocidad;
        }
        if input.is_action_pressed("mover_abajo") {
            jugador_y += velocidad;
        }
        if input.is_action_pressed("mover_izquierda") {
            jugador_x -= velocidad;
        }
        if input.is_action_pressed("mover_derecha") {
            jugador_x += velocidad;
        }

        // Límites
        jugador_x = jugador_x.clamp(20.0, 1260.0);
        jugador_y = jugador_y.clamp(20.0, 700.0);

        // Estela
        if frame % 3 == 0 {
            estela.push((jugador_x, jugador_y));
            if estela.len() > 50 {
                estela.remove(0);
            }
        }

        // Dibujar estela
        for (i, &(x, y)) in estela.iter().enumerate() {
            let alpha = i as f32 / estela.len() as f32;
            let color = match color_idx {
                0 => ColorRydit::Blanco,
                1 => ColorRydit::Amarillo,
                2 => ColorRydit::Naranja,
                3 => ColorRydit::Rojo,
                4 => ColorRydit::Magenta,
                5 => ColorRydit::Cyan,
                6 => ColorRydit::Azul,
                _ => ColorRydit::Verde,
            };
            gfx.draw_circle(x as i32, y as i32, (6.0 * alpha) as i32, color);
        }

        // Jugador
        let color = match color_idx {
            0 => ColorRydit::Blanco,
            1 => ColorRydit::Amarillo,
            2 => ColorRydit::Naranja,
            3 => ColorRydit::Rojo,
            4 => ColorRydit::Magenta,
            5 => ColorRydit::Cyan,
            6 => ColorRydit::Azul,
            _ => ColorRydit::Verde,
        };
        gfx.draw_circle(jugador_x as i32, jugador_y as i32, 20, color);

        // UI
        let fps = gfx.get_fps();
        gfx.draw_text("=== 🎮 Demo Input Map ===", 20, 30, 28, ColorRydit::Blanco);
        gfx.draw_text(&format!("FPS: {}", fps), 20, 70, 22, ColorRydit::Verde);
        gfx.draw_text(&format!("Pos: ({:.0}, {:.0})", jugador_x, jugador_y), 20, 100, 18, ColorRydit::Cyan);
        gfx.draw_text(&format!("Saltos: {} | Resets: {}", saltos, resets), 20, 130, 16, ColorRydit::Magenta);
        
        let acciones = input.get_active_actions();
        gfx.draw_text(&format!("Acciones: {:?}", acciones), 20, 160, 16, ColorRydit::Amarillo);
        
        gfx.draw_text("W,A,S,D=Mover | Space=Saltar | R=Reset | ESC=Salir", 20, 690, 16, ColorRydit::Gris);

        gfx.end_draw();
        frame += 1;
    }

    println!("\n✅ Demo: {} frames", frame);
}
